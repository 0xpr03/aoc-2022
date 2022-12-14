#[aoc_generator(day12)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

#[aoc(day12, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    use std::collections::VecDeque;
    let COL_LENGTH: usize = input.iter().position(|x| *x == b'\n').unwrap();
    // dbg!(COL_LENGTH);
    // const COL_LENGTH: usize = 8;
    let COL_LENGTH_INP: usize = COL_LENGTH + 1;
    let ROWS: usize = input.iter().filter(|x| **x == b'\n').count();
    // const ROWS: usize = 5;

    let mut visited = vec![Option::<(usize, usize)>::None; COL_LENGTH * ROWS];

    let start = input.iter().position(|x| *x == b'S').unwrap();
    // dbg!(start);
    let start = (start % COL_LENGTH_INP, start / COL_LENGTH);
    // dbg!(start);
    // assert_eq!(start,(0,20));
    const N: usize = 1024;
    // x,y
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(1024);

    queue.push_back(start);

    let mut neighbours: Stack<(usize, usize), 4> = Stack {
        data: [(0, 0); 4],
        len: 0,
    };

    let mut end = (0, 0);

    while let Some((x, y)) = queue.pop_front() {
        neighbours.len = 0;
        let height = input[y * COL_LENGTH_INP + x];

        // add & find neighbours
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < (COL_LENGTH - 1) {
            // len vs start-by-0
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < (ROWS - 1) {
            // len vs start-by-0
            neighbours.push((x, y + 1));
        }
        // dbg!(neighbours);

        while neighbours.len > 0 {
            let (xn, yn) = neighbours.pop();

            // check visited
            if visited[yn * COL_LENGTH + xn].is_some() && (xn, yn) != end {
                // println!("Visited: ({x},{y}):{}->({xn},{yn})",height as char);
                continue;
            }
            // compute diff
            let height_n = input[yn * COL_LENGTH_INP + xn];
            // println!("Checking ({x},{y}):{}->({xn},{yn}):{}",height as char, height_n as char);
            let diff = height.abs_diff(height_n);
            match (
                height > height_n && height_n != b'E' && height_n != b'S',
                diff,
                height,
                height_n,
            ) {
                (true, _, _, _)
                | (_, 0..=1, _, _)
                | (_, _, b'S', b'a'..=b'b')
                | (_, _, _, b'S') => {
                    visited[yn * COL_LENGTH + xn] = Some((x, y));
                    queue.push_back((xn, yn));
                }
                (_, _, b'y'..=b'z', b'E') => {
                    visited[yn * COL_LENGTH + xn] = Some((x, y));
                    //queue.push((xn,yn)); // for path
                    end = (xn, yn);
                    queue.clear();
                    break;
                }
                _ => {
                    // println!("Skip ({x},{y}):{}->({xn},{yn}):{} diff {diff}",height as char,height_n as char);
                    // skip, not E/S or height diff of 1
                    continue;
                }
            }
        }
    }
    // print_visited(&visited, COL_LENGTH, ROWS-1);
    let mut moves = 0;
    let mut node = Some(end);
    while let Some((x, y)) = node {
        // println!("{x},{y}");
        node = visited[y * COL_LENGTH + x];
        moves += 1;
        if node == Some(start) {
            break;
        }
    }
    moves
}

