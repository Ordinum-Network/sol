use anchor_lang::prelude::{Pubkey, pubkey};

pub const SPONSOR_SEED:&[u8] = b"sponsor";
pub const TRIAL_SEED:&[u8] = b"trial";
pub const ESCROW_SEED:&[u8] = b"escrow";
pub const COORDINATOR_SEED:&[u8] = b"coordinator";
pub const PATIENT_SEED:&[u8] = b"patient";
pub const VISIT_RECORD:&[u8] = b"visit_record";
pub const PHASE:&[u8] = b"phase";
pub const PAYMENT:&[u8] = b"payment";

pub const USDC_MINT: Pubkey = pubkey!("FNwaF4jp4H1xd3y7u6XvCWKf2LgH4wuwrQdBTh9vQXZi");