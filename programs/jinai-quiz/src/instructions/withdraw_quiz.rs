use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::quiz_account::QuizAccount;
use crate::state::quiz_status::QuizStatus;
use crate::error::QuizError;

#[derive(Accounts)]
pub struct WithdrawQuiz<'info> {
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
    pub player_token_accounts: UncheckedAccount<'info>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawQuiz<'info> {
    pub fn handler(ctx: Context<WithdrawQuiz>) -> Result<()> {
        // Quiz must not have started
        require!(
            ctx.accounts.quiz_account.status == QuizStatus::Recruiting || 
            ctx.accounts.quiz_account.status == QuizStatus::ReadyToStart,
            QuizError::InvalidQuizState
        );

        // Get the PDA seeds for signing
        let binding = ctx.accounts.quiz_account.key();
        let seeds = &[
            b"quiz-token-account",
            binding.as_ref(),
            &[ctx.bumps.quiz_token_account],
        ];
        let signer = &[&seeds[..]];

        // Process refunds to all players
        for (player_key, _) in ctx.accounts.quiz_account.player_scores.iter() {
            let refund_amount = ctx.accounts.quiz_account.bet_amount;
            
            let cpi_accounts = TransferChecked {
                from: ctx.accounts.quiz_token_account.to_account_info(),
                mint: ctx.accounts.quiz_mint.to_account_info(),
                to: ctx.accounts.player_token_accounts.to_account_info(),
                authority: ctx.accounts.quiz_account.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            );

            token_interface::transfer_checked(
                cpi_ctx,
                refund_amount,
                6  // Use appropriate decimals for your token
            )?;
        }

        // Update quiz status
        let quiz_account = &mut ctx.accounts.quiz_account;
        quiz_account.status = QuizStatus::Cancelled;
        
        msg!("Quiz cancelled and funds refunded");
        Ok(())
    }
}