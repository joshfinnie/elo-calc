mod lib;

use crate::lib::elo_rating;

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    let rating_1 = args[1].parse::<i32>().unwrap();
    let rating_2 = args[2].parse::<i32>().unwrap();
    let k = args[3].parse::<i32>().unwrap();
    let outcome = &args[4];

    let ratings = elo_rating(rating_1, rating_2, k, outcome);

    println!(
        "Player 1 Rating: {}\nPlayer 2 Rating: {}",
        ratings.0, ratings.1
    )
}
