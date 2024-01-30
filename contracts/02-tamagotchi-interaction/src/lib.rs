#![no_std]

use gstd::{exec, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_interaction_io::{Tamagotchi, TmgAction, TmgEvent};

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let name: String = msg::load()
        .expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name: name.clone(),
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 1,
        fed_block: exec::block_height() as u64,
        entertained: 1,
        entertained_block: exec::block_height() as u64,
        slept: 1,
        slept_block: exec::block_height() as u64
    };

    unsafe {
        TAMAGOTCHI = Some(tamagotchi)
    }

    msg::reply(
        TmgEvent::Name(name),
        0
    ).unwrap();
}

#[no_mangle]
extern fn handle() {
    let input_msg = msg::load().expect("Error in loading Tmg Input Message");
    let tmg = unsafe {
        TAMAGOTCHI.as_mut().expect("The contract is not initialized")
    };
    match input_msg {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0).expect("Name not loaded correctly");
        }
        TmgAction::Age => {
            msg::reply(TmgEvent::Age(exec::block_timestamp() - tmg.date_of_birth), 0).expect("Age not loaded correctly");
        }
        TmgAction::Feed => {
            tmg.fed -= (exec::block_height() as u64 - tmg.fed_block) * HUNGER_PER_BLOCK;
            tmg.fed += FILL_PER_FEED;
            tmg.fed_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Fed, 0).expect("Not fed correctly");
        }
        TmgAction::Entertain => {
            tmg.entertained -= (exec::block_height() as u64 - tmg.entertained_block) * BOREDOM_PER_BLOCK;
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            tmg.entertained_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Entertained, 0).expect("Not entertained correctly");
        }
        TmgAction::Sleep => {
            tmg.slept -= (tmg.slept_block - exec::block_height() as u64) * ENERGY_PER_BLOCK;
            tmg.slept_block += FILL_PER_SLEEP;
            tmg.slept_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Slept, 0).expect("Not slept correctly");
        }
    }

}

#[no_mangle]
extern fn state() {
    let tmg = unsafe {
        TAMAGOTCHI.as_ref().expect("The contract is not initialized")
    };
    msg::reply(tmg, 0).expect("Failed to share state");
}
