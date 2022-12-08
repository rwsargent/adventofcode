use std::{io::Cursor, collections::HashMap, str::Lines};

use crate::reader::PuzzleInput;

#[derive(Clone, PartialEq)]
enum FsType {
    Dir,
    File
}

#[derive(Clone)]
struct FsNode {
    size: u32,
    name: String,
    parent: String,
    fs_type: FsType,
}


fn parse<'a>(mut fs: HashMap<String, FsNode>, mut lines: Lines<'a>, parent: &str) -> (HashMap<String, FsNode>, Lines<'a>, u32) {
    let mut dir_size = 0;
    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    return (fs, lines, dir_size);
                }
                let sub_dir_size;
                let dir_name = format!("{}/{}", parent, parts[2]);
                (fs, lines, sub_dir_size) = parse(fs, lines, dir_name.as_str());
                dir_size += sub_dir_size;
                match fs.get_mut(dir_name.as_str()) {
                    Some(dir) => {
                        dir.size += sub_dir_size;
                    },
                    None => {}
                }
            }
        }
        else {
            let (size, fs_type) = if parts[0] == "dir" {
                (0, FsType::Dir)
            } else {
                let size = parts[0].parse::<u32>().unwrap();
                dir_size += size;
                (size, FsType::File)
            };

            fs.insert(format!("{}/{}", parent, parts[1]), FsNode{
                size,
                name: parts[1].to_owned(),
                parent: parent.to_owned(),
                fs_type, 
            });   
        }
    }
    return (fs, lines, dir_size)
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_file("resources/day7.txt").unwrap();

    let (fs, _, _) = parse(HashMap::new(), input.as_lines(), "");
    // for (_, node) in fs {
    //     println!("{} ({})", node.name, node.size)
    // }
    // println!("total: {}", total);
    let sum:u32 =fs.iter().filter(|(_, p)| {
        p.fs_type == FsType::Dir && p.size <= 100000
    }).map(|(_, p)| p.size)
    .sum();
    println!("Sum: {}", sum);

}

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_file("resources/day7.txt").unwrap();

    let (fs, _, total_size) = parse(HashMap::new(), input.as_lines(), "");
    let available_space = 70000000 - total_size;
    let min:u32 = fs.iter()
        .filter(|(_, p)|p.fs_type == FsType::Dir && available_space + p.size >= 30000000)
        .map(|(_, p)| p.size)
        .min().unwrap();
    println!("Sum: {}", min);

}