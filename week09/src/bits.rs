pub fn get_bits(num: u64, start: u32, count: u32) -> u32 {
    let mask: u64 = (1u64 << count) - 1;
    ((num >> start) & mask) as u32
}

pub fn sign_extend(num: u64, start: u32) -> i64 {
    let dist = 64 - start;
    let shifted = (num << dist) as i64;
    shifted >> dist
}
