use std::path::PathBuf;


enum Event {
    AddFiles(Vec<PathBuf>),
    Compare
}
