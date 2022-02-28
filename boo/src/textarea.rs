use super::command::Command;

pub struct TextArea {
    pub width: u16,
    pub height: u16,
    pub command: Command,
}

impl TextArea {
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().expect("terminal size to be readable");

        Self {
            width,
            height,
            command: Command::new(),
        }
    }

    pub fn update_command<F: FnOnce(&mut Command) -> ()>(&mut self, update: F) -> () {
        self.clear();

        update(&mut self.command);
        
        self.print();
    }

    pub fn print(&self) -> () {
        let lines = self.command.get_lines(self.width);

        let mut first = true;
        for line in lines {
            if first {
                first = false;
                print!("> {}", line);
            } else {
                print!("\r\n  {}", line);
            }
        }
    }

    pub fn clear(&self) -> () {
        let line_count = self.command.get_lines(self.width).len() - 1;

        if line_count > 0 {
            print!("{}", termion::scroll::Down(line_count as u16));
        }

        print!("{}{}", termion::cursor::Goto(1, self.height), termion::clear::AfterCursor);
    }
}
