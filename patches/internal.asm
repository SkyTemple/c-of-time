.nds
.include "symbols.asm"

.open "overlay11.bin", overlay11_start
  .org ScriptSpecialProcessCall
    b cotInternalTrampolineScriptSpecialProcessCall
.close 

.open "overlay29.bin", overlay29_start
  .org ApplyItemEffectHookAddr
    b cotInternalTrampolineApplyItemEffect
  .org ApplyMoveEffectHookAddr
    b cotInternalTrampolineApplyMoveEffect
.close
