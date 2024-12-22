fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    input.lines().map(|l| l.parse::<usize>().unwrap()).map(|x| {
        let mut secret_num = x;

        for _ in 0..2000 {
            let n = secret_num * 64;
            secret_num = mix(secret_num, n);
            secret_num = prune(secret_num);

            let n = secret_num / 32;
            secret_num = mix(secret_num, n);
            secret_num = prune(secret_num);

            let n = secret_num * 2048;
            secret_num = mix(secret_num, n);
            secret_num = prune(secret_num);
        }

        secret_num
    }).sum()
}

fn mix(secret: usize, x: usize) -> usize {
    secret ^ x
}

fn prune(secret: usize) -> usize {
    secret.rem_euclid(16777216)
}
