#pragma once

#define CODEBIN_MOVE(move_name) \
    void move_name(move_effect_input*, struct entity*, struct entity*, struct move*);\
    __attribute__ ((used, naked, section(".codebin." #move_name ".init"))) void move_name##_init() {\
        asm volatile("push  {r6,r7,r10}");\
        asm volatile("mov r0,sp");\
        asm volatile("mov r1,r9");\
        asm volatile("mov r2,r4");\
        asm volatile("mov r3,r8");\
        asm volatile("bl " #move_name);\
        asm volatile("pop  {r6,r7,r10}");\
        asm volatile("b MoveJumpAddress");\
    }\
    __attribute__ ((used, section(".codebin." #move_name))) void move_name

#define CODEBIN_ITEM(item_name) \
    void item_name(struct entity*, struct entity*, struct item*, bool);\
    __attribute__ ((used, naked, section(".codebin." #item_name ".init"))) void item_name##_init() {\
        asm volatile("mov r0,r8");\
        asm volatile("mov r1,r7");\
        asm volatile("mov r2,r6");\
        asm volatile("mov r3,r9");\
        asm volatile("bl " #item_name);\
        asm volatile("b ItemJumpAddress");\
    }\
    __attribute__ ((used, section(".codebin." #item_name))) void item_name

#define CODEBIN_SP(sp_name) \
    void sp_name(struct script_routine*, uint32_t, short, short, int*);\
    __attribute__ ((used, naked, section(".codebin." #sp_name ".init"))) void sp_name##_init() {\
        asm volatile("sub r0,r13,#4");\
        asm volatile("mov r1,#0");\
        asm volatile("push  {r0,r1}");\
        asm volatile("mov r0,r4");\
        asm volatile("mov r1,r5");\
        asm volatile("mov r2,r7");\
        asm volatile("mov r3,r6");\
        asm volatile("bl " #sp_name);\
        asm volatile("pop  {r0,r1}");\
        asm volatile("mov r0,r1");\
        asm volatile("b ProcJumpAddress");\
    }\
    __attribute__ ((used, section(".codebin." #sp_name))) void sp_name
