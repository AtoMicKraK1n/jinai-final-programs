use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::quiz_account::QuizAccount;
use crate::state::quiz_status::QuizStatus;
use crate::error::QuizError;

#[derive(Accounts)]
pub struct ConnectPlayers<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub quiz_account: Account<'info, QuizAccount>,
    
    #[account(
        mut,
        token::mint = quiz_mint,
        token::authority = player
    )]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"quiz-token-account", quiz_account.key().as_ref()],
        bump,
    )]
    pub quiz_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub quiz_mint: InterfaceAccount<'info, token_interface::Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ConnectPlayers<'info> {
    pub fn handler(ctx: Context<ConnectPlayers>) -> Result<()> {
        let quiz_account = &mut ctx.accounts.quiz_account;
        let player = &ctx.accounts.player;
        
        // Ensure quiz is in recruiting state
        require!(
            quiz_account.status == QuizStatus::Recruiting,
            QuizError::InvalidQuizState
        );
        
        // Check if player has already joined
        require!(
            !quiz_account.players.contains(&player.key()),
            QuizError::PlayerAlreadyJoined
        );
        
        // Check if quiz is full (4 players maximum)
        require!(
            quiz_account.players.len() < 4,
            QuizError::QuizFull
        );

        // Transfer the bet amount using transfer_checked
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.player_token_account.to_account_info(),
            mint: ctx.accounts.quiz_mint.to_account_info(),
            to: ctx.accounts.quiz_token_account.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        
        token_interface::transfer_checked(
            cpi_context,
            quiz_account.bet_amount,
            ctx.accounts.quiz_mint.decimals
        )?;
        
        // Add player to quiz and initialize their score
        quiz_account.players.push(player.key());
        quiz_account.player_scores.push((player.key(), 0));
        quiz_account.pool_amount = quiz_account.pool_amount.checked_add(quiz_account.bet_amount)
            .ok_or(QuizError::Overflow)?;
        
        // If we now have 4 players, update quiz status
        if quiz_account.players.len() == 4 {
            quiz_account.status = QuizStatus::ReadyToStart;
        }
        
        msg!("Player {} joined quiz session", player.key());
        Ok(())
    }
}