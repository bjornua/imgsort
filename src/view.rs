use std::rc::Rc;
use std::cell::RefCell;
use ui;
use state;

struct View {
    ui: ui::UI
}

struct WrappedView (
    Rc<RefCell<View>>
);

impl View {
    fn new() -> Self {
        View {
            ui: ui::UI::new()
        }
    }
    fn update(&mut self, state: &state::State) {
        &self.ui.update(&state);
    }
}

impl WrappedView {
    fn new() -> Self {
        WrappedView(Rc::new(RefCell::new(View::new())))
    }
    fn update(&self) {

    }
}
