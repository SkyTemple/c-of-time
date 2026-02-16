.nds
.include "symbols.asm"

.open "arm9.bin", arm9_start
  .ifdef HookScriptMenuRequestCheck
  .org ShowKeyboard
    push {r3-r8,lr}
    bl HookKeyboardCheck

  .org ShowKeyboardTypeCase3
    b HookKeyboardCustomPrompt

  .org ShowKeyboardReturn
    pop {r3-r8,pc}

  .org PreprocessStringFromIdCallsite
    bl CustomPreprocessStringFromId
  .endif
.close

.open "overlay11.bin", overlay11_start
  .org ScriptSpecialProcessCall
    b cotInternalTrampolineScriptSpecialProcessCall

  .ifdef HookScriptMenuRequestCheck
  .org ScriptMenuRequestDefaultCase
    b HookScriptMenuRequestCheck
  
  .org ScriptMenuUpdateDefaultCase
    b HookScriptMenuUpdateCheck
  .endif

  .ifdef HookOpcodeCheck
  .org OpcodeCheck
    b HookOpcodeCheck
  
  .org GetParameterCount
    bl HookGetParameterCount
  .endif
.close 

.open "overlay29.bin", overlay29_start
  .org ApplyItemEffectHookAddr
    bl cotInternalTrampolineApplyItemEffect
  .org ApplyMoveEffectHookAddr
    nop       // Normally there would be a bne here that skips the next three lines; we don't want to do that, because we want to always run the hook. We still use the condition flags to skip the function that needs to be skipped in trampolines.s.
    mov r0,r9 // UNCHANGED
    mov r1,r4 // UNCHANGED
    bl  cotInternalTrampolineApplyMoveEffect
.close
