use std::path::PathBuf;
use std::cmp::Ordering;


pub enum Event {
    AddUnsortedFiles(Vec<PathBuf>),
    Compare(Ordering),
}
