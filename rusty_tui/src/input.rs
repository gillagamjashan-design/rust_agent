//! Input handling and history

use std::collections::VecDeque;

pub struct InputHistory {
    history: VecDeque<String>,
    position: usize,
    max_size: usize,
}

impl InputHistory {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            position: 0,
            max_size: 100,
        }
    }
    
    pub fn add(&mut self, input: String) {
        if !input.is_empty() {
            self.history.push_front(input);
            if self.history.len() > self.max_size {
                self.history.pop_back();
            }
        }
        self.position = 0;
    }
    
    pub fn previous(&mut self) -> Option<&String> {
        if self.position < self.history.len() {
            let result = self.history.get(self.position);
            self.position += 1;
            result
        } else {
            None
        }
    }
    
    pub fn next(&mut self) -> Option<&String> {
        if self.position > 0 {
            self.position -= 1;
            self.history.get(self.position)
        } else {
            None
        }
    }
}
