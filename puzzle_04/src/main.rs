use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string};
use std::io::Read;

fn main() {
    println!("Part 1 - {}", parse("D:/github/AdventOfCode2023/puzzle_04/src/input.txt",
                                  line_processor_part1, game_score_extractor_part1));
    println!("Part 2 - {}", parse("D:/github/AdventOfCode2023/puzzle_04/src/input.txt",
                                  line_processor_part2, game_score_extractor_part2));
}

fn parse<P, E>(filename: &str, line_processor: P, game_score_extractor: E) -> i32
    where P: FnMut(&str) -> Card, E : Fn(&Vec<Card>) -> i32 {
    let cards = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(line_processor)
        .collect::<Vec<Card>>();

    game_score_extractor(&cards)
}
fn line_processor_part1(line: &str) -> Card {
    parse_card(line)
}

fn parse_card(line: &str) -> Card {
    let parsed_line = line.split(":").collect::<Vec<&str>>();
    let parsed_numbers = parsed_line[1].split(" | ").collect::<Vec<&str>>();
    let winning: Vec<_>= parsed_numbers[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect();
    let mine: Vec<_> = parsed_numbers[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect();
    let card_id = parsed_line[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1]
        .trim().parse::<i32>().ok().unwrap();
    Card {
        id: card_id,
        winning: winning.iter().map(|no| CardNumber {value: *no}).collect(),
        mine: HashSet::from_iter(mine.iter().map(|m| CardNumber { value: *m}))
    }
}

fn game_score_extractor_part1(all: &Vec<Card>) -> i32 {
    all.iter().fold(0, |acc, current_card| acc + get_matches_count((current_card)))
}

fn line_processor_part2(line: &str) -> Card {
    parse_card(line)
}

fn game_score_extractor_part2(all: &Vec<Card>) -> i32 {
    let mut weights = all.iter().map(|c| (c.id, 1)).collect::<HashMap<i32, i32>>();

    for current_card in all {
        let matching = get_cards_count(current_card);
        for _curr_count in 0..*weights.get(&current_card.id).unwrap() {
            for n in current_card.id + 1..=current_card.id + matching {
                if weights.get(&n).is_some() {
                    weights.insert(n, weights.get(&n).unwrap() + 1);
                }
            }
        }
    }
    weights.iter().map(|(key, value)| value).sum()
}

fn get_matches_count(current_card: &Card) -> i32 {
    current_card.winning.iter().enumerate().fold(0, |acc, (idx, card_number)| if (current_card.mine.contains(card_number)) {
        if acc == 0 { 1 } else { acc * 2 }
    } else { acc })
}

fn get_cards_count(current_card: &Card) -> i32 {
    current_card.winning.iter().fold(0, |acc, card_number| acc + if current_card.mine.contains
    (card_number) { 1 } else { 0 })
}
#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning: Vec<CardNumber>,
    mine: HashSet<CardNumber>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CardNumber {
    value: i32
}