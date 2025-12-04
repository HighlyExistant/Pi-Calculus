#[derive(Debug)]
pub struct Lexer {
    text: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self { text, cursor: 0 }
    }
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    #[inline]
    pub fn shift_cursor(&mut self) {
        self.cursor += 1;
    }
    pub fn skip_whitespace(&mut self) {
        let mut slice = self.text[self.cursor..self.text.len()].chars();
        while let Some(char) = slice.next() {
            if char.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }
    pub fn get_char(&mut self) -> Option<char> {
        self.text.chars().nth(self.cursor)
    }
    pub fn next_char(&mut self) -> Option<char> {
        let c = self.get_char()?;
        self.shift_cursor();
        Some(c)
    }
}

pub trait FromLexer: Sized {
    fn from_lexer(lexer: &mut Lexer) -> Option<Self>;
}