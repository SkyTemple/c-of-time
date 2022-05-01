.align 4
cotInternalTrampolineScriptSpecialProcessCall:
  // If the special process ID is >= 100, handle it as a custom special process
  cmp r1, #100
  bge CustomScriptSpecialProcessCall

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
  bl CustomApplyItemEffect
  // Check if true was returned
  cmp r0, #1

  // Load saved registers
  popeq {r0-r9, r11, r12}

  // If yes, exit the original function by jumping to the link register
  beq ApplyItemEffectJumpAddr

  pop {r0-r9, r11, r12}

  // Restore the instruction that was replaced with the patch and call the original function
  cmp r0, #0
  b ApplyItemEffectHookAddr+4

.align 4
cotInternalTrampolineApplyMoveEffect:
  // Backup registers
  push {r0-r9, r11, r12}

  // Setup move_effect_input struct
  ldr r10, =move_effect_input
  str r6, [r10] // move_id
  str r7, [r10, #0x4] // item_id
  mov r0, #0
  str r0, [r10, #0x8] // out_dealt_damage

  // Call the hook function
  mov r0, r10
  mov r1, r9
  mov r2, r4
  mov r3, r8
  bl CustomApplyMoveEffect

  // Check if true was returned
  cmp r0, #1

  // Load saved registers
  popeq {r0-r9, r11, r12}
  ldreq r10, =move_effect_input_out_dealt_damage

  // If yes, exit the original function
  beq ApplyMoveEffectJumpAddr

  pop {r0-r9, r11, r12}

  // Restore the instruction that was replaced with the patch and call the original function
  mov r1, #0x1
  b ApplyMoveEffectHookAddr+4

.align 4
move_effect_input:
  .word 0
  .word 0
move_effect_input_out_dealt_damage:
  .word 0
