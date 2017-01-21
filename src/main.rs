#![allow(dead_code,unused_imports)]
// NOTE: Use binary insertion sort (it has a low amount of comparisons)
// NOTE: GTK_DEBUG=interactive,layout to debug layout

mod image;
mod ui;
mod state;
mod event;
mod view;

extern crate gtk;
extern crate gdk_pixbuf;
extern crate glib;

use gtk::{Window, WindowType};
use gtk::prelude::*;

fn handle_event(event: event::Event, state: state::State) -> state::State {

}

use state::State;
use view::View;
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let state = State::new();

    let view = View::new(&state, event_handler);
    let event_handler = event::make_handler(state, handle_event);
}
/*
    window.set_title("Image Ranker");
    window.set_default_size(350, 70);

    let state = Rc::new(RefCell::new(state::State::new()));

    state.borrow_mut().add_files(vec![

    ]);
    let view: Rc<RefCell<Option<views::Main>>> = Rc::new(RefCell::new(None));

    fn update_view(view: &Rc<RefCell<Option<views::Main>>>, state: &Rc<RefCell<state::State>>) {
        view.borrow_mut().as_mut().unwrap().update(&state.borrow());
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
        let realview = views::Main::new(&window, on_files, on_compare);
        realview.update(&state.borrow());

        let mut view = view.borrow_mut();
        *view = Some(realview);
    };

    window.add(view.borrow().as_ref().unwrap().get_gtk_box());

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}


*/
