use eos_rs_patches_def::PatchesDef;
use quote::quote;
use syn::parse_macro_input;

/// This generates the entrypoints for item & move effects and special processes.
/// This macro needs to be called exactly once and only in the `main` module.
///
/// # Arguments
/// - All but the last argument can be one of these:
///   - `Raw Patch`
///   - `Item Effect`
///   - `Move Effect`
///   - `Special Process`
/// - The last argument is optional and can be `ASM glue code`.
///
/// ## Raw Patch
/// A raw patch that can be used any way you want. You will need to add `ASM glue code`
/// that will actually branch to this patch for it to actually do anything.
///
/// The patch function MUST be `extern "C"`, `pub` and be annotated with `#[no_mangle]`.
/// The arguments it receives depend on the glue code / where it is inserted in the machine code
/// of the ROM.
///
/// Raw patches must call [`register_logger`](https://eosrs.pmdcollab.org/armv5te-none-ndseoseabi-na/doc/eos_rs/log_impl/fn.register_logger.html)
/// for log macros to work.
///
/// ### Syntax of argument
/// `<Function Name; Rust Identifier>`
///
/// ### Signature of raw patch functions registered this way:
/// ```
/// #[no_mangle] pub extern "C" fn function(/* ? */) /* -> ? */ { /* ... */ }
/// ```
///
/// Can be optionally marked unsafe.
///
/// ### Note
/// At the present time, defining raw patches in this macro doesn't actually do anything.
/// Your patch will work just fine without it, however this could change in the future,
/// so please add your patches.
///
/// ## Item Effect
/// Registers a function that will be called for the defined item when it is used in a dungeon.
///
/// ### Syntax of argument
/// `<Function Name; Rust Identifier>: item_effect <Item ID; Literal number or Path>`
///
/// Item ID is the ID of the item itself, not an internal item effect ID.
///
/// ### Signature of raw patch functions registered this way:
/// ```
/// pub fn function(
///     effects: &eos_rs::api::dungeon_mode::DungeonEffectsEmitter,
///     user: &mut eos_rs::api::objects::DungeonEntity,
///     target: &mut eos_rs::api::objects::DungeonEntity,
///     used_item: &mut eos_rs::api::objects::DungeonItem,
///     is_thrown: bool
/// ) { /* ... */ }
/// ```
///
/// ## Move Effect
/// Registers a function that will be called for the defined move when it is used in a dungeon.
///
/// ### Syntax of argument
/// `<Function Name; Rust Identifier>: move_effect <Move ID; Literal number or Path>`
///
/// Move ID is the ID of the move itself, not an internal move effect ID.
///
/// ### Signature of raw patch functions registered this way:
/// ```
/// pub fn function(
///     effects: &eos_rs::api::dungeon_mode::DungeonEffectsEmitter,
///     user: &mut eos_rs::api::objects::DungeonEntity,
///     target: &mut eos_rs::api::objects::DungeonEntity,
///     used_move: &mut eos_rs::api::objects::Move
/// ) { /* ... */ }
/// ```
///
/// ## Special Process
/// Registers a function that can be called from the script engine using the "special process"
/// mechanism.
///
/// ### Syntax of argument
/// `<Function Name; Rust Identifier>: special_process <Process ID; Literal number>`
///
/// Process ID is the ID as used in the script engine (first parameter to the `ProcessSpecial`
/// opcode).
///
/// ### Signature of raw patch functions registered this way:
/// ```
/// pub fn function(arg1: i16, arg2: i16, ov11: &eos_rs::api::overlay::OverlayLoadLease<11>) -> i32 { /* ... */ 0 }
/// ```
///
/// ## ASM glue code
/// This is a literal string. It will later during the build process be converted into a `cotpatch`
/// file that is placed in the `patches/` directory of `c-of-time`.
///
/// See the documentation in the `README.md` of the `c-of-time` repository for more information
/// (section `Usage`; [on Github](https://github.com/tech-ticks/c-of-time#usage)).
///
/// # What this will generate
/// This will generate three functions at the place it's called:
///
/// ```
/// /// This function is called from C code of `c-of-time` (see `src/item_effects.c` in the
/// /// `c-of-time` repository).
/// /// It will select all item effects defined in this macro for the related item and call it.
/// /// This function will also set up the logger.
/// /// It will log a warning if it can't find any item effects.
/// ///
/// /// NOTE: By default `src/item_effects.c` is written in a way that will only call
/// /// the Rust codebase, if there is no item effect already defined in C code.
/// #[no_mangle]
/// pub unsafe extern "C" fn eos_rs_apply_item_effect(
///     user: *mut eos_rs::ffi::entity,
///     target: *mut eos_rs::ffi::entity,
///     used_item: *mut eos_rs::ffi::item,
///     is_thrown: eos_rs::ffi::bool_
/// ) -> eos_rs::ffi::bool_ { /* ... */ }
///
/// /// See `eos_rs_apply_item_effect`, this is exactly the same, but for move effects.
/// #[no_mangle]
/// pub unsafe extern "C" fn eos_rs_apply_move_effect(
///     data: *mut eos_rs::ffi::move_effect_input,
///     user: *mut eos_rs::ffi::entity,
///     target: *mut eos_rs::ffi::entity,
///     used_move: *mut eos_rs::ffi::move_
/// ) -> eos_rs::ffi::bool_ { /* ... */ }
///
/// /// See `eos_rs_apply_item_effect`, this is exactly the same, but for special processes.
/// #[no_mangle]
/// pub unsafe extern "C" fn eos_rs_call_special_process(
///     unknown: *mut eos_rs::ffi::undefined4,
///     special_process_id: eos_rs::ctypes::c_uint,
///     arg1: eos_rs::ctypes::c_short,
///     arg2: eos_rs::ctypes::c_short,
///     return_val: *mut i32
/// ) { /* ... */ }
/// ```
///
/// # Example
/// ```
/// use eos_rs_proc::patches;  // use `eos_rs::patches` from public code. The proc crate is private.
///
/// // `has_high_health`, `print_args` etc. are functions.
/// // See `rust/src/main.rs` in the default `c-of-time` repository for more details.
/// patches! {
///     has_high_health,
///     print_args: special_process 101,
///     just_panic: special_process 102,
///     oran_berry_burn: item_effect eos_rs::api::items::ItemId::ITEM_ORAN_BERRY,
///     cut_badly_poisoned: eos_rs::api::moves:: MoveId::MOVE_CUT,
///     "
/// HasLowHealth+0:
///   B has_high_health
///     "
/// }
/// ```
///
/// # Notes
/// The ASM glue code is not processed at runtime, this is done by the `eos-rs-build` crate
/// during build.
/// It will extract the ASM glue code to the `patches/` directory in the `c-of-time`
/// base directory (in the default configuration of the `c-of-time` repository.
#[proc_macro]
pub fn patches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This may SEEM silly, but for whatever reason it won't work without.
    #[allow(clippy::useless_conversion)]
    let input: proc_macro::TokenStream = input.into();
    let def: PatchesDef = parse_macro_input!(input as PatchesDef);

    let item_effects_cases = def.item_effects.iter().map(|(idx, fn_name)| {
        quote! {
            #idx => {#fn_name(&effects, user, target, used_item, is_thrown > 0); 1},
        }
    });

    let move_effects_cases = def.move_effects.iter().map(|(idx, fn_name)| {
        quote! {
            #idx => {data.out_dealt_damage = #fn_name(&effects, user, target, used_move) as u8; 1},
        }
    });

    let special_processes_cases = def.special_processes.iter().map(|(idx, fn_name)| {
        quote! {
            #idx => *return_val = #fn_name(arg1, arg2, &lease),
        }
    });

    (quote! {
        #[no_mangle]
        pub unsafe extern "C" fn eos_rs_apply_item_effect(
            user: *mut eos_rs::ffi::entity,
            target: *mut eos_rs::ffi::entity,
            used_item: *mut eos_rs::ffi::item,
            is_thrown: eos_rs::ffi::bool_
        ) -> eos_rs::ffi::bool_ {
            eos_rs::log_impl::register_logger();
            let effects = eos_rs::api::dungeon_mode::DungeonEffectsEmitter::new_unchecked();
            let user = &mut *user;
            let target = &mut *target;
            let used_item = &mut *used_item;
            match used_item.id.val() {
                #(#item_effects_cases)*
                _ => 0,
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn eos_rs_apply_move_effect(
            data: *mut eos_rs::ffi::move_effect_input,
            user: *mut eos_rs::ffi::entity,
            target: *mut eos_rs::ffi::entity,
            used_move: *mut eos_rs::ffi::move_,
        ) -> eos_rs::ffi::bool_ {
            eos_rs::log_impl::register_logger();
            let effects = eos_rs::api::dungeon_mode::DungeonEffectsEmitter::new_unchecked();
            let user = &mut *user;
            let target = &mut *target;
            let used_move = &mut *used_move;
            let data = &mut *data;
            match used_move.id.val() {
                #(#move_effects_cases)*
                _ => 0,
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn eos_rs_call_special_process(
            unknown: *mut eos_rs::ffi::undefined4,
            special_process_id: eos_rs::ctypes::c_uint,
            arg1: eos_rs::ctypes::c_short,
            arg2: eos_rs::ctypes::c_short,
            return_val: *mut i32
        ) {
            let return_val = unsafe { &mut*return_val };
            eos_rs::log_impl::register_logger();
            let lease = eos_rs::api::overlay::OverlayLoadLease::<11>::acquire_unchecked();
            match special_process_id {
                #(#special_processes_cases)*
                _ => eos_rs::prelude::warn!("Unhandled special process id: {}", special_process_id),
            }
        }
    })
    .into()
}
