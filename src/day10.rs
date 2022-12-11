#[aoc_generator(day10)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

#[aoc(day10, part1, Schokis)]
// #[inline(always)]
pub fn part1(input: &[u8]) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut cycle_adder = 0;
    for v in input.split(|x|*x == b'\n') {
        if v.is_empty() {
            break;
        }
        if v[0] == b'n' {
            cycle += 1;
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    cycle_adder += cycle * x;
                    // println!("noop cycle: {}, adder: {}, x: {}",cycle,cycle_adder,x);
                }
                _ => (),
            }
        } else {
            for _ in 0..2 {
                cycle += 1;
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        cycle_adder += cycle * x;
                        // println!("addx cycle: {}, adder: {}, x: {}",cycle,cycle_adder,x);
                    }
                    _ => (),
                }
            }
            
            let imm = if v.len() == 8 {
                ((v[6] as i32) * 10
            + (v[7] as i32) - 528) * -1
            } else if v.len() == 7 {
                if v[5] == b'-' {
                    (v[6] - b'0') as i32 * -1
                } else {
                    (v[5] as i32) * 10
                    + (v[6] as i32) - 528
                }
            } else {
                (v[5] - b'0') as i32
            };
            // println!("{:?}",(imm,cycle,x));
            x += imm;
        }
    }
    cycle_adder
}

#[aoc(day10, part2, Schokis)]
// #[inline(always)]
pub fn part2(input: &[u8]) -> String {
    let mut x = 1;
    let mut cycle = 0;
    let mut output = vec![b'.'; 6*40];
    for v in input.split(|x|*x == b'\n') {
        if v.is_empty() {
            break;
        }
        if v[0] == b'n' {
            let pos = cycle % 40;
            if pos == x-1 || pos == x || pos == x+1 {
                output[cycle as  usize] = b'#';
            }
            cycle += 1;
        } else {
            for _ in 0..2 {
                let pos = cycle % 40;
                if pos == x-1 || pos == x || pos == x+1 {
                    output[cycle as  usize] = b'#';
                }
                cycle += 1;
            }
            
            let imm = if v.len() == 8 {
                ((v[6] as i32) * 10
            + (v[7] as i32) - 528) * -1
            } else if v.len() == 7 {
                if v[5] == b'-' {
                    (v[6] - b'0') as i32 * -1
                } else {
                    (v[5] as i32) * 10
                    + (v[6] as i32) - 528
                }
            } else {
                (v[5] - b'0') as i32
            };
            // println!("{:?}",(imm,cycle,x));
            x += imm;
        }
    }
    unsafe { String::from_utf8_unchecked(output) }
}