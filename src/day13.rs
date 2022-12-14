#[aoc_generator(day13)]
pub fn input_generator(input: &[u8]) -> Vec<u8> {
    // cargo aoc trims the input
    let mut data: Vec<u8> = input.iter().copied().collect();
    if data.last() != Some(&b'\n') {
        println!("Patching missing newline");
        data.push(b'\n');
    }
    data
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Entity {
    Value(u16),
    List(Vec<Entity>)
}

impl Ord for Entity {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self,other) {
            (Entity::Value(a), Entity::Value(b)) => a.cmp(b),
            (Entity::Value(a), Entity::List(_)) => Entity::List(vec![Entity::Value(*a)]).cmp(other),
            (Entity::List(_), Entity::Value(b)) => self.cmp(&Entity::List(vec![Entity::Value(*b)])),
            (Entity::List(la), Entity::List(lb)) => {
                for (a,b) in la.iter().zip(lb) {
                    let v = a.cmp(b);
                    if v.is_ne() {
                        return v;
                    }
                }
                la.len().cmp(&lb.len())
            },
        }
    }
}

impl PartialOrd for Entity {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

const AMOUNT: usize = 100;
#[aoc(day13, part1, Schokis)]
pub fn part1(input: &[u8]) -> Stack<usize,AMOUNT> {
    const _start: [u8; 0] = [0; 0];
    let mut left = &_start[..];
    
    let mut ordered: Stack<usize,AMOUNT> = Stack {
        data: [0;AMOUNT],
        len: 0,
    };

    let mut amount = 0;
    for (i,v) in input.split(|x|*x == b'\n').enumerate() {
        match i % 3 {
            0 => left = v,
            1 => {
                let mut right = v;
                
                // println!("l:{}",std::str::from_utf8(left).unwrap());
                let parse_l = parse(&mut left).unwrap();
                // println!("{:?}",parse_l);
                // println!("r:{}",std::str::from_utf8(right).unwrap());
                let parse_r = parse(&mut right).unwrap();
                // println!("{:?}",parse_r);

                if parse_l.cmp(&parse_r).is_le() {
                    ordered.push(i / 3);
                }
                
            },
            _ => (),
        }
    }
    ordered
}

#[aoc(day13, part2, Schokis)]
pub fn part2(input: &[u8]) -> usize {
    const _start: [u8; 0] = [0; 0];
    let mut left = &_start[..];
    
    let mut entries = arrayvec::ArrayVec::<_,302>::new();

    let mut amount = 0;
    for (i,v) in input.split(|x|*x == b'\n').enumerate() {
        let mut v = v;
        match i % 3 {
            0 => entries.push(parse(&mut v).unwrap()),
            1 => entries.push(parse(&mut v).unwrap()),
            _ => (),
        }
    }
    let a = Entity::List(vec![Entity::List(vec![Entity::Value(2)])]);
    let b = Entity::List(vec![Entity::List(vec![Entity::Value(6)])]);
    entries.push(a.clone());
    entries.push(b.clone());
    entries.sort_unstable();
    // for (i,v) in entries.iter().enumerate() {
    //     println!("{i} {:?}",v);
    // }
    let pos_a = entries.iter().position(|v| *v == a ).unwrap_or(0);
    let pos_b = entries.iter().position(|v| *v == b ).unwrap_or(0);
    (pos_a+1) * (pos_b+1)
}

#[inline(always)]
fn parse(line: &mut &[u8]) -> Option<Entity> {
    match (line.get(0),line.get(1)) {
        (Some(a@b'0'..=b'9'), Some(b@b'0'..=b'9')) => {
            *line = &line[2..];
            Some(Entity::Value((*a as u16) * 10
                        + (*b as u16) - 528))
        }
        (Some(a@b'0'..=b'9'),_) => {
            *line = &line[1..];
            Some(Entity::Value((*a - b'0') as u16))
        },
        (Some(b'['),Some(b']')) => {
            *line = &line[2..];
            return Some(Entity::List(Vec::with_capacity(0)));
        }
        (Some(b'['),_) => {
            let mut list = Vec::new();
            *line = &line[1..];
            while let Some(v) = parse(line) {
                list.push(v);
                if line[0] == b']' {
                    *line = &line[1..];
                    return Some(Entity::List(list));
                } else {
                    *line = &line[1..];
                }
            }
            unreachable!()
        },
        _ => {
            *line = &line[1..];
            None
        }
    }
}


#[derive(Copy, Clone)]
pub struct Stack<T: Sized + Copy, const COUNT: usize> {
    len: usize,
    data: [T; COUNT]
}

impl<const COUNT: usize> std::fmt::Display for Stack<usize,COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sum: usize = self.data[0..self.len].iter().map(|v|v+1).sum();
        write!(f, "{}", sum)
    }
}

impl<T: Sized + Copy + std::fmt::Debug, const COUNT: usize> std::fmt::Debug for Stack<T,COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            write!(f, "{:?},", self.data[i])?;
        }
        Ok(())
    }
}

impl<T: Sized + Copy + Default, const COUNT: usize> Stack<T,COUNT> {
    #[inline(always)]
    fn push(&mut self, data: T) {
        //assert!(self.len < COUNT, "pushed Stack out of boundary");
        unsafe {*self.data.get_unchecked_mut(self.len) = data;}
        self.len += 1;
    }
    #[inline(always)]
    fn pop(&mut self) -> T {
        //assert!(self.len > 0, "stack pop underflow");
        self.len -= 1;
        unsafe {*self.data.get_unchecked(self.len)}
    }
}