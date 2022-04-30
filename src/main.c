#include <pmdsky.h>
#include <cot.h>

#define STAT_NONE -1
#define STAT_ATK 0
#define STAT_DEF 1
#define STAT_SP_ATK 2
#define STAT_SP_DEF 3

static void AddToTiedStatList(int stat, int highest_stat_list[4]) {
  // If it's greater or equal, add it to the list of highest stats
  for (int i = 1; i < 4; i++) {
    if (highest_stat_list[i] == STAT_NONE) {
      highest_stat_list[i] = stat;
      return;
    }
  }
}

void CheckBeastBoost(struct entity* user, struct entity* target) {
  if (user != NULL && user->type == ENTITY_MONSTER) {
    struct monster* user_monster = (struct monster*) user->info;

    // Check if the user has Beast Boost (replaces the unused "$$$" ability)
    if (user_monster->ability1.val != ABILITY_UNNAMED_0x74 && user_monster->ability2.val != ABILITY_UNNAMED_0x74) {
      return;
    }

    // This is only called if a valid Pokémon was targeted, so the target was probably defeated if it's invalid
    if (!EntityIsValid(target)) {
      char message_buffer[64];
      Snprintf(message_buffer, 64, "[string:0]'s [CS:G]Beast Boost[CR] activated!");
      LogMessage(user, message_buffer, true);

      // Boost the highest stat. If several stats have the same value, pick one at random
      int tied_stat_count = 1;
      int highest_stats[4] = {STAT_ATK, STAT_NONE, STAT_NONE, STAT_NONE};
      int highest_stat_val = user_monster->atk;
      
      if (user_monster->def == highest_stat_val) {
        AddToTiedStatList(STAT_DEF, highest_stats);
        tied_stat_count++;
      } else if (user_monster->def > highest_stat_val) {
        highest_stats[0] = STAT_DEF;
        highest_stats[1] = STAT_NONE;
        highest_stats[2] = STAT_NONE;
        highest_stats[3] = STAT_NONE;
        highest_stat_val = user_monster->def;
        tied_stat_count = 1;
      }

      if (user_monster->sp_atk == highest_stat_val) {
        AddToTiedStatList(STAT_SP_ATK, highest_stats);
        tied_stat_count++;
      } else if (user_monster->sp_atk > highest_stat_val) {
        highest_stats[0] = STAT_SP_ATK;
        highest_stats[1] = STAT_NONE;
        highest_stats[2] = STAT_NONE;
        highest_stats[3] = STAT_NONE;
        highest_stat_val = user_monster->sp_atk;
        tied_stat_count = 1;
      }

      if (user_monster->sp_def == highest_stat_val) {
        AddToTiedStatList(STAT_SP_DEF, highest_stats);
        tied_stat_count++;
      } else if (user_monster->sp_def > highest_stat_val) {
        highest_stats[0] = STAT_SP_DEF;
        highest_stats[1] = STAT_NONE;
        highest_stats[2] = STAT_NONE;
        highest_stats[3] = STAT_NONE;
        highest_stat_val = user_monster->sp_def;
        tied_stat_count = 1;
      }

      int selected_stat = highest_stats[RandRange(0, tied_stat_count)];
      switch (selected_stat) {
        case STAT_ATK: {
          BoostOffensiveStat(user, user, 0, 1);
          break;
        }
        case STAT_DEF: {
          BoostDefensiveStat(user, user, 0, 1);
          break;
        }
        case STAT_SP_ATK: {
          BoostOffensiveStat(user, user, 1, 1);
          break;
        }
        case STAT_SP_DEF: {
          BoostDefensiveStat(user, user, 1, 1);
          break;
        }
      }
    }
  }
}
