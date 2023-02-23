use ansi_to_tui::IntoText;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    text::Spans,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, AppScreen};

pub const APP_MARGIN: u16 = 2;

pub fn draw_ui<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    match app.screen {
        AppScreen::Init => {}
        AppScreen::Main => {
            draw_packages_screen(f, app);
        }
    }

    if app.update.is_some() {
        draw_upgrade_popup(f, app);
    }
}

pub fn draw_packages_screen<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    if let Some(packages) = &app.packages {
        let items: Vec<Spans> = packages
            .items
            .iter()
            .map(|package| {
                Spans::from(format!(
                    "{} - {} - {}",
                    package.name, package.desc, package.installed[0].version
                ))
            })
            .collect();
        let block = Block::default().title("Packages").borders(Borders::ALL);
        let p = Paragraph::new(items)
            .block(block)
            .scroll((packages.scroll, 0));

        f.render_widget(p, f.size());
    }
}

pub fn draw_upgrade_popup<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let outer_chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(8)
        .split(f.size());
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(outer_chunks[0]);
    let inner_chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .vertical_margin(2)
        .horizontal_margin(2)
        .split(chunks[0]);
    let block = Block::default()
        .title("Upgrading packages")
        .borders(Borders::ALL);

    let text = app.update.as_ref().unwrap().progress.into_text().unwrap();

    let text_height = text.height() as u16;
    let container_height = inner_chunks[0].height;
    let scroll = if text_height > container_height {
        (text_height - container_height, 0)
    } else {
        (0, 0)
    };

    let p = Paragraph::new(text).scroll(scroll);

    f.render_widget(Clear, outer_chunks[0]);
    f.render_widget(block, chunks[0]);
    f.render_widget(p, inner_chunks[0]);
}
