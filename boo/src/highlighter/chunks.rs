
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
        let chunk_length: usize = chunk.source_len();
        let width = self.width as usize;
        let current_line_count = self.source_length / width;
        let next_line_count = (chunk_length + self.source_length) / width;

        if next_line_count > current_line_count {
            // Split chunk in two so that the line is filled but not wrapped by the terminal.
            let (left, right) = chunk.split(
                (self.source_length + chunk_length) % width
            );

            self.chunks.push(left);
            self.chunks.push(right);
        } else {
            self.chunks.push(chunk);
        }

        self.source_length += chunk_length;

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
    fn split(self, index: usize) -> (Self, Self) {
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

// TODO: builder couldn't really work due to lifetime conflicts.
// Should be updated with `fn new(&'a str) -> Self` instead of empty
// data at the beginnig.
// pub struct ChunkBuilder<'a> {
//     source_code: Option<&'a str>,
//     style: Option<&'static str>,
// }
// 
// impl<'a> ChunkBuilder<'a> {
//     pub fn new() -> Self {
//         Self { source_code: None, style: None }
//     }
// 
//     pub fn is_clean<'s>(&'s self) -> bool {
//         self.source_code.is_none()
//     }
// 
//     pub fn style<'s>(&'s mut self, style: &'static str) -> &'s mut Self {
//         self.style.insert(style);
//         self
//     }
// 
//     pub fn source<'s, 'b: 'a>(&'s mut self, source_code: &'b str) -> &'s mut Self {
//         self.source_code.insert(source_code);
//         self
//     }
// 
//     /// Moves values out of the builder into new Chunk.
//     pub fn build<'s>(&'s mut self) -> Result<Chunk<'s>, ()> {
//         if self.source_code.is_none() {
//             return Err(());
//         }
// 
//         Ok(Chunk::new(
//             self.source_code.take().unwrap(),
//             self.style.take().unwrap(),
//         ))
//     }
// }
