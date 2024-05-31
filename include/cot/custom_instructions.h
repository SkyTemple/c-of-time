#pragma once

#include "basedefs.h"
#include <pmdsky.h>

struct custom_instruction {
  uint8_t n_params;
  void (*handler)(uint16_t* args);
  char *name;
};

void DispatchCustomInstruction(int index, uint16_t* args);
extern struct custom_instruction CUSTOM_INSTRUCTIONS[];
extern const int CUSTOM_INSTRUCTION_AMOUNT;
