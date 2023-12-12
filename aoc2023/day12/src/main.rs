fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (squares, counts) = line.split_once(' ').unwrap();
            let squares: Vec<_> = squares.chars().collect();
            let counts: Vec<_> = counts
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            possible_arrangements(&squares, &counts, None)
        })
        .sum()
}

// current_group: If none then not currently placing tiles in a group. If Some(x) then x is the current
// length of the group
fn possible_arrangements(
    squares: &[char],
    counts: &[usize],
    current_group: Option<usize>,
) -> usize {
    // Base case: reached the end of the squares, nothing to place, and not in a group -> Valid
    // configuration
    if squares.is_empty() && counts.is_empty() && current_group.is_none() {
        return 1;
    }

    // Base case: reached the end of the squares and inside a group. If there is only one more
    // group to place and the current group length is that size -> Valid configuration
    // Else -> Invalid configuration
    if squares.is_empty() && counts.len() == 1 {
        if let Some(remaining) = current_group {
            if remaining == counts[0] {
                return 1;
            }
        } else {
            return 0;
        }
    }

    // Base case: reached end of squares and no more valid options -> Invalid configuration
    if squares.is_empty() {
        return 0;
    }

    // Base case: squares still to check and in a group but the number of counts to place has
    // finished -> Invalid configuration
    if current_group.is_some() && counts.is_empty() {
        return 0;
    }

    let configurations = match (squares[0], current_group) {
        // Currently in a group and there is more in that group to place but have reached an empty
        ('.', Some(x)) if x != counts[0] => 0,
        // Otherwise we have finished that group so exit it and move onto the next group count
        ('.', Some(_)) => possible_arrangements(&squares[1..], &counts[1..], None),
        // Not in a group and hit an empty, just continue to the next square
        ('.', None) => possible_arrangements(&squares[1..], counts, None),
        // We hit a tile and now enter a new group
        ('#', None) => possible_arrangements(&squares[1..], counts, Some(1)),
        // We are in a group and find a placed tile to bump the current group by 1
        ('#', Some(_)) => {
            possible_arrangements(&squares[1..], counts, current_group.map(|count| count + 1))
        }
        // We encounter an option and we aren't in a group so the two options from here are to not
        // place a tile and not enter a group, or place a tile and enter a new group
        ('?', None) => {
            possible_arrangements(&squares[1..], counts, None)
                + possible_arrangements(&squares[1..], counts, Some(1))
        }
        // We have an option and we are in a group. Find the configs if we continue in that group,
        // then if we have finished the current group, start searching for the next group and
        // include those configs as well
        ('?', Some(x)) => {
            let continuing_configs =
                possible_arrangements(&squares[1..], counts, current_group.map(|count| count + 1));

            if x == counts[0] {
                return continuing_configs
                    + possible_arrangements(&squares[1..], &counts[1..], None);
            }

            return continuing_configs;
        }
        _ => panic!("Unexpected option"),
    };

    configurations
}
