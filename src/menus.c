#include <pmdsky.h>
#include <cot.h>

// Custom script menus are disabled by default.
// Refer to README.md for more information.

#if CUSTOM_SCRIPT_MENUS

// The following functions aren't in pmdsky-debug yet, so we have to declare them
// here and add their offsets in "symbols/custom_[region].ld".
extern void InitGroundMonsterBaseStats(struct ground_monster* ground_monster);
extern void InitGroundMonsterStatsAndMoveset(struct ground_monster* ground_monster, int level, bool flag);
extern void SetupKeyboard(int index, char* buffer1, char* buffer2);

// The "entry" function called for every single option of the Advanced Menu created by CreateRecruitAnyMonsterMenu. The resulting buffer will be used as the option string for the given `option_id`.
// In this instance, the goal is to make a menu that consists of every Pokémon, so every option will need to show each Pokémon's name!
// `option_id` starts at 0, but the first Pokémon (Bulbasaur) starts at 1, hence the +1.
char* RecruitAnyMonsterOptionEntryFn(char* buffer, int option_id) {
    sprintf(buffer, "[CS:K]%s[CR]", GetNameString(option_id+1));
    return buffer;
}

// The initial menu function called when `message_Menu(80);` is executed in a script, responsible for the creation of the main Advanced Menu and a portrait.
// Like any `create` function, this is only called once.
void CreateRecruitAnyMonsterMenu() {
    struct window_params menu_params = { .x_offset = 2, .y_offset = 2, .box_type = {0xFF} };
    struct window_flags menu_flags = { .a_accept = true, .b_cancel = true, .se_on = true, .partial_menu = true, .menu_lower_bar = true, .no_accept_button = true };
    struct portrait_params* portrait_params = &(GLOBAL_MENU_INFO.portrait_params);
    struct vec2 vec = { .x = 2, .y = -3 };
    InitPortraitParamsWithMonsterId(portrait_params, 1);
    SetPortraitLayout(portrait_params, 4);
    SetPortraitOffset(portrait_params, &vec);
    GLOBAL_MENU_INFO.window_ids[0] = CreateAdvancedMenu(&menu_params, menu_flags, NULL, RecruitAnyMonsterOptionEntryFn, 534, 8);
    GLOBAL_MENU_INFO.window_ids[1] = CreatePortraitBox(0, 3, true);
    ShowPortraitInPortraitBox(GLOBAL_MENU_INFO.window_ids[1], portrait_params);
}

// The final menu function called when `message_Menu(80);` is executed in a script, responsible for the closing of any and all active windows.
// Like any `close` function, this is only called once.
void CloseRecruitAnyMonsterMenu() {
    if(GLOBAL_MENU_INFO.window_ids[0] >= 0)
        CloseAdvancedMenu(GLOBAL_MENU_INFO.window_ids[0]);
    if(GLOBAL_MENU_INFO.window_ids[1] >= 0)
        ClosePortraitBox(GLOBAL_MENU_INFO.window_ids[1]);
    if(GLOBAL_MENU_INFO.window_ids[2] >= 0)
        CloseSimpleMenu(GLOBAL_MENU_INFO.window_ids[2]);
}

