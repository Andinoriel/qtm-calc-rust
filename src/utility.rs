use std::cmp::{max, min};

pub fn clamp_xsize<T>(num: T, min_value: T, max_value: T) -> T
where
    T: Ord,
{
    max(min(num, max_value), min_value)
}
// FIXME: ? -> safe clamp of f64
pub fn clamp_f64(num: f64, min_value: f64, max_value: f64) -> f64 {
    use float_ord::FloatOrd;
    let lhs: f64;
    if FloatOrd(num) < FloatOrd(max_value) {
        lhs = num;
    } else {
        lhs = max_value;
    }
    let ret: f64;
    if FloatOrd(lhs) > FloatOrd(min_value) {
        ret = lhs;
    } else {
        ret = min_value;
    }
    ret
}
