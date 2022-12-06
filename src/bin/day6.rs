const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let input = INPUT;
    let pos = find_start_packet(input).unwrap();
    println!("packet start at: {pos}");
    let pos = find_start_message(input).unwrap();
    println!("message start at: {pos}");
}

#[test]
fn start_packet() {
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

#[test]
fn start_message() {
    assert_eq!(
        Some(19),
        find_start_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    );
    assert_eq!(
        Some(23),
        find_start_message("bvwbjplbgvbhsrlpgdmjqwftvncz")
    );
    assert_eq!(
        Some(23),
        find_start_message("nppdvjthqldpwncqszvftbrmjlhg")
    );
    assert_eq!(
        Some(29),
        find_start_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    );
    assert_eq!(
        Some(26),
        find_start_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    );
}

fn find_start_packet(input: &str) -> Option<usize> {
    find_start(input, 4)
}

fn find_start_message(input: &str) -> Option<usize> {
    find_start(input, 14)
}

fn find_start(input: &str, n: usize) -> Option<usize> {
    let mut was_seem = [0u8; 256];
    let mut diferent_count = 0;
    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        let curr = bytes[i] as usize;
        if was_seem[curr] == 0 {
            diferent_count += 1;
        }
        was_seem[curr] += 1;
        if i < n {
            continue;
        }

        let prev = bytes[i - n] as usize;
        was_seem[prev] -= 1;
        if was_seem[prev] == 0 {
            diferent_count -= 1;
        }

        if diferent_count == n {
            return Some(i + 1);
        }
    }
    None
}
