use gtk;
use state::State;

pub struct ProgressBar {
    widget: gtk::ProgressBar,
}


impl ProgressBar {
    pub fn new() -> Self {
        let progress_bar = ProgressBar {
            widget: gtk::ProgressBar::new(),

        };
        progress_bar.widget.set_show_text(true);
        progress_bar
    }
    pub fn update(&self, state: &State) {
        let compared = state.get_compared_count();
        let compares_left = state.get_approx_sorts_remaining();
        let compares_total = compared + compares_left;

        let fraction = if compares_total == 0 {
            0.
        } else {
            (compared as f64) / (compares_total as f64)
        };

        println!("{}", fraction);

        self.widget.set_fraction(fraction);
        self.widget.set_text(Some(&format!("Comparing {} out of {}", compared, compares_total)));
    }
    pub fn get_gtk_progressbar(&self) -> &gtk::ProgressBar {
        return &self.widget
    }
}