// The menu function called repeatedly while `message_Menu(80);` is active in a script, responsible for continuously checking the status of any menus and handling the logic of what should occur upon player input.
// Like any `update` function, this is called every frame until it returns true.
bool UpdateRecruitAnyMonsterMenu() {
    bool is_menu_finished = false;
    int adv_menu_id = GLOBAL_MENU_INFO.window_ids[0];
    int portrait_id = GLOBAL_MENU_INFO.window_ids[1];
    int simple_menu_id = GLOBAL_MENU_INFO.window_ids[2];
    int current_menu_option;
    int monster_id;
    // Menus in the base game follow a pattern of keeping a "state" of any overarching menu system, then having a switch-statement to track menu progress.
    // A similar convention is followed here. Menu states will start at 0.
    switch(GLOBAL_MENU_INFO.state) {
        case 0:
            // Beginning state; check if the Advanced Menu is still active. If not, save the result and proceed to the next state.
            if(!IsAdvancedMenuActive2(adv_menu_id)) {
                GLOBAL_MENU_INFO.menu_results[0] = GetAdvancedMenuResult(adv_menu_id);
                if(GLOBAL_MENU_INFO.menu_results[0] >= 0)
                    GLOBAL_MENU_INFO.state = 1;
                else {
                    // A value of -1 for `GetAdvancedMenuResult` indicates that the menu was exited without an option being selected (i.e., the B button was pressed).
                    GLOBAL_MENU_INFO.state = -1;
                    GLOBAL_MENU_INFO.return_val = -1;
                }
            }
            // If the menu is active, be sure to continuously update the portrait to show the correct monster!
            else {
                current_menu_option = GetAdvancedMenuCurrentOption(adv_menu_id);
                if(current_menu_option != GLOBAL_MENU_INFO.previous_option) {
                    GLOBAL_MENU_INFO.portrait_params.monster_id.val = current_menu_option + 1;
                    GLOBAL_MENU_INFO.previous_option = current_menu_option;
                    ShowPortraitInPortraitBox(portrait_id, &(GLOBAL_MENU_INFO.portrait_params));
                }
            }
            break;
        case 1:
            // Check if the selected monster has a valid secondary gender. If so, create another menu. If not, attempt to create a new `ground_monster`!
            monster_id = GLOBAL_MENU_INFO.menu_results[0]+1;
            int secondary_gender = GetMonsterGender(monster_id+600);
            if(secondary_gender == GENDER_INVALID)
                GLOBAL_MENU_INFO.state = 3;
            else {
                struct window_params simple_menu_params = { .x_offset = 16, .y_offset = 10, .width = 10, .box_type = {0xFF} };
                struct window_flags simple_menu_flags = { .a_accept = true, .b_cancel = true, .se_on = true };
                struct simple_menu_id_item simple_options[3];
                int starting_text_string_id;
                // Male/Female Text Strings
                #ifdef REGION_NA
                starting_text_string_id = 15531;
                #elif REGION_EU
                starting_text_string_id = 15533;
                #else
                starting_text_string_id = 1106;
                #endif
                for(int i = 0; i < 2; i++) {
                    simple_options[i].string_id = i+starting_text_string_id;
                    simple_options[i]._padding = 0;
                    simple_options[i].result_value = i+1;
                }
                simple_options[2].string_id = NULL;
                simple_options[2]._padding = NULL;
                simple_options[2].result_value = NULL;
                GLOBAL_MENU_INFO.window_ids[2] = CreateSimpleMenuFromStringIds(&simple_menu_params, simple_menu_flags, NULL, simple_options, 3);
                GLOBAL_MENU_INFO.state = 2;
            }
            break;
        case 2:
            // Repeatedly check the Simple Menu, which is made to decide the monster's gender. This state will only run if the selected Pokémon does not have the value GENDER_INVALID for its secondary form's gender.
            if(!IsSimpleMenuActive(simple_menu_id)) {
                GLOBAL_MENU_INFO.menu_results[2] = GetSimpleMenuResult(simple_menu_id);
                if(GLOBAL_MENU_INFO.menu_results[2] > 0) {
                    // Simple Menus allow for setting custom result values per option, unlike Advanced Menus. Still, in this instance, we simply have 1 for Male and 2 for Female.
                    if(GLOBAL_MENU_INFO.menu_results[2] == 2)
                        GLOBAL_MENU_INFO.menu_results[0] += 600;
                    GLOBAL_MENU_INFO.state = 3;
                }
                else {
                    // If the Simple Menu is exited without selecting an option, close the Simple Menu and resume the Advanced Menu.
                    // The Advanced Menu can only be resumed in conjunction with `ResumeAdvancedMenu` and the `menu_flags.partial_menu` flag set upon the Advanced Menu's creation.
                    CloseSimpleMenu(simple_menu_id);
                    ResumeAdvancedMenu(adv_menu_id);
                    // Set the Window ID of our previously-active Simple Menu to -1. This is also another base game convention for windows that have been closed in a menu.
                    // The reason this is necessary is that this Simple Menu isn't guaranteed to be active when this function returns "true."
                    // For example, imagine if the player selects Smeargle first, sees the Male/Female menu, and then changes their mind to then select Mewtwo.
                    // Mewtwo's secondary form has a gender of GENDER_INVALID, so the Simple Menu would never get created a second time.
                    // Were it not for setting the Window ID to -1, we would maintain a stale ID, which may interact strangely if we kept using menu functions with this ID.
                    GLOBAL_MENU_INFO.window_ids[2] = -1;
                    GLOBAL_MENU_INFO.state = 0;
                }
            }
            break;
        case 3:
            // Try to add the selected monster to Chimecho Assembly as a new recruit!
            // Based off of https://github.com/marius851000/eos-marius-patch/blob/master/process/eu_fixed/new_add_recruitable.asm
            int index = GetFirstEmptyMemberIdx(0x214);
            monster_id = GLOBAL_MENU_INFO.menu_results[0]+1;
            // A negative index indicates that there is no more space available for this new recruit.
            if(index > -1) {
                struct ground_monster* new_recruit = GetTeamMember(index);
                new_recruit->is_valid = true;
                new_recruit->id.val = monster_id;
                new_recruit->level_at_first_evo = 0;
                new_recruit->level_at_second_evo = 0;
                new_recruit->joined_at.val = DUNGEON_TEST_DUNGEON;
                new_recruit->joined_at_floor = 1;
                StrncpyName(new_recruit->name, GetNameString(monster_id), 10);
                InitGroundMonsterBaseStats(new_recruit);
                InitGroundMonsterStatsAndMoveset(new_recruit, 1, false);
                SetPokemonJoined(monster_id);
                GLOBAL_MENU_INFO.return_val = index;
            }
            else
                GLOBAL_MENU_INFO.return_val = -2;
            // Regardless of whether the new recruit could be added, finish the menu.
            GLOBAL_MENU_INFO.state = -1;
            break;
        default:
            // If we reach an unexpected state, just end the menu.
            is_menu_finished = true;
    }
    return is_menu_finished;
}

