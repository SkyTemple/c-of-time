.nds
.include "symbols.asm"

.open "overlay11.bin", overlay11_start
  .org ScriptSpecialProcessCall
    b cotInternalTrampolineScriptSpecialProcessCall

  // Remove the comments below to enable custom instructions (see `ground_instructions.c`).

  // .org OpcodeCheck
  //   b HookOpcodeCheck
  //
  // .org GetParameterCount
  //   bl HookGetParameterCount
.close 

.open "overlay29.bin", overlay29_start
  .org ApplyItemEffectHookAddr
    b cotInternalTrampolineApplyItemEffect
  .org ApplyMoveEffectHookAddr
    b cotInternalTrampolineApplyMoveEffect
.close
