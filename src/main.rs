// NOTE: Use binary insertion sort (it has a low amount of comparisons)
// NOTE: GTK_DEBUG=interactive,layout to debug layout

mod image;
mod ui;
mod views;
mod state;

extern crate gtk;
extern crate gdk_pixbuf;
extern crate glib;

use gtk::{Window, WindowType};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    let style_context = window.get_style_context().unwrap();
    let css_style = gtk::CssProvider::new();
    css_style.load_from_data(CSS).unwrap();
    style_context.add_provider(&css_style, 1000000);


    window.set_title("Image Ranker");
    window.set_default_size(350, 70);

    let state = Rc::new(RefCell::new(state::State::new()));
    use std::path::PathBuf;

    state.borrow_mut().add_files(vec![
        PathBuf::from("/home/bjorn/projects/imgsort/images/10.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/9.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/3.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/5.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/8.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/7.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/2.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/6.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/4.jpg"),
        PathBuf::from("/home/bjorn/projects/imgsort/images/1.jpg"),
    ]);
    let view: Rc<RefCell<Option<views::Main>>> = Rc::new(RefCell::new(None));
    let on_files = {
        let state = state.clone();
        let view = view.clone();
        move |files| {
            state.borrow_mut().add_files(files);
            view.borrow_mut().as_mut().unwrap().update_state(&state.borrow());
        }
    };

    {
        let mut view = view.borrow_mut();
        *view = Some(views::Main::new(&window, &state.borrow(), on_files));
    }

    window.add(view.borrow().as_ref().unwrap().get_gtk_box());

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
