mod callback;

use callback::ffi::callme;

fn rcb(x: i32) -> i32 {
    println!("Hello from rcb. x = {}", x);
    7 * x
}

fn main() {
    println!("Main start.");
    callme(rcb);
    println!("Main end.");
}
