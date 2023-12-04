use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read<P>(file: P) -> io::Result<String>
where P: AsRef<Path>, {
    let input = File::open(file)?;
    let mut line = String::new();
    io::BufReader::new(input).read_line(&mut line)?;
    Ok(line)
}

fn readlines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let input = File::open(file)?;
    Ok(io::BufReader::new(input).lines())
}


pub fn inputvec<T>(input: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
    let lines = match readlines(input) {
        Ok(lines) => lines,
        Err(error) => panic!("Error reading file {:?}", error),
    };
    lines.map(|line| line.unwrap())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

/*
 * Helper function to parse a comma separated line
 * into a vector.
 */
pub fn commasep<T>(input: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
    let line = match read(input) {
        Ok(line) => line,
        Err(error) => panic!("Error reading file {:?}", error),
    };
    line.trim()
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect::<Vec<_>>()
}
