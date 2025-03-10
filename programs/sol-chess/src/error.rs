use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User Already In Game")]
    UserAlreadyInGame,
    #[msg("Color Not Available")]
    ColorNotAvailable,
    #[msg("Invalid Game State")]
    InvalidGameState,
    #[msg("Not User's Turn")]
    NotUsersTurn,
    #[msg("Invalid Move")]
    InvalidMove,
    #[msg("King in Check")]
    KingInCheck,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
    #[msg("Not In Game")]
    NotInGame,
    #[msg("Game Already Started")]
    GameAlreadyStarted,
    #[msg("Invalid Adversary User Account")]
    InvalidAdversaryUserAccount,
    #[msg("User Already In Game")]
    AlreadyInGame,
    #[msg("Already Offered Draw")]
    AlreadyOfferedDraw,
<<<<<<< HEAD
    #[msg("Time Has Run Out")]
    TimeHasRunOut,
    #[msg("Time Has Not Run Out")]
    TimeHasNotRunOut,
    #[msg("Not Adversary's Turn")]
    NotAdversaryTurn,
    #[msg("Game Already Over")]
    GameAlreadyOver,
=======
    #[msg("TimeHasRunOut")]
    TimeHasRunOut,
>>>>>>> 012776b1ce9a1e8c7c9a0ef15c03446655027bd0
}
