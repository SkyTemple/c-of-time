//! Traits, structs and functions related to file handling and I/O.

//     - name: FileInitVeneer
//       address:
//         NA: 0x2008204
//         EU: 0x2008204
//       description: |-
//         Likely a linker-generated veneer for FileInit.
//
//         See https://developer.arm.com/documentation/dui0474/k/image-structure-and-generation/linker-generated-veneers/what-is-a-veneer-
//
//         r0: file_stream pointer
//     - name: FileOpen
//       address:
//         NA: 0x2008210
//         EU: 0x2008210
//       description: |-
//         Opens a file from the ROM file system at the given path, sort of like C's fopen(3) library function.
//
//         r0: file_stream pointer
//         r1: file path string
//     - name: FileGetSize
//       address:
//         NA: 0x2008244
//         EU: 0x2008244
//       description: |-
//         Gets the size of an open file.
//
//         r0: file_stream pointer
//         return: file size
//     - name: FileRead
//       address:
//         NA: 0x2008254
//         EU: 0x2008254
//       description: |-
//         Reads the contents of a file into the given buffer, and moves the file cursor accordingly.
//
//         Data transfer mode must have been initialized (with DataTransferInit) prior to calling this function. This function looks like it's doing something akin to calling read(2) or fread(3) in a loop until all the bytes have been successfully read.
//
//         r0: file_stream pointer
//         r1: [output] buffer
//         r2: number of bytes to read
//         return: number of bytes read
//     - name: FileSeek
//       address:
//         NA: 0x20082A8
//         EU: 0x20082A8
//       description: |-
//         Sets a file stream's position indicator.
//
//         This function has the a similar API to the fseek(3) library function from C, including using the same codes for the `whence` parameter:
//         - SEEK_SET=0
//         - SEEK_CUR=1
//         - SEEK_END=2 (maybe not implemented?).
//
//         r0: file_stream pointer
//         r1: offset
//         r2: whence
//     - name: FileClose
//       address:
//         NA: 0x20082C4
//         EU: 0x20082C4
//       description: |-
//         Closes a file.
//
//         Data transfer mode must have been initialized (with DataTransferInit) prior to calling this function.
//
//         Note: It is possible to keep a file stream open even if data transfer mode has been stopped, in which case the file stream can be used again if data transfer mode is reinitialized.
//
//         r0: file_stream pointer
//     - name: LoadFileFromRom
//       address:
//         NA: 0x2008C3C
//         EU: 0x2008C3C
//       description: |-
//         Loads a file from ROM by filepath into a heap-allocated buffer.
//
//         r0: [output] pointer to an IO struct {ptr, len}
//         r1: file path string pointer
//         r2: flags


//     - name: FileInit
//       address:
//         NA: 0x207F3E4
//         EU: 0x207F77C
//       description: |-
//         Initializes a file_stream structure for file I/O.
//
//         This function must always be called before opening a file.
//
//         r0: file_stream pointer

