use core::cell::UnsafeCell;
use tock_registers::interfaces::Readable;

/// The core that is responsible for booting
const BOOT_CORE_ID: u64 = 0;
const CORE_ID_MASK: u64 = 0b11;

extern {
    static __BOOT_STACK_END_EXCLUSIVE: UnsafeCell<u64>;
    static __BSS_START: UnsafeCell<u64>;
    static __BSS_END_INCLUSIVE: UnsafeCell<u64>;
}


#[naked]
#[no_mangle]
#[link_section = ".text._start"]
pub unsafe extern "C" fn _start() -> ! {
    // The first thing we have to do is to setup the SP properly.
    // Theres the extern static __BOOT_STACK_END_EXCLUSIVE, that's
    // defined in the linker script and tells us where the stack
    // ends.
    // After that we'll call the _boot function to handle the rest.
    asm!(
    "adrp x0, {stack_addr}          ",
    "add  x0, x0, :lo12:{stack_addr}",
    "mov  sp, x0                    ",
    "b {boot}                       ",
    "b {wait_forever}               ",
    stack_addr = sym __BOOT_STACK_END_EXCLUSIVE,
    boot = sym _boot,
    wait_forever = sym crate::arch::wait_forever,
    options(noreturn),
    )
}

#[no_mangle]
#[link_section = ".text._boot"]
unsafe extern "C" fn _boot() -> ! {
    // Before we let the kernel take over, we have to set
    // some things up.

    // We have the ensure, that we are on the core
    // that is supposed to boot.
    check_core_is_boot_core();

    // We should initialized .bss (uninitialized
    // statics) to 0.
    // SAFETY:
    //  We just checked, that we are the unique
    //  boot core.
    zero_bss();

    crate::kernel::main();
}


/// Checks if the current core is the boot core and never returns, if not.
#[inline(always)]
fn check_core_is_boot_core() {
    let core_id = cortex_a::registers::MPIDR_EL1.get() & CORE_ID_MASK;

    if core_id != BOOT_CORE_ID {
        crate::arch::wait_forever();
    }
}

/// Zeros out the .bss section
///
/// SAFETY:
/// Only one core may call this function at a time.
#[inline(always)]
unsafe fn zero_bss() {
    let start = __BSS_START.get();
    let end = __BSS_END_INCLUSIVE.get();
    let len = end.offset_from(start);

    for i in 0..=len {
        let ptr = start.offset(i);
        core::ptr::write_volatile(ptr, 0);
    }
}
