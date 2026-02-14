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
    sub sp,sp,#0xC   // allocate stack space for data param of cotInternalDispatchApplyMoveEffect
    str r6,[sp]      // data->move_id
    str r7,[sp,#0x4] // data->item_id
    bl  cotInternalTrampolineApplyMoveEffect
.close
