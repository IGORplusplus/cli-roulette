use std::collections::VecDeque;

#[derive(Debug)]
pub struct Logger {
    log: VecDeque<String>,
    max_size: usize,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            log: VecDeque::new(),
            max_size: 0,
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

    pub fn set_max_lines(&mut self, line_number: usize) {
        self.max_size = line_number;
    }
}    
