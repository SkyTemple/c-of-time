#pragma once

#include "basedefs.h"
#include <pmdsky.h>

// Set this value to 1 to enable support for custom script engine instructions
#define CUSTOM_GROUND_INSTRUCTIONS 0

struct custom_instruction {
  int8_t n_params;
  void (*handler)(struct script_routine* routine, uint16_t* args);
  char *name;
};

void DispatchCustomInstruction(int index, struct script_routine* routine, uint16_t* args);
extern struct custom_instruction CUSTOM_INSTRUCTIONS[];
extern const int CUSTOM_INSTRUCTION_AMOUNT;
