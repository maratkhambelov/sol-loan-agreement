use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Copy, Clone)]
pub enum ContractStatus {
    Uninitialized = 0,
    Active,
    Closed
}


//TODO: add counter для того, чтобы заключать множество контрактов
#[derive(BorshSerialize, BorshDeserialize)]
pub struct LoanContractState {
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub item: Pubkey,
    pub escrow_account: Pubkey,
    pub expected_amount: u64,
    pub status: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ContractItemState {
    pub is_initialized: bool,
    pub name: String,
    pub owner: Pubkey,
    pub user: Option<Pubkey>
}



impl From<ContractStatus> for u64 {
    fn from(status: ContractStatus) -> Self{
        match status {
            ContractStatus::Uninitialized => 0,
            ContractStatus::Active => 1,
            ContractStatus::Closed => 2
        }
    }
}


impl Sealed for LoanContractState {}

impl IsInitialized for LoanContractState {
    fn is_initialized(&self) -> bool {
        self.status != ContractStatus::Uninitialized.into()
    }
}

impl LoanContractState {

    pub fn is_signable(&self) -> bool {
        self.status == ContractStatus::Uninitialized.into() || self.status == ContractStatus::Closed.into()
    }
    pub fn is_active(&self) -> bool {
        self.status == ContractStatus::Active.into()
    }
    pub fn get_account_size() -> usize {
        let lender_size = std::mem::size_of::<Pubkey>();
        let borrower_size = std::mem::size_of::<Pubkey>();
        let item_size = std::mem::size_of::<Pubkey>();
        let deposit_size = std::mem::size_of::<u64>();
        let status_size = std::mem::size_of::<u8>();

        let total_size = lender_size + borrower_size + item_size + deposit_size + status_size;

        (total_size + 7) / 8 * 8

    }

}

impl Sealed for ContractItemState {}

impl IsInitialized for ContractItemState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl ContractItemState {
    pub fn get_account_size(name: String) -> usize {
        return 4 + name.len() + 1 + 32;
    }
}
