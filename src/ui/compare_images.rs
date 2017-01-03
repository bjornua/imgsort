use gtk;
use gtk::prelude::*;
use image;

use std::path::Path;

pub struct CompareImages {
    widget: gtk::Box,
    image0: gtk::Image,
    image1: gtk::Image,
    dimensions: (i32, i32)
}

type ImagePair<'a> = Option<(&'a Path, &'a Path)>;

impl CompareImages {
    pub fn new_from_pair(pair: ImagePair, dimensions: (i32, i32)) -> Self {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        b.set_valign(gtk::Align::Start);
        b.set_homogeneous(true);

        let image0 = gtk::Image::new();
        let image1 = gtk::Image::new();
        b.add(&image0);
        b.add(&image1);

        let images = CompareImages {
            widget: b,
            image0: image0,
            image1: image1,
            dimensions: dimensions,
        };
        images.update_pair(pair);
        images
    }
    pub fn update_pair(&self, pair: ImagePair) {
        match pair {
            None => {
                let pixbuf = image::from_path("./images/placeholder.jpg", self.dimensions).unwrap();
                self.image0.set_from_pixbuf(Some(&pixbuf));
                self.image1.set_from_pixbuf(Some(&pixbuf));
            }
            Some((path0, path1)) => {
                match image::from_path(path0, self.dimensions) {
                    Ok(pixbuf) => self.image0.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg", self.dimensions).unwrap();
                        self.image0.set_from_pixbuf(Some(&err_pixbuf))
                    }
                }
                match image::from_path(path1, self.dimensions) {
                    Ok(pixbuf) => self.image1.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg", self.dimensions).unwrap();
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