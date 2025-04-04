use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct CreateMintAccountArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeConfigurationParam {
    pub swap_fee: f32,                 //  swap percentage
    pub bonding_curve_limitation: u64, //  bonding curve limitation
    pub initial_virtual_sol: u64,      //  sol percentage which moves to pumpfun after bonding curve
    pub initial_virtual_token: u64,    //  sol percentage which moves to pumpfun after bonding curve
    pub create_pool_fee_lamports: u64, //  sol percentage which moves to pumpfun after bonding curve
}

pub const FEE_SEED: &'static [u8] = b"pumpfun_fee";
pub const SOL_POOL_SEED: &'static [u8] = b"sol_pool";
pub const SUPPLY: u64 = 1_000_000_000_000_000_000_u64;

pub const OP_FEE: u64 = 500_000_000_u64;
