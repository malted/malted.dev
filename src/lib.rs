use std::collections::VecDeque;

pub struct ExtendableIterator {
    items: Vec<char>,
    current: usize,
    inserted: VecDeque<char>,
}

impl ExtendableIterator {
    pub fn new(items: Vec<char>) -> Self {
        ExtendableIterator {
            items,
            current: 0,
            inserted: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: char) {
        self.inserted.push_back(item);
    }

    pub fn push_str(&mut self, item: &str) {
        item.chars().for_each(|c| self.inserted.push_back(c));
    }
}

impl Iterator for ExtendableIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.inserted.pop_front() {
            Some(item)
        } else if self.current < self.items.len() {
            let item = self.items[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}
