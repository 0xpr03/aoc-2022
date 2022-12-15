//#![feature(iter_array_chunks)]
#[aoc_generator(day15)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

#[aoc(day15, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    const ASKED_Y: isize = 2000000;
    // const ASKED_Y: isize = 10;
    let mut min_x = isize::MAX;
    let mut max_x = 0;
    let sensors: Vec<_> = input.split(|x|*x == b'\n').filter(|v|!v.is_empty()).map(|v|{
        let iter = v.split(|x|*x == b',' || *x == b':').map(trim_non_digit);
        // iter_next_chunk is marked as slow https://github.com/rust-lang/rust/issues/98326#issuecomment-1166338225
        // so we'll use this
        let data = iter.array_chunks::<4>().next().unwrap();
        // for v in data {
        //     print!("{}#",std::str::from_utf8(v).unwrap());
        // }
        // println!();
        let s_x = atoi(data[0]);
        let s_y = atoi(data[1]);
        let b_x = atoi(data[2]);
        let b_y = atoi(data[3]);

        let distance = s_x.abs_diff(b_x) + s_y.abs_diff(b_y);
        let distance_isize = distance as isize;
        if s_x + distance_isize > max_x {
            max_x = s_x + distance_isize;
        }
        if s_x - distance_isize < min_x {
            min_x = s_x - distance_isize;
        }
        let s = Sensor {
            x: s_x,
            y: s_y,
            bx: b_x,
            by: b_y,
            dist: distance,
        };
        // println!("{} {b_x},{b_y}",std::str::from_utf8(data[2]).unwrap());
        // dbg!(&s);
        s
    }).collect();
    (min_x..=max_x).filter(|x|{
        sensors.iter().any(|s|s.x.abs_diff(*x)+s.y.abs_diff(ASKED_Y) <= s.dist)
    }).count()-1
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    bx: isize,
    by: isize,
    dist: usize,
}

#[inline(always)]
fn atoi(part: &[u8]) -> isize {
    let (mult,part) = match part[0] == b'-' {
        true => (-1,&part[1..]),
        false => (1,part),
    };
    if part.len() == 7 {
        ((part[0] & 0xf) as isize *1000000+
        (part[1] & 0xf) as isize *100000+
        (part[2] & 0xf) as isize *10000+
        (part[3] & 0xf) as isize *1000+
        (part[4] & 0xf) as isize *100+
        (part[5] & 0xf) as isize *10+
        (part[6] & 0xf) as isize) * mult
    } else if part.len() == 6 {
        ((part[0] & 0xf) as isize *100000+
        (part[1] & 0xf) as isize *10000+
        (part[2] & 0xf) as isize *1000+
        (part[3] & 0xf) as isize *100+
        (part[4] & 0xf) as isize *10+
        (part[5] & 0xf) as isize) * mult
    } else if part.len() == 5 {
        ((part[0] & 0xf) as isize *10000+
        (part[1] & 0xf) as isize *1000+
        (part[2] & 0xf) as isize *100+
        (part[3] & 0xf) as isize *10+
        (part[4] & 0xf) as isize) * mult
    } else if part.len() == 4 {
        ((part[0] & 0xf) as isize *1000+
        (part[1] & 0xf) as isize *100+
        (part[2] & 0xf) as isize *10+
        (part[3] & 0xf) as isize) * mult
    } else if part.len() == 3 {
        ((part[0] & 0xf) as isize *100+
        (part[1] & 0xf) as isize *10+
        (part[2] & 0xf) as isize) * mult
    } else if part.len() == 2 {
        ((part[0] & 0xf) as isize *10+
        (part[1] & 0xf) as isize) * mult
    } else if part.len() == 1 {
        ((part[0] & 0xf) as isize) * mult
    } else {
        unreachable!("Atoi Len: {}",part.len());
    }
}

#[inline(always)]
fn trim_non_digit(data: &[u8]) -> &[u8] {
    // taken from rusts trim_ascii
    let mut bytes = data;
    while let [first, rest @ ..] = bytes {
        if (!first.is_ascii_digit()) && *first != b'-' {
            bytes = rest;
        } else {
            break;
        }
    }
    while let [rest  @ .., last] = bytes {
        if (!last.is_ascii_digit()) && *last != b'-' {
            bytes = rest;
        } else {
            break;
        }
    }

    bytes
}