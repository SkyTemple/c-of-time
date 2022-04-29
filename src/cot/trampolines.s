.align 4
cotInternalTrampolineScriptSpecialProcessCall:
  // If the special process ID is >= 1000, handle it as a custom special process
  cmp r1, #100
  bge CustomScriptSpecialProcessCall

  // Otherwise, restore the instruction we've replaced in the patch
  // and run the original function
  push	{r3, r4, r5, r6, r7, r8, r9, sl, fp, lr}
  b ScriptSpecialProcessCall+4

.align 4
cotInternalTrampolineApplyItemEffect:
  // Backup registers
  ldr r6, =reg_backup
  str r0, [r6]
  str r1, [r6, #0x4]
  str r2, [r6, #0x8]
  str r3, [r6, #0xC]
  str r4, [r6, #0x10]
  str r5, [r6, #0x14]
  str lr, [r6, #0x18]

  // Call the hook function
  bl CustomApplyItemEffect
  // Check if true was returned
  cmp r0, #1

  // If yes, exit the original function by jumping to the link register
  ldr r6, =reg_backup
  ldreq lr, [r6, #0x18]
  bxeq lr

  // Load saved registers
  ldr r0, [r6]
  ldr r1, [r6, #0x4]
  ldr r2, [r6, #0x8]
  ldr r3, [r6, #0xC]
  ldr r4, [r6, #0x10]
  ldr r5, [r6, #0x14]
  ldr lr, [r6, #0x18]

  // Restore the instruction that was replaced with the patch and call the original function
  push {r3, r4, r5, r6, r7, r8, r9, r10, lr}
  b ApplyItemEffect+4

.align 4
cotInternalTrampolineApplyMoveEffect:
  // Backup registers
  ldr r10, =reg_backup
  str r0, [r10]
  str r1, [r10, #0x4]
  str r2, [r10, #0x8]
  str r3, [r10, #0xC]
  str r4, [r10, #0x10]
  str r5, [r10, #0x14]
  str r6, [r10, #0x18]
  str r7, [r10, #0x1C]
  str r8, [r10, #0x20]
  str r9, [r10, #0x24]
  str r11, [r10, #0x28]
  str r12, [r10, #0x2C]
  str r13, [r10, #0x30]
  str lr, [r10, #0x34]

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

  // If yes, exit the original function by jumping to the link register
  ldr r10, =reg_backup
  ldreq lr, [r10, #0x34]
  beq ApplyMoveEffectJumpAddr

  // Load saved registers
  ldr r0, [r10]
  ldr r1, [r10, #0x4]
  ldr r2, [r10, #0x8]
  ldr r3, [r10, #0xC]
  ldr r4, [r10, #0x10]
  ldr r5, [r10, #0x14]
  ldr r6, [r10, #0x18]
  ldr r7, [r10, #0x1C]
  ldr r8, [r10, #0x20]
  ldr r9, [r10, #0x24]
  ldr r11, [r10, #0x28]
  ldr r12, [r10, #0x2C]
  ldr r13, [r10, #0x30]
  ldr lr, [r10, #0x34]

  ldr r10, =move_effect_input_out_dealt_damage

  // Restore the instruction that was replaced with the patch and call the original function
  mov r1, #0x1
  b ApplyMoveEffectHookAddr+4

.align 4
reg_backup:
  .space 0x40
move_effect_input:
  .word 0
  .word 0
move_effect_input_out_dealt_damage:
  .word 0
