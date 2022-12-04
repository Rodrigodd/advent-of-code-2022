const INPUT: &str = include_str!("../../inputs/day4.txt");

/// Range of sections.
#[derive(Clone, Copy)]
struct Range {
    // start of the range, inclusive
    start: u32,
    // ebd of the range, inclusive
    end: u32,
}

fn main() {
    let input = INPUT;

    let pairs = to_pairs(input);
    let count = count_fully_contained(pairs);
    println!("number of fully contained pairs: {count}");

    let pairs = to_pairs(input);
    let count = count_overlapping(pairs);
    println!("number of contained pairs: {count}");
}

#[test]
fn example() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    let pairs = to_pairs(input);
    let count = count_fully_contained(pairs);
    assert_eq!(2, count);
    let pairs = to_pairs(input);
    let count = count_overlapping(pairs);
    assert_eq!(4, count);
}

fn to_pairs(
    input: &str,
) -> impl Iterator<Item = (Range, Range)> + '_ {
    input
        .split('\n')
        // filter out last line, that is empty
        .filter_map(|x| {
            let mut ranges = x.split(',').filter_map(|x| {
                let mut sections = x
                    .split('-')
                    .filter_map(|x| x.parse::<u32>().ok());
                Some(Range {
                    start: sections.next()?,
                    end: sections.next()?,
                })
            });
            Some((ranges.next()?, ranges.next()?))
        })
}

fn count_fully_contained(
    pairs: impl Iterator<Item = (Range, Range)>,
) -> usize {
    pairs
        .filter(|(a, b)| {
            a.start <= b.start && a.end >= b.end
                || b.start <= a.start && b.end >= a.end
        })
        .count()
}

fn count_overlapping(
    pairs: impl Iterator<Item = (Range, Range)>,
) -> usize {
    pairs
        .filter(|(a, b)| a.start <= b.end && a.end >= b.start)
        .count()
}
