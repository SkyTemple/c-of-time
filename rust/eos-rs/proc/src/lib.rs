use quote::quote;
use syn::parse_macro_input;
use eos_rs_patches_def::PatchesDef;

/// This generates the entrypoints for item & move effects and special processes.
/// The ASM glue code is not processed, this is done by the eos-rs-build crate.
#[proc_macro]
pub fn patches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This may SEEM silly, but for whatever reason it won't work without.
    let input: proc_macro::TokenStream = input.into();
    let def: PatchesDef = parse_macro_input!(input as PatchesDef);

    let item_effects_cases = def.item_effects.iter()
        .map(|(idx, fn_name)| quote! {
            #idx => {#fn_name(&effects, user, target, used_item, is_thrown > 0); 1},
        });

    let move_effects_cases = def.move_effects.iter()
        .map(|(idx, fn_name)| quote! {
            #idx => {data.out_dealt_damage = #fn_name(&effects, user, target, used_move) as u8; 1},
        });

    let special_processes_cases = def.special_processes.iter()
        .map(|(idx, fn_name)| quote! {
            #idx => #fn_name(arg1, arg2, &lease),
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
        ) {
            eos_rs::log_impl::register_logger();
            let lease = eos_rs::api::overlay::OverlayLoadLease::<11>::acquire_unchecked();
            match special_process_id {
                #(#special_processes_cases)*
                _ => eos_rs::prelude::warn!("Unhandled special process id: {}", special_process_id),
            }
        }
    }).into()
}
