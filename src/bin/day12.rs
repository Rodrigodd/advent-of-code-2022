const INPUT: &str = include_str!("../../inputs/day12.txt");

fn main() {
    let input = INPUT;
    let map: Map = to_heightmap(input);
    let (dist_start, dist_a) = find_dist(&map);
    println!("start dist: {}", dist_start);
    println!("hiking dist: {}", dist_a);
}

#[test]
fn example() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    let map: Map = to_heightmap(input);
    let (dist_start, dist_a) = find_dist(&map);
    assert_eq!(31, dist_start);
    assert_eq!(29, dist_a);
}

struct Map {
    start: u32,
    end: u32,
    width: u32,
    height: u32,
    heightmap: Vec<u8>,
}

fn to_heightmap(input: &str) -> Map {
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut height: u32 = 0;
    let mut i: u32 = 0;
    let heightmap: Vec<u8> = input
        .lines()
        .flat_map(|x| {
            height += 1;
            x.bytes()
                .map(|x| {
                    if x == b'S' {
                        start = i;
                        i += 1;
                        0
                    } else if x == b'E' {
                        end = i;
                        i += 1;
                        b'z' - b'a'
                    } else {
                        i += 1;
                        x - b'a'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let width = heightmap.len() as u32 / height;
    Map {
        height,
        width,
        heightmap,
        start,
        end,
    }
}

struct Tile {
    visited: bool,
    previous_tile: Option<u32>,
    height: u8,
}

fn find_dist(map: &Map) -> (u32, u32) {
    let &Map {
        start,
        end,
        width,
        height,
        ref heightmap,
    } = map;
    let mut tiles = Vec::from_iter(heightmap.iter().map(|x| Tile {
        visited: false,
        previous_tile: None,
        height: *x,
    }));
    let mut front = Vec::new();
    let mut next_front = vec![end];
    tiles[end as usize].visited = true;

    let mut closest_a = None;
    let mut dist = 0;
    'search: while !next_front.is_empty() {
        std::mem::swap(&mut front, &mut next_front);
        dist += 1;
        next_front.clear();

        for t in front.iter().copied() {
            for n in neighbors(t, width, height) {
                let n = n as usize;
                if tiles[n].visited {
                    continue;
                }
                if tiles[t as usize].height > tiles[n].height + 1 {
                    continue;
                }

                if tiles[n].height == 0 && closest_a.is_none() {
                    closest_a = Some(dist);
                }
                if n == start as usize {
                    break 'search;
                }

                tiles[n].previous_tile = Some(t);
                tiles[n].visited = true;
                next_front.push(n as u32);
            }
        }
    }

    (dist, closest_a.unwrap())
}

fn neighbors(
    t: u32,
    width: u32,
    height: u32,
) -> impl Iterator<Item = u32> {
    (0..4).filter_map(move |i| {
        let n = match i {
            0 => {
                if t % width >= width - 1 {
                    return None;
                }
                t + 1
            }
            1 => {
                if t >= width * (height - 1) {
                    return None;
                }
                t + width
            }
            2 => {
                if t % width == 0 {
                    return None;
                }
                t - 1
            }
            3 => {
                if t < width {
                    return None;
                }
                t - width
            }
            _ => unreachable!(),
        };
        Some(n)
    })
}
