#include <pmdsky.h>
#include <cot_basedefs.h>

// This function is called from a patch in patches/patch.cotpatch
int customGetMovePower(struct entity* entity, struct move* move) {
  // Randomize move power
  int rolledPower = RandRange(1, 100);

  // Print the rolled value to the message log
  char messageBuffer[20];
  Sprintf(messageBuffer, "Rolled move power %d!", rolledPower);
  LogMessage(entity, messageBuffer, true);

  return rolledPower;
}
