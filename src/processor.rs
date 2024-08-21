use crate::error::LoanContractError;
use crate::instruction::LoanInstruction;
use crate::state::{LoanContractState, ContractItemState, ContractStatus};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let instruction = LoanInstruction::unpack(instruction_data)?;
    match instruction {
        LoanInstruction::SignContract { deposit } => {
            sign_contract(program_id, accounts, deposit)
        },
        LoanInstruction::AddItem { name } => {
            add_item(program_id, accounts, name)
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

    // msg!("Signing Contract...");
    // msg!("deposit: {}", deposit);
    //
    //
    // let borrower_account = next_account_info(account_info_iter)?;
    // let lender_account = next_account_info(account_info_iter)?;
    // let pda_item_account = next_account_info(account_info_iter)?;
    // let contract_account = next_account_info(account_info_iter)?;
    // let system_program = next_account_info(account_info_iter)?;
    //
    // // let contract_watcher_account = next_account_info(account_info_iter)?;
    //
    // let (pda_calucated_key, bump_seed) = Pubkey::find_program_address(&[lender_account.key.as_ref(), item_id.as_bytes().as_ref()],  program_id);
    //
    // let mut item_data =
    //     try_from_slice_unchecked::<ContractItemState>(&pda_item_account.data.borrow()).unwrap();
    //
    // if(item_data.owner != *lender_account.key) {
    //     msg!("Invalid seeds for PDA");
    //     return Err(LoanContractError::InvalidOwnerItem.into());
    // }
    //
    // let (pda_contract_calculated_key, bump_seed) = Pubkey::find_program_address(&[
    //     lender_account.key.as_ref(),
    //     borrower_account.key.as_ref(),
    //     item_id.as_bytes().as_ref()],
    //                                                                             program_id);
    //
    //
    //
    // let mut contract_data =
    //     try_from_slice_unchecked::<LoanContractState>(&contract_account.data.borrow()).unwrap();
    //
    // let account_len: usize = 1000;
    //
    // let rent = Rent::get()?;
    // let rent_lamports = rent.minimum_balance(account_len);
    //
    //
    //
    // invoke_signed(
    //     &system_instruction::create_account(
    //         borrower_account.key,
    //         lender_account.key,
    //         rent_lamports,
    //         account_len.try_into().unwrap(),
    //         program_id,
    //     ),
    //     &[
    //         borrower_account.clone(),
    //         lender_account.clone(),
    //         system_program.clone(),
    //     ],
    //     &[&[borrower_account.key.as_ref(), lender_account.key.as_ref(), pda_item_account.key.as_ref(), &[bump_seed]]],
    // )?;
    //
    //
    //
    // contract_data.lender = *lender_account.key;
    // contract_data.borrower = *borrower_account.key;
    // contract_data.deposit = deposit.clone();
    // contract_data.item = *pda_item_account.key;
    // contract_data.status = ContractStatus::Active.into();
    //
    // contract_data.serialize(&mut &mut contract_account.data.borrow_mut()[..])?;
    //
    // // let mut contract_watcher_data =
    // //     try_from_slice_unchecked::<ContractWatcher>(&contract_watcher_account.data.borrow()).unwrap();



    Ok(())
}
pub fn complete_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    todo!();
    Ok(())
}
pub fn terminate_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    todo!();
    Ok(())
}


// //TODO: не initializer, а user_account, или норм? в другом вызове
// let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), name.as_ref()],  program_id);
//
// if pda != *user_account.key {
//     msg!("Invalid seeds for PDA");
//     return Err(LoanContractError::InvalidPDA.into());
// }
//
// let account_len: usize = 1000;
//
// if ContractItemState::get_account_size(name.clone()) > account_len {
//     msg!("Data length is larger than 1000 bytes");
//     return Err(LoanContractError::InvalidDataLength.into());
// }
//
// let rent = Rent::get()?;
// let rent_lamports = rent.minimum_balance(account_len);
//
// invoke_signed(
//     &system_instruction::create_account(
//         initializer.key, // payer
//         user_account.key, // new_account_pda
//         rent_lamports,
//         account_len.try_into().unwrap(),
//         program_id,
//     ),
//     &[
//         initializer.clone(),
//         user_account.clone(),
//         system_program.clone(),
//     ],
//     &[&[initializer.key.as_ref(), name.as_ref(), &[bump_seed]]],
// )?;
//
// let mut account_data = try_from_slice_unchecked::<ContractItemState>(&user_account.data.borrow()).unwrap();
//
// if account_data.is_initialized() {
//     msg!("Account already initialized");
//     return Err(ProgramError::AccountAlreadyInitialized);
// }
//
// account_data.id = name;
// account_data.is_initialized = true;
// account_data.owner = initializer.key.clone();
//
//
// account_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;



