#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Burnable, Upgradeable, Ownable)]
#[openbrush::contract]
pub mod contract_v2 {
    use ink::storage::{
        traits::ManualKey,
        Lazy,
    };
    use openbrush::{
        modifiers,
        traits::Storage,
    };

    const STORAGE_KEY: u32 = openbrush::storage_unique_key!("contract_v2", "fee_collector");

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        fee_collector: Lazy<AccountId, ManualKey<STORAGE_KEY>>,
    }

    #[overrider(psp22::Internal)]
    fn _transfer_from_to(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let from_balance = psp22::Internal::_balance_of(self, &from);

        if from_balance < amount {
            return Err(PSP22Error::InsufficientBalance)
        }

        psp22::Internal::_before_token_transfer(self, Some(&from), Some(&to), &amount)?;

        self.psp22.balances.insert(&from, &(from_balance - amount));

        let new_amount = if let Some(fee_collector) = self.fee_collector.get() {
            let fee = amount / 10;
            let fee_collector_balance = psp22::Internal::_balance_of(self, &fee_collector);
            self.psp22
                .balances
                .insert(&fee_collector, &(fee_collector_balance + fee));
            psp22::Internal::_emit_transfer_event(self, Some(from), Some(to), fee);
            amount - fee
        } else {
            amount
        };

        let to_balance = psp22::Internal::_balance_of(self, &to);
        self.psp22.balances.insert(&to, &(to_balance + new_amount));

        psp22::Internal::_after_token_transfer(self, Some(&from), Some(&to), &amount)?;
        psp22::Internal::_emit_transfer_event(self, Some(from), Some(to), new_amount);

        Ok(())
    }

    #[default_impl(Upgradeable)]
    #[modifiers(only_owner)]
    fn set_code_hash() {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());

            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_fee_collector(&mut self, account: AccountId) -> Result<(), OwnableError> {
            self.fee_collector.set(&account);
            Ok(())
        }
    }
}
