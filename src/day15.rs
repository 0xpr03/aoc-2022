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
pub fn part1(input: &[u8]) -> isize {
    const ASKED_Y: isize = 2000000;
    // const ASKED_Y: isize = 10;
    let mut ranges: Vec<Range> = Vec::with_capacity(25);
    // let mut skipped = 0;
    for v in input.split(|x|*x == b'\n').filter(|v|!v.is_empty()) {
        let iter = v.split(|x|*x == b',' || *x == b':');
        // iter_next_chunk is marked as slow https://github.com/rust-lang/rust/issues/98326#issuecomment-1166338225
        // so we'll use this
        let data = iter.array_chunks::<4>().next().unwrap();

        let s_x = atoi(&data[0][12..]);
        let s_y = atoi(&data[1][3..]);
        let b_x = atoi(&data[2][24..]);
        let b_y = atoi(&data[3][3..]);

        let distance = s_x.abs_diff(b_x) + s_y.abs_diff(b_y);
        let distance_isize = distance as isize;
        if s_y > ASKED_Y && s_y - distance_isize > ASKED_Y {
            continue;
        }
        if s_y < ASKED_Y && s_y + distance_isize < ASKED_Y {
            continue;
        }
        let rem = (distance - s_y.abs_diff(ASKED_Y)) as isize;
        let mut range = Some(Range(s_x - (rem),s_x+(rem)));
        
        for rm in ranges.iter_mut() {
            if let Some(r) = range {
                range = r.adjust(rm);
            } else {
                break;
            }
        }
        if let Some(r) = range {
            ranges.push(r);
        }
    }
    let mut new_ranges = Vec::with_capacity(ranges.len());
    for p in ranges.into_iter() {
        let mut range = Some(p);
        
        for rm in new_ranges.iter_mut() {
            if let Some(r) = range {
                range = r.adjust(rm);
            } else {
                break;
            }
        }
        if let Some(r) = range {
            new_ranges.push(r);
        }
    }
    let mut new_ranges2 = Vec::with_capacity(new_ranges.len());
    for p in new_ranges.into_iter() {
        let mut range = Some(p);
        
        for rm in new_ranges2.iter_mut() {
            if let Some(r) = range {
                range = r.adjust(rm);
            } else {
                break;
            }
        }
        if let Some(r) = range {
            new_ranges2.push(r);
        }
    }
    new_ranges2.iter().map(|x|x.count()).sum()
}


#[derive(Debug,PartialEq, Eq, Copy, Clone)]
struct Range(isize,isize);
impl Range {
    #[inline(always)]
    fn adjust(self,other: &mut Range) -> Option<Range> {
        if self.1 < other.0 || self.0 > other.1 {
            Some(self)
        } else if self.0 < other.0 {
            if self.1 >= other.1 {
                other.0 = self.0;
                other.1 = self.1;
                None
            } else {
                other.0 = self.0;
                None
            }
        } else if self.1 > other.1 {
            other.1 = self.1;
            None
        } else { // contained in other range
            None
        }
    }
    #[inline(always)]
    fn count(&self) -> isize {
        self.1 - self.0
    }
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
    // } else if part.len() == 3 {
    //     ((part[0] & 0xf) as isize *100+
    //     (part[1] & 0xf) as isize *10+
    //     (part[2] & 0xf) as isize) * mult
    // } else if part.len() == 2 {
    //     ((part[0] & 0xf) as isize *10+
    //     (part[1] & 0xf) as isize) * mult
    // } else if part.len() == 1 {
    //     ((part[0] & 0xf) as isize) * mult
    } else {
        unreachable!("Atoi Len: {}",part.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range() {
        // left intersect right from left
        let mut r = Range(2,4);
        assert_eq!(Range(1,3).adjust(&mut r),None);
        assert_eq!(r,Range(1,4));
        // left intersect right from left
        let mut r = Range(1,3);
        assert_eq!(Range(2,4).adjust(&mut r),None);
        assert_eq!(r,Range(1,4));
        // left contains right
        let mut r = Range(2,3);
        assert_eq!(Range(1,5).adjust(&mut r),None);
        assert_eq!(r,Range(1,5));
        // right contains left
        let mut r = Range(1,5);
        assert_eq!(Range(2,3).adjust(&mut r),None);
        assert_eq!(r,Range(1,5));
        // left doesn't intersect right
        let mut r = Range(1,2);
        assert_eq!(Range(3,4).adjust(&mut r),Some(Range(3,4)));
        // left intersect right from left, barely
        let mut r = Range(2,4);
        assert_eq!(Range(1,2).adjust(&mut r),None);
        assert_eq!(r,Range(1,4));
        // left intersect right from right, barely
        let mut r = Range(1,2);
        assert_eq!(Range(2,4).adjust(&mut r),None);
        assert_eq!(r,Range(1,4));
    }
}