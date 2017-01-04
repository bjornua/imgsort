use gtk;
use gtk::prelude::*;
use image::Image;

use std::path::Path;


pub struct CompareImages {
    widget: gtk::Grid,
    image0: gtk::Image,
    image1: gtk::Image,
}

const MAX_WIDTH: i32 = 512;
const MAX_HEIGHT: i32 = 512;

fn resize(img: Image) -> Image {
    img.resize_max(MAX_WIDTH, MAX_HEIGHT)
}
use std::borrow::Cow;
fn load_image(path: &Path) -> (Cow<str>, Image) {
    match Image::from_path(path) {
        Ok(image) => (path.to_string_lossy(), resize(image)),
        Err(_) => (Cow::Borrowed(""), resize(Image::error()))
    }
}

type ImagePair<'a> = (&'a Path, &'a Path);

use std::cmp::Ordering;
impl CompareImages {
    pub fn new_from_pair<F: Fn(Ordering) + 'static>(pair: ImagePair, on_compare: F) -> Self {
        let grid = gtk::Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(20);
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_valign(gtk::Align::Fill);
        grid.set_halign(gtk::Align::Fill);
        grid.set_column_homogeneous(true);

        let image0 = gtk::Image::new();
        let image1 = gtk::Image::new();

        image0.set_hexpand(true);
        image1.set_hexpand(true);

        image0.set_vexpand(true);
        image1.set_vexpand(true);

        grid.add(&image0);
        grid.add(&image1);

        use std::rc::Rc;
        let on_compare = Rc::new(on_compare);
        {
            let button = gtk::Button::new_with_mnemonic("Image _1 is better");
            let on_compare = on_compare.clone();
            button.connect_clicked(move |_| {on_compare(Ordering::Greater) })
            ;
            grid.attach_next_to(&button, Some(&image0), gtk::PositionType::Bottom, 1, 1);
        };
        {
            let button = gtk::Button::new_with_mnemonic("Image _2 is better");
            grid.attach_next_to(&button, Some(&image1), gtk::PositionType::Bottom, 1, 1);
            button.connect_clicked(move |_| on_compare(Ordering::Less));

        };
        let images = CompareImages {
            widget: grid,
            image0: image0,
            image1: image1,
        };
        images.update_pair(pair);
        images
    }
    pub fn update_pair(&self, (path0, path1): ImagePair) {
        let (text0, image0) = load_image(path0);
        let (text1, image1) = load_image(path1);
        image0.update_gtk_image(&self.image0);
        image1.update_gtk_image(&self.image1);
    }
    pub fn get_gtk_box(&self) -> &gtk::Grid {
        &self.widget
    }
}
