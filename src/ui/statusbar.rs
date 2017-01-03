use gtk;
use state::State;

pub struct StatusBar {
    widget: gtk::Statusbar,
    context_id: u32
}


impl StatusBar {
    pub fn new() -> Self {
        let statusbar = gtk::Statusbar::new();
        let context_id = statusbar.get_context_id("Global Context");
        StatusBar {
            widget: gtk::Statusbar::new(),
            context_id: context_id

        }
    }
    pub fn update(&self, state: &State) {
        let sorted = state.get_sorted().len();
        let unsorted = state.get_unsorted().len();
        let total = sorted + unsorted;

        let s = format!("Sorted {} images out of {}", sorted, total);

        self.widget.push(self.context_id, &s);
    }
    pub fn get_gtk_statusbar(&self) -> &gtk::Statusbar {
        return &self.widget
    }
}
