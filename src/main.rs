// NOTE: Use binary insertion sort (it has a low amount of comparisons)
mod image;

extern crate gtk;
extern crate gdk_pixbuf;
extern crate glib;

use gtk::prelude::*;

use gtk::{Box, Button, Window, WindowType, Orientation};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    let image = image::load(&window);

    if let Some(image) = image {
        let b = Box::new(Orientation::Horizontal, 0);
        let i = gtk::Image::new_from_pixbuf(Some(&image));
        b.add(&i);
        window.add(&b);
    }
    window.set_title("Image Sorter");
    window.set_default_size(350, 70);


    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // button.connect_clicked(|_| {
    //     println!("Clicked!");
    // });

    gtk::main();
}

