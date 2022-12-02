const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let mut calories: Vec<u64> = INPUT
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .collect();

    calories.sort_by_key(|x| !x);

    let max = calories[0];
    println!("top: {max}");

    let three_max = calories[0..3].iter().sum::<u64>();
    println!("sum of top 3: {three_max}");
}
