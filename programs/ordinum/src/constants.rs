use anchor_lang::prelude::{Pubkey, pubkey};

pub const SPONSOR_SEED:&[u8] = b"sponsor";
pub const TRIAL_SEED:&[u8] = b"trial";
pub const ESCROW_SEED:&[u8] = b"escrow";
pub const COORDINATOR_SEED:&[u8] = b"coordinator";

pub const USDC_MINT: Pubkey = pubkey!("FF5wsxpVQvCpbtjQWmeZF2kRcrd8h6APCUpwydmq3ytw");