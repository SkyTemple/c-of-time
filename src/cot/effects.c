#include <pmdsky.h>
#include <cot.h>

// Internal dispatch code for item and move effects and special processes to C and Rust.
// These functions are called in trampolines.s.

bool cotInternalDispatchApplyItemEffect(
        struct entity* user, struct entity* target, struct item* item, bool is_thrown
) {
    bool handled = CustomApplyItemEffect(user, target, item, is_thrown);
#ifdef COT_RUST
    // If the Rust runtime of c-of-time is used, ask the Rust implementation to process the effect.
    if (!handled) {
      handled = eos_rs_apply_item_effect(user, target, item, is_thrown);
    }
#endif
    return handled;
}

bool cotInternalDispatchApplyMoveEffect(
        move_effect_input* data, struct entity* user, struct entity* target, struct move* move
) {
    bool handled = CustomApplyMoveEffect(data, user, target, move);
#ifdef COT_RUST
    // If the Rust runtime of c-of-time is used, ask the Rust implementation to process the effect.
    if (!handled) {
      handled = eos_rs_apply_move_effect(data, user, target, move);
    }
#endif
    return handled;
}

int cotInternalDispatchScriptSpecialProcessCall(
        undefined4* unknown, uint32_t special_process_id, short arg1, short arg2
) {
    int return_val = 0;
    bool handled = CustomScriptSpecialProcessCall(unknown, special_process_id, arg1, arg2, &return_val);
    if (!handled) {
#ifdef COT_RUST
      // If the Rust runtime of c-of-time is used, ask it to take over from here.
      eos_rs_call_special_process(unknown, special_process_id, arg1, arg2, &return_val);
#else
      // Otherwise: Log a warning that the special processed went unhandled.
      COT_WARNFMT(COT_LOG_CAT_SPECIAL_PROCESS, "Unhandled special process ID %d", special_process_id);
#endif
    }
    return return_val;
}
