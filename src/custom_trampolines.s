.align 4
TrampolineAfterApplyMoveEffect:
  // Backup registers
  push {r1-r12}

  mov r0, r9
  mov r1, r4
  bl CheckBeastBoost

  // Load saved registers
  pop {r1-r12}

  // Restore the original instruction we've replaced and return
  cmp r10, #0x0
  b ApplyMoveEffectJumpAddr+4
