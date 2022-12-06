const ENTRIES: usize = 64;
const STACKS: usize = 9;

#[aoc(day5, part1, Chars)]
pub fn part1(input: &[u8]) -> String {
    // don't allocate, doesn't matter when or why ;)
    let mut pos = 7;
    let mut stack: [Stack;STACKS] = [Stack::new();STACKS];
    // parse stack
    for line in input.array_chunks::<36>() {
        for (i,part) in line.array_chunks::<4>().enumerate() {
            if part[1] >= b'A' {
                stack[i].data[pos] = part[1];
                if stack[i].len == 0 {
                    stack[i].len = pos + 1;
                }
            }
        }
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
    // dbg_stack(&stack);
    // println!("{:?}",stack);
    // dbg!(stack.len());
    let iter = input.split(|v|*v == b'\n').skip(stack.len()+1);
    // dbg!("Data:");
    for line in iter {
        // stdout.write(*v as char);
        if line.len() == 0 {
			break;
		}
        let (amount, from,to) = match line.len() > 18 {
            false => ((line[5] - b'0') as usize,(line[12] - b'0' - 1) as usize,(line[17] - b'0' - 1) as usize),
            true => (((line[5] as u32) * 10
            + (line[6] as u32) - 528) as usize,(line[13] - b'0' - 1) as usize,(line[18] - b'0' - 1) as usize),
        };
        // println!("{}",std::str::from_utf8(line).unwrap());
        // println!("move {amount} from {from} to {to}");
        for _ in 0..amount {
            let d = stack[from].pop();
            // dbg!(d);
            stack[to].push(d);
        }
        // break;
    }
    // dbg_stack(&stack);
    let mut result = [0; STACKS];
    for i in 0..STACKS {
        result[i] = stack[i].top();
    }
    unsafe {std::str::from_utf8_unchecked(&result).to_owned()}
}

#[aoc(day5, part2, Schokis)]
pub fn part2(input: &[u8]) -> String {
    // don't allocate, doesn't matter when or why ;)
    let mut pos = 7;
    let mut stack: [Stack;STACKS] = [Stack::new();STACKS];
    // parse stack
    for line in input.array_chunks::<36>() {
        for (i,part) in line.array_chunks::<4>().enumerate() {
            if part[1] >= b'A' {
                stack[i].data[pos] = part[1];
                if stack[i].len == 0 {
                    stack[i].len = pos + 1;
                }
            }
        }
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
    // dbg_stack(&stack);
    // println!("{:?}",stack);
    // dbg!(stack.len());
    let iter = input.split(|v|*v == b'\n').skip(stack.len()+1);
    // dbg!("Data:");
    for line in iter {
        // stdout.write(*v as char);
        if line.len() == 0 {
			break;
		}
        let (amount, from,to) = match line.len() > 18 {
            false => ((line[5] - b'0') as usize,(line[12] - b'0' - 1) as usize,(line[17] - b'0' - 1) as usize),
            true => (((line[5] as u32) * 10
            + (line[6] as u32) - 528) as usize,(line[13] - b'0' - 1) as usize,(line[18] - b'0' - 1) as usize),
        };
        // println!("{}",std::str::from_utf8(line).unwrap());
        // println!("move {amount} from {from} to {to}");
        // dbg!(stack[from].len);
        // dbg!(stack[to].len);
        unsafe {
            // we can't get from: &Stack, to: &mut Stack at the same time
            // and I don't wanna copy in between
            // let's rock raw pointers
            let stack = stack.as_mut_ptr();
            let from_r = stack.offset(from as _);
            let to_r = stack.offset(to as _);
            let from_start = (*from_r).len - amount;
            let to_start = (*to_r).len;
            let to_new_len = (*to_r).len + amount;
            let from_rs = (*from_r).data.as_mut_ptr().offset(from_start as _);
            let to_rs = (*to_r).data.as_mut_ptr().offset(to_start as _);
            to_rs.copy_from_nonoverlapping(from_rs, amount);
            (*to_r).len = to_new_len;
            (*from_r).len = from_start;
        }
        // dbg!(stack[from].len);
        // dbg!(stack[to].len);
        // break;
    }
    // dbg_stack(&stack);
    let mut result = [0; STACKS];
    for i in 0..STACKS {
        result[i] = stack[i].top();
    }
    unsafe {std::str::from_utf8_unchecked(&result).to_owned()}
}

fn dbg_stack(d: &[Stack]) {
    let mut line_empty = true;
    let mut height = 0;
    for i in 0..d.len() {
        if d[i].len > height {
            height = d[i].len;
        }
    }
    // dbg!(height);
    for h in (0..height).rev() {
        for i in 0..d.len() {
            if let Some(v) = d[i].get(h) {
                print!("[{}] ",v as char);
                line_empty = false;
            } else {
                print!("    ");
            }
        }
        print!("\n");
    }
}

#[derive(Clone, Copy, Debug)]
struct Stack{
    len: usize,
    data: [u8;ENTRIES]
}

impl Stack {
    const fn new() -> Self {
        Self {
            len: 0,
            data: [0; ENTRIES]
        }
    }
    #[inline(always)]
    fn push(&mut self, data: u8) {
        // assert!(self.len < ENTRIES, "pushed Stack out of boundary");
        self.data[self.len] = data;
        self.len += 1;
    }
    #[inline(always)]
    fn pop(&mut self) -> u8 {
        // assert!(self.len > 0, "popped Stack out of boundary");
        self.len -= 1;
        self.data[self.len]
    }
    #[inline(always)]
    fn top(&self) -> u8 {
        // assert!(self.len > 0, "top on empty Stack");
        self.data[self.len - 1]
    }
    fn get(&self, i: usize) -> Option<u8> {
        match i < self.len {
            true => Some(self.data[i]),
            false => None,
        }
    }
}