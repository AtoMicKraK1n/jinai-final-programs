use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::QuizAccount;
use crate::state::QuizStatus;

#[derive(Accounts)]
#[instruction(bet_amount: u64, num_questions: u8, quiz_topic: String, time_limit_per_question: u32)]
pub struct AppointQuiz<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    
    #[account(
        init,
        payer = host,
        space = QuizAccount::calculate_size(&quiz_topic, num_questions),
        seeds = [b"jinai-quiz", host.key().as_ref()],
        bump
    )]
    pub quiz_account: Account<'info, QuizAccount>,
    
    #[account(
        init,
        payer = host,
        seeds = [b"quiz-token-account", quiz_account.key().as_ref()],
        bump,
        token::mint = quiz_mint,
        token::authority = quiz_account,
        token::token_program = token_program,
    )]
    pub quiz_token_account: InterfaceAccount<'info, TokenAccount>,
    pub quiz_mint: InterfaceAccount<'info, Mint>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> AppointQuiz<'info> {
    pub fn handler(
        ctx: Context<AppointQuiz>,
        bet_amount: u64,
        num_questions: u8,
        quiz_topic: String,
        time_limit_per_question: u32,
    ) -> Result<()> {
        let quiz_account = &mut ctx.accounts.quiz_account;
        
        quiz_account.host = ctx.accounts.host.key();
        quiz_account.bet_amount = bet_amount;
        quiz_account.num_questions = num_questions;
        quiz_account.quiz_topic = quiz_topic;
        quiz_account.time_limit_per_question = time_limit_per_question;
        quiz_account.status = QuizStatus::Recruiting;
        quiz_account.players = vec![];
        quiz_account.current_round = 0;
        quiz_account.round_questions = vec![];
        quiz_account.player_scores = vec![];
        quiz_account.pool_amount = 0;
        quiz_account.bump = ctx.bumps.quiz_account;

        msg!("Quiz session initialized with ID: {}", quiz_account.key());
        Ok(())
    }
}