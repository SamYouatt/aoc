pub struct Grid {
    pub width: isize,
    pub height: isize,
    pub content: Vec<u8>,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().take(1).map(|l| l.chars().count()).sum();

        let content = input.lines().flat_map(|l| l.bytes()).collect();

        Self {
            width,
            height,
            content,
        }
    }

    pub fn contains(coord: Coord) -> bool {
        todo!()
    }
}

pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
