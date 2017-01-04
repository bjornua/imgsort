use gtk;
use gtk::prelude::*;
use state::State;

use ui;

pub struct Main {
    widget: gtk::Box,
    images: ui::CompareImages,
    status_bar: ui::StatusBar,
    progress_bar: ui::ProgressBar,
}

use std::cmp::Ordering;
use std::path::PathBuf;


impl Main {
    pub fn new<F, G>(parent_window: &gtk::Window,
                     state: &State,
                     on_add_files: F,
                     on_compare: G)
                     -> Self
        where F: Fn(Vec<PathBuf>) + 'static,
              G: Fn(Ordering) + 'static
    {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 10);
        b.set_valign(gtk::Align::Fill);

        let menubar = ui::MenuBar::new(parent_window, on_add_files);
        b.pack_start(menubar.get_gtk_menubar(), false, false, 0);

        let grid = gtk::Grid::new();

        let images = ui::CompareImages::new_from_pair(state.get_pair(), on_compare);
        {
            let b = images.get_gtk_box();
            grid.attach(b, 0, 0, 2, 1);
        };
        b.pack_start(&grid, true, true, 0);

        let progress_bar = ui::ProgressBar::new();
        progress_bar.update(state);
        b.pack_start(progress_bar.get_gtk_progressbar(), false, false, 0);

        let status_bar = ui::StatusBar::new();
        status_bar.update(state);
        b.pack_end(status_bar.get_gtk_statusbar(), false, false, 0);

        return Main {
            widget: b,
            images: images,
            status_bar: status_bar,
            progress_bar: progress_bar,
        };
    }
    pub fn update_state(&self, state: &State) {
        self.images.update_pair(state.get_pair());
        self.status_bar.update(state);
        self.progress_bar.update(state);
    }
    pub fn get_gtk_box(&self) -> &gtk::Box {
        &self.widget
    }
}
