use gtk;
use gtk::prelude::*;
use state::State;

use ui;

pub struct Main {
    widget: gtk::Box,
    images: ui::Images,
}

use std::path::PathBuf;

impl Main {
    pub fn new<F: Fn(Vec<PathBuf>) + 'static>(parent_window: &gtk::Window,
                                              state: &State,
                                              on_add_files: F)
                                              -> Self {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
        b.set_valign(gtk::Align::Start);
        b.set_homogeneous(false);

        let menubar = ui::MenuBar::new(parent_window, on_add_files);

        b.add(menubar.get_gtk_menubar());

        let images = ui::Images::new_from_pair(state.get_pair());
        b.add(images.get_gtk_box());

        return Main {
            widget: b,
            images: images,
        };
    }
    pub fn update_state(&self, state: &State) {
        self.images.update_pair(state.get_pair())
    }
    pub fn get_gtk_box(&self) -> &gtk::Box {
        &self.widget
    }
}
