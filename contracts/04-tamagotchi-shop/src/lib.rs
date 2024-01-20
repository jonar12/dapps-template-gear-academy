#![no_std]

use gstd::exec::block_timestamp;
use gstd::{exec, msg};
#[allow(unused_imports)]
use gstd::prelude::*;
use tamagotchi_shop_io::{Tamagotchi, TmgAction, TmgEvent};

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;
static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// TODO: 5️⃣ Add the `approve_tokens` function




#[no_mangle]
extern fn init() {
    let name: String = msg::load()
        .expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name,
        date_of_birth: block_timestamp(),
        owner: msg::source(),
        fed: 1,
        fed_block: exec::block_height() as u64,
        entertained: 1,
        entertained_block: exec::block_height() as u64,
        slept: 1,
        slept_block: exec::block_height() as u64,
        approved_account: None,
        ft_contract_id: Default::default(),
        transaction_id: 0,
        approve_transaction: None,
    };

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

        TmgAction::Transfer(new_owner) => {
            let source = msg::source();
            if source != tmg.owner {
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

        // TODO; 6️⃣ Add handling new actions
        TmgAction::SetFTokenContract(ft_contract_id) => {
            tmg.ft_contract_id = ft_contract_id;
            msg::reply(TmgEvent::FTokenContractSet, 0).expect("Setting FT contract ID operation failed");
        }

        TmgAction::ApproveTokens { account, amount } => {
            tmg.approve_tokens(&account, amount).await;
            msg::reply(TmgEvent::TokensApproved { account, amount }, 0).expect("Error replying to ApproveTokens Action");
        }

        TmgAction::BuyAttribute { store_id, attribute_id } => {
            tmg.buy_attribute(store_id, attribute_id).await;
            msg::reply(TmgEvent::AttributeBought(attribute_id), 0).expect("Error in replying to BuyAttribute Action");
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
