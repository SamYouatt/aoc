use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/011.txt");

    let start = Instant::now();

    println!("Part one: {}", part_one(&input));
    println!("Time taken: {:#?}", start.elapsed());

    let start = Instant::now();

    println!("Part two: {}", part_two(&input));
    println!("Time taken: {:#?}", start.elapsed());
}

fn part_one(input: &str) -> usize {
    let rows: isize = input.lines().count() as isize;
    let cols: isize = input.lines().next().map(|line| line.len()).unwrap() as isize;

    let seat_indexes: Vec<usize> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, pos)| pos == &&b'L')
        .map(|(i, _)| i - (i / (cols as usize + 1)))
        .collect();

    let mut current = vec![State::Floor; (rows * cols) as usize];
    seat_indexes
        .iter()
        .for_each(|seat_index| current[*seat_index] = State::Empty);
    let mut previous = current.to_vec();

    let neighbours: Vec<(usize, Vec<usize>)> = seat_indexes
        .iter()
        // turn each seat index into a tuple of the seat index, and a vector of its neihbours seat indexes
        .map(|seat_index| {
            (
                *seat_index,
                (0..9)
                    // ignore when delta index is 4 because that is the seat itself
                    .filter(|delta_index| delta_index != &4)
                    // turn the delate index into an x and y coord
                    .map(|delta_index| {
                        (
                            (*seat_index as isize % cols) + delta_index % 3 - 1,
                            (*seat_index as isize / cols) + delta_index / 3 - 1,
                        )
                    })
                    // ensure the x and y exist within the bounds of the map
                    .filter(|(x, y)| *x >= 0 && *x < cols && *y >= 0 && *y < rows)
                    // turn x and y back into an index on a 1d version of the seats
                    .map(|(x, y)| (y * cols + x) as usize)
                    // ignore floors
                    .filter(|i| current[*i] == State::Empty)
                    .collect(),
            )
        })
        .collect();

    while {
        // iterating over the list of seat indexes and their neighbours
        for (seat_index, neighbours) in &neighbours {
            // count the number of occupied seats in its neighbours
            let num_occupied = neighbours
                .iter()
                .filter(|neighbour_index| previous[**neighbour_index] == State::Occupied)
                .count();
            // get the current and previous state of the seat (current state value doesn't matter it will only be edited)
            let (current_state, previous_state) =
                (&mut current[*seat_index], previous[*seat_index]);

            // apply the rules for changing state
            match (previous_state, num_occupied) {
                (State::Empty, 0) => *current_state = State::Occupied,
                (State::Occupied, 4..=8) => *current_state = State::Empty,
                _ => *current_state = previous_state,
            }
        }

        // this swaps the position of the current and previous vectors in memory. Essentially it serves the job of setting the previous equal to the current
        // it can just be swapped because the actual values in the previous don't matter, they just get changed never read
        // this saves a lot of time rather than copying the vector each time
        std::mem::swap(&mut current, &mut previous);

        // stop the while loop once they are the same
        current != previous
    } {}

    current
        .iter()
        .filter(|seat| seat == &&State::Occupied)
        .count()
}

fn part_two(input: &str) -> usize {
    let rows: isize = input.lines().count() as isize;
    let cols: isize = input.lines().next().map(|line| line.len()).unwrap() as isize;

    let seat_indexes: Vec<usize> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, pos)| pos == &&b'L')
        .map(|(i, _)| i - (i / (cols as usize + 1)))
        .collect();

    let mut current = vec![State::Floor; (rows * cols) as usize];
    seat_indexes
        .iter()
        .for_each(|seat_index| current[*seat_index] = State::Empty);
    let mut previous = current.to_vec();

    let neighbours: Vec<(usize, Vec<usize>)> = seat_indexes
        .into_iter()
        // turn each seat index into a tuple of the seat index, and a vector of its neihbours seat indexes
        .map(|seat_index| {
            (
                seat_index,
                (0..9)
                    .filter(|delta_index| delta_index != &4)
                    // get x and y offsets for the direction around the seat
                    .map(|direction| (direction % 3 - 1, direction / 3 - 1))
                    // filter those directions which have a seat in their line of site, and map the value to its index
                    .filter_map(|(x_start, y_start)| {
                        (1..)
                            // create open ended iterator starting at 1 to act as the multiplier that iterates along the line of sight in the given direction
                            .map(|multiplier| {
                                (
                                    (seat_index as isize % cols) + x_start * multiplier,
                                    (seat_index as isize / cols) + y_start * multiplier,
                                )
                            })
                            // take all indexes in that direction that stay within the bounds
                            .take_while(|(x, y)| *x >= 0 && *x < cols && *y >= 0 && *y < rows)
                            // turn x and y coords back in to index
                            .map(|(x, y)| (y * cols + x) as usize)
                            // return index of first seat seen in that direction
                            .find(|i| current[*i] == State::Empty)
                    })
                    .collect(),
            )
        })
        .collect();

    while {
        // iterating over the list of seat indexes and their neighbours
        for (seat_index, neighbours) in &neighbours {
            // count the number of occupied seats in its neighbours
            let num_occupied = neighbours
                .iter()
                .filter(|neighbour_index| previous[**neighbour_index] == State::Occupied)
                .count();
            // get the current and previous state of the seat (current state value doesn't matter it will only be edited)
            let (current_state, previous_state) =
                (&mut current[*seat_index], previous[*seat_index]);

            // apply the rules for changing state
            match (previous_state, num_occupied) {
                (State::Empty, 0) => *current_state = State::Occupied,
                (State::Occupied, 5..=8) => *current_state = State::Empty,
                _ => *current_state = previous_state,
            }
        }

        // this swaps the position of the current and previous vectors in memory. Essentially it serves the job of setting the previous equal to the current
        // it can just be swapped because the actual values in the previous don't matter, they just get changed never read
        // this saves a lot of time rather than copying the vector each time
        std::mem::swap(&mut current, &mut previous);

        // stop the while loop once they are the same
        current != previous
    } {}

    current
        .iter()
        .filter(|seat| seat == &&State::Occupied)
        .count()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Occupied,
    Empty,
    Floor,
}
