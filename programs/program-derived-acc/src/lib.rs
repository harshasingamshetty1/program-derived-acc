use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("3Z3dWerEbF5RGtcu7S3avuKRMSnHr8tUoJ69AhEWSDHM");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        // Get Escrow Account
        let escrow = &mut ctx.accounts.escrow;

        // Set from
        escrow.from = ctx.accounts.from.key();
        // Set to
        escrow.to = ctx.accounts.to.key();
        // set amount
        escrow.amount = amount;

        Ok(())
    }
}

/// CreateEscrow context
#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    // Escrow Account PDA
    #[account(
        init,
        // State account seed uses the string "state" and the users' key. 
        // Note that we can only have 1 active transaction
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        // This is used when the PDA account falls on the ed25519 curve line  (which is not intended)
        // The accounts on the curve will have both the Pub/private key
        // Where as the PDA accounts should not have any private key and needs to be signed/operated through the Programs
        bump,
        payer = from,
        space = size_of::<EscrowAccount>() + 16
    )]
    // As this struct holds the data we want to store of the contract
    // Therefore, this means we are creating a  PDA account
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// Escrow Account Structure
#[account]
pub struct EscrowAccount {
    // From address
    pub from: Pubkey,

    // To address
    pub to: Pubkey,

    // Amount that is owed
    pub amount: u64,
}
