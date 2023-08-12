use std::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    let address = 0x012345usize;
    let _r = address as *const i32;
    /* unsafe {
        println!("r: {}", *_r);
    } */

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x012345usize;
    let r = address as *mut i32;
    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("values: {:?}", _values);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {
    println!("unsafe function called");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {}
unsafe impl Foo for i32 {}
