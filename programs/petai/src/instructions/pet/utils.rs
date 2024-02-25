pub use anchor_lang::prelude::*;
use anchor_lang::{solana_program::{entrypoint::ProgramResult, instruction::Instruction, native_token::LAMPORTS_PER_SOL}, InstructionData};
use anchor_spl::{metadata::{Metadata as MetadataProgram, MetadataAccount}, token::Mint};


use crate::{constants::{PLAYER_CLOCKWORK_FEE_IN_SOL, PLAYER_STATE_CRON_SHEDULER, PROGRAM_STATE_SEED}, PetState, PlayerState, ProgramState, ID};

pub struct StartPetUpdateCronThreadAccounts<'info> {
    pub player_state: Account<'info, PlayerState>,
    pub pet_state: Account<'info, PetState>,
    pub state: Account<'info, ProgramState>,
    pub pet_nft_mint: Account<'info, Mint>,
    pub metadata_account: Account<'info, MetadataAccount>,
    pub initializer: Signer<'info>,
    pub thread: SystemAccount<'info>,
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
    pub effects_metas: Vec<AccountMeta>
}


#[inline(never)]
pub fn start_pet_update_cron_tread(accounts: &mut StartPetUpdateCronThreadAccounts, thread_id: &Vec<u8>) -> ProgramResult {
    let mut accounts_metas = crate::accounts::UpdatePetStateCron {
        player_state: accounts.player_state.key(),
        pet_state: accounts.pet_state.key(),
        state: accounts.state.key(),
        thread: accounts.thread.key(),
        pet_nft_mint: accounts.pet_nft_mint.key(),
        metadata_program: accounts.metadata_program.key(),
        system_program: accounts.system_program.key(),
    }
    .to_account_metas(Some(true));

    accounts_metas.append(&mut accounts.effects_metas);

    let target_ix = Instruction {
        program_id: ID,
        accounts: accounts_metas,
        data: crate::instruction::UpdatePetStateCron {
            player_id: accounts.initializer.key()
        }.data(),
    };

    let trigger = clockwork_sdk::state::Trigger::Cron {
        schedule: PLAYER_STATE_CRON_SHEDULER.into(),
        skippable: true,
    };

    clockwork_sdk::cpi::thread_create(
        CpiContext::new_with_signer(
            accounts.clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadCreate {
                payer: accounts.initializer.to_account_info(),
                system_program: accounts.system_program.to_account_info(),
                thread: accounts.thread.to_account_info(),
                authority: accounts.state.to_account_info(),
            },
            &[&[PROGRAM_STATE_SEED.as_bytes(), &[accounts.state.bump]]],
        ),
        (PLAYER_CLOCKWORK_FEE_IN_SOL * LAMPORTS_PER_SOL as f64) as u64,       // user clockwork fee
        thread_id.clone(),              // id
        vec![target_ix.into()], // instructions
        trigger,                // trigger
    )?;

    return Ok(());
}