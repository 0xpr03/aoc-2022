// #![feature(byte_slice_trim_ascii)]
use std::cmp::{min, max};

#[aoc_generator(day14)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}


const AMOUNT: usize = 100;
#[aoc(day14, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    const COL_LENGTH: usize = 1000;
    const ROWS: usize = 1000;
    
    let mut area = [b'.'; COL_LENGTH*ROWS];
    
    for line in input.split(|x|*x == b'\n') {
        if line.is_empty() {
            break;
        }
        let mut last_path: Option<(usize, usize)> = None;
        for part in line.split(|x|*x == b'>') {
            let part = part.trim_ascii_start();
            // assert_eq!(part[3],b',');
            let xn = atoi(part) as usize;
            let yn = if part.len() > 6 && part[6] != b' ' {
                atoi(&part[4..7]) as usize
            } else {
                atoi(&part[4..6]) as usize
            };
            // println!("{}: {xn}:{yn}",std::str::from_utf8(part).unwrap());
            if let Some((x,y)) = last_path {
                if y == yn {
                    for xi in min(xn,x)..=max(xn,x) {
                        area[yn * COL_LENGTH + xi] = b'#';
                    }
                } else {
                    for yi in min(yn,y)..=max(yn,y) {
                        area[yi * COL_LENGTH + xn] = b'#';
                    }
                }
            }
            last_path = Some((xn,yn));
        }
    }

    let mut sand = 0;
    let mut in_bounds = true;
    while in_bounds {
        // println!("{}",sand);
        let mut x = 500;
        let mut y = 0;
        loop {
            // println!("{x},{y}");
            if y >= 999 {
                // for printing only
                // area[y * COL_LENGTH + x] = b'.';
                in_bounds = false;
                break;
            }
            if area[(y+1) * COL_LENGTH + x] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x] = b'o';
                y+= 1;
            } else if area[(y+1) * COL_LENGTH + x-1] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x-1] = b'o';
                x -= 1;
                y+= 1;
            } else if area[(y+1) * COL_LENGTH + x+1] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x+1] = b'o';
                x += 1;
                y+= 1;
            } else {
                sand += 1;
                break;
            }
        }
    }
    // area[0 * COL_LENGTH + 500] = b'+';
    // print_visited(&area, COL_LENGTH, ROWS);
    sand
}

#[aoc(day14, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    const COL_LENGTH: usize = 1000;
    const ROWS: usize = 1000;

    const START_X: usize = 500;
    const START_Y: usize= 0;
    
    let mut area = [b'.'; COL_LENGTH*ROWS];
    
    let mut max_y = 0;
    for line in input.split(|x|*x == b'\n') {
        if line.is_empty() {
            break;
        }
        let mut last_path: Option<(usize, usize)> = None;
        for part in line.split(|x|*x == b'>') {
            let part = part.trim_ascii_start();
            // assert_eq!(part[3],b',');
            let xn = atoi(part) as usize;
            let yn = if part.len() > 6 && part[6] != b' ' {
                atoi(&part[4..7]) as usize
            } else {
                atoi(&part[4..6]) as usize
            };
            if yn > max_y {
                max_y = yn;
            }
            // println!("{}: {xn}:{yn}",std::str::from_utf8(part).unwrap());
            if let Some((x,y)) = last_path {
                if y == yn {
                    for xi in min(xn,x)..=max(xn,x) {
                        area[yn * COL_LENGTH + xi] = b'#';
                    }
                } else {
                    for yi in min(yn,y)..=max(yn,y) {
                        area[yi * COL_LENGTH + xn] = b'#';
                    }
                }
            }
            last_path = Some((xn,yn));
        }
    }

    max_y += 1;

    let mut sand = 0;
    let mut in_bounds = true;
    while in_bounds {
        // println!("{}",sand);
        let mut x = START_X;
        let mut y = START_Y;
        loop {
            // println!("{x},{y}");
            if y >= max_y {
                // for printing only
                // area[y * COL_LENGTH + x] = b'.';
                sand += 1;
                break;
            }
            if area[(y+1) * COL_LENGTH + x] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x] = b'o';
                y+= 1;
            } else if area[(y+1) * COL_LENGTH + x-1] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x-1] = b'o';
                x -= 1;
                y+= 1;
            } else if area[(y+1) * COL_LENGTH + x+1] == b'.' {
                area[y * COL_LENGTH + x] = b'.';
                area[(y+1) * COL_LENGTH + x+1] = b'o';
                x += 1;
                y+= 1;
            } else if x == START_X && y == START_Y {
                sand += 1;
                in_bounds = false;
                break;
            } else {
                sand += 1;
                break;
            }
        }
    }
    // area[0 * COL_LENGTH + 500] = b'+';
    // print_visited(&area, COL_LENGTH, ROWS);
    sand
}

#[inline(always)]
fn atoi(part: &[u8]) -> u32 {
    if part.len() > 2 {
        (part[0] & 0xf) as u32 *100+(part[1] & 0xf) as u32 *10+(part[2] & 0xf) as u32
    } else {
        (part[0] & 0xf) as u32 *10+(part[1] & 0xf) as u32
    }
}

fn print_visited(
    map: &[u8],
    cols: usize,
    rows: usize,
) {
    use std::io::Write;
    let mut stdout = std::io::stdout().lock();
    let start = map.iter().position(|x|*x != b'.').unwrap();
    let end = map.iter().rev().position(|x|*x != b'.').unwrap();
    let end = (cols * rows) - end;
    let min_y = start / cols;
    let max_y = end / cols;
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    for y in min_y..max_y {
        for x in 0..cols {
            let v = map[y * cols + x];
            if v != b'.' {
                if x > max_x {
                    max_x = x;
                }
                if x < min_x {
                    min_x = x;
                }
            }
        }
    }
    println!("{min_x}-{max_x} {min_y}-{max_y}");
    for y in min_y..=(max_y+1) {
        write!(&mut stdout, "{:03}",y);
        for x in (min_x-1)..(max_x+1) {
            let c = map[y * cols + x];
            stdout.write(&[c]).unwrap();
        }
        stdout.write(&[b'\n']).unwrap();
    }
    stdout.flush().unwrap();
}
