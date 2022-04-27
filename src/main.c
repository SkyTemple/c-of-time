#include <pmdsky.h>
#include <cot.h>

// This function is called from a patch in patches/patch.cotpatch
int CustomGetMovePower(struct entity* entity, struct move* move) {
  // Randomize move power
  int rolledPower = RandRange(1, 100);

  // Print the rolled value to the message log
  char messageBuffer[32];
  Snprintf(messageBuffer, 32, "Rolled move power %d!", rolledPower);
  
  LogMessage(entity, messageBuffer, true);

  return rolledPower;
}

// This function isn't in pmdsky-debug yet, so we have to declare it
// here and add its offset in "symbols/custom_[region].ld".
extern void ChangeGlobalBorderColor(bool isPink);

// Special process 100: Change border color
int SpChangeBorderColor(short arg1) {
  ChangeGlobalBorderColor(arg1 != 0);
  return 0;
}

// Called for special process IDs 100 and greater
int CustomScriptSpecialProcessCall(undefined4* unknown, uint32_t special_process_id, short arg1, short arg2) {
  // TODO: arg2 doesn't seem to match the argument in the script engine?
  COT_LOG(COT_LOG_CAT_SPECIAL_PROCESS, "Running special process %d (arg1=%d, arg2=%d)",
    special_process_id, arg1, arg2);

  switch (special_process_id) {
    case 100:
      return SpChangeBorderColor(arg1);

    // Add your own SP's here...

    default:
      COT_WARN(COT_LOG_CAT_SPECIAL_PROCESS, "Unhandled special process ID %d", special_process_id);
  }

  return 0;
}

