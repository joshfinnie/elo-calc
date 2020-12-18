fn probability(rank_a: i32, rank_b: i32) -> f32 {
    let base: f32 = 10.0;
    let diff: f32 = (rank_b - rank_a) as f32;
    return 1.0 / (1.0 + base.powf(diff / 400.0));
}

pub fn elo_rating(rank1: i32, rank2: i32, k: i32, d: &str) -> (i32, i32) {
    let p1 = probability(rank1, rank2);
    let p2 = probability(rank2, rank1);

    match d {
        "w" | "W" => (
            rank1 + (k as f32 * (1.0 - p1)).round() as i32,
            rank2 + (k as f32 * (0.0 - p2)).round() as i32,
        ),
        "l" | "L" => (
            rank1 + (k as f32 * (0.0 - p1)).round() as i32,
            rank2 + (k as f32 * (1.0 - p2)).round() as i32,
        ),
        "d" | "D" => (
            rank1 + (k as f32 * (0.5 - p1)).round() as i32,
            rank2 + (k as f32 * (0.5 - p2)).round() as i32,
        ),
        _ => panic!("Can only have a W, L, or D"),
    }
}

#[test]
fn test_elo() {
    assert_eq!(elo_rating(1200, 1300, 40, "W"), (1226, 1274));
    assert_eq!(elo_rating(1200, 1300, 40, "L"), (1186, 1314));
    assert_eq!(elo_rating(1200, 1300, 40, "D"), (1206, 1294));
}

#[test]
fn test_elo_high() {
    assert_eq!(elo_rating(2600, 2300, 16, "W"), (2602, 2298));
    assert_eq!(elo_rating(2600, 2300, 16, "L"), (2586, 2314));
    assert_eq!(elo_rating(2600, 2300, 16, "D"), (2594, 2306));
}
