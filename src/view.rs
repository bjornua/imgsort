use std::rc::Rc;
use std::cell::RefCell;
use ui;
use state::State;
use event::Event;

pub struct View {
    ui: ui::UI
}

type EventHandler<'a, 'b> = &'a Fn(Event, &'b mut View);

impl View {
    pub fn new(state: &State, f: EventHandler) -> Self {
        View {
            ui: ui::UI::new()
        }
    }
    fn update(&mut self, state: &State) {
        &self.ui.update(&state);
    }
}
