const ENTRIES: usize = 11;
const DIRS: usize = 200;
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
                let pos = if line[6] == b' ' {
                    6
                } else if line[5] == b' ' {
                    5
                } else if line[4] == b' ' {
                    4
                } else {
                    3
                };
                let n = atoi(&unsafe {line.get_unchecked(..pos)});
                // println!("n: {n}");
                sizes.add(n as _);
            }
        }
    }
    while sizes.len > 1 {
        let v = sizes.sum_up();
        if v < LIMIT {
            sum += v;
        } else {
            break;
        }
    }
    sum
}

#[aoc(day7, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    const TOTAL: usize = 70_000_000;
    const REQUIRED: usize = 30000000;
    let mut sizes = Stack::new();
    let mut dirs = Dirs::new();
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
                    dirs.push(v);
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
                
                // let pos = line.iter().position(|v|*v == b' ').unwrap_or(0);
                let pos = if line[6] == b' ' {
                    6
                } else if line[5] == b' ' {
                    5
                } else if line[4] == b' ' {
                    4
                } else {
                    3
                };
                let n = atoi(&unsafe {line.get_unchecked(..pos)});
                // let mut foo = line.split(|x|*x == b' ');
                // let number = foo.next().unwrap();
                // let n = atoi(number);
                // println!("n: {n}");
                sizes.add(n as _);
            }
        }
    }
    while sizes.len > 1 {
        let v = sizes.sum_up();
        dirs.push(v);
    }
    let sum = unsafe {sizes.data.get_unchecked(0)};
    let needed = REQUIRED - (TOTAL - sum);
    let mut dir = REQUIRED;
    for d in unsafe {dirs.data.get_unchecked_mut(0..dirs.len)}.iter() {
        if *d > needed && *d < dir {
            dir = *d;
        }
    }
    dir
}

#[inline(always)]
fn atoi(bytes: &[u8]) -> u32 {
    match bytes.len() {
        // 8 => (bytes[0] as u32) * 10000000 +
        //     (bytes[1] as u32) * 1000000 +
        //     (bytes[2] as u32) * 100000 +
        //     (bytes[3] as u32) * 10000 +
        //     (bytes[4] as u32) * 1000 +
        //     (bytes[5] as u32) * 100 +
        //     (bytes[6] as u32) * 10 +
        //     (bytes[7] as u32) -
        //     533333328,
        // 7 => (bytes[0] as u32) * 1000000 +
        //     (bytes[1] as u32) * 100000 +
        //     (bytes[2] as u32) * 10000 +
        //     (bytes[3] as u32) * 1000 +
        //     (bytes[4] as u32) * 100 +
        //     (bytes[5] as u32) * 10 +
        //     (bytes[6] as u32) -
        //     53333328,
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
            5328,
        _ => 0,
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

struct Dirs {
    len: usize,
    data: [usize; DIRS]
}

impl Dirs {
    const fn new() -> Self {
        Self {
            len: 0,
            data: [0; DIRS]
        }
    }
    #[inline(always)]
    fn push(&mut self, data: usize) {
        // assert!(self.len < ENTRIES, "pushed Stack out of boundary");
        unsafe {*self.data.get_unchecked_mut(self.len) = data;}
        self.len += 1;
    }
}

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
        // assert!(self.len < ENTRIES, "pushed Stack out of boundary");
        unsafe{*self.data.get_unchecked_mut(self.len) = data;}
        self.len += 1;
    }
    #[inline(always)]
    pub fn add(&mut self, data: usize) {
        // assert!(self.len > 0, "add out of bounds");
        unsafe {*self.data.get_unchecked_mut(self.len-1) += data;}
    }
    #[inline(always)]
    fn sum_up(&mut self) -> usize {
        // assert!(self.len > 1, "popped Stack out of boundary");
        self.len -= 1;
        unsafe {
            let v = *self.data.get_unchecked(self.len);
            *self.data.get_unchecked_mut(self.len-1) += v;
            return v;
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