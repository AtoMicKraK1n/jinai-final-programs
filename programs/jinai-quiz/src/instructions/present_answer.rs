use anchor_lang::prelude::*;
use crate::state::quiz_account::QuizAccount;
use crate::state::quiz_status::QuizStatus;
use crate::error::QuizError;

#[derive(Accounts)]
pub struct PresentAnswer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(
        mut,
        constraint = quiz_account.players.contains(&player.key()) @ QuizError::PlayerNotRegistered
    )]
    pub quiz_account: Account<'info, QuizAccount>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> PresentAnswer<'info> {
    pub fn handler(
        ctx: Context<PresentAnswer>,
        answer_index: u8,
        timestamp: i64,
    ) -> Result<()> {
        let quiz_account = &mut ctx.accounts.quiz_account;
        let player = &ctx.accounts.player;
        
        // Quiz must be in progress
        require!(
            quiz_account.status == QuizStatus::InProgress,
            QuizError::InvalidQuizState
        );
        
        // Check if current round is valid
        require!(
            quiz_account.current_round > 0 && 
            quiz_account.current_round <= quiz_account.num_questions as u16,
            QuizError::InvalidRound
        );
        
        // Get current question
        let question_index = (quiz_account.current_round - 1) as usize;
        let current_question = &quiz_account.round_questions[question_index];
        
        // Check if the answer is valid
        require!(
            answer_index < 4, // Assuming 4 options per question
            QuizError::InvalidAnswer
        );
        
        // Check if the answer is within time limit using Clock sysvar
        let clock = Clock::get()?; // Using Clock directly as shown in [(1)](https://solana.com/developers/cookbook/programs/clock)
        let current_time = clock.unix_timestamp;
        
        require!(
            timestamp <= current_time && 
            current_time - timestamp <= quiz_account.time_limit_per_question as i64,
            QuizError::TimeExpired
        );
        
        // Award points if correct
        if answer_index == current_question.correct_answer_index {
            let player_key = player.key();
            if let Some(score) = quiz_account.player_scores.iter_mut().find(|(k, _)| *k == player_key) {
                score.1 = score.1.checked_add(1).ok_or(QuizError::Overflow)?;
                msg!("Player {} answered correctly! New score: {}", player_key, score.1);
            }
        } else {
            msg!("Player {} answered incorrectly", player.key());
        }
        
        // Move to next round
        quiz_account.current_round = quiz_account.current_round
            .checked_add(1)
            .ok_or(QuizError::Overflow)?;
        
        // If all rounds completed, finalize the quiz
        if quiz_account.current_round > quiz_account.num_questions as u16 {
            quiz_account.status = QuizStatus::Completed;
            msg!("Quiz completed! Calculating final results...");
        }
        
        Ok(())
    }
}