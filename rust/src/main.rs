#![no_main]
#![no_std]

extern crate eos_rs;

use eos_rs::prelude::*;
use eos_rs::api::objects::*;
use eos_rs::api::dungeon_mode::{DungeonEffectsEmitter, DungeonEntityExt, DungeonRng, LogMessageBuilder};
use eos_rs::api::fixed::I24F8;
use eos_rs::api::overlay::{CreatableWithLease, OverlayLoadLease};
use eos_rs::api::random;
use eos_rs::ffi;
use eos_rs::log_impl::register_logger;

// This defines the patches that will be written to the game, the syntax should hopefully
// be somewhat self-explanatory.
// Note that only this file (`main.rs`) must contain a call to this macro. And only once.
patches! {
    has_high_health,
    print_args: special_process 101,
    just_panic: special_process 102,
    oran_berry_burn: item_effect item_catalog::ITEM_ORAN_BERRY,
    cut_badly_poisoned: move_effect move_catalog::MOVE_CUT,
    "
HasLowHealth+0:
  B has_high_health
    "
}

// !!!!
// NOTE: When adding your own code and removing the examples here, make sure to also remove
// the example C item and move effects and special processes found in ../../src. But don't remove
// the entire `CustomApplyItemEffect`, `CustomApplyMoveEffect` or `CustomScriptSpecialProcessCall`
// functions, only the example effects and switch cases for them.
// !!!!

/// This silly little patch overrides the low-health check by reversing it.
/// Instead of having less than 25% health being considered having low health,
/// it's the opposite now: If you have more or equal to 25% health, you're
/// considered having low health.
///
/// Just for demonstration purposes it also randomly (1/100 chance) outputs the phrase
/// "oh no! [string:1] has high health!" to the console, where [string:1] is replaced with
/// the monster that is being checked.
///
/// You will notice that this will corrupt the text display (and eventually other parts of the
/// memory) if you pause in the dungeon.
/// This also shows, that even though things are marked "technically safe" here in Rust
/// land, you should probably double check if you can really do everything in the context
/// of the function you patch.
/// If you want to test the item and move effects bundled with this `main.rs`, you probably
/// want to comment out the logic that logs messages in this function first (otherwise good luck
/// using an item or move ;) ).
#[no_mangle]
pub extern "C" fn has_high_health(
    entity: *mut DungeonEntity,
) -> ffi::bool_ {
    // This is only required for non-special process / effects patches.
    register_logger();
    info!("In has_high_health");

    // We don't really need to do this, since the entity will
    // (hopefully) always be valid when this function is called,
    // but for demo purposes, lets say we want to check this.
    assert!(DungeonEntity::is_valid(entity));

    // Alternatively you can also use the low-level functions of the game
    // directly, those are in the `ffi` module.
    // They are all completely outside of the management of Rust and are unsafe.
    // You need to make sure it's actually ok to call them like this:
    unsafe { assert!(ffi::EntityIsValid(entity) > 0); }

    // Get reference from raw pointer. You can generally assume that the game
    // gives you valid pointers. In some cases it might be a good idea to at least
    // do some null-pointer checks, but we don't need this here, especially since
    // we already asserted the entity is valid.
    let entity = unsafe { &mut *entity };

    // We get the info for monsters. This will return None, if the entity isn't a monster.
    // You can also check the type in entity.type_.
    let monster_info = entity
        .info_for_monster()
        .expect("Entity is not a monster");
    let high_hp = monster_info.hp >= monster_info.max_hp_stat / 4;

    // The dungeon mode has it's own RNG function, we use that one here.
    // (For the general one, use the `random` module as shown in `cut_badly_poisoned`.)
    //
    // Doing so requires calling a function from overlay 29. We need
    // to promise the compiler that overlay 29 is loaded.
    // Overlay 29 is the dungeon overlay, so obviously it is loaded,
    // so we can use this unsafe function to get a lease on the overlay,
    // which we can pass to the API function that checks if the entity is valid.
    // We can also get a lease on an overlay in a safe way, by using
    // `acquire`, this checks at runtime if the overlay is actually loaded.
    //
    // We will also need that lease for the `LogMessageBuilder` below.
    let ov29 = unsafe { OverlayLoadLease::<29>::acquire_unchecked() };

    if high_hp && DungeonRng::new(ov29.clone()).rand100() == 0 {
        LogMessageBuilder::new(ov29)
            .check_user_fainted()
            .popup()
            .string(1, entity)
            .log_str(entity, "oh no! [string:1] has high health!");
    }

    high_hp as ffi::bool_
}

/// This just prints the parameters to the log.
pub fn print_args(arg1: i16, arg2: i16, _ov11: &OverlayLoadLease<11>) -> i32 {
    info!("Running special process 101... You put in these parameters: {}, {}", arg1, arg2);
    0
}

/// This demonstrates panics.
pub fn just_panic(_: i16, _: i16, _: &OverlayLoadLease<11>) -> i32 {
    info!("Brace yourself, about to panic...");
    panic!("Rust code panicked! oh no!");
}

/// This causes the oran berry to afflict the burn status on the target.
pub fn oran_berry_burn(
    effects: &DungeonEffectsEmitter,
    user: &mut DungeonEntity,
    target: &mut DungeonEntity,
    used_item: &mut DungeonItem,
    _is_thrown: bool
) {
    info!("oran_berry_burn called.");
    // We check if the item is actually Oran Berry. This isn't really needed,
    // since c-of-time will make sure this is only called for Oran Berry,
    // but for demonstration purposes we do it anyway.
    if used_item.id.val() == item_catalog::ITEM_ORAN_BERRY {
        info!("oran_berry_burn detected Oran Berry.");
        match effects.try_inflict_burn_status(user, target, false, true, false) {
            true => info!("oran_berry_burn successfully burned."),
            false => warn!("oran_berry_burn failed to burn."),
        }
    }
}

/// This causes the target to be badly poisoned when using the Cut move (1 in 4 chance).
/// Move effect functions must return whether or not they dealt damage.
pub fn cut_badly_poisoned(
    effects: &DungeonEffectsEmitter,
    user: &mut DungeonEntity,
    target: &mut DungeonEntity,
    used_move: &mut Move
) -> bool {
    info!("cut_badly_poisoned called.");
    // We check if the move is actually Cut. This isn't really needed,
    // since c-of-time will make sure this is only called for Cut,
    // but for demonstration purposes we do it anyway.
    if used_move.id.val() == move_catalog::MOVE_CUT {
        info!("cut_badly_poisoned detected Cut.");
        if random::rand_i32(0..4) == 0 {
            info!("cut_badly_poisoned rolled a 0.");
            match effects.try_inflict_badly_poisoned_status(user, target, true, false) {
                true => info!("cut_badly_poisoned successfully poisoned."),
                false => warn!("cut_badly_poisoned failed to poison."),
            }
        }
        // At the end we deal the actual damage.
        effects.deal_damage(user, target, used_move, I24F8::from_bits(0x01_00), None) > 0
    } else {
        false
    }
}
