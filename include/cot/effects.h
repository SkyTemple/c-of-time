#pragma once

typedef struct move_effect_input {
  int move_id;
  int item_id;
  bool out_dealt_damage;
} move_effect_input;

// Custom effects handling functions.
bool CustomApplyItemEffect(struct entity* user, struct entity* target, struct item* item, bool is_thrown);
bool CustomApplyMoveEffect(move_effect_input* data, struct entity* user, struct entity* target, struct move* move);
bool CustomScriptSpecialProcessCall(undefined4* unknown, uint32_t special_process_id, short arg1, short arg2, int* return_val);
