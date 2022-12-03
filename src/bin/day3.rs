const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let input = INPUT;
    let rucksacks = to_rucksacks(input);
    let sum: u64 = find_repeats(&rucksacks).map(|x| x as u64).sum();
    println!("sum of priorities: {sum}");
}

#[test]
fn priorities() {
    assert_eq!(
        vec![vec![1, 26, 27, 52], vec![2, 25, 28, 51]],
        to_rucksacks("azAZ\nbyBY\n")
    );
}

#[test]
fn example_given() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    let rucksacks = to_rucksacks(input);
    let sum: u64 = find_repeats(&rucksacks).map(|x| x as u64).sum();
    assert_eq!(157, sum);
}

fn to_rucksacks(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .map(|x| {
            let items = x
                .bytes()
                .map(|x| {
                    if x.is_ascii_lowercase() {
                        x - b'a' + 1
                    } else if x.is_ascii_uppercase() {
                        x - b'A' + 27
                    } else {
                        panic!("unknow item '{x}")
                    }
                })
                .collect::<Vec<_>>();
            // ensure they can be split in equal sized compartiments
            assert!(
                items.len() % 2 == 0,
                "rucksack has even number of items: {}",
                items.len()
            );
            items
        })
        // the trailing \n create a empty sack, filter it out
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
}

fn find_repeats(rucksacks: &[Vec<u8>]) -> impl Iterator<Item = u8> + '_ {
    rucksacks.iter().map(|rucksack| {
        let mut items = [false; 52];
        let l = rucksack.len();
        for item in &rucksack[..l / 2] {
            items[*item as usize - 1] = true;
        }
        for item in &rucksack[l / 2..] {
            if items[*item as usize - 1] {
                return *item;
            }
        }
        panic!("rucksack has no wrong item: '{rucksack:?}")
    })
}
