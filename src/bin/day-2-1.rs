use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/bin/inputs/day-2-input.txt")?;
    let reader = BufReader::new(file);
    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
        let parts = split_line(":", &line);
        let password = parts[1];
        let range = split_line(" ", parts[0]);
        let required_character = range[1];
        let limits = split_line("-", range[0]);
        let lower_limit = limits[0].parse::<usize>().unwrap();
        let upper_limit = limits[1].parse::<usize>().unwrap();

        if password.contains(required_character) {
            let number_of_occurances = password.matches(required_character).count();
            if number_of_occurances >= lower_limit && number_of_occurances <= upper_limit {
                counter = counter + 1;
            }
        }
    }

    println!("{}", counter);

    Ok(())
}

fn split_line<'a>(separator: &'a str, line: &'a str) -> Vec<&'a str> {
    return line.split(separator).collect::<Vec<&str>>();
}