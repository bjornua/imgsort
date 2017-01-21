use std::rc::Rc;
use std::cell::RefCell;
use ui;
use state::State;
use event::Event;

pub struct View {
    ui: ui::UI
}

impl View {
    pub fn new<F: Fn(Event)>(state: &State, f: F) -> Self {
        View {
            ui: ui::UI::new()
        }
    }
    pub fn update(&mut self, state: &State) {
        &self.ui.update(&state);
    }
}
