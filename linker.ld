OUTPUT_ARCH(arm)
MEMORY {
        /* 
                Overlay load address + offset to common area
                see https://docs.google.com/document/d/1Rs4icdYtiM6KYnWxMkdlw7jpWrH7qw5v6LOfDWIiYho
        */
        main    : ORIGIN = 0x23D7FF0, LENGTH = 0x8010
        /* Change/Add memory locations here */
        /* NEW    : ORIGIN = 0x2000000, LENGTH = 0x800 */
        cbmove  : ORIGIN = MoveStartAddress, LENGTH = 0x2598
        cbitem  : ORIGIN = ItemStartAddress, LENGTH = 0xCC4
        cbsp    : ORIGIN = ProcStartAddress, LENGTH = 0x810
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
                Extensions are of format ".text.EXT.[]" where EXT is the extension name
                as described in EXTENSIONS in 'patch.py', [] are per extension sub params
                Two extensions are preloaded:
                - 'file' lets you output compiled binary to a separate file in 'fragments' folder
                  Exact file name is described by sub params
                  You are free to use any memory area for this extension
                - 'codebin' lets you output compiled binary directly to 'BALANCE/waza_cd.bin',
                  'BALANCE/item_cd.bin' and 'BALANCE/process.bin' effects file slots
                  sub params are 'CTYPE.SLOT', where CTYPE is ('move', 'item', 'sp') and
                  SLOT the slot number to replace
                  Memory areas to use are already defined in this file for all three types
                  NOTE: Moves/Items/SPs must be placed inside their dedicated
                  OVERLAY block to share their virtual address
        */
        /*
                Extensions example:
        */
        /*.text.file.out.bin : {
                *(.myout.init)
                *(.myout)
        } >NEW = 0xff*/
        /*
        OVERLAY : {
                .text.codebin.move.23 {
                        *(.codebin.mymove.init)
                        *(.codebin.mymove)
                }
        } >cbmove = 0xff
        OVERLAY : {
                .text.codebin.item.45 {
                        *(.codebin.myitem.init)
                        *(.codebin.myitem)
                }
        } >cbitem = 0xff
        OVERLAY : {
                .text.codebin.sp.67 {
                        *(.codebin.mysp.init)
                        *(.codebin.mysp)
                }
        } >cbsp = 0xff
        */
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
