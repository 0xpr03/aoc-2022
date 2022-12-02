#[aoc(day2, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    let mut i = 0;
    let len = input.len();
    let mut sum = 0;
    for data in input.array_chunks::<4>() {
        let a = data[0] - b'@';
        let b = data[2] - b'W';
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
    }
    if input.len() % 4 != 0 {
        let a = input[input.len() -3] - b'@';
        let b = input[input.len() -1] - b'W';
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
    }
    sum
}

#[aoc(day2, part2, Chars)]
pub fn part2(input: &[u8]) -> i64 {
    let mut i = 0;
    let len = input.len();
    let mut sum = 0;
    for data in input.array_chunks::<4>() {
        let a = data[0] - b'@';
        let b = data[2] - b'W';
        sum += match (a,b) {
            (1,1) => 3,
            (1,2) => 3+1,
            (1,3) => 6+2,
            (2,1) => 1,
            (2,2) => 3+2,
            (2,3) => 6+3,
            (3,1) => 2,
            (3,2) => 3+3,
            (3,3) => 6+1,
            _ => unreachable!()
        };
    }
    if input.len() % 4 != 0 {
        let a = input[input.len() -3] - b'@';
        let b = input[input.len() -1] - b'W';
        sum += match (a,b) {
            (1,1) => 3,
            (1,2) => 3+1,
            (1,3) => 6+2,
            (2,1) => 1,
            (2,2) => 3+2,
            (2,3) => 6+3,
            (3,1) => 2,
            (3,2) => 3+3,
            (3,3) => 6+1,
            _ => unreachable!()
        };
    }
    sum
}
