#include <pmdsky.h>
#include <cot.h>

// Remove the comment in patches/patch.cotpatch to enable this example patch
int CustomGetMovePower(struct entity* entity, struct move* move) {
  // Randomize move power
  int rolledPower = RandRange(1, 100);

  // Print the rolled value to the message log
  char messageBuffer[32];
  Snprintf(messageBuffer, 32, "Rolled move power %d!", rolledPower);
  
  LogMessage(entity, messageBuffer, true);

  return rolledPower;
}

// You can add other patches here...
