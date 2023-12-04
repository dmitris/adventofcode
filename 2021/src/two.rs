use super::utils;


pub fn input(filename: &str) -> Vec<String> {
    utils::inputvec(filename)
}

pub fn part1(lines: Vec<String>) -> i64 {
    lines.iter()
        .map(|line| {
            let (direction, value) = line.split_once(' ').unwrap();
            let intval = value.parse::<i64>().unwrap();
            match direction {
                "forward" => [intval, 0],
                "backward" => [-intval, 0],
                "up" => [0, -intval],
                "down" => [0, intval],
                _ => [0, 0],
            }
        })
        .reduce(|a, b| {
            [a[0] + b[0], a[1] + b[1]]
        }).unwrap()
        .iter()
        .product()
}

pub fn part2(lines: Vec<String>) -> i64 {
    lines.iter()
        .map(|line| {
            let (direction, value) = line.split_once(' ').unwrap();
            let intval = value.parse::<i64>().unwrap();
            match direction {
                "forward" => [intval, 0, 0],
                "backward" => [-intval, 0, 0],
                "up" => [0, 0, -intval],
                "down" => [0, 0, intval],
                _ => [0, 0, 0],
            }
        })
        .reduce(|a, b| {
            if b[0] != 0 {
                [a[0] + b[0], a[1] + a[2] * b[0].abs(), a[2] + b[2]]
            } else {
                [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
            }
        }).unwrap()
        .iter()
        .rev()
        .skip(1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input("sample/two.txt")), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input("sample/two.txt")), 900)
    }
}
