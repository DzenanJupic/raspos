/* The address at which the the kernel binary will be loaded by the Raspberry's firmware */
__RPI_LOAD_ADDR = 0x80000;

ENTRY(__RPI_LOAD_ADDR)

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
    segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
    . =  __RPI_LOAD_ADDR;
    __BOOT_STACK_END_EXCLUSIVE = .;

    .text :
    {
        KEEP(*(.text._start))
        KEEP(*(.text._boot))
        *(.text*)
    } :segment_rx
    
    .rodata : ALIGN(8) { *(.rodata*) } :segment_rx

    .got : ALIGN(8) { *(.got) } :segment_rx
    
    .data : { *(.data*) } :segment_rw
    
    .bss : ALIGN(8)
    {
        __BSS_START = .;
        *(.bss*);
        . = ALIGN(8);
        
        . += 8; /* Fill for the bss == 0 case, so that __bss_start <= __bss_end_inclusive holds */
        __BSS_END_INCLUSIVE = . - 8;
    } :NONE
}
