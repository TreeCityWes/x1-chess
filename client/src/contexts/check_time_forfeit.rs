use crate::*;

pub fn check_time_forfeit(
    client: &Client,
    user: Pubkey,
    adversary_user: Pubkey,
    game: Pubkey,
) -> ClientResult<()> {
    let check_time_forfeit_ix = Instruction {
        program_id: sol_chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(adversary_user, false),
            AccountMeta::new(game, false),
            AccountMeta::new_readonly(clock::ID, false),
        ],
        data: sol_chess::instruction::CheckTimeForfeit {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [check_time_forfeit_ix].to_vec(),
        None,
        "check_time_forfeit".to_string(),
    )?;

    Ok(())
} 