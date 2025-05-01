use anchor_lang::prelude::*;

#[error_code]
pub enum QuizError {
    #[msg("The quiz is in an invalid state for this operation")]
    InvalidQuizState,
    
    #[msg("Player has already joined this quiz")]
    PlayerAlreadyJoined,
    
    #[msg("The quiz is already full")]
    QuizFull,
    
    #[msg("Player is not registered for this quiz")]
    PlayerNotRegistered,
    
    #[msg("Invalid round number")]
    InvalidRound,
    
    #[msg("Invalid answer selection")]
    InvalidAnswer,
    
    #[msg("Time has expired for submitting an answer")]
    TimeExpired,
    
    #[msg("Not enough players to start the quiz")]
    NotEnoughPlayers,
    
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    
    #[msg("The number of questions does not match the expected count")]
    InvalidQuestionCount,

    #[msg("Overflow")]
    Overflow,
}