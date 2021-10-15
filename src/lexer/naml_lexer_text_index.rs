/// Tracks the length of the current text scanned by the lexer.
pub struct NamlLexerTextTracker {
    start: Option<usize>,
    length: usize,
}

impl NamlLexerTextTracker {
    pub fn create() -> NamlLexerTextTracker {
        NamlLexerTextTracker {
            start: None,
            length: 0,
        }
    }

    pub fn start(&self) -> Option<usize> {
        self.start
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn start_if_not_started(&mut self, idx: usize) {
        if self.start.is_none() {
            self.start = Some(idx);
        }
    }
    pub fn is_tracking(&self) -> bool {
        return self.start.is_some();
    }
    pub fn bump_length(&mut self) {
        self.length = self.length + 1
    }
    pub fn reset(&mut self) {
        self.start = None;
        self.length = 0;
    }
    pub fn end(&self) -> Option<usize> {
        match self.start {
            None => None,
            Some(start) => Some(start + self.length)
        }
    }
}