// The initial menu function called to show a keyboard prompt for the player to type in a string.
// This is intended to be used by a variety of menus.
void CreateSimpleKeyboardMenu() {
    SetupKeyboard(GLOBAL_MENU_INFO.id, NULL, NULL);
}

// The menu function called repeatedly to check if the player has finished entering a string.
// This is intended to be used by a variety of menus.
bool UpdateSimpleKeyboardMenu() {
    return IS_BASE_GAME_MENU_FINISHED;
}

// The final menu function called when `message_Menu(81);` is executed in a script, responsible for checking the result of the player-inputted string.
// Simply does a `strncmp` with "shard" and the keyboard string, i.e., returns 0 if the player has inputted "shard" in the keyboard prompt.
void ClosePasswordMenu() {
    #ifdef REGION_JP
    GLOBAL_MENU_INFO.return_val = strncmp((char*)GetKeyboardStringResult(), "L6(J.", 10); // This is still actually "shard"
    #else
    GLOBAL_MENU_INFO.return_val = strncmp((char*)GetKeyboardStringResult(), "shard", 10);
    #endif
}

// The final menu function called when `message_Menu(82);` is executed in a script, responsible for checking the result of the player-inputted string.
// Renames the partner across both its `ground_monster` and `team_member` structs using the string the player inputted in the keyboard prompt.
// Based off of https://github.com/Chesyon/StarterMenuTool/blob/main/skypatches/FixPartnerNameMenu.skypatch
void ClosePartnerNameMenu() {
    char* result = (char*)GetKeyboardStringResult();
    int index = GetMainCharacter2MemberIdx();
    int roster_index = GetActiveRosterIndex(index);
    struct ground_monster* ground_monster = GetTeamMember(index);
    struct team_member* team_member = GetActiveTeamMember(roster_index);
    if(ground_monster != NULL)
        StrncpySimple(ground_monster->name, result, 10);
    if(team_member != NULL)
        StrncpySimple(team_member->name, result, 10);
    SaveScriptVariableValueBytes(VAR_PARTNER_FIRST_NAME, result, 10);
    GLOBAL_MENU_INFO.return_val = 0;
}

// Add your custom script menus to the list below.
// `create` is a pointer to the initial function that will run only once when a custom `message_Menu` runs. This is typically responsible for the initial creation of any windows.
// `close` is a pointer to the final function that will run only once when a custom `message_Menu` runs. This is typically responsible for the final closing of any windows, as well as setting a return value if not yet set.
// `update` is pointer to the function that will continously get called every frame when a custom `message_Menu` runs. This is typically responsible for checking the status of any menus and implementing control flow, i.e., "what happens if the player selects an option?"
// `keyboard_prompt_string_id` is the Text String ID shown when a keyboard prompt is displayed. This may not be necessary for all menus.
// `keyboard_confirm_string_id` is the Text String ID shown when confirming the player's keyboard input. This may not be necessary for all menus.
// Custom script menus use ID 80 + <array index>.
//
// Refer to menus.h for more information on the fields of `custom_menu` and `global_menu_info`!
struct custom_menu CUSTOM_MENUS[] = {
    // ID 80
    // Attempts to add a chosen Pokémon as a new member of Chimecho Assembly!
    // Returns: Chimecho Assembly index of the new recruit if successful. -1 if the player exits the menu, -2 if a new recruit could not be added.
    {
        .create = CreateRecruitAnyMonsterMenu,
        .close = CloseRecruitAnyMonsterMenu,
        .update = UpdateRecruitAnyMonsterMenu
    },
    // ID 81
    // Prompts the player to input a password.
    // Returns: 0 if the player enters "shard" in the keyboard prompt, else otherwise.
    {
        #ifdef REGION_JP
        .keyboard_prompt_string_id = 15586,
        .keyboard_confirm_string_id = 951,
        #else
        .keyboard_prompt_string_id = 263,
        .keyboard_confirm_string_id = 431,
        #endif
        .create = CreateSimpleKeyboardMenu,
        .close = ClosePasswordMenu,
        .update = UpdateSimpleKeyboardMenu
    },
    // ID 82
    // Prompts the player to rename the partner.
    // Returns: Nothing.
    {
        #ifdef REGION_JP
        .keyboard_prompt_string_id = 12749,
        .keyboard_confirm_string_id = 12758,
        #else
        .keyboard_prompt_string_id = 283,
        .keyboard_confirm_string_id = 292,
        #endif
        .create = CreateSimpleKeyboardMenu,
        .close = ClosePartnerNameMenu,
        .update = UpdateSimpleKeyboardMenu
    }
};

struct global_menu_info GLOBAL_MENU_INFO;
const int CUSTOM_MENU_AMOUNT = ARRAY_LENGTH(CUSTOM_MENUS);

#endif
