use std::collections::VecDeque;

#[derive(Debug)]
pub struct Logger {
    log: VecDeque<String>,
    max_size: usize,
}

impl Logger {
    pub fn new(max_size: usize) -> Self {
        Logger {
            log: VecDeque::new(),
            max_size,
        }
    }

    pub fn send_log(&mut self, message: Option<String>) {
        if let Some(msg) = message {
            if self.log.len() >= self.max_size {
                self.log.pop_front();
            }
            self.log.push_back(msg)
        }
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, String>{
        self.log.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, String>{
        self.log.iter_mut()
    }
}    

