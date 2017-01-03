use gtk;
use gtk::prelude::*;
use image;

use std::path::Path;

pub struct CompareImages {
    widget: gtk::Grid,
    image0: gtk::Image,
    image1: gtk::Image,
}

const DIMENSIONS: (i32, i32) = (512, 512);
type ImagePair<'a> = Option<(&'a Path, &'a Path)>;

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
    pub fn update_pair(&self, pair: ImagePair) {
        match pair {
            None => {
                let pixbuf = image::from_path("./images/placeholder.jpg", DIMENSIONS).unwrap();
                self.image0.set_from_pixbuf(Some(&pixbuf));
                self.image1.set_from_pixbuf(Some(&pixbuf));
            }
            Some((path0, path1)) => {
                match image::from_path(path0, DIMENSIONS) {
                    Ok(pixbuf) => self.image0.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg", DIMENSIONS).unwrap();
                        self.image0.set_from_pixbuf(Some(&err_pixbuf))
                    }
                }
                match image::from_path(path1, DIMENSIONS) {
                    Ok(pixbuf) => self.image1.set_from_pixbuf(Some(&pixbuf)),
                    Err(_) => {
                        let err_pixbuf = image::from_path("./images/error.jpg", DIMENSIONS).unwrap();
                        self.image1.set_from_pixbuf(Some(&err_pixbuf))
                    }
                }
            }
        }
    }
    pub fn get_gtk_box(&self) -> &gtk::Grid {
        &self.widget
    }
}
