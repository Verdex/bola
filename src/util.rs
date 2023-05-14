
static mut I : u64 = 0;

pub fn gen_sym(prefix : &str) -> String {
    let x = unsafe {
        I += 1;
        I
    };

    format!("{}::{}", prefix, x)
}