mod error;

const ERROR_IMAGE: &'static [u8] = include_bytes!("../../images/error.jpg");
const PLACEHOLDER_IMAGE: &'static [u8] = include_bytes!("../../images/placeholder.jpg");

use gdk_pixbuf::{Pixbuf, PixbufLoader, InterpType};
// use glib;
// use gtk;
// use config;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use gtk;

pub struct Image {
    pixbuf: Pixbuf
}

impl Image {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Image, error::Error> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(error::Error::IOError(e))
            }
        };
        Image::from_reader(file)
    }
    pub fn from_reader<R: Read>(mut r: R) -> Result<Image,error::Error> {
        let mut content = Vec::new();
        r.read_to_end(&mut content).unwrap();
        Image::from_bytes(&content)
    }
    pub fn placeholder() -> Image {
        Image::from_bytes(PLACEHOLDER_IMAGE).unwrap()
    }
    pub fn error() -> Image {
        Image::from_bytes(ERROR_IMAGE).unwrap()
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Image,error::Error> {
        let loader = PixbufLoader::new();
        loader.loader_write(bytes).unwrap();

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
        Ok(Image { pixbuf: pixbuf })
    }

    pub fn resize_max(&self, max_width: i32, max_height: i32) -> Image {
        let height = self.pixbuf.get_height();
        let width = self.pixbuf.get_width();


        let new_height = (height * max_width) / width;

        let (new_width, new_height) = if new_height <= max_height {
            (max_width, new_height)
        } else {
            ((width * max_height) / height, max_height)
        };

        self.resize(new_width, new_height)
    }
    pub fn resize(&self, width: i32, height: i32) -> Image {
        Image {
            pixbuf: self.pixbuf.scale_simple(width, height, InterpType::Hyper).unwrap()
        }
    }
    pub fn update_gtk_image(&self, gtk_image: &gtk::Image) {
        gtk_image.set_from_pixbuf(Some(self.as_ref()))
    }
}
// use std::ops::Deref;
impl AsRef<Pixbuf> for Image {
    fn as_ref(&self) -> &Pixbuf {
        return &self.pixbuf
    }
}
