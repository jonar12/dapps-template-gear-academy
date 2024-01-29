#![no_std]

use gstd::exec::block_timestamp;
use gstd::{exec, msg, ReservationId};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_auto_io::{Tamagotchi, TmgAction, TmgEvent};

const INITIAL_TAMAGOTCHI_STATUS: u64 = 1000;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;
const TIME_INTERVAL: u32 = 30;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let name: String = msg::load()
        .expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name,
        date_of_birth: block_timestamp(),
        owner: msg::source(),
        fed: INITIAL_TAMAGOTCHI_STATUS,
        fed_block: exec::block_height() as u64,
        entertained: INITIAL_TAMAGOTCHI_STATUS,
        entertained_block: exec::block_height() as u64,
        slept: INITIAL_TAMAGOTCHI_STATUS,
        slept_block: exec::block_height() as u64,
        approved_account: None,
        ft_contract_id: Default::default(),
        transaction_id: 0,
        approve_transaction: None,
        reservations: Vec::new(),
    };

    // TODO: send a delayed message with the action CheckState;
    msg::send_delayed(exec::program_id(), TmgAction::CheckState, 0, TIME_INTERVAL).expect("Error in sending initial CheckState action message");

    unsafe {
        TAMAGOTCHI = Some(tamagotchi)
    }
}

#[gstd::async_main]
async fn main() {
    let input_msg = msg::load().expect("Error in loading Tmg Input Message");
    let tmg = unsafe {
        TAMAGOTCHI.as_mut().expect("The contract is not initialized")
    };
    match input_msg {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0).expect("Name not loaded correctly");
        }

        TmgAction::Age => {
            let age = block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Age not loaded correctly");
        }

        TmgAction::Feed => {
            tmg.update_fed();
            tmg.fed += FILL_PER_FEED;
            tmg.fed_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Fed, 0).expect("Not fed correctly");
        }

        TmgAction::Entertain => {
            tmg.update_entertained();
            tmg.entertained += FILL_PER_ENTERTAINMENT;
            tmg.entertained_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Entertained, 0).expect("Not entertained correctly");
        }

        TmgAction::Sleep => {
            tmg.update_slept();
            tmg.slept_block += FILL_PER_SLEEP;
            tmg.slept_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Slept, 0).expect("Not slept correctly");
        }

        TmgAction::Transfer(new_owner) => {
            let source = msg::source();
            if source != tmg.owner || source != tmg.approved_account.unwrap() {
                panic!("Transfer function is only available to the owner of the Tamagotchi or to the approved account");
            }
            tmg.owner = new_owner;
            msg::reply(TmgEvent::Transferred(tmg.owner), 0).expect("Transference not executed correctly");
        }

        TmgAction::Approve(approved_account) => {
            if msg::source() != tmg.owner {
                panic!("Approve function is only available to the current owner of the Tamagotchi");
            }
            tmg.approved_account = Some(approved_account);
            msg::reply(TmgEvent::Approved(tmg.approved_account.unwrap()), 0).expect("Account approval failed");
        }

        TmgAction::RevokeApproval => {
            if msg::source() != tmg.owner {
                panic!("Approve function is only available to the current owner of the Tamagotchi");
            }
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0).expect("Approval Revoke failed");
        }

        TmgAction::SetFTokenContract(ft_contract_id) => {
            tmg.ft_contract_id = ft_contract_id;
            msg::reply(TmgEvent::FTokenContractSet, 0).expect("Setting FT contract ID operation failed");
        }

        TmgAction::ApproveTokens { account, amount } => {
            // tmg.approve_tokens(&account, amount).await;
            // msg::reply(TmgEvent::TokensApproved { account, amount }, 0).expect("Error replying to ApproveTokens Action");
        }

        TmgAction::BuyAttribute { store_id, attribute_id } => {
            // tmg.buy_attribute(store_id, attribute_id).await;
            // msg::reply(TmgEvent::AttributeBought(attribute_id), 0).expect("Error in replying to BuyAttribute Action");
        }

        TmgAction::CheckState => {
            // Check the state and send a corresponding message if it's needed to (FeedMe, PlayWithMe, etc)
            tmg.update_fed();
            if tmg.fed <= 1 {
                msg::send_from_reservation(*tmg.reservations.last().unwrap(), tmg.owner, TmgEvent::FeedMe, 0).expect("Error in sending message to Tamagotchi owner asking to feed it");
            }

            tmg.update_entertained();
            if tmg.entertained <= 1 {
                msg::send_from_reservation(*tmg.reservations.last().unwrap(), tmg.owner, TmgEvent::PlayWithMe, 0).expect("Error in sending message to Tamagotchi owner asking to play with it");
            }

            tmg.update_slept();
            if tmg.slept <= 1 {
                msg::send_from_reservation(*tmg.reservations.last().unwrap(), tmg.owner, TmgEvent::WantToSleep, 0).expect("Error in sending message to Tamagotchi owner telling it wants to sleep");
            }

            // Send another delayed message to keep checking the state
            msg::send_delayed(exec::program_id(), TmgAction::CheckState, 0, TIME_INTERVAL).expect("Error in sending subsequent CheckState action message");
        }

        TmgAction::ReserveGas {reservation_amount, duration} => {
            let reservation_id = ReservationId::reserve(reservation_amount, duration).expect("Error in reserving gas");
            tmg.reservations.push(reservation_id);
            msg::reply(TmgEvent::GasReserved, 0).expect("Error in replying GasReserved event payload");
        }
    }
}

#[no_mangle]
extern fn state() {
    let tmg = unsafe {
        TAMAGOTCHI.take().expect("The contract is not initialized")
    };
    msg::reply(tmg, 0).expect("Failed to share state");
}
