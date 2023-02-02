
/// Collection of chunks over source code string slice.
pub struct Chunks<'a> {
    chunks: Vec<Chunk<'a>>,
    /// Terminal width
    width: u16,
    /// Lenght of all source code slices in this collection
    source_length: usize,
}

pub struct Chunk<'a> {
    // TODO: Abstract this into a Trait impl so that theres another
    // struct that implements Chunk that represents new line character.
    pub is_new_line: bool,
    source_code: &'a str,
    style: &'static str,
}

impl<'a> Chunks<'a> {
    pub fn new(width: u16) -> Self {
        Self { width, source_length: 0, chunks: Vec::new() }
    }

    pub fn push(&mut self, chunk: Chunk<'a>) -> &mut Self {
        let width = self.width as usize;
        let previous_new_line = self.chunks.last().map_or(false, |c| c.is_new_line);
        let occupied_space = if previous_new_line { 0 } else { self.source_length };
        let space_left = width - (occupied_space % width);

        if chunk.source_len() <= space_left {
            self.source_length += chunk.source_len();
            self.chunks.push(chunk);
        } else {
            let mut pushed = 0;
            let mut chunk_to_split = chunk;

            loop {
                let space_left_split = usize::min(width - ((occupied_space + pushed) % width), chunk_to_split.source_len());
                let (left, right) = chunk_to_split.split_at(space_left_split);

                pushed += left.source_len();
                self.chunks.push(left);

                if right.source_len() == 0 {
                    break;
                }

                chunk_to_split = right;
            }

            self.source_length += pushed;
        }

        self
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        let mut line = String::new();
        let width = self.width as usize;
        let mut len: usize = 0;

        for chunk in &self.chunks {
            // TODO: see note about `is_new_line` on `Chunks` struct.
            if chunk.is_new_line {
                let line_to_append = std::mem::replace(&mut line, String::new());
                lines.push(line_to_append);
                len = 0;
                continue;
            }

            if chunk.source_len() + len > width {
                let line_to_append = std::mem::replace(&mut line, String::new());
                lines.push(line_to_append);
                len = 0;
            }

            len += chunk.source_len();
            chunk.append_to_line(&mut line);
        }

        lines.push(line);

        lines
    }
}

impl<'a> Chunk<'a> {
    /// Splits chunk at `index` into two exactly styled chunks.
    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.source_code.split_at(index);

        (
            Self { style: self.style, source_code: left, is_new_line: false, },
            Self { style: "", source_code: right, is_new_line: false, },
        )
    }

    pub fn new(source_code: &'a str, style: &'static str) -> Self {
        Self { source_code, is_new_line: false, style }
    }

    pub fn new_line() -> Self {
        Self { source_code: "", is_new_line: true, style: "" }
    }

    fn source_len(&self) -> usize {
        self.source_code.len()
    }

    fn append_to_line(&self, line: &mut String) -> () {
        line.push_str(self.style);
        line.push_str(self.source_code);
    }
}

pub struct ChunkBuilder<'a> {
    source_code: Option<&'a str>,
    style: Option<&'static str>,
}

impl<'a> ChunkBuilder<'a> {
    pub fn new() -> Self {
        Self { source_code: None, style: None }
    }

    pub fn reset(&mut self) {
        self.source_code = None;
        self.style = None;
    }

    pub fn style(&mut self, style: &'static str) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn source(&mut self, source_code: &'a str) -> &mut Self {
        self.source_code = Some(source_code);
        self
    }

    pub fn build(&self) -> Result<Chunk<'a>, ()> {
        if self.source_code.is_none() {
            return Err(());
        }

        Ok(Chunk::new(
            self.source_code.as_ref().unwrap(),
            self.style.as_ref().unwrap(),
        ))
    }
}
