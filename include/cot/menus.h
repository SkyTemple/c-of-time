#pragma once

#include "basedefs.h"
#include <pmdsky.h>

// Set this value to 1 to enable support for custom script engine menus
#define CUSTOM_SCRIPT_MENUS 0

struct custom_menu {
  uint16_t keyboard_prompt_string_id; // The string used for the first keyboard menu prompt, e.g., "What is your partner's nickname?"
  uint16_t keyboard_confirm_string_id; // The string used for the final Yes/No confirmation prompt upon inputting a string, e.g., "Is the name [string0] OK?"
  void (*create)(); // Called only once; initializes the script menu.
  void (*close)(); // Called only once, when the update function returns true.
  bool (*update)(); // Called every frame while the script menu is active. Returns true if the script menu should close.
};

struct global_menu_info {
  int id; // ID of the current custom script menu active!
  int state; // To track script menu progress!
  int return_val; // Value ultimately returned by message_Menu in a script!
  int previous_option; // Indicates the last option that was hovered over in a menu. A prime use case is changing another window based on if the player changes the option they're hovering the cursor on.
  struct portrait_params portrait_params; // Global portrait params to easily reference for portrait functions!
  int menu_results[20]; // To store previous results of menus across update calls!
  int window_ids[20]; // Maximum number of windows that can be active at a time.
  // Can add more fields here as necessary to use for custom script menus!
};

void InitializeCustomScriptMenu(int index);
bool DispatchCustomScriptMenu(int index, int* return_val);
extern struct custom_menu CUSTOM_MENUS[];
extern struct global_menu_info GLOBAL_MENU_INFO;
extern const int CUSTOM_MENU_AMOUNT;
extern bool IS_BASE_GAME_MENU_FINISHED;
