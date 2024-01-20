#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, msg};
use gstd::prelude::*;
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{AttributeId, TransactionId, StoreAction, StoreEvent};


#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
    // TODO: 2️⃣ Add new fields
    pub ft_contract_id: ActorId,
    pub transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
}

impl Tamagotchi {
    pub async fn approve_tokens(&mut self, account: &ActorId, amount: u128) {
        msg::send_for_reply_as::<_, FTokenEvent>(
            self.ft_contract_id,
            FTokenAction::Message {
                transaction_id: self.transaction_id,
                payload: LogicAction::Approve {
                    approved_account: *account,
                    amount,
                },
            },
            0,
            0,
        )
            .expect("Error in sending a message `FTokenAction::Message`");
    }

    pub async fn buy_attribute(&mut self, store_id: ActorId, attribute_id: AttributeId) {
        msg::send_for_reply_as::<_, StoreEvent>(
            store_id.clone(),
            StoreAction::BuyAttribute {
                attribute_id
            },
            0,
            0
        )
            .expect("Error in sending message BuyAttribute to Store");
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    // TODO: 3️⃣ Add new actions
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    // TODO: 4️⃣ Add new events
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Tamagotchi>;
}
