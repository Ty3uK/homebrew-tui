use std::{
    error::Error,
    io::{self, BufReader, Read},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use app::{App, AppPackages, AppPackagesUpdate, AppScreen, AppShared};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use homebrew::{get_installed_packages, upgrade_packages};
use keyboard_events::{get_keyboard_event, KeyboardEvent};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::draw_ui;

mod app;
mod homebrew;
mod keyboard_events;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    let app_shared = Arc::new(Mutex::new(App::new()));
    let app_shared_clone = app_shared.clone();

    std::thread::spawn(move || {
        let packages = get_installed_packages().unwrap();
        let mut app = app_shared_clone.lock().unwrap();
        app.packages = Some(AppPackages::new(packages));
        app.screen = AppScreen::Main;
    });

    let mut terminal = init_terminal()?;
    let res = run_tui(&mut terminal, &app_shared);

    reset_terminal()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    let stdout = io::stdout();

    crossterm::execute!(&stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn reset_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_tui<B>(terminal: &mut Terminal<B>, app_shared: &AppShared) -> io::Result<()>
where
    B: Backend,
{
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut prev_event: Option<Event> = None;

    loop {
        let mut app = app_shared.lock().unwrap();

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        let terminal_height = terminal.size()?.height;

        if event::poll(timeout).unwrap() {
            let event = event::read()?;
            match get_keyboard_event(&event, &prev_event) {
                // Quit and close events
                KeyboardEvent::Quit => return Ok(()),
                KeyboardEvent::Close => {
                    if let Some(update) = app.update.as_mut() {
                        update.process.kill()?;
                        app.update = None;
                    }
                }
                // Scroll events
                KeyboardEvent::ScrollDown => {
                    app.packages.as_mut().unwrap().scroll_down(terminal_height);
                }
                KeyboardEvent::ScrollUp => {
                    app.packages.as_mut().unwrap().scroll_up();
                }
                KeyboardEvent::ScrollPageDown => {
                    app.packages
                        .as_mut()
                        .unwrap()
                        .scroll_page_down(terminal_height);
                }
                KeyboardEvent::ScrollPageUp => {
                    app.packages
                        .as_mut()
                        .unwrap()
                        .scroll_page_up(terminal_height);
                }
                KeyboardEvent::ScrollEnd => {
                    app.packages.as_mut().unwrap().scroll_end(terminal_height);
                }
                KeyboardEvent::ScrollStart => {
                    app.packages.as_mut().unwrap().scroll_start();
                }
                // Screen events
                KeyboardEvent::Update => {
                    if app.update.is_none() {
                        let mut process = upgrade_packages().unwrap();

                        if let Some(stdout) = process.stdout.take() {
                            let app_shared = app_shared.clone();
                            std::thread::spawn(move || {
                                let buf_reader = BufReader::new(stdout);
                                for byte in buf_reader.bytes().flatten() {
                                    let mut app = app_shared.lock().unwrap();
                                    let update = app.update.as_mut().unwrap();
                                    update.progress.push(byte);
                                }
                            });
                        }

                        app.update = Some(AppPackagesUpdate::new(vec![], process));
                    }
                }
                // Drop the other
                KeyboardEvent::Other => {}
            };
            prev_event = Some(event);
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        terminal.draw(move |f| draw_ui(f, &app))?;
    }
}
