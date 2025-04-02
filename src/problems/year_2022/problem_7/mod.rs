use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Debug)]
struct File {
    size: u64,
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<Directory>>,
    files: RefCell<Vec<File>>,
    subdirs: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<Directory>>) -> Self {
        Directory {
            name,
            parent,
            files: RefCell::new(Vec::new()),
            subdirs: RefCell::new(HashMap::new()),
        }
    }

    fn get_total_size(&self) -> u64 {
        // Sum of all files in this directory
        let file_size: u64 = self.files.borrow().iter().map(|f| f.size).sum();
        // Add sizes of all subdirectories
        let subdir_size: u64 = self
            .subdirs
            .borrow()
            .values()
            .map(|d| d.get_total_size())
            .sum();
        file_size + subdir_size
    }
}

fn parse_terminal_output(lines: &[String]) -> Rc<Directory> {
    let root = Rc::new(Directory::new("/".to_string(), None));
    let mut current_dir = Rc::clone(&root);
    
    let mut lines_iter = lines.iter().peekable();

    while let Some(line) = lines_iter.next() {
        if line.starts_with("$ ") {
            let cmd = line[2..].trim();
            if cmd.starts_with("cd ") {
                let target = cmd[3..].trim();
                match target {
                    "/" => current_dir = Rc::clone(&root),
                    ".." => {
                        if let Some(parent) = &current_dir.parent {
                            current_dir = Rc::clone(parent);
                        }
                    }
                    dir_name => {
                        let maybe_subdir_clone = current_dir.subdirs.borrow().get(dir_name).map(Rc::clone);
                        if let Some(subdir_clone) = maybe_subdir_clone {
                            current_dir = subdir_clone;
                        } else {
                            eprintln!("Warning: Attempted to cd into non-existent directory \'{}\' in \'{}\'", dir_name, current_dir.name);
                        }
                    }
                }
            }
        } else if line.starts_with("dir ") {
            let dir_name = line[4..].trim().to_string();
            if !current_dir.subdirs.borrow().contains_key(&dir_name) {
                let new_dir = Rc::new(Directory::new(
                    dir_name.clone(),
                    Some(Rc::clone(&current_dir)),
                ));
                current_dir.subdirs.borrow_mut().insert(dir_name, Rc::clone(&new_dir));
            }
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(size) = parts[0].parse::<u64>() {
                    let file = File {
                        size,
                    };
                    current_dir.files.borrow_mut().push(file);
                } else {
                    eprintln!("Warning: Could not parse file size from line: {}", line);
                }
            } else if !line.is_empty() {
                eprintln!("Warning: Unrecognized line format: {}", line);
            }
        }
    }
    
    root
}

fn find_small_directories(root: &Rc<Directory>, max_size: u64) -> Vec<Rc<Directory>> {
    let mut small_dirs = Vec::new();
    
    fn traverse(dir: &Rc<Directory>, max_size: u64, small_dirs: &mut Vec<Rc<Directory>>) {
        let size = dir.get_total_size();
        if size <= max_size {
            small_dirs.push(Rc::clone(dir));
        }
        for subdir in dir.subdirs.borrow().values() {
            traverse(subdir, max_size, small_dirs);
        }
    }
    
    traverse(root, max_size, &mut small_dirs);
    small_dirs
}

fn solve_part1(input_text: &str) -> u64 {
    let lines: Vec<String> = input_text.lines().map(String::from).collect();
    let root = parse_terminal_output(&lines);
    let small_dirs = find_small_directories(&root, 100_000);
    small_dirs.iter().map(|d| d.get_total_size()).sum()
}

fn find_smallest_directory_to_delete(root: &Rc<Directory>, required_space: u64) -> Option<u64> {
    let total_used = root.get_total_size();
    let total_space = 70_000_000;
    let current_free = total_space - total_used;
    let space_needed = required_space - current_free;

    let mut candidates = Vec::new();
    
    fn traverse(dir: &Rc<Directory>, space_needed: u64, candidates: &mut Vec<u64>) {
        let size = dir.get_total_size();
        if size >= space_needed {
            candidates.push(size);
        }
        for subdir in dir.subdirs.borrow().values() {
            traverse(subdir, space_needed, candidates);
        }
    }
    
    traverse(root, space_needed, &mut candidates);
    candidates.into_iter().min()
}

fn solve_part2(input_text: &str) -> u64 {
    let lines: Vec<String> = input_text.lines().map(String::from).collect();
    let root = parse_terminal_output(&lines);
    find_smallest_directory_to_delete(&root, 30_000_000)
        .expect("No directory found that would free up enough space")
}

pub fn solve() {
    println!("Problem 6");
    let input_path = PathBuf::from("src/problems/year_2022/problem_6/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let result1 = solve_part1(&input_text);
    let result2 = solve_part2(&input_text);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
} 