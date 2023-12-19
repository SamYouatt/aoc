#[derive(Clone, Copy)]
pub struct StatRange {
    pub start: usize,
    pub end: usize,
}

impl StatRange {
    pub fn new(start: usize, end: usize) -> StatRange {
        StatRange { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start + 1
    }
}
