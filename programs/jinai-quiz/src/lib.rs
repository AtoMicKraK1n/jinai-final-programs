use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use state::*;
use error::*;
use instructions::*;

declare_id!("Dfvg9Ve4mDkPEsAEkZi44r4Axns1sdHMZQoKNBsJzjYJ");

#[program]
pub mod quiz_program {
    use super::*;

    pub fn appoint_quiz(
        ctx: Context<AppointQuiz>,
        bet_amount: u64, 
        num_questions: u8, 
        quiz_topic: String, 
        time_limit_per_question: u32
    ) -> Result<()> {
        instructions::appoint_quiz::AppointQuiz::handler(
            ctx, 
            bet_amount, 
            num_questions, 
            quiz_topic, 
            time_limit_per_question
        )?;
        Ok(())
    }

    pub fn connect_players(ctx: Context<ConnectPlayers>) -> Result<()> {
        instructions::connect_players::ConnectPlayers::handler(ctx)?;
        Ok(())
    }

    pub fn initiate_quiz(ctx: Context<InitiateQuiz>, questions: Vec<QuizQuestion>) -> Result<()> {
        instructions::initiate_quiz::InitiateQuiz::handler(
            ctx,
            questions
        )?;
        Ok(())
    }

    pub fn present_answer(ctx: Context<PresentAnswer>, answer_index: u8, timestamp: i64) -> Result<()> {
        instructions::present_answer::PresentAnswer::handler(
            ctx,
            answer_index, 
            timestamp
        )?;
        Ok(())
    }

    pub fn scatter_rewards(ctx: Context<ScatterRewards>) -> Result<()> {
        instructions::scatter_rewards::ScatterRewards::handler(ctx)?;
        Ok(())
    }

    pub fn withdraw_quiz(ctx: Context<WithdrawQuiz>) -> Result<()> {
        instructions::withdraw_quiz::WithdrawQuiz::handler(ctx)?;
        Ok(())
    }
}