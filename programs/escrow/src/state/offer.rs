use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    // identifier of the offer
    pub id: u64,

    // who made the offer
    pub owner: Pubkey,

    // the token mint of the token being offered
    pub token_mint_a: Pubkey,

    // the token mint of the token wanted
    pub token_mint_b: Pubkey,

    // the amount of token b being wanted
    pub token_b_wanted_amount: u64,

    // the amount of token a being offered
    pub token_a_offered_amount: u64,

    // used to calculate the address for this account, we save it as a performance optimization
    pub bump: u8,
}
