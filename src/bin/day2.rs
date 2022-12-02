const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let score = compute_score(INPUT);
    println!("total score: {score}");
}

#[test]
fn example_given() {
    let input = "A Y\nB X\nC Z\n";
    assert_eq!(15, compute_score(input));
}

fn compute_score(input: &str) -> u64 {
    // the filter map is only filtering the last line, that is empty
    let plays = input.split('\n').filter_map(|x| {
        let mut s = x.split(' ');
        Some((
            s.next()?.bytes().next()? - b'A',
            s.next()?.bytes().next()? - b'X',
        ))
    });
    let mut score = 0;
    for play in plays {
        let move_score = match play.1 {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => panic!("unknown play '{play:?}'"),
        };
        let match_score = match play {
            (x, y) if x == (y + 1) % 3 => 0,
            (x, y) if x == y => 3,
            (x, y) if x == (y + 2) % 3 => 6,
            _ => panic!("unknown play '{play:?}'"),
        };
        score += move_score + match_score;
    }
    score
}
