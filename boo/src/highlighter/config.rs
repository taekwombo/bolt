use termion::color;

static HIGHLIGHT_GROUPS: &'static [&'static str] = &[
    "keyword",
    "number",
    "comment",
];

static EMPTY: &'static str = "";

pub struct HighlightColors {
    keyword: &'static str,
    number: &'static str,
    gray: &'static str,
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
        let number: &'static str = color::Blue.fg_str();
        let gray: &'static str = Box::leak(color::AnsiValue::grayscale(20).fg_string().into_boxed_str());

        Self {
            reset,
            number,
            gray,
            keyword: yellow,
        }
    }

    pub fn get_color(&self, highlight_index: usize) -> &'static str {
        match highlight_index {
            0 => self.keyword,
            1 => self.number,
            2 => self.gray,
            _ => EMPTY,
        }
    }
}

