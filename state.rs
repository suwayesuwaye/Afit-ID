use anchor_lang::prelude::*;

// Maximum length for a name, used for space calculation.
pub const MAX_NAME_LENGTH: usize = 32;

#[account]
#[derive(Default)] // For NameAccount::default() in rename logic
pub struct NameAccount {
    pub is_initialized: bool, // Useful if not using init_if_needed or managing state lifecycle manually
    pub owner: Pubkey,        // The current owner of the name registration
    pub name: String,         // The registered name
    pub address: Pubkey,      // The Solana address associated with this name
    pub cooldown_until: i64,  // Timestamp for cooldown periods
}

impl NameAccount {
    // 8 (discriminator) is handled by Anchor's space calculation if using `#[account(space = ...)]` directly on Account struct.
    // Here, this LEN is for manual space calculation if needed elsewhere or for reference.
    // is_initialized: bool (1)
    // owner: Pubkey (32)
    // name: String (4 for len + MAX_NAME_LENGTH)
    // address: Pubkey (32)
    // cooldown_until: i64 (8)
    pub const LEN: usize = 1 + 32 + (4 + MAX_NAME_LENGTH) + 32 + 8;
}

#[account]
#[derive(Default)]
pub struct AddressAccount {
    pub is_initialized: bool,
    pub name: String, // The name registered to the address this account represents
}

impl AddressAccount {
    // is_initialized: bool (1)
    // name: String (4 for len + MAX_NAME_LENGTH)
    pub const LEN: usize = 1 + (4 + MAX_NAME_LENGTH);
}

#[account]
#[derive(Default)]
pub struct PendingUpdateAccount {
    pub is_initialized: bool,
    pub new_address: Pubkey, // The new address pending confirmation
}

impl PendingUpdateAccount {
    // is_initialized: bool (1)
    // new_address: Pubkey (32)
    pub const LEN: usize = 1 + 32;
}

#[account]
#[derive(Default)]
pub struct ProgramConfig {
    pub is_initialized: bool,
    pub owner: Pubkey,          // Program owner (e.g., for admin functions)
    pub pending_owner: Pubkey,  // For ownership transfer
    pub registration_fee: u64,
}

impl ProgramConfig {
    // is_initialized: bool (1)
    // owner: Pubkey (32)
    // pending_owner: Pubkey (32)
    // registration_fee: u64 (8)
    pub const LEN: usize = 1 + 32 + 32 + 8;
}