#include <pmdsky.h>
#include <cot.h>

// This function isn't in pmdsky-debug yet, so we have to declare it
// here and add its offset in "symbols/custom_[region].ld".
extern void ChangeGlobalBorderColor(int color_type);

// Special process 100: Change border color
// Based on https://github.com/SkyTemple/eos-move-effects/blob/master/example/process/set_frame_color.asm
static int SpChangeBorderColor(short arg1) {
  ChangeGlobalBorderColor(arg1);
  return 0;
}

// Called for special process IDs 100 and greater.
//
// Set return_val to the return value that should be passed back to the game's script engine. Return true,
// if the special process was handled.
bool CustomScriptSpecialProcessCall(undefined4* unknown, uint32_t special_process_id, short arg1, short arg2, int* return_val) {
  switch (special_process_id) {
    case 100:
      *return_val = SpChangeBorderColor(arg1);
      return true;

    // Add your own SP's here...

    default:
      return false;
  }
}
