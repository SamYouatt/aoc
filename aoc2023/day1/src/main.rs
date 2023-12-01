use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(&input);
    println!("part 1: {}", answer1);

    let answer2 = part_2(&input);
    println!("part 2: {}", answer2);
}

fn part_1(input: &str) -> usize {
    input.lines().fold(0, |total, line| {
        let digits: Vec<char> = line.chars().filter(|char| char.is_digit(10)).collect();
        let first_digit = digits.first().unwrap_or(&'0');
        let last_digit = digits.last().unwrap_or(&'0');

        total
            + format!("{}{}", first_digit, last_digit)
                .parse()
                .unwrap_or(0)
    })
}

fn part_2(input: &str) -> usize {
    let find_digit = Regex::new(r#"\d|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
    let find_digit_r = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    input.lines().fold(0, |total, line| {
        let first_digit = text_to_int(find_digit.find(&line).map(|x| x.as_str()).unwrap_or("0"));

        let last_digit = text_to_int(
            &find_digit_r
                .find(&line.chars().rev().collect::<String>())
                .map(|x| x.as_str())
                .unwrap_or("0")
                .chars()
                .rev()
                .collect::<String>(),
        );

        total
            + format!("{}{}", first_digit, last_digit)
                .parse()
                .unwrap_or(0)
    })
}

fn text_to_int(number: &str) -> usize {
    match number {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
