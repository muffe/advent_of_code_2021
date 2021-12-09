use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("data.txt");
    let mut num_increases = 0;
    let mut previous_number: i64 = -1;

    for line in lines {
        let line_as_int: i64 = line.parse().unwrap();

        if previous_number != -1 && previous_number < line_as_int {
            num_increases = num_increases + 1;
        }

        previous_number = line_as_int;
    }

    println!("{}", num_increases);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}