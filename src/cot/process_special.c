#include <pmdsky.h>
#include <cot.h>

void cotInternalTrampolineScriptSpecialProcessCall() {
  // If the special process ID is >= 1000, handle it as a custom special process
  __asm("cmp r1, #100");
  __asm("bge CustomScriptSpecialProcessCall");

  // Otherwise, restore the instruction we've replaced in the patch
  // and run the original function
  __asm("push	{r3, r4, r5, r6, r7, r8, r9, sl, fp, lr}");
  __asm("b ScriptSpecialProcessCall+4");
}
