fn main() {
    let input_part1 = vec![(47, 207), (84, 1394), (74, 1209), (67, 1014)];
    let input_part2 = vec![(47847467, 207139412091014)];

    println!("Part 1: {0}", calculate_race(input_part1));
    println!("Part 2: {0}", calculate_race(input_part2));
}

fn calculate_race(input: Vec<(i64, i64)>) -> i64 {
    input
        .into_iter()
        .map(|(upper_time_limit, distance)| (1..upper_time_limit)
            .map(|curr_time| if (curr_time * (upper_time_limit - curr_time) > distance) { 1 } else { 0 })
            .sum::<i64>())
        .product::<i64>()
}
