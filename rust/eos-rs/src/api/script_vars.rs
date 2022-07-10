//! Code for handling script variables.

use crate::ctypes::c_void;
use crate::ffi;
use crate::ffi::script_var_type;
use alloc::ffi::CString;
use alloc::vec;
use alloc::vec::Vec;
use core::ffi::CStr;
use core::marker::PhantomData;
use core::ptr;

/// A script opcode ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ScriptOpcodeId = ffi::script_opcode_id;
impl Copy for ScriptOpcodeId {}

/// This impl provides general metadata about script opcodes in the game.
impl ScriptOpcodeId {
    /// Returns the ID struct for the script opcode with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing script opcode),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this script opcode.
    pub const fn id(&self) -> u32 {
        self.0
    }
}

/// A script variable ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ScriptVariableId = ffi::script_var_id;
impl Copy for ScriptVariableId {}

/// This impl provides general metadata about script variables in the game.
impl ScriptVariableId {
    /// Returns the ID struct for the script variable with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing script variable),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this script variable.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Whether or not this variable is a local variable (as opposed to a global one).
    fn is_local(&self) -> bool {
        self.0 >= ScriptVariableId::VAR_LOCAL0.0
    }
}

pub trait AsScriptVariableId {
    fn as_id(&self) -> ScriptVariableId;
}

impl AsScriptVariableId for ScriptVariableId {
    fn as_id(&self) -> ScriptVariableId {
        *self
    }
}

/// Value types of script variables.
///
/// You can convert into this from [`script_var_type::Type`] with
/// [`TryInto::try_into`] / [`TryFrom::try_from`].
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ScriptVariableValueType {
    None = script_var_type::VARTYPE_NONE,
    Bit = script_var_type::VARTYPE_BIT,
    String = script_var_type::VARTYPE_STRING,
    U8 = script_var_type::VARTYPE_UINT8,
    U16 = script_var_type::VARTYPE_UINT16,
    U32 = script_var_type::VARTYPE_UINT32,
    I8 = script_var_type::VARTYPE_INT8,
    I16 = script_var_type::VARTYPE_INT16,
    I32 = script_var_type::VARTYPE_INT32,
    Special = script_var_type::VARTYPE_SPECIAL,
}

impl TryFrom<script_var_type::Type> for ScriptVariableValueType {
    type Error = ();

    fn try_from(value: script_var_type::Type) -> Result<Self, Self::Error> {
        match value {
            script_var_type::VARTYPE_NONE => Ok(ScriptVariableValueType::None),
            script_var_type::VARTYPE_BIT => Ok(ScriptVariableValueType::Bit),
            script_var_type::VARTYPE_STRING => Ok(ScriptVariableValueType::String),
            script_var_type::VARTYPE_UINT8 => Ok(ScriptVariableValueType::U8),
            script_var_type::VARTYPE_UINT16 => Ok(ScriptVariableValueType::U16),
            script_var_type::VARTYPE_UINT32 => Ok(ScriptVariableValueType::U32),
            script_var_type::VARTYPE_INT8 => Ok(ScriptVariableValueType::I8),
            script_var_type::VARTYPE_INT16 => Ok(ScriptVariableValueType::I16),
            script_var_type::VARTYPE_INT32 => Ok(ScriptVariableValueType::I32),
            script_var_type::VARTYPE_SPECIAL => Ok(ScriptVariableValueType::Special),
            _ => Err(()),
        }
    }
}

/// Value of a script variable.
#[derive(Clone, Eq, PartialEq)]
pub enum ScriptVariableValue {
    None,
    Bit(bool),
    String(CString),
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
    Special(i32),
}

impl ScriptVariableValue {
    /// Type of this value.
    pub fn var_type(&self) -> ScriptVariableValueType {
        match self {
            ScriptVariableValue::None => ScriptVariableValueType::None,
            ScriptVariableValue::Bit(_) => ScriptVariableValueType::Bit,
            ScriptVariableValue::String(_) => ScriptVariableValueType::String,
            ScriptVariableValue::U8(_) => ScriptVariableValueType::U8,
            ScriptVariableValue::U16(_) => ScriptVariableValueType::U16,
            ScriptVariableValue::U32(_) => ScriptVariableValueType::U32,
            ScriptVariableValue::I8(_) => ScriptVariableValueType::I8,
            ScriptVariableValue::I16(_) => ScriptVariableValueType::I16,
            ScriptVariableValue::I32(_) => ScriptVariableValueType::I32,
            ScriptVariableValue::Special(_) => ScriptVariableValueType::Special,
        }
    }