// pub fn sign_contract(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     deposit: u64,
// ) -> ProgramResult {
//     msg!("Signing Contract...");
//     msg!("deposit: {}", deposit);
//
//     let account_info_iter = &mut accounts.iter();
//
//     let borrower_account = next_account_info(account_info_iter)?;
//     let lender_account = next_account_info(account_info_iter)?;
//     let pda_item_account = next_account_info(account_info_iter)?;
//     let contract_account = next_account_info(account_info_iter)?;
//     let system_program = next_account_info(account_info_iter)?;
//
//     // let contract_watcher_account = next_account_info(account_info_iter)?;
//
//     if !borrower_account.is_signer {
//         msg!("Missing required signature");
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//
//     let (pda_calucated_key, bump_seed) = Pubkey::find_program_address(&[lender_account.key.as_ref(), item_id.as_bytes().as_ref()],  program_id);
//
//     if pda_calucated_key != *pda_item_account.key {
//         msg!("Invalid seeds for PDA");
//         return Err(ProgramError::InvalidArgument);
//     }
//
//     let mut item_data =
//         try_from_slice_unchecked::<ContractItemState>(&pda_item_account.data.borrow()).unwrap();
//
//     if(item_data.owner != *lender_account.key) {
//         msg!("Invalid seeds for PDA");
//         return Err(LoanContractError::InvalidOwnerItem.into());
//     }
//
//     let (pda_contract_calculated_key, bump_seed) = Pubkey::find_program_address(&[
//         lender_account.key.as_ref(),
//         borrower_account.key.as_ref(),
//         item_id.as_bytes().as_ref()],
//                                                                                 program_id);
//
//     if pda_contract_calculated_key != *contract_account.key {
//         msg!("Invalid seeds for PDA");
//         return Err(ProgramError::InvalidArgument);
//     }
//
//     // let (contract_watcher_calculated_key, bump_seed) = Pubkey::find_program_address(&[
//     //     lender_account.key.as_ref(),
//     //     borrower_account.key.as_ref(),
//     //     item_id.as_bytes().as_ref(),
//     //     "watcher".as_ref()
//     // ],  program_id);
//
//
//     // if contract_watcher_calculated_key != *contract_watcher_account.key {
//     //     msg!("Invalid seeds for PDA");
//     //     return Err(ProgramError::InvalidArgument);
//     // }
//     //
//     // let mut contract_watcher_data =
//     //     try_from_slice_unchecked::<ContractWatcher>(&contract_watcher_account.data.borrow()).unwrap();
//
// /*    if(contract_watcher_data.has_active){
//         msg!("Has active contract");
//         return Err(LoanContractError::UnavailableSignActiveContract.into());
//     }
// */
//
//     let mut contract_data =
//         try_from_slice_unchecked::<LoanContractState>(&contract_account.data.borrow()).unwrap();
//
//     let account_len: usize = 1000;
//
//     if LoanContractState::get_account_size() > account_len {
//         msg!("Data length is larger than 1000 bytes");
//         return Err(LoanContractError::InvalidDataLength.into());
//     }
//
//     let rent = Rent::get()?;
//     let rent_lamports = rent.minimum_balance(account_len);
//
//
//
//     // Проверка депозита на аккаунте заемщика
//     if **borrower_account.lamports.borrow() < deposit {
//         msg!("Insufficient funds for deposit");
//         return Err(ProgramError::InsufficientFunds);
//     }
//
//     if contract_data.is_initialized() {
//         msg!("Account already initialized");
//         return Err(ProgramError::AccountAlreadyInitialized);
//     }
//
//     invoke_signed(
//         &system_instruction::create_account(
//             borrower_account.key,
//             lender_account.key,
//             rent_lamports,
//             account_len.try_into().unwrap(),
//             program_id,
//         ),
//         &[
//             borrower_account.clone(),
//             lender_account.clone(),
//             system_program.clone(),
//         ],
//         &[&[borrower_account.key.as_ref(), lender_account.key.as_ref(), pda_item_account.key.as_ref(), &[bump_seed]]],
//     )?;
//
//
//
//     contract_data.lender = *lender_account.key;
//     contract_data.borrower = *borrower_account.key;
//     contract_data.deposit = deposit.clone();
//     contract_data.item = *pda_item_account.key;
//     contract_data.status = ContractStatus::Active.into();
//
//     contract_data.serialize(&mut &mut contract_account.data.borrow_mut()[..])?;
//
//     // let mut contract_watcher_data =
//     //     try_from_slice_unchecked::<ContractWatcher>(&contract_watcher_account.data.borrow()).unwrap();
//
//
//
//     Ok(())
// }




// let data = &mut *pda_item.try_borrow_mut_data()?;

// let item: ContractItemState = ContractItemState::try_from_slice(data).map_err(|_| LoanContractError::InvalidItemData)?;


// let mut item_account_data = try_from_slice_unchecked::<ContractItemState>(&data);
//     try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();



