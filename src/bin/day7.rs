const INPUT: &str = include_str!("../../inputs/day7.txt");

fn main() {
    let input = INPUT;
    let tree = build_tree(input);
    println!(
        "sum is: {}",
        total_size_folders_less_than_100_000(&tree)
    );
    println!("need to delete: {}", size_of_folder_to_delete(&tree));
}

#[test]
fn example() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    let tree = build_tree(input);
    dbg!(&tree);
    assert_eq!(95437, total_size_folders_less_than_100_000(&tree));
    assert_eq!(24933642, size_of_folder_to_delete(&tree));
}

#[derive(Debug)]
pub enum Node {
    File {
        name: String,
        size: u64,
    },
    Folder {
        name: String,
        nodes: Vec<usize>,
        parent: usize,
    },
}

fn build_tree(input: &str) -> Vec<Node> {
    use Node::*;
    let mut tree = Vec::new();
    let mut curr = None;
    for line in input.lines() {
        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(dir) = command.strip_prefix("cd") {
                let dir = dir.trim();
                if curr.is_none() {
                    tree.push(Folder {
                        name: dir.to_string(),
                        nodes: Vec::new(),
                        parent: 0,
                    });
                    curr = Some(0);
                    continue;
                }
                let Folder { nodes, parent,..} = &tree[curr.unwrap()] else{
                    panic!("curr is not Folder??");
                };
                if dir == ".." {
                    curr = Some(*parent);
                    continue;
                }
                let find = nodes.iter().copied().find(|x| {
                    matches!(
                        &tree[*x],
                        Folder { name, .. } if name == dir
                    )
                });

                if let Some(node) = find {
                    curr = Some(node);
                } else {
                    let i = tree.len();
                    let c = curr.unwrap();
                    let Folder { nodes,.. } = &mut tree[c] else {
                        panic!("curr is not Folder??");
                    };
                    nodes.push(i);
                    tree.push(Folder {
                        name: dir.to_string(),
                        nodes: Vec::new(),
                        parent: c,
                    });
                    curr = Some(i);
                }
            }
        } else if let Some(dir) = line.strip_prefix("dir") {
            let dir = dir.trim();
            let i = tree.len();
            let c = curr.unwrap();
            let Folder { nodes,.. } = &mut tree[c] else {
                panic!("curr is not Folder??");
            };
            nodes.push(i);
            tree.push(Folder {
                name: dir.to_string(),
                nodes: Vec::new(),
                parent: c,
            });
        } else {
            let Some((size, file)) = line.split_once(' ') else {
                panic!("not split");
            };
            let size = size.parse::<u64>().unwrap();
            let file = file.trim();

            let i = tree.len();
            let c = curr.unwrap();
            let Folder { nodes,.. } = &mut tree[c] else {
                panic!("curr is not Folder??");
            };
            nodes.push(i);
            tree.push(File {
                name: file.to_string(),
                size,
            });
        }
    }
    tree
}

fn size_of(tree: &Vec<Node>, i: usize) -> u64 {
    match &tree[i] {
        Node::File { size, .. } => *size,
        Node::Folder { nodes, .. } => {
            nodes.iter().copied().map(|x| size_of(tree, x)).sum()
        }
    }
}

fn total_size_folders_less_than_100_000(tree: &Vec<Node>) -> u64 {
    let mut sum = 0;
    let mut parents = vec![0];
    while let Some(curr) = parents.pop() {
        if let Node::Folder { nodes, .. } = &tree[curr] {
            let size = size_of(tree, curr);
            if size <= 100_000 {
                sum += size;
            }
            parents.extend_from_slice(nodes);
        }
    }
    sum
}

fn size_of_folder_to_delete(tree: &Vec<Node>) -> u64 {
    let used_size = size_of(tree, 0);
    let free_size = 70_000_000 - used_size;
    let need_to_delete = 30_000_000u64.wrapping_sub(free_size);
    let mut min_size = u64::MAX;
    let mut parents = vec![0];
    while let Some(curr) = parents.pop() {
        if let Node::Folder { nodes, .. } = &tree[curr] {
            let size = size_of(tree, curr);
            if size >= need_to_delete && size < min_size {
                min_size = size;
            }
            parents.extend_from_slice(nodes);
        }
    }
    min_size
}
