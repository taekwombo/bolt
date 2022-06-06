use super::command::Command;
use super::highlighter::QueryHighlighter;

pub struct TextArea {
    pub width: u16,
    pub height: u16,
    pub command: Command,
    highlighter: QueryHighlighter,
}

impl TextArea {
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().expect("terminal size to be readable");

        Self {
            width,
            height,
            highlighter: QueryHighlighter::new(),
            command: Command::new(),
        }
    }

    pub fn update_command<F: FnOnce(&mut Command) -> ()>(&mut self, update: F) -> () {
        self.clear();

        update(&mut self.command);
        
        self.print();
    }

    pub fn print(&mut self) -> () {
        let query = self.command.get_buffer().as_str();
        let chunks = self.highlighter.parse(query, self.width - 2);

        let mut first = true;
        for line in chunks.get_lines() {
            if first {
                first = false;
                print!("> {}", line);
            } else {
                print!("\r\n  {}", line);
            }
        }
    }

    pub fn clear(&mut self) -> () {
        let query = self.command.get_buffer().as_str();
        let chunks = self.highlighter.parse(query, self.width - 2);
        let lines = chunks.get_lines().len();

        // If there is more than 1 visual line, then scroll cursor.
        if lines > 1 {
            // Scroll `lines` minus 1 - there is no need to scroll the cursor line.
            print!("{}", termion::scroll::Down((lines - 1) as u16));
        }

        print!("{}{}", termion::cursor::Goto(1, self.height), termion::clear::AfterCursor);
    }
}
