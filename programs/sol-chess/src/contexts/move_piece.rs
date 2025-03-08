use crate::*;

#[derive(Accounts)]
pub struct MovePiece<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut, address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> MovePiece<'info> {
    pub fn process(&mut self, from: Square, to: Square) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            clock,
            ..
        } = self;
        let color = game.get_current_player_color();

<<<<<<< HEAD
        // Check if the game is already over
        require!(
            game.game_state.is_still_going(),
            CustomError::GameAlreadyOver
        );

        // Check if the player has run out of time
        if game.time_control.is_not_first_move() && !game.has_time(color, clock.unix_timestamp) {
            // Player has run out of time, declare the opponent as winner
            game.set_winner(color.get_opposite());
            
            // Handle wager if the game has one
            if game.has_wager() {
                let winner_pubkey = game.get_player_pubkey(color.get_opposite());
                if user.key() == winner_pubkey {
                    // User is the winner
                    user.increase_balance(game.get_wager() * 2);
                } else {
                    // Adversary is the winner
                    adversary_user.increase_balance(game.get_wager() * 2);
                }
            }
            
            // Update Elo ratings if the game is rated
            if game.is_rated() {
                let winner_pubkey = game.get_player_pubkey(color.get_opposite());
                if user.key() == winner_pubkey {
                    // User is the winner
                    user.won_against(adversary_user.get_elo());
                    adversary_user.lost_against(user.get_elo());
                } else {
                    // Adversary is the winner
                    adversary_user.won_against(user.get_elo());
                    user.lost_against(adversary_user.get_elo());
                }
            }
            
            return Err(CustomError::TimeHasRunOut.into());
        }

=======
        require!(
            game.has_time(color, clock.unix_timestamp),
            CustomError::TimeHasRunOut
        );

>>>>>>> 012776b1ce9a1e8c7c9a0ef15c03446655027bd0
        require!(
            user.key() == game.get_current_player_pubkey(),
            CustomError::NotUsersTurn
        );

        require!(
            game.get_adversary_player(color).eq(&adversary_user.key()),
            CustomError::InvalidAdversaryUserAccount
        );

        require!(
            game.is_valid_move(color, from, to),
            CustomError::InvalidMove
        );

        game.move_piece(color, from, to);

        require!(game.not_in_check(color), CustomError::KingInCheck);

        game.next_turn();

        game.reset_draw_state();

        if game.in_checkmate(color.get_opposite()) {
            game.set_winner(color);
            if game.has_wager() {
<<<<<<< HEAD
                user.increase_balance(game.get_wager() * 2);
=======
                user.increase_balance(game.get_wager() * 2)
>>>>>>> 012776b1ce9a1e8c7c9a0ef15c03446655027bd0
            }

            if game.is_rated() {
                user.won_against(adversary_user.get_elo());
                adversary_user.lost_against(user.get_elo());
            }
        }

<<<<<<< HEAD
        // Update time control with the current timestamp
=======
>>>>>>> 012776b1ce9a1e8c7c9a0ef15c03446655027bd0
        game.update_time_control(color, clock.unix_timestamp);

        Ok(())
    }
}
