use itertools::Itertools;

use super::utils;

pub fn input(filename: &str) -> Vec<i64> {
    utils::commasep(filename)
}

pub fn eval(op: i64, left: i64, right: i64) -> i64 {
    match op {
        1 => return left + right,
        2 => return left * right,
        _ => panic!("Error invalid opcode"),
    }
}

pub fn part1(codes: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut memory = codes.clone();
    memory[1] = noun;
    memory[2] = verb;
    let mut ptr = 0;
    loop {
        let li = memory[ptr + 1] as usize;
        let ri = memory[ptr + 2] as usize;
        let ti = memory[ptr + 3] as usize;
        memory[ti] = eval(memory[ptr], memory[li], memory[ri]);
        ptr += 4;
        if ptr >= memory.len() || memory[ptr] == 99 {
            break;
        }
    }
    memory[0]
}

/*
 * Here we use memory.to_vec() to create a new vector,
 * with the same elements as vec by default cannot be cloned
 */
pub fn part2(memory: Vec<i64>) -> i64 {
    (1..100).cartesian_product(1..100)
        .map(|(noun, verb)| (noun, verb, part1(memory.to_vec(), noun, verb)))
        .filter(|(_, _, result)| *result == 19690720)
        .take(1)
        .map(|(noun, verb, _)| 100 * noun + verb)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input("sample/two.txt"), 9, 10), 3500)
    }

}
