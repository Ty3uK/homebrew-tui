use std::{
    process::Child,
    sync::{Arc, Mutex},
};

use crate::{homebrew::Package, ui::APP_MARGIN};

#[derive(Debug, PartialEq)]
pub enum AppScreen {
    Init,
    Main,
}

#[derive(Debug)]
pub struct AppPackagesUpdate {
    pub progress: Vec<u8>,
    pub process: Child,
}

impl AppPackagesUpdate {
    pub fn new(progress: Vec<u8>, process: Child) -> Self {
        Self { progress, process }
    }
}

#[derive(Debug)]
pub struct AppPackages {
    pub items: Vec<Package>,
    pub scroll: u16,
}

impl AppPackages {
    pub fn new(items: Vec<Package>) -> Self {
        Self { items, scroll: 0 }
    }

    pub fn scroll_down(&mut self, terminal_height: u16) {
        let packages_count = self.items.len() as u16;
        if self.scroll + terminal_height <= packages_count + (APP_MARGIN / 2) {
            self.scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_page_down(&mut self, terminal_height: u16) {
        let packages_count = self.items.len() as u16;
        let new_scroll = self.scroll + terminal_height;
        let max_size = packages_count - terminal_height + APP_MARGIN;

        self.scroll = if new_scroll <= max_size {
            new_scroll
        } else {
            max_size
        };
    }

    pub fn scroll_page_up(&mut self, terminal_height: u16) {
        let new_scroll = (self.scroll as i16) - (terminal_height as i16);

        self.scroll = if new_scroll >= 0 {
            new_scroll as u16
        } else {
            0
        };
    }

    pub fn scroll_end(&mut self, terminal_height: u16) {
        let packages_count = self.items.len() as u16;
        let max_size = packages_count - terminal_height + APP_MARGIN;
        self.scroll = max_size;
    }

    pub fn scroll_start(&mut self) {
        self.scroll = 0;
    }
}

#[derive(Debug)]
pub struct App {
    pub screen: AppScreen,
    pub packages: Option<AppPackages>,
    pub update: Option<AppPackagesUpdate>,
    pub quitting: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            packages: None,
            screen: AppScreen::Init,
            update: None,
            quitting: false,
        }
    }
}

pub type AppShared = Arc<Mutex<App>>;
