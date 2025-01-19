#include <pmdsky.h>
#include <cot/basedefs.h>
#include <cot/logging.h>
#include <cot/custom_instructions.h>

// Loosely based on https://github.com/Adex-8x/jam-patches/blob/master/strung-up-by-sketches/CustomOpcodes/asm_patches/patch_ov36.asm#L137

#if CUSTOM_GROUND_INSTRUCTIONS

// const instead of #define so the constant can be referenced in Assembly
__attribute((used)) const int FIRST_CUSTOM_OPCODE = 0x1000;

__attribute((naked)) void HookOpcodeCheck(void) {
    asm volatile("cmp r5,r7");
    asm volatile("bge NewInstructions");
    asm volatile("cmp r5,r0");
    asm volatile("b OpcodeCheck+4");
}

__attribute((naked)) void NewInstructions(void) {
    asm volatile("sub r5,r5,r7");

    asm volatile("mov r0,r5"); // Opcode (offset from FIRST_CUSTOM_OPCODE)
    asm volatile("mov r1,r4"); // Current script routine pointer
    asm volatile("mov r2,r6"); // Argument list
    asm volatile("bl DispatchCustomInstruction");

    asm volatile("b ScriptEngineReturnTwo");
}

__attribute((naked)) void HookGetParameterCount(void) {
    asm volatile("ldr r7,=FIRST_CUSTOM_OPCODE");
    asm volatile("ldr r7,[r7]");
    asm volatile("cmp r5,r7");
    asm volatile("ldrge r0,=CUSTOM_INSTRUCTIONS");
    asm volatile("subge r1,r5,r7");
    asm volatile("ldrge r8,=12");   // Struct size
    asm volatile("mulge r8,r1,r8"); // Scale index by struct size
    asm volatile("movge r1,r8");
    asm volatile("ldrsb r0,[r0,r1]");
    asm volatile("bx r14");

    // workaround for "invalid literal constant: pool needs to be closer"
    asm volatile(".ltorg");
}

__attribute((used)) void DispatchCustomInstruction(int index, struct script_routine* routine, uint16_t* args) {
    if (index < 0 || index >= CUSTOM_INSTRUCTION_AMOUNT) {
        COT_ERRORFMT(COT_LOG_CAT_INSTRUCTIONS, "Custom opcode %d out of bounds", index);
        return;
    }

    struct custom_instruction* instruction = &CUSTOM_INSTRUCTIONS[index];
    COT_LOGFMT(COT_LOG_CAT_INSTRUCTIONS, "Running custom instruction '%s' with %d arguments (opcode %d, index %d)", instruction->name, instruction->n_params, FIRST_CUSTOM_OPCODE + index, index);
    instruction->handler(routine, args);
}

#endif