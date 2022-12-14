//#![feature(array_windows)]

macro_rules! test_initial {
    ($a:expr, $to:literal, $skip:ident) => {
        if let Some(v) = find_big_pos(&$a[0..$to], $a[$to]) {
            if v > $skip {
                $skip = v;
            }
        }
    };
}

#[aoc(day6, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    let mut skip = 0;
    if input[1] == input[0] {
        skip = 1;
    }
    test_initial!(input, 2, skip);
    test_initial!(input, 3, skip);

    for (index, part) in input.array_windows::<4>().enumerate() {
        // dbg!(skip);
        // dbg!(index);
        for (i, v) in part[0..3].iter().enumerate().rev() {
            let skip_n = index + i;
            if skip_n <= skip {
                break;
            }
            if *v == part[3] {
                skip = skip_n;
                break;
            }
        }
        if skip < index {
            return index + 4;
        }
    }
    0
}

#[aoc(day6, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    let mut skip = 0;
    if input[1] == input[0] {
        skip = 1;
    }
    test_initial!(input, 2, skip);
    test_initial!(input, 3, skip);
    test_initial!(input, 4, skip);
    test_initial!(input, 5, skip);
    test_initial!(input, 6, skip);
    test_initial!(input, 7, skip);
    test_initial!(input, 8, skip);
    test_initial!(input, 9, skip);
    test_initial!(input, 10, skip);
    test_initial!(input, 11, skip);
    test_initial!(input, 12, skip);
    test_initial!(input, 13, skip);

    for (index, part) in input.array_windows::<14>().enumerate() {
        // dbg!(skip);
        // dbg!(index);
        for (i, v) in part[0..13].iter().enumerate().rev() {
            let skip_n = index + i;
            if skip_n <= skip {
                break;
            } else if *v == part[13] {
                skip = skip_n;
                break;
            }
        }
        if skip < index {
            return index + 14;
        }
    }
    0
}

#[inline(always)]
fn find_big_pos(slice: &[u8], data: u8) -> Option<usize> {
    for (i, v) in slice.iter().enumerate().rev() {
        if *v == data {
            return Some(i);
        }
    }
    None
}
