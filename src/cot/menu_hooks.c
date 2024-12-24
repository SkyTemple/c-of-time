#include <pmdsky.h>
#include <cot/basedefs.h>
#include <cot/logging.h>
#include <cot/menus.h>

// Loosely based on https://github.com/Adex-8x/mm5-patches/blob/main/src/menus.c

#if CUSTOM_SCRIPT_MENUS

// const instead of #define so the constant can be referenced in Assembly
__attribute((used)) const int FIRST_CUSTOM_SCRIPT_MENU = 80;

__attribute((naked)) void HookKeyboardCheck(void) {
    asm volatile("ldr r12,=FIRST_CUSTOM_SCRIPT_MENU");
    asm volatile("ldr r12,[r12]");
    asm volatile("mov r8,r0");
    asm volatile("cmp r0,r12");
    asm volatile("movlt r6,r0");
    asm volatile("movge r6,#3"); // The base-game partner nickname menu goes unused and doesn't actually do anything when completed, so it gets repurposed here!
    asm volatile("bx r14");

    asm volatile(".ltorg");
}

__attribute((naked)) void HookKeyboardCustomPrompt(void) {
    asm volatile("ldr r12,=FIRST_CUSTOM_SCRIPT_MENU");
    asm volatile("ldr r12,[r12]");
    asm volatile("subs r12,r8,r12");
    asm volatile("ldrpl r2,=CUSTOM_MENUS");
    asm volatile("lslpl r12,r12,#0x4"); // Struct size
    asm volatile("ldrplh r12,[r2,r12]");
    #ifdef REGION_NA
    asm volatile("ldrpl r0,[r1,#0x0]");
    #elif REGION_EU
    asm volatile("ldrpl r0,[r0,#0x0]");
    #else
    asm volatile("ldrpl r0,[r3,#0x0]");
    #endif
    asm volatile("addpl r0,r0,#0x100");
    asm volatile("strplh r12,[r0,#0xA6]"); // Text String used for the keyboard prompt
    asm volatile("b ShowKeyboardTypeDefaultCase");

    asm volatile(".ltorg");
}

__attribute((naked)) void HookScriptMenuRequestCheck(void) {
    asm volatile("mov r0,r5");
    asm volatile("bl InitializeCustomScriptMenu");
    asm volatile("b ScriptMenuRequestFinalize");
}

__attribute((naked)) void HookScriptMenuUpdateCheck(void) {
    asm volatile("bl DispatchCustomScriptMenu");
    asm volatile("b ScriptMenuUpdateFinalize");
}

bool CustomMenuIsOutOfRange(int index) {
    return index < 0 || index >= CUSTOM_MENU_AMOUNT;
}

__attribute((used)) int CustomPreprocessStringFromId(char* output, int output_size, int string_id, struct preprocessor_flags flags, struct preprocessor_args* args) {
    int index = GLOBAL_MENU_INFO.id - FIRST_CUSTOM_SCRIPT_MENU;
    if (!CustomMenuIsOutOfRange(index)) {
        struct custom_menu* script_menu = &CUSTOM_MENUS[index];
        string_id = script_menu->keyboard_confirm_string_id;
    }
    return PreprocessStringFromId(output, output_size, string_id, flags, args);
}

__attribute((used)) void InitializeCustomScriptMenu(int menu_id) {
    int index = menu_id - FIRST_CUSTOM_SCRIPT_MENU;
    if (CustomMenuIsOutOfRange(index)) {
        COT_ERRORFMT(COT_LOG_CAT_MENUS, "Custom request for script menu %d out of bounds", menu_id);
        return;
    }

    MemZero(&GLOBAL_MENU_INFO, sizeof(struct global_menu_info));
    GLOBAL_MENU_INFO.id = menu_id;
    ArrayFill32(-1, GLOBAL_MENU_INFO.window_ids, sizeof(GLOBAL_MENU_INFO.window_ids));
    struct custom_menu* script_menu = &CUSTOM_MENUS[index];
    COT_LOGFMT(COT_LOG_CAT_MENUS, "Running custom script menu %d", menu_id);
    script_menu->create();
}

__attribute((used)) bool DispatchCustomScriptMenu(int menu_id, int* return_val) {
    int index = menu_id - FIRST_CUSTOM_SCRIPT_MENU;
    if (CustomMenuIsOutOfRange(index)) {
        *return_val = -1;
        return true;
    }

    struct custom_menu* script_menu = &CUSTOM_MENUS[index];
    bool is_menu_finished = script_menu->update();
    if(is_menu_finished) {
        script_menu->close();
        GLOBAL_MENU_INFO.id = 0;
        *return_val = GLOBAL_MENU_INFO.return_val;
    }
    return is_menu_finished;
}

#endif
