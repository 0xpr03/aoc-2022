#[aoc(day3, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut counter = [0u8; (b'z' - b'A') as usize + 1];
    for line in input.split(|x| *x == b'\n') {
        counter.iter_mut().for_each(|m| *m = 0);
        let half = line.len() / 2;

        let part_a = &line[..half];
        let part_b = &line[half..];
        for c in part_a {
            // assert_ne!(*c,b'\n');
            // counter[(*c-b'A') as usize] = 1;
            let v = unsafe { counter.get_unchecked_mut((c - b'A') as usize) };
            (*v) = 1;
        }
        for c in part_b {
            // assert_ne!(*c,b'\n');
            let v = unsafe { counter.get_unchecked((c - b'A') as usize) };
            if *v != 0 {
                // if counter[(*c-b'A') as usize] != 0 {
                sum += value(*c) as i64;
                println!("half {half} Found {}", *c);
                break;
            }
        }
    }
    sum
}

#[aoc(day3, part2, Chars)]
pub fn part2(input: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut counter = [0u8; (b'z' - b'A') as usize + 1];
    let mut elf = 0;
    for line in input.split(|v| *v == b'\n') {
        for c in line {
            // assert_ne!(*c,b'\n');
            // counter[(*c-b'A') as usize] = 1;
            let v = unsafe { counter.get_unchecked_mut((c - b'A') as usize) };
            match elf {
                0 => (*v) = 1,
                1 => {
                    if (*v) == 1 {
                        *v = 2;
                    }
                }
                _ => {
                    if *v == 2 {
                        // println!("start {start} i {i} Found {}",*c);
                        sum += value(*c) as i64;
                        break;
                    }
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
        assert_eq!(value(b'A'), 27);
        assert_eq!(value(b'a'), 1);
        assert_eq!(value(b'Z'), 52);
        assert_eq!(value(b'z'), 26);

        assert_eq!(b'A' - b'A', 0);
        assert_eq!(b'a' - b'A', 32);
        assert_eq!(b'z' - b'A', 57);
    }
}

// alternative by anden3
pub fn part1_anden3(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.split_at(s.len() / 2))
        .map(find_common_item)
        .map(get_priority)
        .sum()
}

pub fn find_common_item((a, b): (&[u8], &[u8])) -> u8 {
    let mut found_items = [false; (b'z' - b'A') as usize + 1];

    for item in a {
        found_items[(*item - b'A') as usize] = true;
    }

    for item in b {
        if found_items[(*item - b'A') as usize] {
            return *item;
        }
    }

    unreachable!()
}

pub fn get_priority(item: u8) -> i64 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i64,
        b'A'..=b'Z' => (item - b'A' + 27) as i64,
        _ => unreachable!(),
    }
}
