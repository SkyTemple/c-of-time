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
  // Backup registers
  push {r0-r9, r11, r12}

  // Call the hook function
  mov r0, r8
  mov r1, r7
  mov r2, r6
  mov r3, r9
  bl cotInternalDispatchApplyItemEffect
  // Check if true was returned
  cmp r0, #1

  // Load saved registers
  popeq {r0-r9, r11, r12}

  // If yes, exit the original function
  beq ApplyItemEffectJumpAddr

  pop {r0-r9, r11, r12}

  // Restore the instruction that was replaced with the patch and call the original function
  cmp r0, #0
  b ApplyItemEffectHookAddr+4

.align 4
cotInternalTrampolineApplyMoveEffect:
  push  {lr}
  bne   TryHandleMove
  mov   r0,r9
  mov   r1,r4
  bl    UpdateShopkeeperModeAfterAttack
  TryHandleMove:
  // Stack was already configured to set up data before, EXCEPT out_dealt_damage, which we init to 0 now.
  // Initializing this may be redundant; it should only really matter if you forget to assign out_dealt_damage for a handled move in CustomApplyMoveEffect. If you're an optimization freak like I am, you can comment out the next 2 lines.
  mov   r0,#0
  str   r0,[sp,#0xC]            // Would be #0x8, but SP decreased by 4 when we pushed lr, so we need to counteract that by adding 4 here
  add   r0,sp,#4                // data (would be mov r0,sp if not for the same point mentioned above)
  mov   r1,r9                   // user
  mov   r2,r4                   // target
  mov   r3,r8                   // move
  bl    cotInternalDispatchApplyMoveEffect
  pop   {r1,r2,r3,r10}          // Get original LR (into r1) so we can return to vanilla flow if needed, put data->out_dealt_damage in r10, and reset SP to where it was before we hooked. (We don't do anything with r1/r2, we just need to pop 4 registers so r10 comes from the correct place and the SP is increased by 0x10.) We use r1 instead of lr, because using actual lr will try to pop that LAST.
  cmp   r0,#0                   // Was the move handled?
  bxeq  r1                      // If not, return to vanilla flow
  cmp   r10,#0                  // Instruction right before return point (We don't jump straight to this instruction due for ExtractMoveCode compatibility)
  b     ApplyMoveEffectJumpAddr // Otherwise, we're done!