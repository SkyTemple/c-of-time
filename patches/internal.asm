.nds
.include "symbols.asm"

.open "arm9.bin", arm9_start
  // Remove the comments below to enable custom script menus (see `menus.c`).
  //.org ShowKeyboard
  //  push {r3-r8,lr}
  //  bl HookKeyboardCheck

  //.org ShowKeyboardTypeCase3
  //  b HookKeyboardCustomPrompt

  //.org ShowKeyboardReturn
  //  pop {r3-r8,pc}

  //.org PreprocessStringFromIdCallsite
  //  bl CustomPreprocessStringFromId
.close

.open "overlay11.bin", overlay11_start
  .org ScriptSpecialProcessCall
    b cotInternalTrampolineScriptSpecialProcessCall

  // Remove the comments below to enable custom script menus (see `menus.c`).
  //.org ScriptMenuRequestDefaultCase
  //  b HookScriptMenuRequestCheck
  
  //.org ScriptMenuUpdateDefaultCase
  //  b HookScriptMenuUpdateCheck

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
