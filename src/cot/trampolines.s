.align 4
cotInternalTrampolineScriptSpecialProcessCall:
  // If the special process ID is >= 100, handle it as a custom special process
  cmp r1, #100
  bge cotInternalDispatchScriptSpecialProcessCall

  // Otherwise, restore the instruction we've replaced in the patch
  // and run the original function
  push	{r3, r4, r5, r6, r7, r8, r9, sl, fp, lr}
  b ScriptSpecialProcessCall+4

.align 4
cotInternalTrampolineApplyItemEffect:
  push    {lr}
  mov     r0,r8 // user
  mov     r1,r7 // target
  mov     r2,r6 // item
  mov     r3,r9 // is_thrown
  bl      cotInternalDispatchApplyItemEffect
  pop     {lr}
  cmp     r0,#0 // Was the effect handled?
  ldreqsh r0,[r6,#0x4] // original instruction
  bxeq    lr                      // If not, return to vanilla flow; let vanilla code / ExtractItemCode handle the Item.
  b       ApplyItemEffectJumpAddr // Otherwise, we're done! Jump towards the end of ApplyItemEffect after the item handling code for finalization

.align 4
cotInternalTrampolineApplyMoveEffect:
  mov   r10,#0                          // Redundancy; only really matters if the user forgets to assign out_dealt_damage in CustomApplyMoveEffect.
  push  {r6,r7,r10,lr}                  // Create move_effect_input struct from move_id, item_id, and out_dealt_damage. Save lr for later so we can return from where we hooked.
  bleq  UpdateShopkeeperModeAfterAttack // Original instruction from vanilla code; condition flags were set before our hook
  mov   r0,sp                           // data (put in the stack by the previous push)
  mov   r1,r9                           // user
  mov   r2,r4                           // target
  mov   r3,r8                           // move
  bl    cotInternalDispatchApplyMoveEffect
  pop   {r6,r7,r10,lr}                  // Put move_effect_input back into r6, r7, and r10. Restore lr to what it was coming in so we can return from where we hooked.
  cmp   r0,#0                           // Was the move handled?
  bxeq  lr                              // If not, return to vanilla flow; let vanilla code / ExtractMoveCode handle the move.
  cmp   r10,#0                          // Instruction right before return point (We don't jump straight to this instruction due for ExtractMoveCode compatibility)
  b     ApplyMoveEffectJumpAddr         // Otherwise, we're done! Jump towards the end of ExecuteMoveEffect after the move handling code for finalization