use gtk;
use gtk::prelude::*;

use ui;

pub struct Main {
    parent_window: gtk::Window
}

impl Main {
    pub fn new(parent_window: &gtk::Window) -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
        b.set_valign(gtk::Align::Start);
        b.set_homogeneous(false);

        let menubar = ui::MenuBar::new(parent_window, |x| println!("{:?}", x));
        b.add(menubar.get_menubar());

        let images = ui::Images::new();
        b.add(&images);

        return b
    }
}
