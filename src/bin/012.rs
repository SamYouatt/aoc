fn main() {
    let input = include_str!("../../inputs/012.txt");

    let mut ship = ShipPosition::default();

    input.lines().for_each(|line| {
        let instruction: char = line.as_bytes()[0] as char;
        let amount: usize = atoi::atoi(&line.as_bytes()[1..]).unwrap();

        match instruction {
            'R' => ship.turn_ship(true, amount),
            'L' => ship.turn_ship(false, amount),
            'N' => ship.move_ship(Bearing::North, amount),
            'E' => ship.move_ship(Bearing::East, amount),
            'S' => ship.move_ship(Bearing::South, amount),
            'W' => ship.move_ship(Bearing::West, amount),
            'F' => ship.move_forward(amount),
            _ => panic!("bad instruction, {}", instruction),
        }
    });

    println!("Part one: {}", ship.get_manhattan());
}

#[derive(Debug)]
enum Bearing {
    North,
    South,
    East,
    West,
}

struct ShipPosition {
    longitude: isize,
    latitude: isize,
    bearing: Bearing,
}

impl ShipPosition {
    fn default() -> Self {
        ShipPosition {
            longitude: 0,
            latitude: 0,
            bearing: Bearing::East,
        }
    }

    fn turn_ship(&mut self, clockwise: bool, amount: usize) {
        let rotations = amount / 90;

        match clockwise {
            true => (0..rotations).for_each(|_| match self.bearing {
                Bearing::North => self.bearing = Bearing::East,
                Bearing::East => self.bearing = Bearing::South,
                Bearing::South => self.bearing = Bearing::West,
                Bearing::West => self.bearing = Bearing::North,
            }),
            false => {
                (0..rotations).for_each(|_| match self.bearing {
                    Bearing::North => self.bearing = Bearing::West,
                    Bearing::West => self.bearing = Bearing::South,
                    Bearing::South => self.bearing = Bearing::East,
                    Bearing::East => self.bearing = Bearing::North,
                });
            }
        }
    }

    fn move_ship(&mut self, direction: Bearing, amount: usize) {
        match direction {
            Bearing::North => self.latitude += amount as isize,
            Bearing::East => self.longitude += amount as isize,
            Bearing::South => self.latitude -= amount as isize,
            Bearing::West => self.longitude -= amount as isize,
        }
    }

    fn move_forward(&mut self, amount: usize) {
        match self.bearing {
            Bearing::North => self.latitude += amount as isize,
            Bearing::East => self.longitude += amount as isize,
            Bearing::South => self.latitude -= amount as isize,
            Bearing::West => self.longitude -= amount as isize,
        }
    }

    fn get_manhattan(&self) -> isize {
        self.longitude.abs() + self.latitude.abs()
    }
}
