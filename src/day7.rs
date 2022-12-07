use std::{collections::HashMap, io::Write};

const ENTRIES: usize = 10;
const FILENAME: usize = 20;

#[aoc(day7, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    let mut sizes = Stack::new();
    let mut sum = 0;
    const LIMIT: usize = 100000;
    for line in input.split(|x|*x == b'\n') {
        if line.len() == 0 {
            break;
        }
        // print!("{} ",std::str::from_utf8(line).unwrap());
        match (line[0],line[2]) {
            // $ cd /
            (b'$',b'c') => {
                if line[5] == b'.' {
                    // dbg!(&sizes);
                    let v = sizes.sum_up();
                    // println!("v: {v}");
                    if v < LIMIT {
                        // println!("Summing");
                        sum += v;
                    }
                    // dbg!(&sizes);
                } else {
                    sizes.push(0);
                    // dbg!(&sizes);
                }
            },
            // $ ls
            (b'$',b'l') => {
                // println!();
                continue;
            },
            // dir
            (b'd',_) => {
                // println!();
                continue;
            },
            // 282147 mjtq.ffd
            (_,_) => {
                let mut foo = line.split(|x|*x == b' ');
                let number = foo.next().unwrap();
                let n = atoi(number);
                // println!("n: {n}");
                sizes.add(n as _);
            }
        }
    }
    while sizes.len > 1 {
        let v = sizes.sum_up();
        if v < LIMIT {
            sum += v;
        }
    }
    sum
}

#[inline(always)]
fn atoi(bytes: &[u8]) -> u32 {
    match bytes.len() {
        8 => (bytes[0] as u32) * 10000000 +
            (bytes[1] as u32) * 1000000 +
            (bytes[2] as u32) * 100000 +
            (bytes[3] as u32) * 10000 +
            (bytes[4] as u32) * 1000 +
            (bytes[5] as u32) * 100 +
            (bytes[6] as u32) * 10 +
            (bytes[7] as u32) -
            533333328,
        7 => (bytes[0] as u32) * 1000000 +
            (bytes[1] as u32) * 100000 +
            (bytes[2] as u32) * 10000 +
            (bytes[3] as u32) * 1000 +
            (bytes[4] as u32) * 100 +
            (bytes[5] as u32) * 10 +
            (bytes[6] as u32) -
            53333328,
        6 => (bytes[0] as u32) * 100000 +
            (bytes[1] as u32) * 10000 +
            (bytes[2] as u32) * 1000 +
            (bytes[3] as u32) * 100 +
            (bytes[4] as u32) * 10 +
            (bytes[5] as u32) -
            5333328,
        5 => (bytes[0] as u32) * 10000 +
            (bytes[1] as u32) * 1000 +
            (bytes[2] as u32) * 100 +
            (bytes[3] as u32) * 10 +
            (bytes[4] as u32) -
            533328,
        4 => (bytes[0] as u32) * 1000 +
            (bytes[1] as u32) * 100 +
            (bytes[2] as u32) * 10 +
            (bytes[3] as u32) -
            53328,
        3 => (bytes[0] as u32) * 100 +
            (bytes[1] as u32) * 10 +
            (bytes[2] as u32) -
            5328, // testing only
        v => unreachable!("{}",v),
    }
}

// struct FileName {
//     len: usize,
//     data: [u8; FILENAME]
// }

// impl FileName {
//     const fn new() -> Self {
//         Self {
//             data: [0; FILENAME],
//             len: 0,
//         }
//     }
//     pub fn set(&mut self, name: &[u8]) {
//         self.data.copy_from_slice(name);
//     }
//     pub fn get(&mut self) -> &[u8] {
//         self.data.copy_from_slice(name);
//     }
// }

#[derive(Debug)]
struct Stack{
    len: usize,
    data: [usize;ENTRIES]
}

impl Stack {
    const fn new() -> Self {
        Self {
            len: 0,
            data: [0; ENTRIES]
        }
    }
    #[inline(always)]
    fn push(&mut self, data: usize) {
        assert!(self.len < ENTRIES, "pushed Stack out of boundary");
        self.data[self.len] = data;
        self.len += 1;
    }
    #[inline(always)]
    pub fn add(&mut self, data: usize) {
        assert!(self.len > 0, "add out of bounds");
        self.data[self.len-1] += data;
    }
    #[inline(always)]
    fn pop(&mut self) -> usize {
        assert!(self.len > 0, "popped Stack out of boundary");
        self.len -= 1;
        self.data[self.len]
    }
    #[inline(always)]
    fn sum_up(&mut self) -> usize {
        assert!(self.len > 1, "popped Stack out of boundary");
        self.len -= 1;
        let v = self.data[self.len];
        self.data[self.len-1] += v;
        v
    }
    #[inline(always)]
    fn top<'a>(&'a self) -> usize {
        assert!(self.len > 0, "top on empty Stack");
        self.data[self.len - 1]
    }
    fn get(&self, i: usize) -> Option<usize> {
        match i < self.len {
            true => Some(self.data[i]),
            false => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn audio_test() {
        assert_eq!(atoi("242795".as_bytes()),242795);
        assert_eq!(atoi("24279".as_bytes()),24279);
        assert_eq!(atoi("2427".as_bytes()),2427);
    }
}