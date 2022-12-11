use std::{collections::HashSet, io::{BufRead, Write}};

#[aoc_generator(day9)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

#[aoc(day9, part1, Schokis)]
// #[inline(always)]
pub fn part1(input: &[u8]) -> usize {
    // x,y
    let mut head = (0,0);
    // x,y
    let mut tail = (0,0);
    let mut knownPos: HashSet<(i32,i32)> = HashSet::with_capacity(7000);

    for v in input.split(|x|*x == b'\n') {
        if v.is_empty() {
            break;
        }
        let v_num = &v[2..];
        // dbg!(std::str::from_utf8(v_num).unwrap());
        let amount = if v_num.len() > 1 {
            (v_num[0] as u16) * 10
            + (v_num[1] as u16) - 528
        } else {
            (v_num[0] - b'0') as u16
        };
        // println!("{} {amount}",v[0] as char);
        // debug_assert!(amount < 100);
        for _ in 0..amount {
            // println!("{head:?} {tail:?} In");
            match v[0] {
                b'L' => head.0 -= 1,
                b'R' => head.0 += 1,
                b'D' => head.1 -= 1,
                b'U' => head.1 += 1,
                v => unreachable!("{}",v as char),
            }
            // println!("Diff {:?}",(head.0-tail.0,head.1-tail.1));
            match (head.0-tail.0,head.1-tail.1) {
                // no move
                (0,0) | (1,0) | (-1,0) | (0,1) | (0,-1)=> (),
                (1,1) | (-1,-1) | (1,-1) | (-1,1) => (),
                // lr movement
                (2,0) => tail.0 += 1,
                (-2,0) => tail.0 -= 1,
                // ud movement
                (0,2) => tail.1 += 1,
                (0,-2) => tail.1 -= 1,
                // vertical movement
                (1,2) | (2,1) => {tail.0 += 1;tail.1 +=1},
                (1,-2) | (2,-1) => {tail.0 += 1;tail.1 -=1},
                (-1,-2) | (-2,-1) => {tail.0 -= 1;tail.1 -=1},
                (-1,2) | (-2,1) => {tail.0 -= 1;tail.1 +=1},
                dif => unreachable!("Dif: {dif:?} Head: {head:?} Tail: {tail:?}"),
                // _ => unreachable!(),
            }
            knownPos.insert(tail);
        }
        // println!("{head:?} {tail:?} Past");
    }
    knownPos.len()
}

#[aoc(day9, part2, Schokis)]
// #[inline(always)]
pub fn part2(input: &[u8]) -> usize {
    // x,y, 0 = head, 9 = tail
    let mut snake = [(0,0);10];
    let mut known_pos: HashSet<(i32,i32)> = HashSet::with_capacity(3000);

    for v in input.split(|x|*x == b'\n') {
        if v.is_empty() {
            break;
        }
        let v_num = &v[2..];
        // dbg!(std::str::from_utf8(v_num).unwrap());
        let amount = if v_num.len() > 1 {
            (v_num[0] as u16) * 10
            + (v_num[1] as u16) - 528
        } else {
            (v_num[0] - b'0') as u16
        };
        // println!("{} {amount}",v[0] as char);
        // debug_assert!(amount < 100);
        for _ in 0..amount {
            // println!("\tstep");
            match v[0] {
                b'L' => snake[0].0 -= 1,
                b'R' => snake[0].0 += 1,
                b'D' => snake[0].1 -= 1,
                b'U' => snake[0].1 += 1,
                v => unreachable!("{}",v as char),
            }
            // print_snake(&snake);
            for i in 1..snake.len() {
                // println!("\t{:?} {:?} In",snake[i-1],snake[i]);
                // println!("\tDiff [{i}] {:?}",(snake[i-1].0-snake[i].0,snake[i-1].1-snake[i].1));
                match (snake[i-1].0-snake[i].0,snake[i-1].1-snake[i].1) {
                    // no move
                    (0,0) | (1,0) | (-1,0) | (0,1) | (0,-1)=> break,
                    (1,1) | (-1,-1) | (1,-1) | (-1,1) => break,
                    // lr movement
                    (2,0) => snake[i].0 += 1,
                    (-2,0) => snake[i].0 -= 1,
                    // ud movement
                    (0,2) => snake[i].1 += 1,
                    (0,-2) => snake[i].1 -= 1,
                    // vertical movement
                    (1,2) | (2,1..=2) => {snake[i].0 += 1;snake[i].1 +=1},
                    (1,-2) | (2,-2..=-1) => {snake[i].0 += 1;snake[i].1 -=1},
                    (-1,-2) | (-2,-2..=-1) => {snake[i].0 -= 1;snake[i].1 -=1},
                    (-1,2) | (-2,1..=2) => {snake[i].0 -= 1;snake[i].1 +=1},
                    dif => unreachable!("[{i}] Dif: {dif:?} Head: {:?} Tail: {:?}",snake[i-1],snake[i]),
                    // _ => unreachable!(),
                }
                // print_snake(&snake);
            }
            known_pos.insert(snake[9]);
        }
        // println!("{head:?} {tail:?} Past");
    }
    known_pos.len()
}

fn print_snake(snake: &[(i32,i32)]) {
    let minx = snake.iter().map(|(x,y)|x).min().unwrap();
    let maxx = snake.iter().map(|(x,y)|x).max().unwrap();
    let miny = snake.iter().map(|(x,y)|y).min().unwrap();
    let maxy = snake.iter().map(|(x,y)|y).max().unwrap();
    let mut stdout = std::io::stdout().lock();
    for y in (*miny..=*maxy).rev() {
        for x in *minx..=*maxx {
            let mut c = b'.';
            for (i,(sx,sy)) in snake.iter().enumerate(){
                if *sx == x && *sy == y {
                    c = (i as u8) + b'0';
                    break;
                }
            }
            stdout.write(&[c]).unwrap();
        }
        stdout.write(&[b'\n']).unwrap();
    }
}