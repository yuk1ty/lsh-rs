extern crate lsh_rs;

use lsh_rs::lsh::*;

fn main() {
    let lsh_loop = LshLoop::new();
    match lsh_loop.start() {
        Ok(_) => {}
        Err(err) => println!("{}", err.message),
    }
}
