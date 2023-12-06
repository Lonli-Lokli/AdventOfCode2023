use std::fs::{ read_to_string};

static RED_COUNT: i32 = 12;
static GREEN_COUNT: i32 = 13;
static BLUE_COUNT: i32 = 14;

fn main() {
    println!("Part 1 - {}", parse("D:/github/AdventOfCode2023/puzzle_02/src/input.txt", line_processor_part1, game_score_extractor_part1));
    println!("Part 2 - {}", parse("D:/github/AdventOfCode2023/puzzle_02/src/input.txt", line_processor_part2, game_score_extractor_part2));
}


fn parse<P, E>(filename: &str, mut line_processor: P, game_score_extractor: E) -> i32
    where P: FnMut(&str) -> Option<Game>, E : FnMut(Option<Game>) -> i32 {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(|line| line_processor(line))
        .map(game_score_extractor)
        .sum()
}
fn line_processor_part1(line: &str) -> Option<Game> {
    let game = parse_game(line);

    if get_game_ball_count_by_color(&game, Color::Red) <= RED_COUNT
        && get_game_ball_count_by_color(&game, Color::Green) <= GREEN_COUNT
        && get_game_ball_count_by_color(&game, Color::Blue) <= BLUE_COUNT
    { Some(game) }
    else {
        None
    }
}

fn game_score_extractor_part1(game: Option<Game>) -> i32 {
    game.map_or_else(|| 0, |g| g.id)
}

fn line_processor_part2(line: &str) -> Option<Game> {
    Some(parse_game(line))
}

fn game_score_extractor_part2(game: Option<Game>) -> i32 {
    game.map_or_else(|| 0, |g| get_game_power(g))
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.trim().splitn(2, ':').collect();
    Game {
        id: parse_game_id( parts[0]),
        rounds: parse_rounds(parts[1])
    }
}

fn get_game_power(game: Game) -> i32 {
    get_game_ball_count_by_color(&game, Color::Red)
        * get_game_ball_count_by_color(&game, Color::Green)
        * get_game_ball_count_by_color(&game, Color::Blue)
}

fn parse_game_id(game_id_str: &str) -> i32 {
    game_id_str // 'Game 1'
        .trim()
        .strip_prefix("Game ")
        .and_then(|num| num.parse::<i32>().ok()).unwrap()
}

fn parse_rounds(round_str: &str) -> Vec<Round> {
    round_str // '3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green'
        .trim()
        .split(';')
        .map(|round_desc| { // 3 blue, 4 red
            let round_balls: Vec<Ball> = round_desc.split(',').flat_map(|balls_str| {
                let balls_part: Vec<_> = balls_str.trim().split_whitespace().collect();
                let count = balls_part[0].parse::<i32>().unwrap_or(0);
                let color = parse_color(balls_part[1]);
                (0..count)
                    .map(|_| Ball { color: color.clone() })
                    .collect::<Vec<_>>()
            }).collect();

            Round { balls: round_balls }
        })
        .collect()
}

fn get_game_ball_count_by_color(game: &Game, color: Color) -> i32 {
    game.rounds.iter().map(|round| round.balls.iter().filter(|ball| ball.color == color).count()).max().unwrap_or(0) as i32
}

fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "blue" => Color::Blue,
        "green" => Color::Green,
        "red" => Color::Red,
        _ => Color::Red,
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
struct Round {
    balls: Vec<Ball>,
}

#[derive(Debug, Clone)]
struct Ball {
    color: Color,
}

#[derive(PartialEq, Debug, Clone)]
enum Color {
    Blue,
    Green,
    Red,
}