//#![feature(byte_slice_trim_ascii)]
const SIZE: usize = 50;
#[aoc(day11, part1, Schokis)]
pub fn part1(input: &[u8]) -> usize {
    // using u32 for counting and i32 for items is actually better
    // for p1, but won't work with p2..
    const RUNS: usize = 20;
    let mut monkeys = [Monkey::new();10];
    for (i,v) in input.split(|x|*x == b'\n').enumerate() {
        let monkey = unsafe {monkeys.get_unchecked_mut(i / 7)};
        match i % 7 {
            1 => { // starting items
                let items = unsafe {v.get_unchecked(18..).split(|x|*x == b',')};
                for item in items {
                    let trimmed = item.trim_ascii();
                    monkey.items.push(atoi(trimmed) as _);
                }
            }
            2 => { // operation
                monkey.mul = unsafe {*v.get_unchecked(23) == b'*'};
                if v[25] > b'9' {
                    monkey.factor = Factor::Old;
                } else {
                    monkey.factor = Factor::N(atoi(&v[25..]) as _);
                }
            }
            3 => { // test
                monkey.divisor = atoi(unsafe {v.get_unchecked(21..)}) as _;
            }
            4 => { // if true, throw to monkey
                monkey.true_monkey = atoi(unsafe {v.get_unchecked(29..)});
            }
            5 => { // if false, throw to monkey
                monkey.false_monkey = atoi(unsafe {v.get_unchecked(30..)});
            }
            _ => ()
        }
    }
    unsafe {
    let monkey_amount = monkeys.len();
    let monkeys = monkeys.as_mut_ptr();
    for _ in 0..RUNS {
        for mid in 0..monkey_amount {
            let monkey = monkeys.offset(mid as _);
            let item_amount = (*monkey).items.len;
            for iid in 0..item_amount {
                let item = (*monkey).items.data[iid];
                let item = match ((*monkey).mul,(*monkey).factor) {
                    (true,Factor::Old) => item * item,
                    (false,Factor::Old) => item + item,
                    (true,Factor::N(n)) => item * n,
                    (false,Factor::N(n)) => item + n,
                };
                let item = item / 3;
                let new_monkey = match item % (*monkey).divisor == 0 {
                    true => (*monkey).true_monkey,
                    false => (*monkey).false_monkey,
                };

                let new_monkey = monkeys.offset(new_monkey as _);
                (*new_monkey).items.push(item);
            }
            
            (*monkey).items.len = 0;
            (*monkey).inspected += item_amount;
        }
    }
    }
    let mut first = 0;
    let mut second = 0;
    for m in monkeys {
        if m.inspected > second {
            if m.inspected > first {
                second = first;
                first = m.inspected;
            } else {
                second = m.inspected;
            }
        }
    }
    first * second
}

#[aoc(day11, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    const RUNS: usize = 10000;
    let mut monkeys = [Monkey::new();10];
    let mut amount_monkeys = 0;
    let mut denominator: usize = 1;
    for (i,v) in input.split(|x|*x == b'\n').enumerate() {
        let monkey = unsafe {monkeys.get_unchecked_mut(i / 7)};
        match i % 7 {
            1 => { // starting items
                let items = unsafe {v.get_unchecked(18..).split(|x|*x == b',')};
                for item in items {
                    let trimmed = item.trim_ascii();
                    monkey.items.push(atoi(trimmed) as _);
                }
            }
            2 => { // operation
                monkey.mul = unsafe {*v.get_unchecked(23) == b'*'};
                if v[25] > b'9' {
                    monkey.factor = Factor::Old;
                } else {
                    monkey.factor = Factor::N(atoi(&v[25..]) as _);
                }
            }
            3 => { // test
                let div = atoi(unsafe {v.get_unchecked(21..)});
                monkey.divisor = div as _;
                denominator *= div as usize;
            }
            4 => { // if true, throw to monkey
                monkey.true_monkey = atoi(unsafe {v.get_unchecked(29..)});
            }
            5 => { // if false, throw to monkey
                monkey.false_monkey = atoi(unsafe {v.get_unchecked(30..)});
                amount_monkeys += 1; // could be one of i / 7..
            }
            _ => ()
        }
    }
    unsafe {
    let monkeys = monkeys.as_mut_ptr();
    for _ in 0..RUNS {
        for mid in 0..amount_monkeys {
            let monkey = monkeys.offset(mid as _);
            let item_amount = (*monkey).items.len;
            for iid in 0..item_amount {
                let item = (*monkey).items.data[iid];
                let item = match ((*monkey).mul,(*monkey).factor) {
                    (true,Factor::Old) => item * item,
                    (false,Factor::Old) => item + item,
                    (true,Factor::N(n)) => item * n,
                    (false,Factor::N(n)) => item + n,
                };
                // modulo shenanigans, so we don't overflow
                // has to be a multiple of all divisors used
                let item = item % denominator;
                let new_monkey = match item % (*monkey).divisor == 0 {
                    true => (*monkey).true_monkey,
                    false => (*monkey).false_monkey,
                };

                let new_monkey = monkeys.offset(new_monkey as _);
                (*new_monkey).items.push(item);
            }
            
            (*monkey).items.len = 0;
            (*monkey).inspected += item_amount;
        }
    }
    }
    let mut first = 0;
    let mut second = 0;
    
    // for m in 0..amount_monkeys {
    //     dbg!(monkeys[m]);
    // }
    for m in monkeys {
        // println!("{}",m.inspected);
        if m.inspected > second {
            if m.inspected > first {
                second = first;
                first = m.inspected;
            } else {
                second = m.inspected;
            }
        }
    }
    first * second
}

#[derive(Copy, Clone, Debug)]
struct Monkey {
    /// multiply ? true : add
    mul: bool,
    /// Operation factor: old [mul] factor
    factor: Factor,
    /// starting items
    items: Stack,
    /// divisible by
    divisor: usize,
    /// if true throw to monkey
    true_monkey: i32,
    /// if false throw to monkey
    false_monkey: i32,
    /// amount of inspected items
    inspected: usize,
}

#[derive(Copy, Clone, Debug)]
enum Factor {
    Old,
    N(usize)
}

#[inline(always)]
fn atoi(v: &[u8]) -> i32 {
    if v.len() > 1 {
        (v[0] as i32) * 10
        + (v[1] as i32) - 528
    } else {
        (v[0] - b'0') as i32
    }
}

impl Monkey {
    const fn new() -> Self {
        Self {
            mul: false,
            factor: Factor::N(0),
            items: Stack::new(),
            divisor: 0,
            true_monkey: 0,
            false_monkey: 0,
            inspected: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Stack {
    len: usize,
    data: [usize; SIZE]
}

impl Stack {
    const fn new() -> Self {
        Self {
            len: 0,
            data: [0; SIZE]
        }
    }
    #[inline(always)]
    fn push(&mut self, data: usize) {
        //assert!(self.len < SIZE, "pushed Stack out of boundary");
        unsafe {*self.data.get_unchecked_mut(self.len) = data;}
        self.len += 1;
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}