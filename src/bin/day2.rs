const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let plays = to_plays(INPUT);
    let score = compute_score(plays);
    println!("reasoned strategy: {score}");

    let plays = to_plays(INPUT);
    let plays = apply_strategy(plays);
    let score = compute_score(plays);
    println!("actual strategy: {score}");
}

#[test]
fn example_given() {
    let input = "A Y\nB X\nC Z\n";
    let plays = to_plays(input);
    assert_eq!(15, compute_score(plays));
}

#[test]
fn actual_strategy() {
    let input = "A Y\nB X\nC Z\n";
    let plays = to_plays(input);
    let plays = apply_strategy(plays);
    assert_eq!(12, compute_score(plays));
}

fn to_plays(input: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    // the filter map is only filtering the last line, that is empty
    let plays = input.split('\n').filter_map(|x| {
        let mut s = x.split(' ');
        Some((
            s.next()?.bytes().next()? - b'A',
            s.next()?.bytes().next()? - b'X',
        ))
    });
    plays
}

fn apply_strategy<'a>(
    plays: impl Iterator<Item = (u8, u8)> + 'a,
) -> impl Iterator<Item = (u8, u8)> + 'a {
    plays.map(|(x, y)| (x, (x + y + 2) % 3))
}

fn compute_score(plays: impl Iterator<Item = (u8, u8)>) -> u64 {
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
