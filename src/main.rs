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
use std::cell::{RefCell};
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);

    window.set_title("Image Ranker");
    window.set_default_size(350, 70);

    let state = Rc::new(RefCell::new(state::State::new()));
    use std::path::PathBuf;

    state.borrow_mut().add_files(vec![

    ]);
    let view: Rc<RefCell<Option<views::Main>>> = Rc::new(RefCell::new(None));

    fn update_view(view: &Rc<RefCell<Option<views::Main>>>, state: &Rc<RefCell<state::State>>) {
        // let state = state.clone();
        // let view = view.clone();
        view.borrow_mut().as_mut().unwrap().update_state(&state.borrow());
    }

    let on_files = {
        let state = state.clone();
        let view = view.clone();
        move |files| {
            state.borrow_mut().add_files(files);
            update_view(&view, &state);
        }
    };
    let on_compare = {
        let state = state.clone();
        let view = view.clone();

        move |order| {
            state.borrow_mut().compare(order);
            update_view(&view, &state);
        }
    };
    {
        let mut view = view.borrow_mut();
        *view = Some(views::Main::new(&window, &state.borrow(), on_files, on_compare));
    };

    window.add(view.borrow().as_ref().unwrap().get_gtk_box());

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
