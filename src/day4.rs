#[aoc(day4, part1, Chars)]
pub fn part1(input: &[u8]) -> i64 {
    input
        .split(|x| *x == b'\n')
        .filter(|s| !s.is_empty())
        // split ,
        .map(|x| (x, x.iter().position(|x| *x == b',').unwrap()))
        .map(|(x, pos)| (&x[..pos], &x[pos + 1..]))
        // split a and b by -
        .map(|(a, b)| {
            (
                a,
                a.iter().position(|x| *x == b'-').unwrap(),
                b,
                b.iter().position(|x| *x == b'-').unwrap(),
            )
        })
        .map(|(a, posa, b, posb)| (&a[..posa], &a[posa + 1..], &b[..posb], &b[posb + 1..]))
        // reinterpret bytes to u16, we don't need the actual int values for our comparison
        // as long as we get the endianess right
        .map(|(a0, a1, b0, b1)| (stou16(a0), stou16(a1), stou16(b0), stou16(b1)))
        // using .filter().count() is actually slower than this
        .map(|v| contains(v.0, v.1, v.2, v.3))
        .sum()
}

#[aoc(day4, part2, Chars)]
pub fn part2(input: &[u8]) -> i64 {
    input
        .split(|x| *x == b'\n')
        .filter(|s| !s.is_empty())
        .map(|x| (x, x.iter().position(|x| *x == b',').unwrap()))
        .map(|(x, pos)| (&x[..pos], &x[pos + 1..]))
        .map(|(a, b)| {
            (
                a,
                a.iter().position(|x| *x == b'-').unwrap(),
                b,
                b.iter().position(|x| *x == b'-').unwrap(),
            )
        })
        .map(|(a, posa, b, posb)| (&a[..posa], &a[posa + 1..], &b[..posb], &b[posb + 1..]))
        .map(|(a0, a1, b0, b1)| (stou16(a0), stou16(a1), stou16(b0), stou16(b1)))
        .map(|v| overlaps(v.0, v.1, v.2, v.3))
        .sum()
}

#[inline(always)]
fn overlaps(a1: u16, a2: u16, b1: u16, b2: u16) -> i64 {
    if a2 < b1 || b2 < a1 {
        0
    } else {
        1
    }
}

#[inline(always)]
fn contains(a1: u16, a2: u16, b1: u16, b2: u16) -> i64 {
    if a1 <= b1 && a2 >= b2 {
        1
    } else if a1 >= b1 && a2 <= b2 {
        1
    } else {
        0
    }
}

#[inline(always)]
fn stou16(bytes: &[u8]) -> u16 {
    if bytes.len() > 1 {
        let n: [u8; 2] = bytes[0..2].try_into().unwrap();
        u16::from_be_bytes(n) // relevant for our comparison to use be !
    } else {
        bytes[0] as u16
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_contains() {
        assert_eq!(contains(2, 4, 4, 5), 0);
        assert_eq!(contains(2, 4, 1, 5), 1);
        assert_eq!(contains(2, 4, 2, 4), 1);
        assert_eq!(contains(2, 4, 3, 4), 1);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(overlaps(2, 4, 1, 2), 1);
        assert_eq!(overlaps(2, 4, 3, 4), 1);
        assert_eq!(overlaps(2, 4, 1, 5), 1);
        assert_eq!(overlaps(2, 4, 3, 5), 1);
        assert_eq!(overlaps(2, 4, 1, 1), 0);
        assert_eq!(overlaps(2, 4, 5, 6), 0);
    }
}
