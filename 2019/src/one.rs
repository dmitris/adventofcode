use super::utils;

pub fn input(filename: &str) -> Vec<i64> {
    utils::inputvec(filename)
}

pub fn part1(masses: Vec<i64>) -> i64 {
    masses.iter().map(|mass| (mass / 3) - 2)
        .fold(0, |fuel, req| fuel + req)
}

fn recurse(num: i64) -> i64 {
    match num {
        x if x < 1 => 0,
        _ => num + recurse(num/3 - 2),
    }
}

pub fn part2(masses: Vec<i64>) -> i64 {
    masses.iter().map(|&mass| recurse((mass / 3) - 2))
        .fold(0, |fuel, req| fuel + req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input("sample/one.txt")), 658)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input("sample/one.txt")), 970)
    }
}
