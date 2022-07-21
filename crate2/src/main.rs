use crate1::FOO;
use crate1::X;

static BAR: &'static i32 = FOO;
static Y: &i32 = X;

// static mut A: &'static mut i32 = &mut 42;
// static mut B: &'static mut i32 = A;

fn main() {
    println!("FOO pointer is: {:p}", FOO);
    println!("BAR pointer is: {:p}", BAR);
    assert_eq!(FOO as *const i32, BAR as *const i32);

    println!("X pointer is: {:p}", X);
    println!("Y pointer is: {:p}", Y);
    assert_eq!(X as *const i32, Y as *const i32);
}
