use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

static NEEDED_DISK_SPACE: usize = 30_000_000;
static TOTAL_DISK_SPACE: usize = 70_000_000;

#[derive(Debug, Clone, Default)]
struct Directory {
    name: OsString,
    files: usize,
    directories: Vec<Directory>,
}

impl Directory {
    fn size(&self) -> usize {
        let size = self.directories.iter().fold(0, |acc, dir| acc + dir.size());
        self.files + size
    }

    fn flatten(&self) -> Vec<Directory> {
        let mut out = Vec::new();
        let mut stack = self.directories.clone();

        while let Some(dir) = stack.pop() {
            let size = dir.size();
            out.push(Directory {
                name: dir.name,
                files: size,
                directories: Vec::new(),
            });
            stack.extend(dir.directories.into_iter());
        }

        out
    }
}

impl Directory {
    fn get_dir(&mut self, path: &Path) -> &mut Directory {
        let mut dir = self;
        for component in path.iter().skip(1) {
            let idx =
                if let Some(idx) = dir.directories.iter().position(|dir| dir.name == component) {
                    idx
                } else {
                    dir.directories.push(Directory {
                        name: component.to_owned(),
                        files: 0,
                        directories: Vec::new(),
                    });
                    dir.directories.len() - 1
                };
            dir = &mut dir.directories[idx]
        }
        dir
    }
}

fn main() {
    let mut path = PathBuf::new();
    let mut root = Directory::default();

    for line in std::io::stdin().lines().map(Result::unwrap) {
        if line.starts_with("$ cd") {
            let target = line.splitn(3, ' ').nth(2).unwrap();
            if target == "/" {
                path.clear();
                path.push("/");
            } else if target == ".." {
                path.pop();
            } else {
                path.push(target);
            }
            continue;
        }

        if line.starts_with("$ ls") | line.starts_with("dir") {
            continue;
        }

        let mut split = line.split(' ');
        let size: usize = split.next().unwrap().parse().unwrap();

        let dir = root.get_dir(&path);
        dir.files += size;
    }

    let flattened = root.flatten();
    let sum: usize = flattened
        .iter()
        .filter(|dir| dir.size() <= 100000)
        .map(|dir| dir.size())
        .sum();
    println!("Part 1: {}", sum);

    let more_space_needed = NEEDED_DISK_SPACE - (TOTAL_DISK_SPACE - root.size());
    let smallest_matching = flattened
        .iter()
        .filter(|dir| dir.size() >= more_space_needed)
        .min_by_key(|dir| dir.size())
        .unwrap();
    println!("Part 2: {}", smallest_matching.size());
}
