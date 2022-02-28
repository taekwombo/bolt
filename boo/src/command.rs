use std::collections::VecDeque;

pub struct Command {
    history: VecDeque<String>,
    buffer: String,
    index: usize,
}

impl Command {
    pub fn new() -> Self {
        Self {
            index: 0,
            buffer: String::new(),
            history: VecDeque::with_capacity(10),
        }
    }

    #[inline]
    fn replace_buffer(&mut self, index: usize) -> () {
        self.buffer.clear();
        self.buffer.push_str(&self.history[index]);
    }
    
    pub fn add(&mut self) -> () {
        self.history.push_front(self.buffer.clone());
        self.index = 0;
    }

    pub fn get_buffer(&self) -> &String {
        &self.buffer
    }

    pub fn get_buffer_mut(&mut self) -> &mut String {
        &mut self.buffer
    }

    pub fn use_prev_command(&mut self) -> () {
        let history_len = self.history.len();

        match history_len {
            0 => (),
            1 => {
                if self.buffer != self.history[0] {
                    self.replace_buffer(0);
                }
            },
            len => {
                let index = (len + self.index - 1) % len;

                self.replace_buffer(index);
                self.index = index;
            },
        }
    }

    pub fn use_next_command(&mut self) -> () {
        let history_len = self.history.len();

        match history_len {
            0 => (),
            1 => {
                if self.buffer != self.history[0] {
                    self.replace_buffer(0);
                }
            },
            len => {
                let index = (self.index + 1) % len;

                self.replace_buffer(index);
                self.index = index;
            },
        }
    }

    pub fn get_lines<'a>(&'a self, width: u16) -> Vec<&'a str> {
        let line_iterator = self.buffer.split('\n');
        let mut lines: Vec<&str> = Vec::with_capacity(line_iterator.size_hint().0);

        for line in line_iterator {
            if line.len() > width as usize {
                let splits = line.len() / width as usize;

                for idx in 0..splits {
                    lines.push(&line[(idx * width as usize)..((idx + 1) * width as usize)]);
                }
            } else {
                lines.push(line);
            }
        }

        lines
    }
}
