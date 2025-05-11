use anchor_lang::prelude::*;
use solana_program::clock::Clock;

declare_id!("EbqyVUScNA86Qg3AiMQB7ThQ6BwSpcR2563Uh65q85iK");

#[program]
pub mod receipt_pdas {
    use super::*;

    pub fn mint(ctx: Context<Receipts>) -> Result<()> {
        let receipt = &mut ctx.accounts.pda_account;
        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp as u64;
        receipt.timestamps.push(timestamp);
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let receipt = &mut ctx.accounts.pda_account;
        receipt.timestamps = Vec::new();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub client: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [signer.key().as_ref(), client.key().as_ref()],
        payer = signer,
        space = 8 + 4,
        bump,
    )]
    pub pda_account: Account<'info, Receipt>, //this is provided by the client, and verified at runtime
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Receipts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub client: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [signer.key().as_ref(), client.key().as_ref()],
        bump,
        realloc = pda_account.get_new_size(),
        realloc::payer = signer,
        realloc::zero = false
    )]
    pub pda_account: Account<'info, Receipt>, //this is provided by the client, and verified at runtime
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Receipt {
    #[max_len(0)]
    pub timestamps: Vec<u64>,
}

impl Receipt{
   pub fn get_new_size(&self) -> usize { // adding one element
      let discriminator_size = 8;
      let empty_vec_size = 4;
      let vec_conents_size = (self.timestamps.len() + 1) * 8;
      [
        discriminator_size,
        empty_vec_size,
        vec_conents_size
      ].into_iter().sum()
   }
}
