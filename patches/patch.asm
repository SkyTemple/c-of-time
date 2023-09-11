// Replace the "GetMovePower" function with a custom one.
// Since a branch is inserted at the start of the function, the function is practically
// replaced with our own. The "b" instruction doesn't modify the link register, so
// execution will continue after the call to `GetMovePower` once our function returns.

.nds
.include "symbols.asm"

.open "overlay29.bin", overlay29_start
    .org GetMovePower
        // Remove the comment below to enable this example
        // b CustomGetMovePower
.close
