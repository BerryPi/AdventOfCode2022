use std::{cell::{RefCell}, rc::Rc};

use itertools::Itertools;

#[derive(Debug)]
enum FsItem {
    File(String, i32),
    Dir(String, Vec<usize>)
}

fn parse_file(line: &str) -> FsItem {
    let (size_s, name) = line.split_once(' ').unwrap();
    FsItem::File(name.to_string(), size_s.parse::<i32>().unwrap())
}

fn parse_dir(line: &str) -> FsItem {
    let (_, name) = line.split_once(' ').unwrap();
    FsItem::Dir(name.to_string(), Vec::new())
}

fn handle_ls<'a>(pwd: Rc<RefCell<FsItem>>, filesystem: &mut Vec<Rc<RefCell<FsItem>>>, lines: &mut impl Iterator<Item = &'a String>) {
    if let FsItem::Dir(_, contents) = &mut *pwd.borrow_mut() {
        for next_line in lines {
            // If the line starts with a digit, it's a file + size
            if next_line.chars().next().unwrap().is_numeric() {
                let file = parse_file(next_line);
                let file_id = filesystem.len();
                filesystem.push(Rc::new(RefCell::new(file)));
                contents.push(file_id);
            }
            // Assume it's a dir otherwise
            else {
                let dir = parse_dir(next_line);
                let dir_id = filesystem.len();
                filesystem.push(Rc::new(RefCell::new(dir)));
                contents.push(dir_id);
            }
        }
    }
}

fn handle_cd(path_stack: &mut Vec<usize>, filesystem: &[Rc<RefCell<FsItem>>], target: &str) {
    // To root
    if "target" == "/" {
        while path_stack.len() > 1 {
            path_stack.pop();
        }
    }
    // Go to parent, unless we're at root
    else if target == ".." {
        if path_stack.len() > 1 {
            path_stack.pop();
        }
    }
    // Otherwise push the named directory onto the stack
    else {
        let pwd = filesystem[*path_stack.last().unwrap()].clone();
        if let FsItem::Dir(_, contents) = &*pwd.borrow_mut() {
            for item_id in contents {
                let curr_item = filesystem[*item_id].clone();
                if let FsItem::Dir(name, _) = &*curr_item.borrow() {
                    if name == target {
                        path_stack.push(*item_id);
                        break;
                    }
                };
            }
        };
    }
}

// Returns the root node
fn parse_input(input: Vec<String>) -> Vec<Rc<RefCell<FsItem>>> {
    let root = FsItem::Dir("/".to_string(), Vec::new());
    let mut path_stack = vec![0];
    let mut filesystem = vec![Rc::new(RefCell::new(root))];

    let mut line_iter = input.iter();
    while let Some(next_line) = line_iter.next() {
        if next_line.starts_with("$ ls") {
            let mut contents = line_iter.take_while_ref(|l| !l.starts_with('$'));
            let pwd = filesystem[*path_stack.last().unwrap()].clone();
            handle_ls(pwd, &mut filesystem, &mut contents);
        }
        else if next_line.starts_with("$ cd") {
            let parts = next_line.split(' ');
            let target = parts.last().unwrap();
            handle_cd(&mut path_stack, &filesystem, target);
        }
    }
    filesystem
}

// Returning the total size would be the clean way to do this, but this way we get cached
// values out and avoid redoing calculations.
fn get_total_size(id: usize, filesystem: &Vec<Rc<RefCell<FsItem>>>, cache: &mut Vec<i32>) -> i32{
    let item = &*filesystem[id].borrow();
    match item {
        FsItem::File(_, size) => {
            cache[id] = *size;
            *size
        },
        FsItem::Dir(_, contents) => {
            let mut total = 0;
            for child in contents {
                total += get_total_size(*child, filesystem, cache);
            }
            cache[id] = total;
            total
        }
    }
}

pub fn part1(input: Vec<String>) -> String {
    let filesystem = parse_input(input);
    let mut item_sizes = vec![0; filesystem.len()];
    let _ = get_total_size(0, &filesystem, &mut item_sizes);

    let with_sizes = filesystem.iter().zip(item_sizes);
    let meets_conditions = with_sizes
        .filter(|(item, size)| matches!(*item.borrow(), FsItem::Dir(_, _)) && *size <= 100_000);
    let total_size = meets_conditions
        .map(|(_, size)| size)
        .sum::<i32>();
    
    total_size.to_string()
}

pub fn part2(input: Vec<String>) -> String {
    let filesystem = parse_input(input);
    let mut item_sizes = vec![0; filesystem.len()];
    let _ = get_total_size(0, &filesystem, &mut item_sizes);

    let total_space = 70_000_000;
    let needed_free_space = 30_000_000;
    let used_space = item_sizes[0];
    let actual_free_space = total_space - used_space;
    
    let space_to_free = needed_free_space - actual_free_space;

    let with_sizes = filesystem.iter().zip(item_sizes);
    let meets_conditions = with_sizes
        .filter(|(item, size)| matches!(*item.borrow(), FsItem::Dir(_, _)) && *size >= space_to_free);
    
    let smallest_qualifier = meets_conditions.min_by_key(|(_, size)| *size);
    let smallest_size = smallest_qualifier.unwrap().1;

    smallest_size.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pretty_print_item(filesystem: &Vec<Rc<RefCell<FsItem>>>, dir: usize, indent_level: usize) {
        let item = &*filesystem[dir].borrow();
        if let FsItem::File(name, size) = item {
            println!("{:indent$} - {} {}", "", name, size, indent=indent_level * 2);
        }
        else if let FsItem::Dir(name, contents) = item {
            println!("{:indent$} - {}", "", name, indent=indent_level * 2);
            for desc in contents {
                pretty_print_item(filesystem, *desc, indent_level + 1);
            }
        }
    }

    #[test]
    fn test_input_parse() {
        let input = vec!["$ cd /", "$ ls", "dir a", "14848514 b.txt", "8504156 c.dat", "dir d", "$ cd a", "$ ls", "dir e",
            "29116 f", "2557 g", "62596 h.lst", "$ cd e", "$ ls", "584 i", "$ cd ..", "$ cd ..", "$ cd d", "$ ls", "4060174 j",
            "8033020 d.log", "5626152 d.ext", "7214296 k"]
            .iter().map(|&s| s.into()).collect::<Vec<String>>();
        
        let filesystem = parse_input(input);
        pretty_print_item(&filesystem, 0, 0)
    }
}