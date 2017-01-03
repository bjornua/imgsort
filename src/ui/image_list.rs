#![allow(dead_code, unused_variables, unused_imports)]
use gtk;
use gtk::prelude::*;

use std::path::Path;
use std::cmp::min;

use image;

pub struct ImageList {
    widget: gtk::Box,
}


impl ImageList {
    pub fn new(paths: &[&Path], selection: usize, dimensions: (i32, i32)) -> Self {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let selection_begin = if selection <= 2 {
            0
        } else {
            selection - 2
        };
        let selection_end = min(selection_begin + 5, paths.len());
        let selection = selection - selection_begin;

        let paths = &paths[selection_begin..selection_end];

        for (n, path) in paths.into_iter().enumerate() {
            let pixbuf = image::from_path(path, dimensions).unwrap();
            let image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
            if n == selection {
                image.set_name("image_active");
            } else {
                image.set_name("image_inactive");
            }
            b.add(&image);
        }

        ImageList {
            widget: b
        }
    }
    pub fn get_gtk_box(&self) -> &gtk::Box {
        return &self.widget
    }
}
