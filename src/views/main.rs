use gtk;
use gtk::prelude::*;

use ui;

pub fn init() -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
    b.set_valign(gtk::Align::Start);
    b.set_homogeneous(false);

    let toolbar = ui::menubar::init();
    b.add(&toolbar);

    let images = ui::images::init();
    b.add(&images);

    return b



}
