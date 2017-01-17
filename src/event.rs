use std::path::PathBuf;
use std::cmp::Ordering;


enum Event {
    AddUnsortedFiles(Vec<PathBuf>),
    Compare(Ordering),
}
