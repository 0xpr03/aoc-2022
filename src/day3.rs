#[aoc(day3, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut start = 0;
    let mut counter = [0u8;58];
    for (i,c) in input.iter().enumerate() {
        if *c == b'\n' {
            counter.iter_mut().for_each(|m| *m = 0);
            let half = start+((i-start) / 2);
            
            let part_a = &input[start..half];
            let part_b = &input[half..i];
            for c in part_a {
                // assert_ne!(*c,b'\n');
                // counter[(*c-b'A') as usize] = 1;
                let v = unsafe {counter.get_unchecked_mut((c-b'A') as usize)};
                (*v) = 1;
            }
            for c in part_b {
                // assert_ne!(*c,b'\n');
                let v = unsafe {counter.get_unchecked((c-b'A') as usize)};
                if *v != 0 {
                // if counter[(*c-b'A') as usize] != 0 {
                    sum += value(*c) as i64;
                    // println!("start {start} half {half} i {i} Found {}",*c);
                    break;
                }
            }
            start = i + 1;
        }
    }
    let i = input.len();
    counter.iter_mut().for_each(|m| *m = 0);
    let half = start+((i-start) / 2);

    let part_a = &input[start..half];
    let part_b = &input[half..i];
    for c in part_a {
        // assert_ne!(*c,b'\n');
        // counter[(*c-b'A') as usize] = 1;
        let v = unsafe {counter.get_unchecked_mut((c-b'A') as usize)};
        (*v) = 1;
    }
    for c in part_b {
        // assert_ne!(*c,b'\n');
        let v = unsafe {counter.get_unchecked((c-b'A') as usize)};
        if *v != 0 {
        // if counter[(*c-b'A') as usize] != 0 {
            sum += value(*c) as i64;
            // println!("start {start} half {half} i {i} Found {}",*c);
            break;
        }
    }
    sum
}

#[aoc(day3, part2, Chars)]
pub fn part2(input: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut counter = [0u8;58];
    let mut elf = 0;
    for line in input.split(|v| *v == b'\n') {
        for c in line {
            // assert_ne!(*c,b'\n');
            // counter[(*c-b'A') as usize] = 1;
            let v = unsafe {counter.get_unchecked_mut((c-b'A') as usize)};
            match elf {
                0 => (*v) = 1,
                1 => {
                    if (*v) == 1 {
                        *v = 2;
                    }
                },
                _ => if *v == 2 {
                    // println!("start {start} i {i} Found {}",*c);
                    sum += value(*c) as i64;
                    break;
                }
            }
        }
        if elf < 2 {
            elf += 1;
        } else {
            counter.iter_mut().for_each(|m| *m = 0);
            elf = 0;
        }
    }
    sum
}

#[inline(always)]
fn value(c: u8) -> u8 {
    if c > b'Z' {
        c - 96
    } else {
        c - 38
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_value() {
        assert_eq!(value(b'A'),27);
        assert_eq!(value(b'a'),1);
        assert_eq!(value(b'Z'),52);
        assert_eq!(value(b'z'),26);

        assert_eq!(b'A'-b'A',0);
        assert_eq!(b'a'-b'A',32);
        assert_eq!(b'z'-b'A',58);
    }
}