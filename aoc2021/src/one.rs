use super::utils;


pub fn input(filename: &str) -> Vec<i64> {
    utils::inputvec(filename)
}

pub fn part1(numbers: Vec<i64>) -> usize {
    numbers.windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();
}

pub fn part2(numbers: Vec<i64>) -> usize {
    numbers.windows(3)
        .map(|triplet| triplet[0] + triplet[1] + triplet[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count(); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input("sample/one.txt")), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input("sample/one.txt")), 5)
    }

}
