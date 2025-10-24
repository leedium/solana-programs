use solana_program::entrypoint::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};

// teslls solana that this is the entrypoint for the program
entrypoint!(initialize_process);

// entrypoint function
fn initialize_process(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    msg!("Hello World");
    msg!("Our program's id is {}", program_id);
    Ok(())
}

#[cfg(test)]
mod tests;
