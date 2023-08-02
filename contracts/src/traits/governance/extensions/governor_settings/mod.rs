use openbrush::traits::Timestamp;

#[openbrush::trait_definition]
pub trait GovernorSettings {
    #[ink(message)]
    fn voting_delay(&self) -> Timestamp;

    #[ink(message)]
    fn voting_period(&self) -> Timestamp;

    #[ink(message)]
    fn proposal_threshold(&self) -> u128;

    #[ink(message)]
    //todo: #[openbrush::modifiers(only_governance)]
    fn set_voting_delay(&mut self, new_voting_delay: Timestamp);

    #[ink(message)]
    //todo: #[openbrush::modifiers(only_governance)]
    fn set_voting_period(&mut self, new_voting_period: Timestamp);

    #[ink(message)]
    //todo: #[openbrush::modifiers(only_governance)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128);
}