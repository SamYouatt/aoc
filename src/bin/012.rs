fn main() {
    let input = include_str!("../../inputs/012.txt");

    let mut ship = Ship::default();

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

    println!("Part one: {}", ship.manhattan());

    let mut waypoint_ship = WaypointShip::default();

    input.lines().for_each(|line| {
        let instruction: char = line.as_bytes()[0] as char;
        let amount: usize = atoi::atoi(&line.as_bytes()[1..]).unwrap();

        match instruction {
            'R' => waypoint_ship.rotate_waypoint(true, amount),
            'L' => waypoint_ship.rotate_waypoint(false, amount),
            'N' => waypoint_ship.move_waypoint(Bearing::North, amount),
            'E' => waypoint_ship.move_waypoint(Bearing::East, amount),
            'S' => waypoint_ship.move_waypoint(Bearing::South, amount),
            'W' => waypoint_ship.move_waypoint(Bearing::West, amount),
            'F' => waypoint_ship.move_to_waypoint(amount),
            _ => panic!("bad instruction, {}", instruction),
        }
    });

    println!("Part two: {}", waypoint_ship.manhattan());
}

#[derive(Debug)]
enum Bearing {
    North,
    South,
    East,
    West,
}

struct Ship {
    longitude: isize,
    latitude: isize,
    bearing: Bearing,
}

impl Ship {
    fn default() -> Self {
        Ship {
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

    fn manhattan(&self) -> isize {
        self.longitude.abs() + self.latitude.abs()
    }
}

struct WaypointShip {
    longitude: isize,
    latitude: isize,
    waypoint_x: f32,
    waypoint_y: f32,
}

impl WaypointShip {
    fn default() -> Self {
        WaypointShip {
            longitude: 0,
            latitude: 0,
            waypoint_x: 10.0,
            waypoint_y: 1.0,
        }
    }

    fn move_waypoint(&mut self, direction: Bearing, amount: usize) {
        match direction {
            Bearing::North => self.waypoint_y += amount as f32,
            Bearing::East => self.waypoint_x += amount as f32,
            Bearing::South => self.waypoint_y -= amount as f32,
            Bearing::West => self.waypoint_x -= amount as f32,
        }
    }

    fn move_to_waypoint(&mut self, times: usize) {
        (0..times).for_each(|_| {
            self.longitude += self.waypoint_x as isize;
            self.latitude += self.waypoint_y as isize;
        })
    }

    fn rotate_waypoint(&mut self, clockwise: bool, amount: usize) {
        let rotation = match clockwise {
            true => (amount as f32 * -1.0).to_radians(),
            false => (amount as f32).to_radians(),
        };

        let s = rotation.sin();
        let c = rotation.cos();

        let new_x = (self.waypoint_x * c - self.waypoint_y * s).round();
        let new_y = (self.waypoint_x * s + self.waypoint_y * c).round();

        self.waypoint_x = new_x;
        self.waypoint_y = new_y;
    }

    fn manhattan(&self) -> isize {
        self.longitude.abs() + self.latitude.abs()
    }
}

#[test]
fn test_waypoint_ship() {
    let instructions = "F10\nN3\nF7\nR90\nF11";

    let mut waypoint_ship = WaypointShip::default();

    instructions.lines().for_each(|line| {
        let instruction: char = line.as_bytes()[0] as char;
        let amount: usize = atoi::atoi(&line.as_bytes()[1..]).unwrap();

        match instruction {
            'R' => waypoint_ship.rotate_waypoint(true, amount),
            'L' => waypoint_ship.rotate_waypoint(false, amount),
            'N' => waypoint_ship.move_waypoint(Bearing::North, amount),
            'E' => waypoint_ship.move_waypoint(Bearing::East, amount),
            'S' => waypoint_ship.move_waypoint(Bearing::South, amount),
            'W' => waypoint_ship.move_waypoint(Bearing::West, amount),
            'F' => waypoint_ship.move_to_waypoint(amount),
            _ => panic!("bad instruction, {}", instruction),
        }
    });

    assert_eq!(waypoint_ship.manhattan(), 286);
}
