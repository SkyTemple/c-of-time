use crate::api::dungeon_mode::entity::DungeonEntity;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::api::random::{rand_u16_internal, Rng};
use crate::ctypes::c_int;
use crate::ffi;
use core::hint::unreachable_unchecked;
use core::ops::RangeBounds;

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

struct DungeonRngImpl;

impl Rng for DungeonRngImpl {
    fn rand16(&mut self) -> u16 {
        unsafe { ffi::DungeonRand16Bit() as u16 }
    }

    /// NOT SUPPORTED BY THIS.
    fn rand32(&mut self) -> u32 {
        // SAFETY: This will never be called. It is not used in this module and it is not exposed.
        unsafe { unreachable_unchecked() }
    }

    /// NOT SUPPORTED BY THIS.
    fn rand_range32(&mut self, _x: c_int, _y: c_int) -> c_int {
        // SAFETY: This will never be called. It is not used in this module and it is not exposed.
        unsafe { unreachable_unchecked() }
    }
}

impl DungeonRng {
    /// Generates a seed with which to initialize the dungeon PRNG.
    ///
    /// The seed is calculated by starting with a different seed, the "preseed" x0 (defaults to 1,
    /// but can be set by other functions). The preseed is iterated twice with the same recurrence
    /// relation used in the primary LCG to generate two pseudorandom 32-bit numbers x1 and x2.
    /// The output seed is then computed as
    ///
    /// ```text
    /// seed = (x1 & 0xFF0000) | (x2 >> 0x10) | 1
    /// ```
    ///
    /// The value x1 is then saved as the new preseed.
    ///
    /// This method of seeding the dungeon PRNG appears to be used only sometimes, depending on
    /// certain flags in the data for a given dungeon.
    pub fn generate_dungeon_rng_seed(&self) -> u32 {
        unsafe { ffi::GenerateDungeonRngSeed() }
    }

    /// Gets the current preseed stored in the global dungeon PRNG state.
    ///
    /// See [`Self::generate_dungeon_rng_seed`] for more information.
    pub fn get_dungeon_rng_preeseed(&self) -> u32 {
        unsafe { ffi::GetDungeonRngPreseed() }
    }

    /// Gets the current preseed stored in the global dungeon PRNG state.
    ///
    /// See [`Self::generate_dungeon_rng_seed`] for more information.
    pub fn set_dungeon_rng_preeseed(&mut self, seed: u32) {
        unsafe { ffi::SetDungeonRngPreseed(seed) }
    }

    /// Initialize (or reinitialize) the dungeon PRNG with a given seed. The primary LCG and the
    /// five secondary LCGs are initialized jointly, and with the same seed.
    pub fn init_dungeon_rng(&mut self, seed: u32) {
        unsafe { ffi::InitDungeonRng(seed) }
    }

    /// Generates a random number between the beginning and end of the range.
    /// If the range is unbounded, min and/or max values are bound to
    /// 0 ([`u16::MIN`]) and [`u16::MAX`] respectively.
    ///
    /// Note that this uses the dungeon PRNG as opposed to the functions in [`crate::api::random`].
    ///
    /// Random numbers are generated with a linear congruential generator (LCG). The game actually
    /// maintains 6 separate sequences that can be used for generation: a primary LCG and 5
    /// secondary LCGs. The generator used depends on parameters set on the global PRNG state.
    ///
    /// All dungeon LCGs have a modulus of 2^32 and a multiplier of 1566083941
    /// (see symbol DUNGEON_PRNG_LCG_MULTIPLIER).
    /// The primary LCG uses an increment of 1, while the secondary LCGs use an increment of
    /// 2531011 (see DUNGEON_PRNG_LCG_INCREMENT_SECONDARY symbol).
    ///
    /// So, for example, the primary LCG uses the recurrence relation:
    ///
    /// ```text
    /// x = (1566083941 * x_prev + 1) % 2^32
    /// ```
    ///
    /// Since the dungeon LCGs generate 32-bit integers rather than 16-bit, the primary LCG yields
    /// 16-bit values by taking the upper 16 bits of the computed 32-bit value. The secondary LCGs
    /// yield 16-bit values by taking the lower 16 bits of the computed 32-bit value.
    ///
    /// All of the dungeon LCGs have a hard-coded default seed of 1, but in practice the
    /// seed is set with a call to InitDungeonRng during dungeon initialization.
    ///
    /// The range must contain at least one element, or this will panic.
    /// Same if the start bound is excluded.
    pub fn rand_u16<R: RangeBounds<u16>>(&self, range: R) -> u16 {
        rand_u16_internal(&mut DungeonRngImpl, range)
    }

    /// Compute a pseudorandom integer on the interval [0, 100) using the dungeon PRNG.
    pub fn rand100(&self) -> u32 {
        unsafe { ffi::DungeonRand100() }
    }

    /// Returns the result of a possibly biased coin flip (a Bernoulli random variable) with some
    /// success probability `p`, using the dungeon PRNG
    /// (`true` has a probability `p`, `false` has (`1-p`)).
    ///
    /// `success_percentage` is `100*p`.
    pub fn rand_outcome(&self, success_percentage: i32) -> bool {
        unsafe { ffi::DungeonRandOutcome(success_percentage) > 0 }
    }

    /// Like [`Self::rand_outcome`], but specifically for user-target interactions.
    ///
    /// This modifies the underlying random process depending on factors like Serene Grace, and
    /// whether or not either entity has fainted.
    ///
    /// A percentage of 0 is treated specially and guarantees success.
    pub fn rand_outcome_user_target_interaction(
        &self,
        user: &DungeonEntity,
        target: &DungeonEntity,
        success_percentage: i32,
    ) -> bool {
        unsafe {
            ffi::DungeonRandOutcomeUserTargetInteraction(
                force_mut_ptr!(user),
                force_mut_ptr!(target),
                success_percentage,
            ) > 0
        }
    }

    /// Like [`Self::rand_outcome`], but specifically for user actions.
    ///
    /// This modifies the underlying random process to factor in Serene Grace (and checks whether
    /// the user is a valid entity).
    ///
    /// A percentage of 0 is treated specially and guarantees success.
    pub fn rand_outcome_user_action(&self, user: &DungeonEntity, success_percentage: i32) -> bool {
        unsafe { ffi::DungeonRandOutcomeUserAction(force_mut_ptr!(user), success_percentage) > 0 }
    }

    /// Sets the dungeon PRNG to use the primary LCG for subsequent random number generation.
    pub fn set_primary_rng(&mut self) {
        unsafe { ffi::DungeonRngSetPrimary() }
    }

    /// Sets the dungeon PRNG to use one of the 5 secondary LCGs for subsequent random number
    /// generation.
    pub fn set_secondary_rng(&mut self, idx: i32) {
        unsafe { ffi::DungeonRngSetSecondary(idx) }
    }

    /// Sets the dungeon PRNG to use the primary LCG for subsequent random number generation,
    /// and also resets the secondary LCG index back to 0.
    ///
    /// Similar to [`Self::set_primary_rng`], but it doesn't modify the secondary LCG
    /// index if it was already set to something other than 0.
    pub fn unset_secondary_rng(&mut self, idx: i32) {
        unsafe { ffi::DungeonRngSetSecondary(idx) }
    }
}
