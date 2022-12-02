#[aoc(day2, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    let mut i = 0;
    let len = input.len();
    let mut sum = 0;
    loop {
        match i < len {
            true => {
                let a = unsafe {input.get_unchecked(i) - b'@'};
                let b = unsafe {input.get_unchecked(i+2) - b'W'};
                sum += match (a,b) {
                    // draw
                    (1,1) => 1+3,
                    (2,2) => 2+3,
                    (3,3) => 3+3,
                    // win
                    (3,1) => 1+6,
                    (1,2) => 2+6,
                    (2,3) => 3+6,
                    // loss
                    (2,1) => 1,
                    (3,2) => 2,
                    (1,3) => 3,
                    _ => unreachable!()
                };
                i += 4;
            },
            false => break,
        }
    }
    sum
}
