use crate::error::LoanContractError;
use crate::instruction::LoanInstruction;
use crate::state::{LoanContractState, ContractItemState, ContractStatus};
use borsh::BorshSerialize;
use solana_program::{account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, program_pack::IsInitialized, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = LoanInstruction::unpack(instruction_data)?;
    match instruction {
        LoanInstruction::AddItem { name } => {
            add_item(program_id, accounts, name)
        },
        LoanInstruction::SignContract { deposit } => {
            sign_contract(program_id, accounts, deposit)
        },
        LoanInstruction::CompleteContract { } => {
            complete_contract(program_id, accounts)
        },
        LoanInstruction::TerminateContract { } => {
            terminate_contract(program_id, accounts)
        }
    }
}
pub fn add_item(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let owner_account = next_account_info(account_info_iter)?;
    let contract_item_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    //TODO: добавить валидации
    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    let (calculated_contract_item_pubkey, bump_seed) = Pubkey::find_program_address(&[owner_account.key.as_ref(), name.as_ref()],  program_id);

    invoke_signed(
        &system_instruction::create_account(
            owner_account.key,
            contract_item_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            owner_account.clone(),
            contract_item_account.clone(),
            system_program.clone(),
        ],
        &[&[owner_account.key.as_ref(), name.as_ref(), &[bump_seed]]],
    )?;

    let mut contract_item_data = try_from_slice_unchecked::<ContractItemState>(&contract_item_account.data.borrow()).unwrap();

    contract_item_data.owner = *owner_account.key;
    contract_item_data.name = name;
    contract_item_data.user = None;
    contract_item_data.is_initialized = true;

    contract_item_data.serialize(&mut &mut contract_item_account.data.borrow_mut()[..])?;

    Ok(())

}

pub fn sign_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    deposit: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let borrower_account = next_account_info(account_info_iter)?;
    let contract_account = next_account_info(account_info_iter)?;

    let lender_account = next_account_info(account_info_iter)?;
    let item_contract_account = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;
    // // let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    // // if !rent.is_exempt(borrower_account.lamports(), borrower_account.data_len()) {
    // // }
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;


    //TODO: добавить валидации
    let account_len: usize = 1000;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);


    let (calculated_contract_pubkey, bump_seed) = Pubkey::find_program_address(&[
        borrower_account.key.as_ref(),
        lender_account.key.as_ref(),
        item_contract_account.key.as_ref()
    ],  program_id);

    invoke_signed(
        &system_instruction::create_account(
            borrower_account.key,
            contract_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            borrower_account.clone(),
            contract_account.clone(),
        ],
        &[&[
            borrower_account.key.as_ref(),
            lender_account.key.as_ref(),
            item_contract_account.key.as_ref(), &[bump_seed]]],
    )?;


    let mut contract_data = try_from_slice_unchecked::<LoanContractState>(&contract_account.data.borrow()).unwrap();

    contract_data.borrower = *borrower_account.key;
    contract_data.lender = *lender_account.key;
    contract_data.item = *item_contract_account.key;
    contract_data.escrow_account = *escrow_account.key;
    contract_data.expected_amount = deposit;
    contract_data.status = ContractStatus::Active.into();

    let mut contract_item_data = try_from_slice_unchecked::<ContractItemState>(&item_contract_account.data.borrow()).unwrap();
    contract_item_data.user = Some(*borrower_account.key);

    contract_data.serialize(&mut &mut contract_account.data.borrow_mut()[..])?;
    contract_item_data.serialize(&mut &mut item_contract_account.data.borrow_mut()[..])?;

    msg!("signed contract successfully" );

    Ok(())
}


pub fn complete_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let lender_account = next_account_info(account_info_iter)?;

    let contract_account = next_account_info(account_info_iter)?;
    let contract_item_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Проверка, что contract_account принадлежит программе
    if contract_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut contract_item_data = try_from_slice_unchecked::<ContractItemState>(&contract_item_account.data.borrow()).unwrap();
    contract_item_data.user = None;

    let mut contract_data = try_from_slice_unchecked::<LoanContractState>(&contract_account.data.borrow()).unwrap();

    contract_data.status = ContractStatus::Closed.into();

    contract_data.serialize(&mut &mut contract_account.data.borrow_mut()[..])?;
    contract_data.serialize(&mut &mut contract_item_account.data.borrow_mut()[..])?;

    msg!("complete contract successfully" );

    Ok(())
}

pub fn terminate_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let lender_account = next_account_info(account_info_iter)?;

    let contract_account = next_account_info(account_info_iter)?;
    let contract_item_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Проверка, что contract_account принадлежит программе
    if contract_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut contract_item_data = try_from_slice_unchecked::<ContractItemState>(&contract_item_account.data.borrow()).unwrap();
    contract_item_data.user = None;

    let mut contract_data = try_from_slice_unchecked::<LoanContractState>(&contract_account.data.borrow()).unwrap();

    contract_data.status = ContractStatus::Closed.into();

    contract_data.serialize(&mut &mut contract_account.data.borrow_mut()[..])?;
    contract_data.serialize(&mut &mut contract_item_account.data.borrow_mut()[..])?;

    msg!("terminate contract successfully" );

    Ok(())
}

