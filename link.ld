__rpi_load_addr = 0x80000;

ENTRY(__rpi_load_addr)

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
}

SECTIONS
{
    . =  __rpi_load_addr;

    .text :
    {
        KEEP(*(.text._start))
    } :segment_rx
}