#[aoc(day12, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    use std::collections::VecDeque;
    let COL_LENGTH: usize = input.iter().position(|x| *x == b'\n').unwrap();
    // dbg!(COL_LENGTH);
    // const COL_LENGTH: usize = 8;
    let COL_LENGTH_INP: usize = COL_LENGTH + 1;
    let ROWS: usize = input.iter().filter(|x| **x == b'\n').count();
    // const ROWS: usize = 5;
    // dbg!(ROWS);
    // dbg!(COL_LENGTH);

    let mut visited = vec![Option::<(usize, usize)>::None; COL_LENGTH * ROWS];

    let start = input.iter().position(|x| *x == b'E').unwrap();
    // dbg!(start);
    let start = (start % COL_LENGTH_INP, start / COL_LENGTH);
    // dbg!(start);
    // assert_eq!(start,(0,20));
    const N: usize = 1024;
    // x,y
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(1024);

    queue.push_back(start);

    let mut neighbours: Stack<(usize, usize), 4> = Stack {
        data: [(0, 0); 4],
        len: 0,
    };

    let mut end = (0, 0);

    while let Some((x, y)) = queue.pop_front() {
        neighbours.len = 0;
        let height = input[y * COL_LENGTH_INP + x];

        // add & find neighbours
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < (COL_LENGTH - 1) {
            // len vs start-by-0
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < (ROWS - 1) {
            // len vs start-by-0
            neighbours.push((x, y + 1));
        }
        // dbg!(neighbours);

        while neighbours.len > 0 {
            let (xn, yn) = neighbours.pop();

            // check visited
            if visited[yn * COL_LENGTH + xn].is_some() && (xn, yn) != end {
                // println!("Visited: ({x},{y}):{}->({xn},{yn})",height as char);
                continue;
            }
            // compute diff
            let height_n = input[yn * COL_LENGTH_INP + xn];
            // println!("Checking ({x},{y}):{}->({xn},{yn}):{}",height as char, height_n as char);
            let diff = height.abs_diff(height_n);
            match (
                height < height_n && height != b'E' && height_n != b'a',
                diff,
                height,
                height_n,
            ) {
                (_, _, b'b', b'a') => {
                    visited[yn * COL_LENGTH + xn] = Some((x, y));
                    //queue.push((xn,yn)); // for path
                    end = (xn, yn);
                    queue.clear();
                    break;
                }
                (true, _, _, _) | (_, 0..=1, _, _) | (_, _, b'E', b'y'..=b'z') => {
                    visited[yn * COL_LENGTH + xn] = Some((x, y));
                    queue.push_back((xn, yn));
                }
                _ => {
                    // println!("Skip ({x},{y}):{}->({xn},{yn}):{} diff {diff}",height as char,height_n as char);
                    // skip, not E/S or height diff of 1
                    continue;
                }
            }
        }
    }
    // print_visited(&visited, COL_LENGTH, ROWS, start,end);
    let mut moves = 0;
    let mut node = Some(end);
    while let Some((x, y)) = node {
        // println!("{x},{y}");
        node = visited[y * COL_LENGTH + x];
        moves += 1;
        if node == Some(start) {
            break;
        }
    }
    moves
}

#[derive(Copy, Clone, Debug)]
struct Stack<T: Sized + Copy, const COUNT: usize> {
    len: usize,
    data: [T; COUNT],
}

impl<T: Sized + Copy + Default, const COUNT: usize> Stack<T, COUNT> {
    #[inline(always)]
    fn push(&mut self, data: T) {
        //assert!(self.len < COUNT, "pushed Stack out of boundary");
        unsafe {
            *self.data.get_unchecked_mut(self.len) = data;
        }
        self.len += 1;
    }
    #[inline(always)]
    fn pop(&mut self) -> T {
        //assert!(self.len > 0, "stack pop underflow");
        self.len -= 1;
        unsafe { *self.data.get_unchecked(self.len) }
    }
}

fn print_visited(
    visited: &[Option<(usize, usize)>],
    cols: usize,
    rows: usize,
    start: (usize, usize),
    end: (usize, usize),
) {
    use std::io::Write;
    println!("Start: {},{}", start.0, start.1);
    println!("End: {},{}", end.0, end.1);
    let mut stdout = std::io::stdout().lock();
    for y in 0..rows {
        for x in 0..cols {
            let mut c = b'O';
            if (x, y) == start {
                c = b'S';
            } else if (x, y) == end {
                c = b'E';
            } else if visited[y * cols + x].is_some() {
                c = b'X';
            }
            stdout.write(&[c]).unwrap();
        }
        stdout.write(&[b'\n']).unwrap();
    }
    stdout.flush().unwrap();
}
