
use std::cmp::Ordering;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct State {
    unsorted: Vec<PathBuf>,
    sorted: Vec<PathBuf>,
    compare_begin: usize,
    compare_end: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            unsorted: Vec::new(),
            sorted: Vec::new(),
            compare_begin: 0,
            compare_end: 0,
        }
    }
    pub fn add_files(&mut self, mut files: Vec<PathBuf>) {
        if self.sorted.len() == 0 {
            if let Some(first_element) = files.pop() {
                self.sorted.push(first_element)
            }
        }
        self.compare_begin = 0;
        self.compare_end = self.sorted.len();
        self.unsorted.append(&mut files);
    }
    fn get_right_idx(&self) -> usize {
        (self.compare_end + self.compare_begin) / 2
    }
    pub fn get_pair(&self) -> Option<(&Path, &Path)> {
        let left = match self.unsorted.last() {
            Some(p) => p,
            None => return None,
        };
        let right = match self.sorted.get(self.get_right_idx()) {
            Some(p) => p.as_ref(),
            None => return None,
        };

        Some((left, right))
    }
    pub fn compare(&mut self, ord: Ordering) {
        let idx = self.get_right_idx();
        match ord {
            Ordering::Less => self.compare_end = idx,
            Ordering::Greater => self.compare_begin = idx + 1,
            Ordering::Equal => self.mark_sorted(idx),
        }
        if self.compare_end == self.compare_begin {
            let compare_begin = self.compare_begin;
            self.mark_sorted(compare_begin)
        }
    }
    fn mark_sorted(&mut self, position: usize) {
        self.sorted.insert(position, self.unsorted.pop().unwrap())
    }
    pub fn get_unsorted(&self) -> Vec<&Path> {
        self.unsorted.iter().map(|x| x.as_path()).collect()
    }
    pub fn get_sorted(&self) -> Vec<&Path> {
        self.sorted.iter().map(|x| x.as_path()).collect()
    }
}
