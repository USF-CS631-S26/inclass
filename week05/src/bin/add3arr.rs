use std::env;
use std::process;

extern "C" {
    fn add3arr_s(arr: *const i32) -> i32;
}

fn add3arr_c(arr: &[i32; 3]) -> i32 {
    arr[0] + arr[1] + arr[2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("usage: add3arr a b c");
        process::exit(-1);
    }

    let arr: [i32; 3] = [
        args[1].parse().unwrap_or(0),
        args[2].parse().unwrap_or(0),
        args[3].parse().unwrap_or(0),
    ];

    let r = add3arr_c(&arr);
    println!("add3arr_c({:p}) = {}", arr.as_ptr(), r);

    let r = unsafe { add3arr_s(arr.as_ptr()) };
    println!("add3arr_s({:p}) = {}", arr.as_ptr(), r);
}
