extern "C" {
    fn get_bitseq_s(num: u32, start: u32, end: u32) -> u32;
}

fn get_bitseq(num: u32, start: u32, end: u32) -> u32 {
    let shifted = num >> start;
    let mask = (1u32 << (end - start + 1)) - 1;
    shifted & mask
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (num, start, end) = if args.len() > 3 {
        (
            args[1].parse::<u32>().unwrap(),
            args[2].parse::<u32>().unwrap(),
            args[3].parse::<u32>().unwrap(),
        )
    } else {
        (552, 3, 5)
    };

    let r = get_bitseq(num, start, end);
    println!("Rust: {}", r);

    let r = unsafe { get_bitseq_s(num, start, end) };
    println!("Asm: {}", r);
}
