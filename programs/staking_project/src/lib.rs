use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("55d8NRKMvawmEUSbU6Sw52JDo5ox7Y65GCFAubpTRByy");

#[program]
pub mod staking_project {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUser>, money: u64) -> Result<()> {
        let new_user = &mut ctx.accounts.user_account;
        new_user.owner = *ctx.accounts.user.key;
        new_user.amount_staked = money;
        new_user.start_time = Clock::get()?.unix_timestamp;
        msg!("New user is: {}", new_user.owner);
        Ok(())
    }

    pub fn initialize_manager(ctx: Context<InitializeList>) -> Result<()> {
        let new_manager = &mut ctx.accounts.manager_account;
        new_manager.total_staked = 0;
        new_manager.stakers = [Pubkey::default(); 20];
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let manager = &mut ctx.accounts.manager_account;
        let user = &mut ctx.accounts.user_account;

        user.amount_staked += amount;
        user.start_time = Clock::get()?.unix_timestamp;

        manager.total_staked += amount;

        if !manager.stakers.contains(&ctx.accounts.user.key()) {
            for i in 0..manager.stakers.len() {
                if manager.stakers[i] == Pubkey::default() {
                    manager.stakers[i] = ctx.accounts.user.key();
                    break;
                }
            }
        }
        
    
        msg!("User {} staked {} SOL", ctx.accounts.user.key(), amount);
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let manager = &mut ctx.accounts.manager_account;
        let user = &mut ctx.accounts.user_account;

        let current_time = Clock::get()?.unix_timestamp;

        let time_staked = current_time - user.start_time;
        require!(time_staked > 0, CustomError::NotEnoughTimeStaked);

        let apr: f64 = 0.05;
        let reward = (user.amount_staked as f64) * apr * (time_staked as f64 / 31_536_000.0);
        let reward_lamports = reward as u64;
    
        require!(reward_lamports > 0, CustomError::NoRewardsAvailable);
    
        // Cập nhật số dư trong smart contract
        manager.total_staked += reward_lamports;
    
        // Gửi thưởng cho người chơi
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += reward_lamports;
        **ctx.accounts.program_account.to_account_info().try_borrow_mut_lamports()? -= reward_lamports;
    
        // Reset thời gian stake để tránh farm reward
        user.start_time = current_time;
    
        msg!(
            "User {} claimed {} SOL as reward",
            ctx.accounts.user.key(),
            reward_lamports
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeList<'info> {
    #[account(init, payer = manager, space = 8 + 4 + 32*20 )]
    pub manager_account: Account<'info, GlobalStateAccount>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub manager_account: Account<'info, GlobalStateAccount>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>, 
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
pub struct InitializeUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
    pub struct ClaimReward<'info> {
    #[account(mut)]
    pub manager_account: Account<'info, GlobalStateAccount>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>, 
    pub system_program: Program<'info, System>,
    }

#[account]

pub struct User{
    pub owner: Pubkey,
    pub amount_staked: u64, 
    pub start_time: i64,
}

#[account]

pub struct GlobalStateAccount {
    pub total_staked: u64,
    pub stakers: [Pubkey; 20], 
}

#[error_code]
pub enum CustomError {
    #[msg("Not enough time has passed to claim rewards.")]
    NotEnoughTimeStaked,
    #[msg("No rewards available to claim.")]
    NoRewardsAvailable,
}
