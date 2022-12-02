#[aoc(day1, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    let mut start = 0;
    let mut last_sum: u32 = 0;
    let mut max: u32 = 0;
    let mut newline = false;
    let data = input;
    for (index, byte) in data.iter().enumerate() {
        if *byte == 10 {
            if newline {
                if last_sum > max {
                    max = last_sum;
                }
                last_sum = 0;
            } else {
                let v = &data[start..index];
                last_sum += atoi(v);
                newline = true;
            }
        } else if newline{
            start = index;
            newline = false;
        }
    }
    if last_sum > max {
        max = last_sum;
    }
    max as _
}

#[inline(always)]
fn atoi(bytes: &[u8]) -> u32 {
    if bytes.len() == 5 {
        (bytes[0] as u32) * 10000 +
        (bytes[1] as u32) * 1000 +
        (bytes[2] as u32) * 100 +
        (bytes[3] as u32) * 10 +
        (bytes[4] as u32) -
        533328
    } else {
        (bytes[0] as u32) * 1000 +
        (bytes[1] as u32) * 100 +
        (bytes[2] as u32) * 10 +
        (bytes[3] as u32) -
        53328
    }
}

fn atoi_old(bytes: &[u8]) -> i64 {
    let mut result = 0;
    for byte in bytes {
        result = result * 10 + (byte - b'0') as i64;
    }
    result
}

pub fn part1origin(input: &str) -> i64 {
    input.trim().split("\n\n")
        .map(|group|group.split("\n")
            .map(|v|atoi_radix10::parse::<i64>(v.as_bytes()).unwrap())
            .fold(0, |acc,x|acc+x))
        .max().unwrap()
}


#[aoc(day1, part2, Chars)]
pub fn part2(input: &str) -> i64 {
    let mut start = 0;
    let mut last_sum: u32 = 0;
    let mut newline = false;
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let data = input.trim().as_bytes();
    for (index, byte) in data.iter().enumerate() {
        if *byte == 10 {
            if newline {
                if last_sum > third {
                    if last_sum > second {
                        if last_sum > first {
                            third = second;
                            second = first;
                            first = last_sum;
                        } else {
                            third = second;
                            second = last_sum;
                        }
                    } else {
                        third = last_sum;
                    }
                }
                last_sum = 0;
            } else {
                let v = &data[start..index];
                last_sum += atoi(v);
                newline = true;
            }
        } else if newline{
            start = index;
            newline = false;
        }
    }
    if last_sum > third {
        if last_sum > second {
            if last_sum > first {
                third = second;
                second = first;
                first = last_sum;
            } else {
                third = second;
                second = last_sum;
            }
        } else {
            third = last_sum;
        }
    }
    (first + second + third) as _
}


// pub fn part2(input: &str) -> i64 {
//     let mut first = 0;
//     let mut second = 0;
//     let mut third = 0;
//     for group in input.trim().split("\n\n") {
//         let cal: i64 = group.split("\n").map(|v|atoi_radix10::parse_from_str::<i64,_>(v).unwrap()).fold(0, |acc,x|acc+x);
//         if cal > third {
//             if cal > second {
//                 if cal > first {
//                     third = second;
//                     second = first;
//                     first = cal;
//                 } else {
//                     third = second;
//                     second = cal;
//                 }
//             } else {
//                 third = cal;
//             }
//         }
//     }
//     first + second + third
// }