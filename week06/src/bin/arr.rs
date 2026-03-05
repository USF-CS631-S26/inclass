extern "C" {
    fn arr_get_s(arr: *const i32, i: i32) -> i32;
    fn arr_set_s(arr: *mut i32, i: i32, v: i32);
}

fn arr_get(arr: &[i32], i: usize) -> i32 {
    arr[i]
}

fn arr_set(arr: &mut [i32], i: usize, v: i32) {
    arr[i] = v;
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    let r = arr_get(&arr, 2);
    println!("Rust get arr[2]: {}", r);

    let r = unsafe { arr_get_s(arr.as_ptr(), 2) };
    println!("Asm get arr[2]: {}", r);

    arr_set(&mut arr, 2, 99);
    let r = arr_get(&arr, 2);
    println!("Rust set arr[2]=99, get arr[2]: {}", r);

    arr_set(&mut arr, 2, 3); // reset
    unsafe { arr_set_s(arr.as_mut_ptr(), 2, 99) };
    let r = unsafe { arr_get_s(arr.as_ptr(), 2) };
    println!("Asm set arr[2]=99, get arr[2]: {}", r);
}
