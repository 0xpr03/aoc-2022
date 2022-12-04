// from darkfire000
// "this is just C"
#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(core_intrinsics)]
#![feature(split_array)]
use std::{mem::transmute, simd::Simd};

macro_rules! extract_inc {
    ($dat:ident, $out:tt, $search:expr) => {
        if *$dat.add(1) == $search {
            $out = *$dat as i16;
            $dat = $dat.add(2);
        } else {
            $out = i16::from_be_bytes(*transmute::<*const u8, &[u8; 2]>($dat));
            $dat = $dat.add(3);
        }
    };
}

pub fn run(mut dataset: &[u8]) -> i64 {
    let mut dat = dataset.as_ptr();
    let dat_end = dataset.as_ptr_range().end;

    let mut lhs = Simd::from_array([0, 0]);
    let mut rhs = Simd::from_array([0, 0]);

    let mut sum = 0;

    while dat < dat_end {
        unsafe {
            extract_inc!(dat, (lhs[0]), b'-');
            extract_inc!(dat, (lhs[1]), b',');
            extract_inc!(dat, (rhs[0]), b'-');
            extract_inc!(dat, (rhs[1]), b'\n');

            let diff = lhs - rhs;

            let is_subset =
                diff[0].is_negative() != diff[1].is_negative() || diff[0] == 0 || diff[1] == 0;
            if is_subset {
                sum += 1;
            }
        }
    }

    sum
}
