use std::fs;

struct Dir {
    name: String,
    size: usize,
    files: Vec<(String, usize)>,
    dirs: Vec<Dir>,
}


fn part_1(root: &Dir) -> usize {
    let mut size = if root.size <= 100000 {root.size} else {0};
    for dir in &root.dirs {
        size += part_1(&dir);
    }
    size
}


fn part_2(root: &Dir, free: usize) -> usize {
    let mut size = 70000000;
    if free + root.size >= 30000000 {
        size = root.size;
    }
    for dir in &root.dirs {
        let child_size = part_2(&dir, free);
        if child_size < size {
            size = child_size;
        }
    }
    size
}


fn parse_fs(cmds: &[&str], cwd: &str, mut i: usize) -> (usize, Dir) {
    let mut curr_dir = Dir {
        name: String::from(cwd),
        size: 0,
        files: Vec::new(),
        dirs: Vec::new()
    };

    let cmd: Vec<&str> = cmds[i].split(" ").collect();

    // skip ls
    if cmd[1] == "ls" {
        i += 1;
    }

    while i < cmds.len() {
        let cmd: Vec<&str> = cmds[i].split(" ").collect();
        if cmd[1] == "cd" &&
           cmd[2] == ".." {
            return (i+1, curr_dir);
        }
        else {
            if cmd[1] == "cd" {
                let (r_i, child_dir) = parse_fs(cmds, cmd[2], i+1);
                curr_dir.size += child_dir.size;
                curr_dir.dirs.push(child_dir);
                i = r_i;
            }
            else {
                if cmd[0] != "dir" {
                    let size: usize = cmd[0].parse().unwrap();
                    curr_dir.files.push((String::from(cmd[1]), size));
                    curr_dir.size += size;
                }
                i += 1;
            }
        }
    }
    (i, curr_dir)
}

fn day07(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let cmds: Vec<&str> = data.lines().collect();

    let (i, root) = parse_fs(&cmds, "/", 1);
    println!("Part 1: {}", part_1(&root));

    println!("Part 2: {}", part_2(&root, 70000000 - root.size));

}


fn main() {
    day07("input/input.txt");
}
