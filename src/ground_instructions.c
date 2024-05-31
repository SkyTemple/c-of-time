#include <pmdsky.h>
#include <cot.h>

// Custom script engine instructions are disabled by default.
// Refer to README.md for more information.

#if 0

// Overwrites the default dialogue box attributes with the given values.
//
// # Arguments
// - `x`: x position
// - `y`: y position
// - `width`: dialogue box width
// - `height`: dialogue box height
// - `screen`: 0 = bottom screen, 1 = top screen
// - `frame`: 0xFD = default, 0xFA = invisible, ...
void OpSetDialogueBoxAttributes(uint16_t* args) {
    int x = ProcessScriptParam(args[0]);
    int y = ProcessScriptParam(args[1]);
    int width = ProcessScriptParam(args[2]);
    int height = ProcessScriptParam(args[3]);
    int screen = ProcessScriptParam(args[4]);
    int frame = ProcessScriptParam(args[5]);

    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.x_offset = x;
    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.y_offset = y;
    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.width = width;
    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.height = height;
    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.screen.val = screen;
    DIALOGUE_BOX_DEFAULT_WINDOW_PARAMS.box_type.val = frame;

    COT_LOGFMT(COT_LOG_CAT_INSTRUCTIONS, "Setting dialogue box attributes: x=%d, y=%d, width=%d, height=%d, screen=%d, frame=%d", x, y, width, height, screen, frame);
}

// Saves the set of held/pressed buttons into $EVENT_LOCAL as a bitfield.
//
// # Arguments
// - `mode`: 0 = pressed buttons, 1 = held buttons
void OpCheckInputStatus(uint16_t* args) {
    int mode = ProcessScriptParam(args[0]);

    int buttons = 0;
    if (mode == 0) {
        GetPressedButtons(0, (undefined*) &buttons);
    } else {
        GetHeldButtons(0, (undefined*) &buttons);
    }
    SaveScriptVariableValue(NULL, VAR_EVENT_LOCAL, buttons);
}

// Add your custom instructions to the list below.
// `handler` is a pointer to your handler function (see the examples above).
// `n_params` must match the number of instructions used in your handler function.
// Custom instructions use ID 0x1000 + <array index>.
//
// Refer to README.md for instructions on how to access custom instructions in SkyTemple!
struct custom_instruction CUSTOM_INSTRUCTIONS[] = {
    // ID 0x1000
    {
        .name = "SetDialogueBoxAttributes",
        .handler = OpSetDialogueBoxAttributes,
        .n_params = 6
    },
    // ID 0x1001
    {
        .name = "CheckInputStatus",
        .handler = OpCheckInputStatus,
        .n_params = 1
    }
};

__attribute((used)) const int CUSTOM_INSTRUCTION_AMOUNT = ARRAY_LENGTH(CUSTOM_INSTRUCTIONS);

#endif