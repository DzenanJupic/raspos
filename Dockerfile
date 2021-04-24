FROM rustembedded/osdev-utils

WORKDIR /code
COPY ./target/raspos.img raspos.img
COPY ./target/raspos raspos

CMD ["qemu-system-aarch64", "-M", "raspi3", "-d", "in_asm", "-display", "none", "-kernel", "raspos.img"]
