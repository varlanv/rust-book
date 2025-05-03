use _12_1_minigrep::{App, UnsafeArgs};
use std::env;

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    App::new(UnsafeArgs::new(&args_vec, ignore_case)).run()
}
