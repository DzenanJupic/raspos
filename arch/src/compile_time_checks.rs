#[cfg(not(any(
target_arch = "aarch64",
target_arch = "x86_64",
)))]
compile_error!("unsupported target arch");

#[cfg(all(
feature = "qemu",
not(target_arch = "x86_64")
))]
compile_error!("qemu is currently only supported for x86_64 targets");
