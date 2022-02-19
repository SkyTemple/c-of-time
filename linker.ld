OUTPUT_ARCH(arm)
MEMORY {
        overlay36     : ORIGIN = 0x023A7080, LENGTH = 0x38F80
}

SECTIONS {
        .text : {

                FILL (0xABCD)

                __text_start = . ;

                __init_array_start = .;
                KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*)))
                KEEP (*(.init_array))
                __init_array_end = .;
                __fini_array_start = .;
                KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*)))
                KEEP (*(.fini_array))
                __fini_array_end = .;
                
                *(.init)
                *(.text)
                *(.ctors)
                *(.dtors)
                *(.rodata)
                *(.fini)
                *(COMMON)
                __text_end  = . ;
                . = ALIGN(4);
        } >overlay36 = 0xff

        .bss : {
                __bss_start__ = . ;
                *(.bss)
                __bss_end__ = . ;

                _end = __bss_end__ ;
                __end__ = __bss_end__ ;
                . = ALIGN(4);
        } >overlay36 = 0xff
}
