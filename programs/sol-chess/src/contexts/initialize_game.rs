use crate::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(init,payer=payer,space=8 + size_of::<Game>(), seeds=[SEED_GAME,user.key().as_ref(),&user.games.to_be_bytes()], bump)]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> InitializeGame<'info> {
    pub fn process(&mut self, game_config: GameConfig, game_bump: u8) -> Result<()> {
        let InitializeGame {
            game, user, clock, ..
        } = self;

<<<<<<< HEAD
        // If the game has a wager, check if the user has sufficient balance
        if game_config.has_wager() {
            require!(
                user.has_sufficient(game_config.get_wager()),
                CustomError::InsufficientBalance
            );
            
            // Lock the wager amount by decreasing the user's balance
            user.decrease_balance(game_config.get_wager());
        }

=======
>>>>>>> 012776b1ce9a1e8c7c9a0ef15c03446655027bd0
        game.new(
            game_config,
            clock.unix_timestamp,
            user.key(),
            user.games,
            game_bump,
        )?;
        user.increment_games();
        Ok(())
    }
}
