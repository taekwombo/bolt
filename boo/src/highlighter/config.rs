use termion::color;

static HIGHLIGHT_GROUPS: &'static [&'static str] = &[
    "keyword",
];

static EMPTY: &'static str = "";

pub struct HighlightColors {
    keyword: &'static str,
    reset: &'static str,
}

impl HighlightColors {
    pub fn groups() -> &'static [&'static str] {
        HIGHLIGHT_GROUPS
    }

    pub fn reset(&self) -> &'static str {
        self.reset
    }

    pub fn new() -> Self {
        let yellow: &'static str = color::Yellow.fg_str();
        let reset: &'static str = color::Reset.fg_str();

        Self {
            reset,
            keyword: yellow,
        }
    }

    pub fn get_color(&self, highlight_index: usize) -> &'static str {
        match highlight_index {
            0 => self.keyword,
            _ => EMPTY,
        }
    }
}

