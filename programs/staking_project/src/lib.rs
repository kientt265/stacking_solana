use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("55d8NRKMvawmEUSbU6Sw52JDo5ox7Y65GCFAubpTRByy");


#[account]

pub struct User{
    pub owner: Pubkey,
    pub amount_staked: u64, 
    pub start_time: i64,
}

#[account]

pub struct Config {
    pub admin: Pubkey, 
    pub sum_money_staked: u64,
    pub percent_profit: u8,
    
}


#[program]
pub mod staking_project {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUser>, money: u64) -> Result<()> {
        let new_user = &mut ctx.accounts.user_account;
        new_user.owner = *ctx.accounts.user.key;
        new_user.amount_staked = money;
        new_user.start_time = 0;
        msg!("New user is: {:?}", new_user.owner);
        Ok(())
    }

    pub fn initialize_manager(ctx: Context<InitializeManager>, percent_profit_init: u8) -> Result<()> {
        let new_manager = &mut ctx.accounts.manager_account;
        new_manager.admin = *ctx.accounts.manager.key;
        new_manager.sum_money_staked = 0;
        new_manager.percent_profit =  percent_profit_init;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let manager = &mut ctx.accounts.manager_account;
        let user = &mut ctx.accounts.user_account;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                }
            ),
            amount,
        )?;
        user.amount_staked += amount;
        user.start_time = Clock::get()?.unix_timestamp;
        manager.sum_money_staked += amount;

        
        msg!("User  staked {} SOL", amount);
        Ok(())
    }

    
}



#[derive(Accounts)] 
pub struct InitializeUser<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 32  + 8, 
        seeds = [b"staking", user.key().as_ref()], 
        bump
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeManager<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 1 )]
    pub manager_account: Account<'info, Config>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {

    #[account(mut)]
     payer: Signer<'info>, 

    #[account(mut)]
     recipient: SystemAccount<'info>,

    #[account(mut)]
     pub user_account: Account<'info, User>,
      
    #[account(mut)]
     pub manager_account: Account<'info, Config>, 
     system_program: Program<'info, System>,
}







// #[error_code]
// pub enum CustomError {
//     #[msg("Not enough time has passed to claim rewards.")]
//     NotEnoughTimeStaked,
//     #[msg("No rewards available to claim.")]
//     NoRewardsAvailable,
// }
