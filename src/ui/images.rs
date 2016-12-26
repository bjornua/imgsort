
use gtk;
use gtk::prelude::*;

use image;

pub struct Images;

impl Images {
    pub fn new() -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        b.set_valign(gtk::Align::Start);
        b.set_homogeneous(true);

        if let Ok(image) = image::from_path("./images/1.jpg") {
            b.add(&image);
        }
        if let Ok(image) = image::from_path("./images/10.jpg") {
            b.add(&image);
        }
        b
    }
}