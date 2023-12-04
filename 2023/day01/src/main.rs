use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    // let res1 = part1(&input)?;
    // println!("part 1: {res1}");

    let res2 = part2(&input)?;
    println!("part 2: {res2}");
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        println!("[part1] {}", line);
        sum += 10
            * line
                .chars()
                .find_map(|c| c.to_digit(10))
                .ok_or("no first digit")?;
        sum += line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .ok_or("no last digit")?;
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        // call to_digit_exp with substrings of line until it returns Some
        // and add the value to sum
        let line_len = line.len();
        let mut calibration_value: u32 = 0;
        for i in 0..line_len {
            if let Some(d) = to_digit_exp(&line[i..]) {
                // println!("[part2] #1 i={i} OK d={d} line={line}");
                calibration_value = 10 * u32::from(d);
                break;
            }
        }
        for i in 0..line_len {
            if let Some(d) = to_digit_exp(&line[(line_len - i - 1)..]) {
                // println!("[part2] #2 i={i} OK d={d} line={line}");
                calibration_value += u32::from(d);
                break;
            }
        }
        sum += calibration_value;
        // println!("[part2] line: {line},  calibration_value: {calibration_value}, sum: {sum}");
    }
    Ok(sum.into())
}

// to_digit_exp takes &str and returns Option<u8> with the number if
// the string starts with a digit or one of the following strings:
// one, two, three, four, five, six, seven, eight, and nine,
// otherwise None.
fn to_digit_exp(s: &str) -> Option<u8> {
    if s.is_empty() {
        None
    } else if s.starts_with("1") || s.starts_with("one") {
        Some(1)
    } else if s.starts_with("2") || s.starts_with("two") {
        Some(2)
    } else if s.starts_with("3") || s.starts_with("three") {
        Some(3)
    } else if s.starts_with("4") || s.starts_with("four") {
        Some(4)
    } else if s.starts_with("5") || s.starts_with("five") {
        Some(5)
    } else if s.starts_with("6") || s.starts_with("six") {
        Some(6)
    } else if s.starts_with("7") || s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("8") || s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("9") || s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}
