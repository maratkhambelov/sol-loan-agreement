use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum LoanInstruction {
    /// `AddItem` используется для инициации предмета, который будет использоваться в договоре.
    /// В этой инструкции участвуют следующие аккаунты:
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `borrower account`: Аккаунт владельца предмета.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[]` `system_program`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    AddItem {
        name: String,
    },
    /// `SignContract` используется для инициации договора займа.
    /// В этой инструкции участвуют следующие аккаунты:
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `borrower_account`: Аккаунт заемщика, подписывающего договор.
    /// - `[writable]` `account contract`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[]` `lender_account`: Аккаунт кредитора, предоставляющего средства или предмет залога.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[writable]` `escrow_account`: Временный аккаунт для хранения депозита или других активов до выполнения условий контракта.
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
    /// - `[]` `system_program`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    SignContract {
        deposit: u64,
    },
    /// excluded:
    /// - `[]` `rent sysvar account`: Системный аккаунт для расчета платы за аренду (SYSVAR_RENT_PUBKEY).

    /// `CompleteContract` используется для завершения контракта после выполнения всех условий.
    ///   Деньги, хранящиеся на `temp_account`, возвращаются обратно заемщику (`borrower account`).
    /// excluded:
    ///  - `[]` `borrower account`: (ЗАЧЕМ НУЖЕН? - используем ключ из контракта) Аккаунт заемщика, которому будут возвращены средства.
    /// - `[writable]` `escrow_account`: Временный аккаунт для хранения депозита, из которого средства будут возвращены заемщику.
    /// Accounts expected:
    ///
    /// - `[signer]` `lender account`: Аккаунт владельца предмета.
    /// - `[writable]` `contract_account`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
    /// - `[]` `system_program`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    CompleteContract {},

    /// `TerminateContract` используется для досрочного расторжения контракта.
    ///   Деньги, хранящиеся на `temp_account`, возвращаются кредитору (`lender account`) в случае нарушения условий.
    /// excluded:
    /// - `[]` `system program account`: (?) НУЖЕН ЛИ? Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    /// - `[writable]` `escrow_account`: Временный аккаунт для хранения депозита, из которого средства будут возвращены заемщику.
    /// Accounts expected:
    ///
    /// - `[signer]` `lender account`: Аккаунт владельца предмета.
    /// - `[writable]` `contract_account`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
    /// - `[]` `system_program`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    TerminateContract {},
}

#[derive(BorshDeserialize)]
struct ContractItemPayload {
    name: String,
}

#[derive(BorshDeserialize)]
struct ContractPayload {
    deposit: u64,
}
impl LoanInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = ContractItemPayload::try_from_slice(rest).unwrap();
                Self::AddItem {
                    name: payload.name,
                }
            },
            1 => {
                let payload = ContractPayload::try_from_slice(rest).unwrap();
                Self::SignContract {
                    deposit: payload.deposit,
                }
            },
            2 => {
                Self::CompleteContract {}
            },
            3 => {
                Self::TerminateContract {}
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}
