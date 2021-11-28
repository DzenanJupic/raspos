use crate::arch;

/// This is the entrypoint of the kernel. It's the responsibility of the arch-dependent _start
/// function to call into the kernel.
#[inline(never)]
pub fn main() -> ! {
    #[cfg(test)] {
        crate::test_main();
        arch::shut_down(arch::ExitCode::Success);
    }

    println!("Hello From Rust!");
    arch::shut_down(arch::ExitCode::Success);
}
