#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, exec, ReservationId};
use gstd::prelude::*;

pub type TransactionId = u64;
pub type AttributeId = u32;

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;


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
    pub ft_contract_id: ActorId,
    pub transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
    pub reservations: Vec<ReservationId>
}

impl Tamagotchi {
    pub fn update_fed(&mut self) {
        self.fed -= (exec::block_height() as u64 - self.fed_block) * HUNGER_PER_BLOCK;
    }
    pub fn update_entertained(&mut self) {
        self.entertained -= (exec::block_height() as u64 - self.entertained_block) * BOREDOM_PER_BLOCK;
    }
    pub fn update_slept(&mut self) {
        self.slept -= (exec::block_height() as u64 - self.slept_block) * ENERGY_PER_BLOCK;
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
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    CheckState,
    ReserveGas {
        reservation_amount: u64,
        duration: u32
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
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
    FeedMe,
    PlayWithMe,
    WantToSleep,
    MakeReservation,
    GasReserved
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
