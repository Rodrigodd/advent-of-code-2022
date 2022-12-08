const INPUT: &str = include_str!("../../inputs/day8.txt");

fn main() {
    let input = INPUT;
    let trees = to_height_grid(input);
    let visibles = visibility_grid(trees);
    println!(
        "number of visible tree: {}",
        visibles
            .values
            .iter()
            .copied()
            .map(|x| x as u64)
            .sum::<u64>()
    )
}

#[test]
fn example() {
    let input = "30373
25512
65332
33549
35390
";
    let trees = to_height_grid(input);
    let visibles = visibility_grid(trees);
    assert_eq!(
        21,
        visibles
            .values
            .iter()
            .copied()
            .map(|x| x as u64)
            .sum::<u64>()
    )
}

struct Grid<T: Copy> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

fn to_height_grid(input: &str) -> Grid<u8> {
    let mut height = 0;
    let values: Vec<u8> = input
        .lines()
        .flat_map(|x| {
            height += 1;
            x.bytes().map(|x| x - b'0')
        })
        .collect();
    let width = values.len() / height;
    assert_eq!(width * height, values.len());
    Grid {
        width,
        height,
        values,
    }
}

fn visibility_grid(trees: Grid<u8>) -> Grid<bool> {
    let w = trees.width;
    let h = trees.height;
    let mut visibility = vec![false; trees.values.len()];
    let trees = trees.values;

    // border is always visible
    for j in 0..w {
        visibility[j] = true;
        visibility[(h - 1) * w + j] = true;
    }
    for i in 1..h - 1 {
        visibility[i * w] = true;
        visibility[i * w + w - 1] = true;
    }

    for i in 1..h - 1 {
        let mut max_left = trees[i * w];
        for j in 1..w - 1 {
            let height = trees[i * w + j];
            if height > max_left {
                visibility[i * w + j] = true;
                max_left = height;
            }
        }

        let mut max_right = trees[i * w + w - 1];
        for j in (1..w - 1).rev() {
            let height = trees[i * w + j];
            if height > max_right {
                visibility[i * w + j] = true;
                max_right = height;
            }
        }
    }

    for j in 1..w - 1 {
        let mut max_up = trees[j];
        for i in 1..h - 1 {
            let height = trees[i * w + j];
            if height > max_up {
                visibility[i * w + j] = true;
                max_up = height;
            }
        }

        let mut max_down = trees[(h - 1) * w + j];
        for i in (1..h - 1).rev() {
            let height = trees[i * w + j];
            if height > max_down {
                visibility[i * w + j] = true;
                max_down = height;
            }
        }
    }

    Grid {
        width: w,
        height: h,
        values: visibility,
    }
}
