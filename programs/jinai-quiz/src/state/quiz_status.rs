use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Default)]
pub enum QuizStatus {
    #[default]
    Recruiting,    // Quiz is accepting players
    ReadyToStart,  // Quiz has enough players and can be started
    InProgress,    // Quiz is currently running
    Completed,     // All questions have been answered
    Finalized,     // Rewards distributed
    Cancelled,     // Quiz was cancelled before completion
}