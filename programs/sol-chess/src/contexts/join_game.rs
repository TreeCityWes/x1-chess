use crate::*;

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
}

impl<'info> JoinGame<'info> {
    pub fn process(&mut self, color: Color) -> Result<()> {
        let Self { user, game, .. } = self;

        // Check if the color is available
        require!(game.color_available(color), CustomError::ColorNotAvailable);
        
        // Check if the user is already in a game
        require!(user.not_in_game(), CustomError::AlreadyInGame);

        // Check if the game has a wager and the user has sufficient balance
        if game.has_wager() {
            let wager = game.get_wager();
            require!(user.has_sufficient(wager), CustomError::InsufficientBalance);
            
            // Lock the wager amount
            user.decrease_balance(wager);
        }

        // Update user and game state
        user.set_game(game.key());
        game.join_game(user.key(), color);

        // If both players have joined, start the game
        if game.is_full() {
            game.start_game();
        }

        Ok(())
    }
}
