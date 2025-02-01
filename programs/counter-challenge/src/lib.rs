use anchor_lang::prelude::*;

declare_id!("3EjBmyZf3Z3uiSpHJQRzJnae42q7qcwuHEQ6CKWRXbKb");

#[program]
pub mod counter_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter =&mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter has been initializated {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter =&mut ctx.accounts.counter;
        msg!("Counter before increment {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter after increment {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter =&mut ctx.accounts.counter;
        msg!("Counter before decrement {}", counter.count);

        if counter.count == 0 {
            return Err(ProgramError::InvalidArgument.into());
        }

        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter after decrement {}", counter.count);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
  pub count: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

const ANCHOR_DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>
}