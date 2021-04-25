FROM rustembedded/osdev-utils

WORKDIR /code
COPY ./target/kernel.img kernel.img

CMD ["qemu-system-aarch64", "-M", "raspi3", "-d", "in_asm", "-display", "none", "-kernel", "kernel.img"]
