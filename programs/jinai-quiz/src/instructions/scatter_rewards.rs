use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::quiz_account::QuizAccount;
use crate::state::quiz_status::QuizStatus;
use crate::error::QuizError;

#[derive(Accounts)]
pub struct ScatterRewards<'info> {
    #[account(
        mut,
        has_one = host @ QuizError::Unauthorized,
    )]
    pub quiz_account: Account<'info, QuizAccount>,

    pub host: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"quiz-token-account", quiz_account.key().as_ref()],
        bump,
    )]
    pub quiz_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub quiz_mint: InterfaceAccount<'info, token_interface::Mint>,

    /// CHECK: Token accounts will be checked in handler
    #[account(mut)]
    pub winner_token_accounts: UncheckedAccount<'info>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ScatterRewards<'info> {
    pub fn handler(ctx: Context<ScatterRewards>) -> Result<()> {
        // Quiz must be completed
        require!(
            ctx.accounts.quiz_account.status == QuizStatus::Completed,
            QuizError::InvalidQuizState
        );
        
        // Get final scores and ranks
        let mut player_rankings: Vec<(Pubkey, u32)> = ctx.accounts.quiz_account.player_scores
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        
        // Sort by score in descending order
        player_rankings.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Calculate reward distribution based on position
        let total_pool = ctx.accounts.quiz_account.pool_amount;
        let mut rewards = vec![];
        
        let distribution = [50, 30, 15, 5]; // Percentages for 1st, 2nd, 3rd, 4th
        
        for (i, (player_key, _)) in player_rankings.iter().enumerate() {
            if i < distribution.len() {
                let reward_amount = (total_pool * distribution[i] as u64) / 100;
                rewards.push((*player_key, reward_amount));
            }
        }

        // Store the seeds for later use
        let binding = ctx.accounts.quiz_account.key();
        let seeds = &[
            b"quiz-token-account",
            binding.as_ref(),
            &[ctx.bumps.quiz_token_account],
        ];
        let signer = &[&seeds[..]];

        // Distribute rewards using transfer_checked
        for (player_key, reward_amount) in rewards {
            let cpi_accounts = TransferChecked {
                from: ctx.accounts.quiz_token_account.to_account_info(),
                mint: ctx.accounts.quiz_mint.to_account_info(),
                to: ctx.accounts.winner_token_accounts.to_account_info(),
                authority: ctx.accounts.quiz_account.to_account_info(), 
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            );

            token_interface::transfer_checked(
                cpi_ctx,
                reward_amount,
                6 // Using standard 6 decimals, adjust based on your token's decimals
            )?;
        }
        
        // Mark quiz as finalized after all transfers
        let quiz_account = &mut ctx.accounts.quiz_account;
        quiz_account.status = QuizStatus::Finalized;
        
        msg!("Rewards distributed successfully");
        Ok(())
    }
}