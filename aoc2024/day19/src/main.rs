fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

#[derive(PartialEq, Eq)]
enum Towel {
    Red,
    White,
    Green,
    Ublue,
    Black,
}

impl Towel {
    fn from_byte(byte: u8) -> Towel {
        match byte {
            b'r' => Towel::Red,
            b'u' => Towel::Ublue,
            b'g' => Towel::Green,
            b'b' => Towel::Black,
            b'w' => Towel::White,
            _ => panic!("bad towel"),
        }
    }
}

fn part_1(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(", ")
        .map(|t| t.bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let designs = designs
        .lines()
        .map(|d| d.trim().bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    designs.iter().filter(|d| possible(&d, &towels)).count()
}

fn possible(design: &[Towel], towels: &Vec<Vec<Towel>>) -> bool {
    if design.is_empty() {
        return true;
    }

    for t in towels {
        if design.starts_with(t) {
            if possible(&design[t.len()..], towels) {
                return true;
            }
        }
    }

    false
}
