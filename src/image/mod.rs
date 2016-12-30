mod error;

use gdk_pixbuf::{Pixbuf, PixbufLoader, InterpType};
// use glib;
// use gtk;
use config;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn get_new_dimensions((width, height): (i32, i32)) -> (i32, i32) {
    let new_height = (height * config::MAX_WIDTH) / width;

    if new_height <= config::MAX_HEIGHT {
        (config::MAX_WIDTH, new_height)
    } else {
        ((width * config::MAX_HEIGHT) / height, config::MAX_HEIGHT)
    }
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Pixbuf, error::Error> {
    // let path = path.as_ref();

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(error::Error::IOError(e))
        }
    };

    let mut content = Vec::new();
    let _ = file.read_to_end(&mut content);

    let loader = PixbufLoader::new();
    loader.loader_write(&content).unwrap();

    let pixbuf = match loader.get_pixbuf() {
        Some(pixbuf) => {
            loader.close().unwrap();
            pixbuf
        }
        None => {
            loader.close().unwrap();
            return Err(error::Error::NoPixbuf);
        }
    };
    let (w, h) = get_new_dimensions((pixbuf.get_width(), pixbuf.get_height()));
    let pixbuf = pixbuf.scale_simple(w, h, InterpType::Hyper).unwrap();
    Ok(pixbuf)
}