    /// Returns the value as raw i32. For String, only the first character's value is returned.
    pub fn as_raw(&self) -> i32 {
        match self {
            ScriptVariableValue::None => 0,
            ScriptVariableValue::Bit(value) => *value as i32,
            ScriptVariableValue::String(value) => {
                if value.as_bytes().is_empty() {
                    0
                } else {
                    value.as_bytes()[0] as i32
                }
            }
            ScriptVariableValue::U8(value) => *value as i32,
            ScriptVariableValue::U16(value) => *value as i32,
            ScriptVariableValue::U32(value) => *value as i32,
            ScriptVariableValue::I8(value) => *value as i32,
            ScriptVariableValue::I16(value) => *value as i32,
            ScriptVariableValue::I32(value) => *value,
            ScriptVariableValue::Special(value) => *value,
        }
    }
}

/// This trait can be used to simply convert script variable values to their
/// primitive types. See [`ScriptVariableRead::value`] for an example.
pub trait UnwrapScriptVariableValueAs<T> {
    /// Force cast the variable into the given type.
    ///
    /// This will panic if:
    /// - The type `T` does not match the type represented by the `as_type`.
    /// - The `as_type` does not match the type of the value variant.
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> T;
}

impl UnwrapScriptVariableValueAs<()> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) {
        assert_eq!(
            as_type,
            ScriptVariableValueType::None,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::None => (),
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<bool> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> bool {
        assert_eq!(
            as_type,
            ScriptVariableValueType::Bit,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::Bit(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<CString> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> CString {
        assert_eq!(
            as_type,
            ScriptVariableValueType::String,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::String(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<u8> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> u8 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::U8,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::U8(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<u16> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> u16 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::U16,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::U16(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<u32> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> u32 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::U32,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::U32(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<i8> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> i8 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::I8,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::I8(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<i16> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> i16 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::I16,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::I16(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

impl UnwrapScriptVariableValueAs<i32> for ScriptVariableValue {
    fn unwrap_as(self, as_type: ScriptVariableValueType) -> i32 {
        assert_eq!(
            as_type,
            ScriptVariableValueType::I32,
            "Invalid use of `unwrap_as`"
        );
        match self {
            ScriptVariableValue::I32(v) => v,
            ScriptVariableValue::Special(v) => v,
            _ => panic!("Invalid use of `unwrap_as`"),
        }
    }
}

/// Helper struct for manipulating the global and local script variables.
pub struct ScriptVariables(PhantomData<()>);

impl ScriptVariables {
    /// Returns a struct for manipulating global and local script variables.
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global mutable variable (`static mut`), see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Initialize the script variable values table (SCRIPT_VARS_VALUES).
    ///
    /// The whole table is first zero-initialized. Then, all script variable values are first
    /// initialized to their defaults, after which some of them are overwritten with other
    /// hard-coded values.
    pub fn init(&mut self) {
        // SAFETY: This modifies global data, but it contains primitive values only.
        unsafe { ffi::InitScriptVariableValues() }
    }

    /// Get a reference to the global variable. This must not be used
    /// for local variables.
    pub fn global_variable(&self, var_id: ScriptVariableId) -> GlobalScriptVariableRef {
        assert!(!var_id.is_local());
        GlobalScriptVariableRef(var_id, PhantomData)
    }

    /// Get a mutable reference to the global variable. This must not be used
    /// for local variables.
    pub fn global_variable_mut(&mut self, var_id: ScriptVariableId) -> GlobalScriptVariableMut {
        assert!(!(var_id.is_local()));
        GlobalScriptVariableMut(var_id, PhantomData)
    }

    /// Get a reference to the local variable. This signature
    /// will probably update when we know the type of that table.
    ///
    /// # Safety
    /// The pointer to the local variable table must be valid.
    pub unsafe fn local_variable(
        &self,
        local_var_vals: *mut c_void,
        var_id: ScriptVariableId,
    ) -> LocalScriptVariableRef {
        assert!(var_id.is_local());
        LocalScriptVariableRef(local_var_vals, var_id, PhantomData)
    }

    /// Get a mutable reference to the local variable. This signature
    /// will probably update when we know the type of that table.
    ///
    /// # Safety
    /// The pointer to the local variable table must be valid.
    pub unsafe fn local_variable_mut(
        &mut self,
        local_var_vals: *mut c_void,
        var_id: ScriptVariableId,
    ) -> LocalScriptVariableMut {
        assert!(var_id.is_local());
        LocalScriptVariableMut(local_var_vals, var_id, PhantomData)
    }

    /// Saves event flag script variables (see the code for an exhaustive list) to their respective
    /// BACKUP script variables, but only in certain game modes.
    ///
    /// This function prints the debug string "EventFlag BackupGameMode %d" with the game mode.
    pub fn event_flag_backup(&mut self) {
        unsafe { ffi::EventFlagBackup() }
    }

    /// Runs EventFlagBackup, then copies the script variable values table (SCRIPT_VARS_VALUES) to
    /// the given pointer.
    ///
    /// **Important:** This high-level function assumes that the variable data is 1024 bytes long.
    /// Should you somehow extend the table, you should use the low-level
    /// [`ffi::DumpScriptVariableValues`] directly.
    pub fn dump_script_variable_values(&mut self) -> [u8; 1024] {
        let mut data = [0; 1024];
        unsafe {
            ffi::DumpScriptVariableValues(data.as_mut_ptr() as *mut c_void);
        }
        data
    }

    /// Restores the script variable values table (SCRIPT_VARS_VALUES) with the given data.
    ///
    /// **Important:** This high-level function assumes that the variable data is 1024 bytes long.
    /// Should you somehow extend the table, you should use the low-level
    /// [`ffi::RestoreScriptVariableValues`] directly.
    pub fn restore_script_variable_values(&mut self, src: &[u8; 1024]) -> bool {
        unsafe { ffi::RestoreScriptVariableValues(force_mut_ptr!(src)) > 0 }
    }

    /// Initializes an assortment of event flag script variables (see the code for an exhaustive
    /// list).
    pub fn init_event_flags(&mut self) {
        unsafe { ffi::InitEventFlagScriptVars() }
    }

    /// Initializes most of the SCENARIO_* script variables (except SCENARIO_TALK_BIT_FLAG for some
    /// reason). Also initializes the PLAY_OLD_GAME variable.
    pub fn init_scenario_script_vars(&mut self) {
        unsafe { ffi::InitScenarioScriptVars() }
    }

    /// Initializes the WORLD_MAP_* script variable values (IDs 0x55-0x57).
    pub fn init_world_map_script_vars(&mut self) {
        unsafe { ffi::InitWorldMapScriptVars() }
    }

    pub fn init_dungeon_list_script_vars(&mut self) {
        unsafe { ffi::InitDungeonListScriptVars() }
    }

    /// Saves scenario flag script variables (SCENARIO_SELECT, SCENARIO_MAIN_BIT_FLAG) to their
    /// respective BACKUP script variables, but only in certain game modes.
    ///
    /// This function prints the debug string "ScenarioFlag BackupGameMode %d" with the game mode.
    pub fn scenario_flag_backup(&mut self) {
        unsafe { ffi::ScenarioFlagBackup() }
    }

    /// Gets the language type.
    ///
    /// This is the value backing the special LANGUAGE_TYPE script variable.
    pub fn get_language_type(&self) -> i32 {
        unsafe { ffi::GetLanguageType() }
    }

    /// Returns the current value of the NOTIFY_NOTE script variable.
    pub fn get_notify_note(&self) -> bool {
        unsafe { ffi::GetNotifyNote() > 0 }
    }

    /// Sets the current value of the NOTIFY_NOTE script variable.
    pub fn set_notify_note(&mut self, value: bool) {
        unsafe { ffi::SetNotifyNote(value as ffi::bool_) }
    }

    /// Gets the value of the GAME_MODE script variable.
    pub fn get_game_mode(&self) -> i32 {
        unsafe { ffi::GetGameMode() }
    }

    /// Gets the special episode type from the SPECIAL_EPISODE_TYPE script variable.
    pub fn get_special_episode_type(&self) -> i32 {
        unsafe { ffi::GetSpecialEpisodeType() }
    }
}

/// Reference to a global script variable, see [`ScriptVariableRead`].
pub struct GlobalScriptVariableRef<'a>(ScriptVariableId, PhantomData<&'a ()>);

/// Mutable reference to a global script variable, see
/// [`ScriptVariableRead`] and [`ScriptVariableWrite`].
pub struct GlobalScriptVariableMut<'a>(ScriptVariableId, PhantomData<&'a ()>);

/// Reference to a local script variable, see [`ScriptVariableRead`].
pub struct LocalScriptVariableRef<'a>(*mut c_void, ScriptVariableId, PhantomData<&'a ()>);

/// Mutable reference to a local script variable, see
/// [`ScriptVariableRead`] and [`ScriptVariableWrite`].
pub struct LocalScriptVariableMut<'a>(*mut c_void, ScriptVariableId, PhantomData<&'a ()>);

/// Read actions for script variables.
pub trait ScriptVariableRead: PartialEq + Eq {
    #[doc(hidden)]
    fn internal_local_var_table(&self) -> *mut c_void;

    /// Returns the variable ID
    fn id(&self) -> ScriptVariableId;

    /// Loads a script variable descriptor for a given ID.
    fn descriptor(&self) -> &ffi::script_var {
        let mut out = ffi::script_var_desc {
            desc: ptr::null_mut(),
            value: ptr::null_mut(),
        };
        unsafe {
            ffi::LoadScriptVariableRaw(&mut out, self.internal_local_var_table(), self.id());
            let ffi::script_var_desc { desc, .. } = out;
            &*desc
        }
    }

    /// Returns whether or not this is a local variable (as opposed to a global one).
    fn is_local(&self) -> bool {
        self.id().is_local()
    }

    /// Returns the type of the variable
    fn var_type(&self) -> ScriptVariableValueType {
        let desc = self.descriptor();
        desc.type_
            .val()
            .try_into()
            .expect("The variable has a corrupted type.")
    }

    /// Returns whether or not the variable is an array.
    fn is_array(&self) -> bool {
        self.capacity() > 1
    }

    /// Returns the number of elements in the array or 1 if it's not an array.
    fn capacity(&self) -> usize {
        let desc = self.descriptor();
        desc.n_values as usize
    }

    /// Returns the name of the variable.
    fn name(&self) -> &str {
        let desc = self.descriptor();
        unsafe {
            let c_str = CStr::from_ptr(desc.name);
            c_str.to_str().unwrap()
        }
    }

    /// Loads the value of a script variable. If this variable is an array, the value at index 0
    /// is returned.
    ///
    /// This will return the value of the variable as a `i32`, no matter the type of the variable.
    fn value_raw(&self) -> i32 {
        unsafe { ffi::LoadScriptVariableValue(self.internal_local_var_table(), self.id()) }
    }

    /// Loads the value of a script variable at some index (for script variables that are arrays).
    ///
    /// This will return the value of the variable as a `i32`, no matter the type of the variable.
    fn value_raw_indexed(&self, index: i32) -> i32 {
        unsafe {
            ffi::LoadScriptVariableValueAtIndex(self.internal_local_var_table(), self.id(), index)
        }
    }

    /// Loads the value of a script variable.
    ///
    /// If this variable is an array, the value at index 0 is returned.
    /// Special case: If the type is a string, the entire string is returned.
    ///
    /// This will return the value of the variable as an enum variant of [`ScriptVariableValue`],
    /// the variant will depend on the type of this variable. You can get the type with
    /// [`Self::var_type`] and force a cast with that:
    ///
    /// ```
    /// use eos_rs::api::script_vars::{ScriptVariableRead, UnwrapScriptVariableValueAs};
    ///
    /// fn demo(script_var: impl ScriptVariableRead) {
    ///     // Assuming that `script_var` is a boolean variable. If it isn't, this will panic.
    ///     let value: bool = script_var.value().unwrap_as(script_var.var_type());
    /// }
    /// ```
    fn value(&self) -> ScriptVariableValue {
        let desc = self.descriptor();
        let typ: ScriptVariableValueType = desc
            .type_
            .val()
            .try_into()
            .expect("The variable has a corrupted type.");
        let capacity = desc.n_values as i32;
        match typ {
            ScriptVariableValueType::None => ScriptVariableValue::None,
            ScriptVariableValueType::Bit => ScriptVariableValue::Bit(self.value_raw() > 0),
            ScriptVariableValueType::String => unsafe {
                ScriptVariableValue::String(CString::from_vec_unchecked(
                    (0..capacity)
                        .map(|i| self.value_raw_indexed(i) as u8)
                        .collect::<Vec<u8>>(),
                ))
            },
            ScriptVariableValueType::U8 => ScriptVariableValue::U8(self.value_raw() as u8),
            ScriptVariableValueType::U16 => ScriptVariableValue::U16(self.value_raw() as u16),
            ScriptVariableValueType::U32 => ScriptVariableValue::U32(self.value_raw() as u32),
            ScriptVariableValueType::I8 => ScriptVariableValue::I8(self.value_raw() as i8),
            ScriptVariableValueType::I16 => ScriptVariableValue::I16(self.value_raw() as i16),
            ScriptVariableValueType::I32 => ScriptVariableValue::I32(self.value_raw()),
            ScriptVariableValueType::Special => ScriptVariableValue::Special(self.value_raw()),
        }
    }

    /// Loads the value of a script variable at some index (for script variables that are arrays).
    ///
    /// Panics if the read is out of bounds.
    ///
    /// Special case: If the type is a string, the character at the given position is returned
    /// (but still as a valid single-character CString).
    ///
    /// This will return the value of the variable as an enum variant of `ScriptVariableValue`,
    /// see [`Self::value`] for more information.
    fn value_indexed(&self, index: i32) -> ScriptVariableValue {
        let desc = self.descriptor();
        let typ = desc
            .type_
            .val()
            .try_into()
            .expect("The variable has a corrupted type.");
        let capacity = desc.n_values;
        assert!(index <= capacity as i32, "Out-of-bounds.");
        match typ {
            ScriptVariableValueType::None => ScriptVariableValue::None,
            ScriptVariableValueType::Bit => {
                ScriptVariableValue::Bit(self.value_raw_indexed(index) > 0)
            }
            ScriptVariableValueType::String => unsafe {
                let val = self.value_raw_indexed(index) as u8;
                if val == 0 {
                    return ScriptVariableValue::String(CString::default());
                }
                ScriptVariableValue::String(CString::from_vec_with_nul_unchecked(vec![val, 0]))
            },
            ScriptVariableValueType::U8 => {
                ScriptVariableValue::U8(self.value_raw_indexed(index) as u8)
            }
            ScriptVariableValueType::U16 => {
                ScriptVariableValue::U16(self.value_raw_indexed(index) as u16)
            }
            ScriptVariableValueType::U32 => {
                ScriptVariableValue::U32(self.value_raw_indexed(index) as u32)
            }
            ScriptVariableValueType::I8 => {
                ScriptVariableValue::I8(self.value_raw_indexed(index) as i8)
            }
            ScriptVariableValueType::I16 => {
                ScriptVariableValue::I16(self.value_raw_indexed(index) as i16)
            }
            ScriptVariableValueType::I32 => ScriptVariableValue::I32(self.value_raw_indexed(index)),
            ScriptVariableValueType::Special => {
                ScriptVariableValue::Special(self.value_raw_indexed(index))
            }
        }
    }

    /// Loads the sum of all values of the script variable (for script variables that are
    /// arrays).
    fn sum(&self) -> i32 {
        unsafe { ffi::LoadScriptVariableValueSum(self.internal_local_var_table(), self.id()) }
    }

    /// Checks if two script variables have equal values. For arrays, compares elementwise for the
    /// length of the first variable.
    ///
    /// Hint: Implementors of this trait also implement `PartialEq` and `Eq`, so you can use those
    /// instead if the underlying type is the same.
    fn var_eq<S: AsScriptVariableId>(&self, other: S) -> bool {
        if self.id().is_local() != other.as_id().is_local() {
            return false;
        }
        unsafe {
            ffi::ScriptVariablesEqual(self.internal_local_var_table(), self.id(), other.as_id()) > 0
        }
    }
}

impl<T> AsScriptVariableId for T
where
    T: ScriptVariableRead,
{
    fn as_id(&self) -> ScriptVariableId {
        self.id()
    }
}

/// Write actions for script variables.
pub trait ScriptVariableWrite: ScriptVariableRead {
    /// Zero-initialize the values of the given script variable.
    fn zero_init(&mut self, var_id: ScriptVariableId) {
        assert!(var_id.is_local());
        unsafe { ffi::ZinitScriptVariable(self.internal_local_var_table(), self.id()) }
    }

    /// Writes the given value to a script variable.
    fn write_raw(&mut self, value: i32) {
        // SAFETY: The game makes sure the value fits.
        unsafe { ffi::SaveScriptVariableValue(self.internal_local_var_table(), self.id(), value) }
    }

    /// Writes the given value to a script variable at the given index (if this is an array).
    ///
    /// Panics if the write is out of bounds.
    fn write_raw_indexed(&mut self, index: i32, value: i32) {
        let desc = self.descriptor();
        let capacity = desc.n_values;
        assert!(index <= capacity as i32, "Out-of-bounds.");
        // SAFETY: We make sure the variable in an array, the index is in bound & the game makes sure the value fits.
        unsafe {
            ffi::SaveScriptVariableValueAtIndex(
                self.internal_local_var_table(),
                self.id(),
                index,
                value,
            )
        }
    }

    /// Writes the given value to a script variable.
    ///
    /// If this is an array, it's written to position 0.
    /// Special case: If the type is a string, the entire string is written. Panics if the string
    /// would overflow the array.
    ///
    /// If the value type doesn't match, this panics.
    fn write(&mut self, value: ScriptVariableValue) {
        let desc = self.descriptor();
        let typ = desc
            .type_
            .val()
            .try_into()
            .expect("The variable has a corrupted type.");
        assert_eq!(
            value.var_type(),
            typ,
            "The type of the value to write doesn't match the variable's type."
        );
        if let ScriptVariableValue::String(value) = value {
            let value = value.as_bytes();
            if value.len() > desc.n_values as usize {
                panic!("The string to write to the variable is too long.");
            }
            for (i, b) in value.iter().enumerate() {
                self.write_raw_indexed(i as i32, *b as i32)
            }
        } else {
            self.write_raw(value.as_raw())
        }
    }

    /// Writes to the given variable as if it were a scenario variable.
    ///
    /// # Safety
    /// You must make sure yourself that this variable is a scenario variable. No bounds or type
    /// checking is done.
    unsafe fn write_as_scenario(&mut self, value1: u8, value2: u8) {
        ffi::SetScenarioScriptVar(self.id(), value1, value2)
    }

    /// Writes the given value to a script variable (if this is an array).
    ///
    /// Panics if the write is out of bounds.
    ///
    /// If the value type doesn't match, this panics.
    ///
    /// For string type variables, the input value must be a valid single-character CString.
    /// Only the character at the specified index is changed.
    fn write_indexed(&mut self, index: i32, value: ScriptVariableValue) {
        let desc = self.descriptor();
        let typ = desc
            .type_
            .val()
            .try_into()
            .expect("The variable has a corrupted type.");
        assert_eq!(
            value.var_type(),
            typ,
            "The type of the value to write doesn't match the variable's type."
        );
        let capacity = desc.n_values;
        assert!(index <= capacity as i32, "Out-of-bounds.");
        self.write_raw_indexed(index, value.as_raw())
    }
}

impl<'a> PartialEq for GlobalScriptVariableRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::ScriptVariablesEqual(ptr::null_mut(), self.0, other.0) > 0 }
    }
}

impl<'a> Eq for GlobalScriptVariableRef<'a> {}

impl<'a> ScriptVariableRead for GlobalScriptVariableRef<'a> {
    fn internal_local_var_table(&self) -> *mut c_void {
        ptr::null_mut()
    }

    fn id(&self) -> ScriptVariableId {
        self.0
    }
}

impl<'a> PartialEq for GlobalScriptVariableMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::ScriptVariablesEqual(ptr::null_mut(), self.0, other.0) > 0 }
    }
}

impl<'a> Eq for GlobalScriptVariableMut<'a> {}

impl<'a> ScriptVariableRead for GlobalScriptVariableMut<'a> {
    fn internal_local_var_table(&self) -> *mut c_void {
        ptr::null_mut()
    }

    fn id(&self) -> ScriptVariableId {
        self.0
    }
}

impl<'a> ScriptVariableWrite for GlobalScriptVariableMut<'a> {}

impl<'a> PartialEq for LocalScriptVariableRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 {
            return false;
        }
        unsafe { ffi::ScriptVariablesEqual(self.0, self.1, other.1) > 0 }
    }
}

impl<'a> Eq for LocalScriptVariableRef<'a> {}

impl<'a> ScriptVariableRead for LocalScriptVariableRef<'a> {
    fn internal_local_var_table(&self) -> *mut c_void {
        self.0
    }

    fn id(&self) -> ScriptVariableId {
        self.1
    }
}

impl<'a> PartialEq for LocalScriptVariableMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.0 != other.0 {
            return false;
        }
        unsafe { ffi::ScriptVariablesEqual(self.0, self.1, other.1) > 0 }
    }
}

impl<'a> Eq for LocalScriptVariableMut<'a> {}

impl<'a> ScriptVariableRead for LocalScriptVariableMut<'a> {
    fn internal_local_var_table(&self) -> *mut c_void {
        self.0
    }

    fn id(&self) -> ScriptVariableId {
        self.1
    }
}

impl<'a> ScriptVariableWrite for LocalScriptVariableMut<'a> {}
