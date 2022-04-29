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

// Called for special process IDs 100 and greater
int CustomScriptSpecialProcessCall(undefined4* unknown, uint32_t special_process_id, short arg1, short arg2) {
  // TODO: arg2 doesn't seem to match the argument in the script engine?
  COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "Running special process %d (arg1=%d, arg2=%d)",
    special_process_id, arg1, arg2);

  switch (special_process_id) {
    case 100:
      return SpChangeBorderColor(arg1);

    // Add your own SP's here...  

    default:
      COT_WARNFMT(COT_LOG_CAT_SPECIAL_PROCESS, "Unhandled special process ID %d", special_process_id);
  }

  return 0;
}
