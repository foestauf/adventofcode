pub fn solve() {
    // Helper to parse rock paths and fill a cave
    fn fill_cave(paths: &[&str], width: usize, height: usize, with_floor: bool) -> Vec<Vec<char>> {
        let mut cave = vec![vec!['.'; width]; height];
        if with_floor {
            for x in 0..width {
                cave[height - 1][x] = '#';
            }
        }
        for path in paths {
            let points: Vec<(usize, usize)> = path.split(" -> ")
                .map(|p| {
                    let coords: Vec<usize> = p.split(',')
                        .map(|c| c.parse().unwrap())
                        .collect();
                    (coords[0], coords[1])
                })
                .collect();
            for i in 0..points.len() - 1 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[i + 1];
                if x1 == x2 {
                    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                    for y in start..=end {
                        cave[y][x1] = '#';
                    }
                } else if y1 == y2 {
                    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                    for x in start..=end {
                        cave[y1][x] = '#';
                    }
                }
            }
        }
        cave
    }

    // Simulate sand for part 1 (no floor, stop when sand falls into abyss)
    fn simulate_part1(mut cave: Vec<Vec<char>>, source_x: usize) -> usize {
        let height = cave.len();
        let width = cave[0].len();
        let mut sand_count = 0;
        'outer: loop {
            let (mut x, mut y) = (source_x, 0);
            loop {
                if y + 1 >= height {
                    break 'outer;
                }
                if cave[y + 1][x] == '.' {
                    y += 1;
                } else if x > 0 && cave[y + 1][x - 1] == '.' {
                    y += 1;
                    x -= 1;
                } else if x + 1 < width && cave[y + 1][x + 1] == '.' {
                    y += 1;
                    x += 1;
                } else {
                    cave[y][x] = 'o';
                    sand_count += 1;
                    break;
                }
            }
        }
        sand_count
    }

    // Simulate sand for part 2 (with floor, stop when source is blocked)
    fn simulate_part2(mut cave: Vec<Vec<char>>, source_x: usize) -> usize {
        let height = cave.len();
        let width = cave[0].len();
        let mut sand_count = 0;
        loop {
            let (mut x, mut y) = (source_x, 0);
            loop {
                if y + 1 >= height {
                    break;
                }
                if cave[y + 1][x] == '.' {
                    y += 1;
                } else if x > 0 && cave[y + 1][x - 1] == '.' {
                    y += 1;
                    x -= 1;
                } else if x + 1 < width && cave[y + 1][x + 1] == '.' {
                    y += 1;
                    x += 1;
                } else {
                    cave[y][x] = 'o';
                    sand_count += 1;
                    if y == 0 && x == source_x {
                        return sand_count;
                    }
                    break;
                }
            }
        }
    }

    let input = include_str!("input.txt");
    let paths: Vec<&str> = input.lines().collect();
    let highest_y = paths.iter()
        .flat_map(|path| path.split(" -> "))
        .map(|p| p.split(',').nth(1).unwrap().parse::<usize>().unwrap())
        .max().unwrap();
    let width = 1000;
    let source_x = 500;
    // Part 1: no floor, cave height just enough to fit rocks
    let cave1 = fill_cave(&paths, width, highest_y + 3, false);
    let part1 = simulate_part1(cave1, source_x);
    // Part 2: with floor, cave height = highest_y + 3
    let cave2 = fill_cave(&paths, width, highest_y + 3, true);
    let part2 = simulate_part2(cave2, source_x);
    println!("Part 1: Units of sand that come to rest before falling into the abyss: {}", part1);
    println!("Part 2: Units of sand that come to rest before source is blocked: {}", part2);
}