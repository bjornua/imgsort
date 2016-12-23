// NOTE: Use binary insertion sort (it has a low amount of comparisons)
// NOTE: GTK_DEBUG=interactive,layout to debug layout

mod image;
mod ui;
mod views;

extern crate gtk;
extern crate gdk_pixbuf;
extern crate glib;


use gtk::{Window, WindowType};
use gtk::prelude::*;


fn main () {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);


    window.set_title("Image Sorter");
    window.set_default_size(350, 70);

    let view = views::main::init();
    window.add(&view);

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
