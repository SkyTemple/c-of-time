use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ffi;

/// Helper struct for dungeon RNG.
pub struct DungeonRng(OverlayLoadLease<29>);

impl CreatableWithLease<29> for DungeonRng {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self.0
    }
}

impl DungeonRng {
    // TODO
    //     - name: GenerateDungeonRngSeed
    //       address:
    //         NA: 0x22EA980
    //         EU: 0x22EB330
    //       description: |-
    //         Generates a seed with which to initialize the dungeon PRNG.
    //
    //         The seed is calculated by starting with a different seed, the "preseed" x0 (defaults to 1, but can be set by other functions). The preseed is iterated twice with the same recurrence relation used in the primary LCG to generate two pseudorandom 32-bit numbers x1 and x2. The output seed is then computed as
    //           seed = (x1 & 0xFF0000) | (x2 >> 0x10) | 1
    //         The value x1 is then saved as the new preseed.
    //
    //         This method of seeding the dungeon PRNG appears to be used only sometimes, depending on certain flags in the data for a given dungeon.
    //
    //         return: RNG seed
    //     - name: GetDungeonRngPreseed
    //       address:
    //         NA: 0x22EA9CC
    //         EU: 0x22EB37C
    //       description: |-
    //         Gets the current preseed stored in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
    //
    //         return: current dungeon RNG preseed
    //     - name: SetDungeonRngPreseed
    //       address:
    //         NA: 0x22EA9DC
    //         EU: 0x22EB38C
    //       description: |-
    //         Sets the preseed in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
    //
    //         r0: preseed
    //     - name: InitDungeonRng
    //       address:
    //         NA: 0x22EA9EC
    //         EU: 0x22EB39C
    //       description: |-
    //         Initialize (or reinitialize) the dungeon PRNG with a given seed. The primary LCG and the five secondary LCGs are initialized jointly, and with the same seed.
    //
    //         r0: seed
    //     - name: DungeonRand16Bit
    //       address:
    //         NA: 0x22EAA20
    //         EU: 0x22EB3D0
    //       description: |-
    //         Computes a pseudorandom 16-bit integer using the dungeon PRNG.
    //
    //         Note that the dungeon PRNG is only used in dungeon mode (as evidenced by these functions being in overlay 29). The game uses another lower-quality PRNG (see arm9.yml) for other needs.
    //
    //         Random numbers are generated with a linear congruential generator (LCG). The game actually maintains 6 separate sequences that can be used for generation: a primary LCG and 5 secondary LCGs. The generator used depends on parameters set on the global PRNG state.
    //
    //         All dungeon LCGs have a modulus of 2^32 and a multiplier of 1566083941 (see DUNGEON_PRNG_LCG_MULTIPLIER). The primary LCG uses an increment of 1, while the secondary LCGs use an increment of 2531011 (see DUNGEON_PRNG_LCG_INCREMENT_SECONDARY). So, for example, the primary LCG uses the recurrence relation:
    //           x = (1566083941*x_prev + 1) % 2^32
    //
    //         Since the dungeon LCGs generate 32-bit integers rather than 16-bit, the primary LCG yields 16-bit values by taking the upper 16 bits of the computed 32-bit value. The secondary LCGs yield 16-bit values by taking the lower 16 bits of the computed 32-bit value.
    //
    //         All of the dungeon LCGs have a hard-coded default seed of 1, but in practice the seed is set with a call to InitDungeonRng during dungeon initialization.
    //
    //         return: pseudorandom int on the interval [0, 65535]
    //     - name: DungeonRandInt
    //       address:
    //         NA: 0x22EAA98
    //         EU: 0x22EB448
    //       description: |-
    //         Compute a pseudorandom integer under a given maximum value using the dungeon PRNG.
    //
    //         r0: high
    //         return: pseudorandom integer on the interval [0, high - 1]
    //     - name: DungeonRandRange
    //       address:
    //         NA: 0x22EAAC0
    //         EU: 0x22EB470
    //       description: |-
    //         Compute a pseudorandom value between two integers using the dungeon PRNG.
    //
    //         r0: x
    //         r1: y
    //         return: pseudorandom integer on the interval [min(x, y), max(x, y) - 1]
    //     - name: DungeonRandOutcome
    //       address:
    //         NA:
    //           - 0x22EAB20
    //           - 0x22EAB50
    //       description: |-
    //         Returns the result of a possibly biased coin flip (a Bernoulli random variable) with some success probability p, using the dungeon PRNG.
    //
    //         r0: success percentage (100*p)
    //         return: true with probability p, false with probability (1-p)
    //     - name: DungeonRngUnsetSecondary
    //       address:
    //         NA: 0x22EAC34
    //         EU: 0x22EB5E4
    //       description: |-
    //         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation, and also resets the secondary LCG index back to 0.
    //
    //         Similar to DungeonRngSetPrimary, but DungeonRngSetPrimary doesn't modify the secondary LCG index if it was already set to something other than 0.
    //
    //         No params.
    //     - name: DungeonRngSetSecondary
    //       address:
    //         NA: 0x22EAC4C
    //         EU: 0x22EB5FC
    //       description: |-
    //         Sets the dungeon PRNG to use one of the 5 secondary LCGs for subsequent random number generation.
    //
    //         r0: secondary LCG index
    //     - name: DungeonRngSetPrimary
    //       address:
    //         NA: 0x22EAC64
    //       description: |-
    //         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation.
    //
    //         No params.
    //     - name: DungeonRandOutcomeUserTargetInteraction
    //       address:
    //         NA: 0x2324934
    //         EU: 0x232539C
    //       description: |-
    //         Like DungeonRandOutcome, but specifically for user-target interactions.
    //
    //         This modifies the underlying random process depending on factors like Serene Grace, and whether or not either entity has fainted.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: base success percentage (100*p). 0 is treated specially and guarantees success.
    //     - name: DungeonRandOutcomeUserAction
    //       address:
    //         NA: 0x2324A20
    //         EU: 0x2325488
    //       description: |-
    //         Like DungeonRandOutcome, but specifically for user actions.
    //
    //         This modifies the underlying random process to factor in Serene Grace (and checks whether the user is a valid entity).
    //
    //         r0: entity pointer
    //         r1: base success percentage (100*p). 0 is treated specially and guarantees success.

    /// Compute a pseudorandom integer on the interval [0, 100) using the dungeon PRNG.
    pub fn rand100(&self) -> u32 {
        unsafe { ffi::DungeonRand100() }
    }

}
