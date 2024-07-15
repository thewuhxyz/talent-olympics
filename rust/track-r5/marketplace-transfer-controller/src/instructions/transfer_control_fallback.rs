use anchor_lang::prelude::*;
use spl_transfer_hook_interface::instruction::TransferHookInstruction;

use crate::__private;

pub fn transfer_control_fallback<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> Result<()> {
    let instruction = TransferHookInstruction::unpack(data)?;
    // match instruction discriminator to transfer hook interface execute instruction
    // token2022 program CPIs this instruction on token transfer
    match instruction {
        TransferHookInstruction::Execute { amount } => {
            let amount_bytes = amount.to_le_bytes();

            // invoke custom transfer hook instruction on our program
            __private::__global::transfer_control(program_id, accounts, &amount_bytes)?
        }
        _ => return Err(ProgramError::InvalidInstructionData.into()),
    }

    Ok(())
}
