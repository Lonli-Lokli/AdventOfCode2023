use std::fs::{ read_to_string};
use std::str::FromStr;
use std::io::{ BufRead };

fn main() {
    println!("Part 1 - {}", parse("D:/github/AdventOfCode2023/puzzle_01/src/input.txt", line_processor_part1));
    println!("Part 2 - {}", parse("D:/github/AdventOfCode2023/puzzle_01/src/input.txt", line_processor_part2));
}


fn parse<P>(filename: &str, mut line_processor: P) -> i32 where P: FnMut(&str) -> String {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(|line| line_processor(line))
        .map(|y| i32::from_str(y.as_str()).unwrap())
        .sum()
}
fn line_processor_part1(line: &str) -> String {
    format!("{0}{1}",
            get_number(line),
            get_number(&line.chars().rev().collect::<String>())
    )
}

fn line_processor_part2(line: &str) -> String {
    let enriched_line = enrich_line(line);
    format!("{0}{1}",
            get_number(&enriched_line),
            get_number(&enriched_line.chars().rev().collect::<String>()))
}
fn get_number(input: &str) -> char {
    input.chars().nth(input.find(|x: char| x.is_digit(10)).unwrap().to_string().parse::<usize>().unwrap()).unwrap()
}
fn enrich_line(input: &str) -> String {
    input.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}
