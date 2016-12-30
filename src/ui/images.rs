use gtk;
use gtk::prelude::*;
use image;

use std::path::Path;

pub struct Images {
    widget: gtk::Box,
    image0: gtk::Image,
    image1: gtk::Image,
}

type ImagePair<'a> = Option<(&'a Path, &'a Path)>;

impl Images {
    pub fn new_from_pair(pair: ImagePair) -> Images {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        b.set_valign(gtk::Align::Start);
        b.set_homogeneous(true);

        let image0 = gtk::Image::new();
        let image1 = gtk::Image::new();
        b.add(&image0);
        b.add(&image1);

        let images = Images {
            widget: b,
            image0: image0,
            image1: image1,
        };
        images.update_pair(pair);
        images
    }
    pub fn update_pair(&self, pair: ImagePair) {
        match pair {
            None => {
                let pixbuf = image::from_path("./images/placeholder.jpg").unwrap();
                self.image0.set_from_pixbuf(Some(&pixbuf));
                self.image1.set_from_pixbuf(Some(&pixbuf));
            }
            Some((path0, path1)) => {
                match image::from_path(path0) {
                    Ok(pixbuf) => self.image0.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg").unwrap();
                        self.image0.set_from_pixbuf(Some(&err_pixbuf))
                    }
                }
                match image::from_path(path1) {
                    Ok(pixbuf) => self.image1.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg").unwrap();
                        self.image1.set_from_pixbuf(Some(&err_pixbuf))
                    }
                }
            }
        }

    }
    pub fn get_gtk_box(&self) -> &gtk::Box {
        &self.widget
    }
}