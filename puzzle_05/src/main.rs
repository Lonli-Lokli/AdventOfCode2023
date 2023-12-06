use std::fs::{read_to_string};
use std::iter::zip;
use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::*;


fn main() {
    println!("Part 1 - {}", parse("D:/github/AdventOfCode2023/puzzle_05/src/input.txt",
                                  parse_seeds_part1));
    println!("Part 2 - {}", parse("D:/github/AdventOfCode2023/puzzle_05/src/input.txt",
                                  parse_seeds_part2));
}

fn parse<E>(filename: &str, parse_ranges: E) -> i128
    where E : Fn(&Vec<&str>) -> Vec<Range>  {
    let file_content = read_to_string(filename).unwrap();
    let blocks = file_content.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut ranges = parse_ranges(&blocks);
    let conversions = parse_conversions(&blocks);


    let m = MultiProgress::new();
    let min = ranges.par_iter().map(|range| {
        let mut min_seed = i128::MAX;
        let pb = m.add(ProgressBar::new(range.length as u64));
        for seed in range.start..range.end + 1 {
            let mut plant = Plant {id: seed, status: PlantStatus::Seed};
            while plant.status != PlantStatus::Location {
                move_to_next_status(&mut plant, &conversions)
            }
            pb.inc(1);
            min_seed = min_seed.min(plant.id);
        }
        pb.finish_with_message("Completed");
        min_seed
    }).min().unwrap();
    m.clear().unwrap();
    min
}

fn parse_seeds_part1(blocks: &Vec<&str>) -> Vec<Range> {
    blocks[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.trim()
        .parse::<i128>
    ().ok().unwrap()).map(|seed| Range {start: seed, end: seed, length: 1})
        .collect()
}
fn parse_conversions(blocks: &Vec<&str>) -> Vec<Conversion> {
    blocks.iter().skip(1).map(|block| {
        let lines = block.lines().collect::<Vec<&str>>();
        let (initial_status, final_status) = parse_conversion_type(lines[0]);
        lines.iter().skip(1).map(|line| {
            let ranges = line.split_whitespace().collect::<Vec<&str>>();

            let dest = ranges[0].parse::<i128>().unwrap();
            let source = ranges[1].parse::<i128>().unwrap();
            let length = ranges[2].parse::<i128>().unwrap();
            Conversion {
                from: Range { start: source, end: source + length - 1, length },
                to: Range { start: dest, end: dest + length - 1, length },
                initial_status,
                final_status
            }
        }).collect::<Vec<_>>()
    }).flatten().collect()
}

fn move_to_next_status(plant: &mut Plant, conversions: &Vec<Conversion>) {
    let actions = conversions.iter().filter(|c| c.initial_status == plant.status);
    let related = actions.filter(|a| plant.id >= a.from.start && plant.id <= a.from.end).next();
    plant.id = related.map_or(plant.id, |act| act.to.start - act.from.start + plant.id );
    plant.status = conversions.iter().filter(|c| c.initial_status == plant.status).next().unwrap().final_status;
}
fn parse_conversion_type(line: &str) -> (PlantStatus, PlantStatus) {
    match line {
        "seed-to-soil map:" => (PlantStatus::Seed, PlantStatus::Soil),
        "soil-to-fertilizer map:" => (PlantStatus::Soil, PlantStatus::Fertilizer),
        "fertilizer-to-water map:" => (PlantStatus::Fertilizer, PlantStatus::Water),
        "water-to-light map:" => (PlantStatus::Water, PlantStatus::Light),
        "light-to-temperature map:" => (PlantStatus::Light, PlantStatus::Temperature),
        "temperature-to-humidity map:" => (PlantStatus::Temperature, PlantStatus::Humidity),
        "humidity-to-location map:" => (PlantStatus::Humidity, PlantStatus::Location),
        _ => panic!()
    }

}

fn parse_seeds_part2(blocks: &Vec<&str>) -> Vec<Range> {
    let numbers = blocks[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| x.trim().parse::<i128>().ok().unwrap())
        .collect::<Vec<i128>>();
    let first = &numbers.iter().enumerate().filter(|(idx, _)| idx % 2 == 0).map(|(_, n)| *n)
        .collect::<Vec<i128>>();
    let second =  &numbers.iter().enumerate().filter(|(idx, _)| idx % 2 == 1).map(|(_, n)| *n)
        .collect::<Vec<i128>>();
    zip(first, second)
        .map(|(x, y)| Range {start: *x, end: *x + *y, length: *y})
        .collect()
}

#[derive(Debug, Clone)]
struct Plant {
    id: i128,
    status: PlantStatus,
}

#[derive(Debug, Clone)]
struct Conversion {
    from: Range,
    to: Range,
    initial_status: PlantStatus,
    final_status: PlantStatus
}

#[derive(Debug, Clone)]
struct Range {
    start: i128,
    end: i128,
    length: i128
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum PlantStatus {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}