// if pda != *pda_account.key {
//     msg!("Invalid seeds for PDA");
//     return Err(ProgramError::InvalidArgument);
// }
//
// if rating > 5 || rating < 1 {
//     msg!("Rating cannot be higher than 5");
//     return Err(ReviewError::InvalidRating.into());
// }
//
//
// let account_len: usize = 1000;
//
// if MovieAccountState::get_account_size(title.clone(), description.clone()) > account_len {
//     msg!("Data length is larger than 1000 bytes");
//     return Err(ReviewError::InvalidDataLength.into());
// }
//
// let rent = Rent::get()?;
// let rent_lamports = rent.minimum_balance(account_len);
//
// invoke_signed(
//     &system_instruction::create_account(
//         initializer.key,
//         pda_account.key,
//         rent_lamports,
//         account_len.try_into().unwrap(),
//         program_id,
//     ),
//     &[
//         initializer.clone(),
//         pda_account.clone(),
//         system_program.clone(),
//     ],
//     &[&[
//         initializer.key.as_ref(),
//         title.as_bytes().as_ref(),
//         &[bump_seed],
//     ]],
// )?;
//
// msg!("PDA created: {}", pda);
//
// msg!("unpacking state account");
// let mut account_data =
//     try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
// msg!("borrowed account data");
//
// msg!("checking if movie account is already initialized");
// if account_data.is_initialized() {
//     msg!("Account already initialized");
//     return Err(ProgramError::AccountAlreadyInitialized);
// }
//
// account_data.discriminator = MovieAccountState::DISCRIMINATOR.to_string();
// account_data.reviewer = *initializer.key;
// account_data.title = title;
// account_data.rating = rating;
// account_data.description = description;
// account_data.is_initialized = true;
//
// msg!("serializing account");
// account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
// msg!("state account serialized");
//
//
// msg!("create comment counter");
// let rent = Rent::get()?;
// let counter_rent_lamports = rent.minimum_balance(MovieCommentCounter::SIZE);
//
// let (counter, counter_bump) =
//     Pubkey::find_program_address(&[pda.as_ref(), "comment".as_ref()], program_id);
// if counter != *pda_counter.key {
//     msg!("Invalid seeds for PDA");
//     return Err(ProgramError::InvalidArgument);
// }
//
// invoke_signed(
//     &system_instruction::create_account(
//         initializer.key,
//         pda_counter.key,
//         counter_rent_lamports,
//         MovieCommentCounter::SIZE.try_into().unwrap(),
//         program_id,
//     ),
//     &[
//         initializer.clone(),
//         pda_counter.clone(),
//         system_program.clone(),
//     ],
//     &[&[pda.as_ref(), "comment".as_ref(), &[counter_bump]]],
// )?;
// msg!("comment counter created");
//
// let mut counter_data =
//     try_from_slice_unchecked::<MovieCommentCounter>(&pda_counter.data.borrow()).unwrap();
//
// msg!("checking if counter account is already initialized");
// if counter_data.is_initialized() {
//     msg!("Account already initialized");
//     return Err(ProgramError::AccountAlreadyInitialized);
// }
//
// counter_data.discriminator = MovieCommentCounter::DISCRIMINATOR.to_string();
// counter_data.counter = 0;
// counter_data.is_initialized = true;
// msg!("comment count: {}", counter_data.counter);
// counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;
//
// msg!("deriving mint authority");
// let (mint_pda, _mint_bump) = Pubkey::find_program_address(&[b"token_mint"], program_id);
// let (mint_auth_pda, mint_auth_bump) = Pubkey::find_program_address(&[b"token_auth"], program_id);
//
// if *token_mint.key != mint_pda {
//     msg!("Incorrect token mint");
//     return Err(ReviewError::IncorrectAccountError.into());
// }
//
// if *mint_auth.key != mint_auth_pda {
//     msg!("Mint passed in and mint derived do not match");
//     return Err(ReviewError::InvalidPDA.into());
// }
//
// if *user_ata.key != get_associated_token_address(initializer.key, token_mint.key) {
//     msg!("Incorrect token mint");
//     return Err(ReviewError::IncorrectAccountError.into());
// }
//
// if *token_program.key != TOKEN_PROGRAM_ID {
//     msg!("Incorrect token program");
//     return Err(ReviewError::IncorrectAccountError.into());
// }
//
// msg!("Minting 10 tokens to User associated token account");
// invoke_signed(
//     // Instruction
//     &spl_token::instruction::mint_to(
//         token_program.key,
//         token_mint.key,
//         user_ata.key,
//         mint_auth.key,
//         &[],
//         10*LAMPORTS_PER_SOL,
//     )?,
//     &[token_mint.clone(), user_ata.clone(), mint_auth.clone()],
//     &[&[b"token_auth", &[mint_auth_bump]]],
// )?;