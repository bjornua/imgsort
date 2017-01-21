use std::path::PathBuf;
use std::cmp::Ordering;
use state;


pub enum Event {
    Initialize(state::State),
    AddUnsortedFiles(Vec<PathBuf>),
    Compare(Ordering),
}
