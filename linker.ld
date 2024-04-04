OUTPUT_ARCH(arm)
MEMORY {
        /* 
                Overlay load address + offset to common area
                see https://docs.google.com/document/d/1Rs4icdYtiM6KYnWxMkdlw7jpWrH7qw5v6LOfDWIiYho
        */
        main    : ORIGIN = 0x23D7FF0, LENGTH = 0x8010
        /* Change/Add memory locations here */
        /* NEW    : ORIGIN = 0x2000000, LENGTH = 0x800 */
}

SECTIONS {
        /*
                Add more sections for the linker, each section using a different memory location
                This allows to spread your code on different overlays
                You can change the section of one symbol (function or variable) in your C code using: __attribute__((section(".YOURSECTIONNAME")))
                Note that section names are standardized and in the form ".text.OV.NB.SPECIAL"
                Where OV is between 'ov' (ARM9 Overlay) and 'arm' (Main ARM Binary)
                NB is the OV number (7 or 9 for 'arm', any ARM9 Overlay number for 'ov')
                SPECIAL is a distinct name, this is only to distinguish between 2 sections on the same overlay
        */
        /*.text.OV.NB.SPECIAL : {
                *(.YOURSECTIONNAME)
        } >NEW = 0xff*/
        /*
                Main Section (Default section, in ARM9 overlay 36)
                Note that you should add your special sections before that
        */
        .text.ov.36.main : {
                *(.init)
                *(.text)
                *(.text.*)
                *(.fini)
                *(.ctors)
                *(.dtors)
                *(.rodata)
                *(.rodata.*)
                *(.data)
                *(.data.*)
                *(COMMON)
                . = ALIGN(4);
                *(.bss)
        } >main = 0xff
}
