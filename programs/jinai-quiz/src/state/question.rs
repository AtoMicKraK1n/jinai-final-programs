use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct QuizQuestion {
    pub question_text: String,
    pub options: [String; 4],
    pub correct_answer_index: u8,
}

impl QuizQuestion {
    pub const SIZE: usize = 
        4 + 256 +          // question_text (assume max 256 chars)
        (4 + 128) * 4 +    // options (assume max 128 chars each)
        1;                 // correct_answer_index
}