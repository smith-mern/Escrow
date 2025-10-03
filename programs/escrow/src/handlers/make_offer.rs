use super::shared::transfer_tokens;
use crate::{error::ErrorCode, state::Offer};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token, token_interface::{Mint, TokenAccount, TokenInterface}
};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer <'info> {
    // Used to manage associated token accounts
    // ie where a wallet hollds a specific type of token
    pub associated_token_program: Program<'info, AssociatedToken>,

    // work with either the classic token program
    // the newer token program
    pub token_program: Interface<'info, TokenInterface>,

    // used to create accounts
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = associated_token_program,
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    // creating the offer 
    #[account(
        init, 
        payer = maker,
        space = Offer::DISCRIMINATOR.len() + Offer::INIT_SPACE,
        seeds = [b"offer", id.to_le_bytes().as_ref()],
        bump,
    )]
    pub offer: Account<'info, Offer>,

    // vault account 
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = associated_token_program,
    )] 
    pub vault: InterfaceAccount<'info, TokenAccount>,                                                                     
}

// Handle the make offer instruction by:
// 1. Moving the tokens from the maker's ATA to the vault
// 2. Saving the details of the offer to the offer account
pub fn make_offer(context: Context<MakeOffer>, id: u64, token_a_offered_amount: u64, token_b_wanted_amount: u64) -> Result<()> {
    // validate amounts - checking if they want more than zero
    require!(token_a_offered_amount > 0, ErrorCode::InvalidAmount);
    require!(token_b_wanted_amount > 0, ErrorCode::InvalidAmount);

    // validate token mints are different 
    require!(context.accounts.token_mint_a.key() != context.accounts.token_mint_b.key(), ErrorCode::InvalidTokenMint);

    // move the tokens from the maker's ATA to the vault 
    transfer_tokens(
        from: &context.accounts.maker_token_account_a,
        to: &context.accounts.vault, &token_a_offered_amount,
        mint: &context.accounts.token_mint_a,
        authority: &context.accounts.maker.to_account_info(), &context.accounts.token_program,
        owning_pda_seeds: None,
    )Result<() Error>
    .map_err(op:|_| ErrorCode::InsufficientTakerBalance);
    Ok(())
}
