use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum LoanInstruction {
    /// `SignContract` используется для инициации договора займа.
    /// В этой инструкции участвуют следующие аккаунты:
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `borrower account`: Аккаунт заемщика, подписывающего договор.
    /// - `[]` `lender account`: Аккаунт кредитора, предоставляющего средства или предмет залога.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[writable]` `temp_account`: Временный аккаунт для хранения депозита или других активов до выполнения условий контракта.
    /// - `[writable]` `account contract`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[]` `rent sysvar account`: Системный аккаунт для расчета платы за аренду (SYSVAR_RENT_PUBKEY).
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
    /// - `[]` `system program account`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    SignContract {
        deposit: u64,
    },
    /// `AddItem` используется для инициации предмета, который будет использоваться в договоре.
    /// В этой инструкции участвуют следующие аккаунты:
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `borrower account`: Аккаунт владельца предмета.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[]` `system program account`: Аккаунт системной программы для создания и управления аккаунтами (SystemProgram::ID).
    AddItem {
        name: String,
    },
    /// `CompleteContract` используется для завершения контракта после выполнения всех условий.
    /// Деньги, хранящиеся на `temp_account`, возвращаются обратно заемщику (`borrower account`).
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `lender account`: Аккаунт владельца предмета.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[writable]` `account contract`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[writable]` `temp_account`: Временный аккаунт для хранения депозита, из которого средства будут возвращены заемщику.
    /// - `[]` `borrower account`: Аккаунт заемщика, которому будут возвращены средства.
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
    CompleteContract {},
    /// `TerminateContract` используется для досрочного расторжения контракта.
    /// Деньги, хранящиеся на `temp_account`, возвращаются кредитору (`lender account`) в случае нарушения условий.
    ///
    /// Accounts expected:
    ///
    /// - `[signer]` `lender account`: Аккаунт владельца предмета.
    /// - `[writable]` `item_contract_account`: Аккаунт, содержащий информацию о предмете залога (`ContractItemState`).
    /// - `[writable]` `account contract`: Аккаунт, содержащий основной контракт (`LoanContractState`).
    /// - `[writable]` `temp_account`: Временный аккаунт для хранения депозита, из которого средства будут возвращены заемщику.
    /// - `[]` `token program account`: Аккаунт программы токенов SPL для выполнения операций с токенами (TOKEN_PROGRAM_ID).
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
