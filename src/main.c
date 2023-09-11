#include <pmdsky.h>
#include <cot.h>

// Remove the comment in patches/patch.asm to enable this example patch.
// `attribute((used))` is required to prevent the compiler from optimizing out the function
// if it's only used in a patch.
__attribute__((used)) int CustomGetMovePower(struct entity* entity, struct move* move) {
  // Randomize move power
  int rolledPower = RandRange(1, 100);

  // Print the rolled value to the message log
  char messageBuffer[32];
  snprintf(messageBuffer, 32, "Rolled move power %d!", rolledPower);
  
  LogMessage(entity, messageBuffer, true);

  return rolledPower;
}

// You can add other patches here...
