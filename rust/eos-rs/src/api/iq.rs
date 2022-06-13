//! Structs and functions to interact with the data of items in a general context.

use crate::ffi;

/// An IQ Group ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type IqGroupId = ffi::iq_group_id;
impl Copy for IqGroupId {}

/// This impl provides general metadata about IQ Groups in the game.
impl IqGroupId {
    /// Returns the ID struct for the IQ Group with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing IQ Group),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this IQ Group.
    pub const fn id(&self) -> u32 {
        self.0
    }
}

/// An IQ Skill ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type IqSkillId = ffi::iq_skill_id;
impl Copy for IqSkillId {}

/// This impl provides general metadata about IQ Skills in the game.
impl IqSkillId {
    /// Returns the ID struct for the IQ Skill with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing IQ Skill),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this IQ Skill.
    pub const fn id(&self) -> u32 {
        self.0
    }
}
