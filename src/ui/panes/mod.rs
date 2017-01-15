pub mod menubar;
pub mod compare_images;
pub mod statusbar;
pub mod progressbar;

pub use self::compare_images::CompareImages;
pub use self::statusbar::StatusBar;
pub use self::menubar::MenuBar;
pub use self::progressbar::ProgressBar;


use gtk;
use gtk::prelude::*;
use state::State;

use ui;

pub struct Main {
    widget: gtk::Box,
    images: ui::CompareImages,
    progress_bar: ui::ProgressBar,
}

use std::cmp::Ordering;
use std::path::PathBuf;


impl Main {
    pub fn new<F, G>(parent_window: &gtk::Window,
                     on_add_files: F,
                     on_compare: G)
                     -> Self
        where F: Fn(Vec<PathBuf>) + 'static,
              G: Fn(Ordering) + 'static
    {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 10);
        b.set_valign(gtk::Align::Fill);

        // let menubar = ui::MenuBar::new(parent_window, on_add_files);
        // b.pack_start(menubar.get_gtk_menubar(), false, false, 0);

        let images = ui::CompareImages::new(on_compare);
        b.pack_start(images.get_gtk_box(), true, true, 0);

        let progress_bar = ui::ProgressBar::new();
        b.pack_start(progress_bar.get_gtk_progressbar(), false, false, 0);

        return Main {
            widget: b,
            images: images,
            progress_bar: progress_bar,
        };
    }
    pub fn update(&self, state: &State) {
        self.images.set_pair(state.get_pair());
        self.progress_bar.update(state);
    }
    pub fn get_gtk_box(&self) -> &gtk::Box {
        &self.widget
    }
}
