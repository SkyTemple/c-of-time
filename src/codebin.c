/*
  Examples of CODEBIN function declarations
  Macros CODEBIN_MOVE, CODEBIN_ITEM and CODEBIN_SP are defined in <cot/codebin.h>
  CODEBIN_*(#name#) defines two functions "#name#_init" and "#name#"
  placed in sections ".codebin.#name#.init" and ".codebin.#name#" respectively
  You shouldn't assign anything else to section ".codebin.#name#.init"
  as it is used to write initialization code at the start of effect
*/
/*
#include <pmdsky.h>
#include <cot.h>

CODEBIN_MOVE(mymove) (move_effect_input* data, struct entity* user, struct entity* target, struct move* move) {
    // Code your Move here...
}

CODEBIN_ITEM(myitem) (struct entity* user, struct entity* target, struct item* item, bool is_thrown) {
    // Code your Item here...
}

CODEBIN_SP(mysp) (struct script_routine* routine, uint32_t special_process_id, short arg1, short arg2, int* return_val) {
    // Code your SP here...
}
*/