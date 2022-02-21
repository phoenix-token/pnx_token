use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PnxContract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    minter: LookupMap<AccountId, bool>,
}

const DATA_IMAGE_SVG_PNX_ICON: &str = "data:/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiMwNzA2MzgiLz48cGF0aCBkPSJNMjguNDU1IDI5LjczMmMtMi41MTQgMi44NTUtNy4wNSA2LjEyLTExLjAxNyA2LjE1My0yLjUxNy0uMzI0LTQuODM3LTEuMTE2LTYuODE1LTIuMjYyIDIuNTAyLjE3NCA1LjUxNS0uMjc3IDguNDI3LTEuNzkgMS4zOTktLjYwNSAzLjYzMy0yLjIzOCAzLjcxOC0yLjI5Ny0uMDY2LjAzNy0yLjQ5IDEuNDgyLTMuOTQ2IDEuOTY1LTQuMzQ0IDEuNzkxLTguNjQ3IDEuMzEyLTEwLjc2Mi4yNzItMi40OTktMi4yMzgtNC4wMy01LjE4LTQuMDYtOC40MS4zMyAzLjEwMyA1Ljg4OCA2LjI1NCAxMS41NCA1LjEyNyA2LjQtMS4yNzcgMTAuNzM0LTcuNjQ1IDE1LjgzNC04LjQxNCAxLjE1OS0uMTc1IDIuNDY2LS4wNyAzLjM2Ny41MzggMS4wNzUuNzI2IDEuMjI3IDEuNTUgMS4yNTUgMi42NTIuMTc5IDYuNjktNi40MzUgMTIuMTc5LTE0LjYzIDEyLjczNCAyLjg4My0xLjQwOSA1LjQzNy0zLjg2NSA3LjA4OS02LjI2OFoiIGZpbGw9InVybCgjYSkiLz48cGF0aCBkPSJNMTEuNTQ1IDEwLjI2OGMyLjUxNC0yLjg1NSA3LjA1LTYuMTIgMTEuMDE3LTYuMTUzIDIuNTE3LjMyMyA0LjgzNyAxLjExNiA2LjgxNSAyLjI2Mi0yLjUwMi0uMTc0LTUuNTE1LjI3Ny04LjQyNyAxLjc5LTEuMzk5LjYwNS0zLjYzMyAyLjIzOC0zLjcxOCAyLjI5Ny4wNjYtLjAzNyAyLjQ5LTEuNDgyIDMuOTQ2LTEuOTY1IDQuMzQ0LTEuNzkxIDguNjQ4LTEuMzEyIDEwLjc2Mi0uMjcyIDIuNDk5IDIuMjM4IDQuMDMgNS4xOCA0LjA2IDguNDEtLjMzLTMuMTAzLTUuODg4LTYuMjU0LTExLjU0LTUuMTI3LTYuNCAxLjI3Ny0xMC43MzMgNy42NDUtMTUuODM0IDguNDE0LTEuMTU5LjE3NS0yLjQ2Ni4wNy0zLjM2Ny0uNTM4LTEuMDc1LS43MjYtMS4yMjctMS41NS0xLjI1NS0yLjY1Mi0uMTc5LTYuNjkgNi40MzUtMTIuMTggMTQuNjMtMTIuNzM0LTIuODgzIDEuNDA5LTUuNDM3IDMuODY1LTcuMDg5IDYuMjY4WiIgZmlsbD0idXJsKCNiKSIvPjxkZWZzPjxsaW5lYXJHcmFkaWVudCBpZD0iYSIgeDE9IjM2IiB5MT0iMjgiIHgyPSI5LjU1NiIgeTI9IjI4IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHN0b3Agc3RvcC1jb2xvcj0iIzAwRjhEQiIvPjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iIzA4RjhDQyIvPjwvbGluZWFyR3JhZGllbnQ+PGxpbmVhckdyYWRpZW50IGlkPSJiIiB4MT0iNCIgeTE9IjEyIiB4Mj0iMzAuNDQ0IiB5Mj0iMTIiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIj48c3RvcCBzdG9wLWNvbG9yPSIjMDBGOERCIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdG9wLWNvbG9yPSIjMDhGOENDIi8+PC9saW5lYXJHcmFkaWVudD48L2RlZnM+PC9zdmc+";

#[near_bindgen]
impl PnxContract {
    #[init]
    pub fn initialize(owner_id: ValidAccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "PNX".to_string(),
                symbol: "PNX".to_string(),
                icon: Some(DATA_IMAGE_SVG_PNX_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 18,
            },
        )
    }

    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            minter: LookupMap::new(b"t".to_vec()),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token
            .internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    pub fn add_minter(&mut self, account_id: ValidAccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "NO PERMISSION"
        );

        self.minter.insert(account_id.as_ref(), &true);
    }

    pub fn remove_minter(&mut self, account_id: ValidAccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "NO PERMISSION"
        );

        self.minter.remove(&account_id.as_ref());
    }

    pub fn mint(&mut self, account_id: ValidAccountId, amount: U128) {
        let is_minter: bool = self.minter.get(&env::predecessor_account_id()).unwrap();

        assert!(is_minter, "! Minter");

        self.token
            .internal_deposit(account_id.as_ref(), amount.into());
    }

    pub fn burn(&mut self, amount: U128) {
        self.token
            .internal_withdraw(&env::signer_account_id(), amount.into());
    }
}

near_contract_standards::impl_fungible_token_core!(PnxContract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(PnxContract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for PnxContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = PnxContract::initialize(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = PnxContract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = PnxContract::initialize(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(
            contract.ft_balance_of(accounts(2)).0,
            (TOTAL_SUPPLY - transfer_amount)
        );
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
