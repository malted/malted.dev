use std::collections::VecDeque;

struct ExtendableIterator {
    items: Vec<char>,
    current: usize,
    inserted: VecDeque<char>,
}

impl ExtendableIterator {
    fn new(items: Vec<char>) -> Self {
        ExtendableIterator {
            items,
            current: 0,
            inserted: VecDeque::new(),
        }
    }

    fn push(&mut self, item: char) {
        self.inserted.push_back(item);
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

fn main() {
    let initial_items = vec!['a', 'b', 'c'];

    let mut iter = ExtendableIterator::new(initial_items);

    while let Some(item) = iter.next() {
        print!("{}", item);

        if item == 'b' {
            iter.push('1');
            iter.push('2');
        }
    }
    println!(); // For a newline at the end
}
