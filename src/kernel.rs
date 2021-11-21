/// This is the entrypoint of the kernel. It's the responsibility of the arch-dependent _start
/// function to call into the kernel.
/// When the kernel returns, the caller may shutdown the system.
#[inline(never)]
pub fn main() {
    println!("Hello From Rust!");
    panic!("This is the end");
}
