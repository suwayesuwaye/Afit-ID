use anchor_lang::prelude::*;


#[error_code] 
pub enum NameRegistryError {
    #[msg("Invalid name format")]
    InvalidNameFormat,

    #[msg("Name already taken")]
    NameTaken,

    #[msg("Insufficient fee")]
    InsufficientFee,

    #[msg("Name already registered for address")]
    NameAlreadyRegistered,

    #[msg("Not name owner")]
    NotNameOwner,

    #[msg("Invalid address")]
    InvalidAddress,

    #[msg("Cooldown period not over")]
    CooldownNotOver,

    #[msg("No pending update")]
    NoPendingUpdate,

    #[msg("Not the pending address")]
    NotPendingAddress,

    #[msg("Not contract owner")]
    NotContractOwner,

    #[msg("Invalid new owner")]
    InvalidNewOwner,

    #[msg("Not the pending contract owner")]
    NotPendingContractOwner,

    #[msg("Account not initialized")]
    NotInitialized,

    #[msg("Account already initialized")]
    AlreadyInitialized,
}