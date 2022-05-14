#include <pmdsky.h>
#include <cot.h>

// Elixir: Refills 10 PP of each move
static void ItemElixir(struct entity* target) {
  if (target->type == ENTITY_MONSTER) {
    struct monster* target_monster = (struct monster*) target->info;
    for (int i = 0; i < 4; i++) {
      struct move* current_move = &target_monster->moves[i];
      uint8_t max_pp = GetMaxPp(current_move);
      int new_pp = current_move->pp + 10;
      if (new_pp > max_pp) {
        new_pp = max_pp;
      }
      current_move->pp = new_pp;
    }
  }
}

// Called when using items. Should return true if a custom effect was applied.
bool CustomApplyItemEffect(
  struct entity* user, struct entity* target, struct item* item, bool is_thrown
) {
  COT_LOGFMT(COT_LOG_CAT_EFFECTS, "Running item effect %d", item->id.val);
  switch (item->id.val) {
    case ITEM_MAX_ELIXIR:
      // Replace item 99 (Max Elixir) with custom Elixir effect
      ItemElixir(target);
      // Return true to signal that we've handled the effect
      return true;
    default:
#ifdef COT_RUST
      // If the Rust runtime of c-of-time is used, ask the Rust implementation to process the effect.
      return eos_rs_apply_item_effect(user, target, item, is_thrown);
#else
      // Otherwise: Return false to use the game's normal effect
      return false;
#endif
  }
}
