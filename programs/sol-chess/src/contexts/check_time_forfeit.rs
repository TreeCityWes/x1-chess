use crate::*;

#[derive(Accounts)]
pub struct CheckTimeForfeit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CheckTimeForfeit<'info> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            clock,
            ..
        } = self;

        // Check if the game is still in progress
        require!(game.is_still_going(), CustomError::InvalidGameState);
        
        // Check if the user is in the game
        require!(game.is_in_game(user.key()), CustomError::NotInGame);
        
        // Get the color of the current player
        let current_color = game.get_current_player_color();
        
        // Get the color of the user
        let user_color = game.get_player_color(user.key());
        
        // Check if it's the adversary's turn (user can only claim time forfeit on opponent's turn)
        require!(current_color != user_color, CustomError::NotAdversaryTurn);
        
        // Check if the adversary is the correct account
        require!(
            game.get_adversary_player(user_color).eq(&adversary_user.key()),
            CustomError::InvalidAdversaryUserAccount
        );
        
        // Check if the time has run out for the current player
        require!(
            game.time_control.is_not_first_move() && 
            !game.has_time(current_color, clock.unix_timestamp),
            CustomError::TimeHasNotRunOut
        );
        
        // Set the user as the winner
        game.set_winner(user_color);
        
        // Handle wager if the game has one
        if game.has_wager() {
            user.increase_balance(game.get_wager() * 2);
        }
        
        // Update Elo ratings if the game is rated
        if game.is_rated() {
            user.won_against(adversary_user.get_elo());
            adversary_user.lost_against(user.get_elo());
        }
        
        Ok(())
    }
} 