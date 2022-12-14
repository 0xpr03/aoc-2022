#[aoc_generator(day8)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

// example
// const LENGTH: usize = 5;
const LENGTH: usize = 99;
// \n
const ROW_LENGTH: usize = LENGTH + 1;
#[aoc(day8, part1, Schokis)]
// #[inline(always)]
pub fn part1(input: &[u8]) -> usize {
    let mut matrix = [false; LENGTH * LENGTH];

    // fill outer sides
    (0..LENGTH).into_iter().for_each(|id| matrix[id] = true);
    (1..(LENGTH))
        .into_iter()
        .for_each(|y| matrix[y * LENGTH - 1] = true);
    (1..(LENGTH - 1))
        .into_iter()
        .for_each(|y| matrix[y * LENGTH] = true);
    ((LENGTH - 1) * LENGTH..LENGTH * LENGTH)
        .into_iter()
        .for_each(|id| matrix[id] = true);

    // dbg!("left to right");
    // left to right
    for y in 0..(LENGTH) {
        let mut max = input[y * (ROW_LENGTH)];
        // dbg!(max-b'0');
        for x in 0..(LENGTH) {
            let v = input[y * (ROW_LENGTH) + x];
            // dbg!(v-b'0');
            if max < v {
                matrix[y * LENGTH + x] = true;
                max = v;
            }
        }
    }
    // print_matrix(&matrix);
    // dbg!("right to left");
    // right to left
    for y in 0..(LENGTH) {
        let mut max = input[y * (ROW_LENGTH) + (LENGTH - 1)];
        // dbg!(max-b'0');
        for x in 0..(LENGTH) {
            let v = input[y * (ROW_LENGTH) + (LENGTH - x - 1)];
            // dbg!(v-b'0');
            if max < v {
                matrix[y * LENGTH + (LENGTH - x - 1)] = true;
                max = v;
            }
        }
    }
    // print_matrix(&matrix);
    // dbg!("top to bottom");
    // top to bottom
    for x in 0..(LENGTH) {
        let mut max = input[x];
        // dbg!(max-b'0');
        for y in 0..(LENGTH) {
            let v = input[y * ROW_LENGTH + x];
            // dbg!(v-b'0');
            if max < v {
                matrix[y * LENGTH + x] = true;
                max = v;
            }
        }
    }
    // print_matrix(&matrix);
    // dbg!("bottom to top");
    // bottom to top
    for x in 0..(LENGTH) {
        let mut max = input[(ROW_LENGTH - 2) * ROW_LENGTH + x];
        // dbg!(max-b'0');
        for y in 0..(LENGTH) {
            let v = input[(ROW_LENGTH - y - 2) * ROW_LENGTH + x];
            // dbg!(v-b'0');
            if max < v {
                matrix[(LENGTH - y - 1) * LENGTH + x] = true;
                max = v;
            }
        }
    }
    // print_matrix(&matrix);
    matrix.into_iter().filter(|&v| v).count()
}

#[aoc(day8, part2, Schokis)]
pub fn part2(input: &[u8]) -> u32 {
    // let mut matrix = [0u32; LENGTH * LENGTH];

    let mut max = 0;
    for y in 0..LENGTH {
        for x in 0..LENGTH {
            let val = input[y * (ROW_LENGTH) + x];
            // dbg!(val-b'0');
            let mut sum = 1;
            // right
            let mut adder = 0;
            for xid in (x + 1)..LENGTH {
                let vid = input[y * ROW_LENGTH + xid];
                adder += 1;
                if vid >= val {
                    break;
                }
            }
            sum *= adder;

            // left
            let mut adder = 0;
            if x != 0 {
                // prevent underflow
                for xid in (0..x).rev() {
                    let vid = input[y * ROW_LENGTH + xid];
                    adder += 1;
                    if vid >= val {
                        break;
                    }
                }
            }
            sum *= adder;

            // top
            let mut adder = 0;
            if y != 0 {
                // prevent underflow
                for yid in (0..y).rev() {
                    let vid = input[yid * ROW_LENGTH + x];
                    adder += 1;
                    if vid >= val {
                        break;
                    }
                }
            }
            sum *= adder;

            // down
            let mut adder = 0;
            for yid in (y + 1)..LENGTH {
                let vid = input[yid * ROW_LENGTH + x];
                // dbg!(vid-b'0');
                adder += 1;
                if vid >= val {
                    break;
                }
            }
            sum *= adder;

            // matrix[y * LENGTH + x] = sum;
            if sum > max {
                max = sum;
            }
        }
    }
    // print_matrix_2(&matrix);
    max
}

fn print_matrix(matrix: &[bool]) {
    println!("matrix:");
    for y in 0..LENGTH {
        for x in 0..LENGTH {
            let v = match matrix[y * LENGTH + x] {
                true => 'x',
                false => 'o',
            };
            print!("{v}");
        }
        print!("\n");
    }
}

fn print_matrix_2(matrix: &[u32]) {
    println!("matrix:");
    for y in 0..LENGTH {
        for x in 0..LENGTH {
            print!("{}\t", matrix[y * LENGTH + x]);
        }
        print!("\n");
    }
}
