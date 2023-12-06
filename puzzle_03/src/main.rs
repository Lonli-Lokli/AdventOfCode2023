use std::fs::{read_to_string};
use std::io::Read;

fn main() {
    println!("Part 1 - {}", parse("D:/github/AdventOfCode2023/puzzle_03/src/input.txt", line_processor_part1, game_score_extractor_part1));
    println!("Part 2 - {}", parse("D:/github/AdventOfCode2023/puzzle_03/src/input.txt", line_processor_part2, game_score_extractor_part2));
}

fn parse<P, E>(filename: &str, mut line_processor: P, mut game_score_extractor: E) -> i32
    where P: FnMut(&str, usize) -> Vec<Creature>, E : Fn(&Vec<Creature>, &Creature) -> i32 {
    let creatures: Vec<_> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .enumerate()
        .map(|(idx, line)| line_processor(line, idx))
        .flatten()
        .collect();

    creatures.iter().fold(0, |acc, x| {
        acc + game_score_extractor(&creatures, x)})
}
fn line_processor_part1(line: &str, line_no: usize) -> Vec<Creature> {
    parse_creatures(line, line_no)
}

fn game_score_extractor_part1(all: &Vec<Creature>, current: &Creature) -> i32 {
    if current.kind == CreatureKind::PartNumber && all.iter().any(|c| c.kind !=
        CreatureKind::PartNumber && is_descendant(c, current)) { current.value } else { 0 }
}

fn is_descendant(symbol_creature: &Creature, part_creature: &Creature) -> bool {
    are_lines_adjacent((&symbol_creature.start, &symbol_creature.end), (&part_creature.start,
                                                                      &part_creature.end))
}

fn are_lines_adjacent((line_a_start, line_a_end): (&Point2D, &Point2D), (line_b_start, line_b_end):
(&Point2D, &Point2D)) -> bool {
    let adj = line_a_end.y.abs_diff(line_b_end.y) <= 1 && (line_a_start.x.abs_diff(line_b_start.x) <= 1 || line_a_start.x
        .abs_diff(line_b_end.x) <= 1 || line_a_end.x.abs_diff(line_b_start.x) <= 1 || line_a_end
        .x.abs_diff(line_b_end.x) <= 1);
    adj
}

fn line_processor_part2(line: &str, line_no: usize) -> Vec<Creature> {
    parse_creatures(line, line_no)
}

fn parse_creatures(line: &str, line_no: usize) -> Vec<Creature> {
    let mut creatures: Vec<Creature> = vec![];
    let mut start = 0;
    let mut state = SearchState::NotFound;
    let mut part_value: String = String::from("");

    for (idx, c) in line.chars().enumerate() {
        match state {
            SearchState::NotFound => {
                match c {
                    '0'..='9' => {
                        start = idx;
                        part_value =  String::from(c);
                        state = SearchState::FoundPartNumber;
                    }
                    '.' => { part_value = String::from(""); }
                    // 👇🏻 that's all symbols, create them
                    _ => {
                        creatures.push(Creature {
                            value: 0,
                            kind: get_creature_kind_by_symbol(&c),
                            start: Point2D { x: idx, y: line_no },
                            end: Point2D { x: idx, y: line_no }
                        });
                    }
                }
            }
            SearchState::FoundPartNumber => {
                match c {
                    '0'..='9' => {
                        part_value.push(c);
                    }
                    '.' => {
                        // we found all the values from part_number
                        creatures.push(Creature { value: part_value.parse::<i32>().unwrap(),
                            kind: get_creature_kind_by_symbol(&line.chars().nth(idx - 1).unwrap()
                            ), start:
                                Point2D {x: start, y: line_no
                            }, end: Point2D { x: idx - 1, y: line_no } });
                        part_value = String::from("");
                        state = SearchState::NotFound;
                    }
                    // that's all symbols
                    _ => {
                        creatures.push(Creature { value: part_value.parse::<i32>().unwrap(),
                            kind: get_creature_kind_by_symbol(&line.chars().nth(idx - 1).unwrap()
                            ), start: Point2D {x: start, y: line_no
                            }, end: Point2D { x: idx - 1, y: line_no } });
                        creatures.push(Creature {
                            value: 0,
                            kind: get_creature_kind_by_symbol(&c),
                            start: Point2D { x: idx, y: line_no },
                            end: Point2D { x: idx, y: line_no }
                        });
                        part_value = String::from("");
                        state = SearchState::NotFound;
                    }
                }
            }
            SearchState::FoundSymbol => {
                panic!();
            }
        }

    }
    match state {
        SearchState::NotFound => {}
        SearchState::FoundPartNumber => {
            creatures.push(Creature { value: part_value.parse::<i32>().unwrap(), kind:
            get_creature_kind_by_symbol(&line.chars().nth_back(0).unwrap()
            ), start:
            Point2D {x: start, y: line_no }, end: Point2D { x: line.len(), y: line_no } });
        }
        SearchState::FoundSymbol => {
            panic!()
        }
    }
    creatures
}

fn get_creature_kind_by_symbol(ch: &char) -> CreatureKind {
    match (ch) {
        '0'..='9' => CreatureKind::PartNumber,
        '.' => panic!(),
        '*' => CreatureKind::Gear,
        _ => CreatureKind::Symbol
    }
}
fn game_score_extractor_part2(all: &Vec<Creature>, current: &Creature) -> i32 {
    if current.kind != CreatureKind::Gear
    { 0 } else {
        let connections: Vec<_> = all.iter().filter(|c| c.kind == CreatureKind::PartNumber && is_descendant(c,
                                                                                                    current)).collect();
        if connections.len() == 2 {
            connections.iter().map(|c| c.value).product()
        } else { 0 }
    }
}

#[derive(Debug, Clone)]
struct Point2D {
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
struct Creature {
    start: Point2D,
    end: Point2D,
    kind: CreatureKind,
    value: i32
}

#[derive(Debug, Clone, PartialEq)]
enum CreatureKind {
    Empty,
    Symbol,
    Gear,
    PartNumber
}

#[derive(Debug, Clone, PartialEq)]
enum SearchState {
    NotFound,
    FoundPartNumber,
    FoundSymbol
}