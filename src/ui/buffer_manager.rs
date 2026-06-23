use unicode_segmentation::UnicodeSegmentation;

#[derive(Default, Debug)]
pub struct SingleLineBufferManager {
    buffer: String,
    pub cursor: usize,
}

impl SingleLineBufferManager {
    pub fn init_with(buf: String, ptr: usize) -> Self {
        Self { buffer: buf, cursor: ptr }
    }

    pub fn init_empty() -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, c: char) {
        self.buffer.insert(self.get_idx(), c);
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.buffer.remove(self.get_idx());
    }

    pub fn delete(&mut self) {
        if self.cursor == self.buffer.grapheme_indices(true).count() {
            return;
        }

        self.buffer.remove(self.get_idx());
    }

    pub fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn right(&mut self) {
        if self.cursor == self.buffer.grapheme_indices(true).count() {
            return;
        }
        self.cursor += 1;
    }

    pub fn peek(&self) -> &str {
        &self.buffer
    }

    pub fn flush(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.buffer)
    }

    pub fn len(&self) -> usize {
        self.buffer.grapheme_indices(true).count()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    fn get_idx(&self) -> usize {
        self.buffer
            .grapheme_indices(true)
            .nth(self.cursor)
            .map(|(idx, _)| idx)
            .unwrap_or(self.buffer.len())
    }
}
