use anchor_lang::prelude::*;
use crate::state::question::QuizQuestion;
use crate::state::quiz_status::QuizStatus;

#[account]
#[derive(Default)]
pub struct QuizAccount {
    pub host: Pubkey,                        // Creator of the quiz
    pub bet_amount: u64,                     // Amount each player must bet
    pub num_questions: u8,                   // Number of questions in the quiz
    pub quiz_topic: String,                  // Topic of the quiz
    pub time_limit_per_question: u32,        // Time limit for each question in seconds
    pub status: QuizStatus,                  // Current status of the quiz
    pub players: Vec<Pubkey>,                // List of players who have joined
    pub current_round: u16,                  // Current round/question number
    pub round_questions: Vec<QuizQuestion>,  // List of questions
    pub player_scores: Vec<(Pubkey, u32)>, // Map of player scores
    pub pool_amount: u64,                    // Total amount in the pool
    pub bump: u8,                            // PDA bump
}

impl QuizAccount {
    pub fn calculate_size(quiz_topic: &str, num_questions: u8) -> usize {
        8 +                             // Discriminator
        32 +                            // host: Pubkey
        8 +                             // bet_amount: u64
        1 +                             // num_questions: u8
        4 + quiz_topic.len() +          // quiz_topic: String
        4 +                             // time_limit_per_question: u32
        1 +                             // status: QuizStatus
        4 + (32 * 4) +                  // players: Vec<Pubkey> (max 4 players)
        2 +                             // current_round: u16
        4 + (num_questions as usize * QuizQuestion::SIZE) + // round_questions
        (4 + (32 + 4) * 4) +            // player_scores: BTreeMap (max 4 entries)
        8 +                             // pool_amount: u64
        1                               // bump: u8
    }
}