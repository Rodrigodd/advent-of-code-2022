const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let input = INPUT;
    let pos = find_start_packet(input).unwrap();
    println!("packet start at: {pos}");
}

#[test]
fn examples() {
    assert_eq!(
        Some(7),
        find_start_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    );
    assert_eq!(
        Some(5),
        find_start_packet("bvwbjplbgvbhsrlpgdmjqwftvncz")
    );
    assert_eq!(
        Some(6),
        find_start_packet("nppdvjthqldpwncqszvftbrmjlhg")
    );
    assert_eq!(
        Some(10),
        find_start_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    );
    assert_eq!(
        Some(11),
        find_start_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    );
}

fn find_start_packet(input: &str) -> Option<usize> {
    let mut was_seem = [0u8; 256];
    let mut diferent_count = 0;
    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        let curr = bytes[i] as usize;
        if was_seem[curr] == 0 {
            diferent_count += 1;
        }
        was_seem[curr] += 1;
        if i < 4 {
            continue;
        }

        let prev = bytes[i - 4] as usize;
        was_seem[prev] -= 1;
        if was_seem[prev] == 0 {
            diferent_count -= 1;
        }

        if diferent_count == 4 {
            return Some(i + 1);
        }
    }
    None
}
