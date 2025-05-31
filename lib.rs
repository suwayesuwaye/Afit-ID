use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction; // For invoke

mod error;
mod state;
mod validation;

use error::NameRegistryError;
use state::{AddressAccount, NameAccount, PendingUpdateAccount, ProgramConfig};
use validation::*;

declare_id!("3PA3oNbwqiNoRxo7mfBjaFcQHH4i5zjwKmMz7g1idiMK"); // Your program ID

#[allow(unexpected_cfgs)]

#[program]
pub mod instantfolio {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, registration_fee: u64) -> Result<()> {
        let config_account = &mut ctx.accounts.config_account;
        // Anchor's `init` handles "already initialized" if it's a new PDA.
        // If this account can be re-initialized or is not a PDA, this check is fine.
        // However, with `#[account(init...)]`, this check is usually redundant.
        // if config_account.is_initialized {
        //     return err!(NameRegistryError::AlreadyInitialized);
        // }

        config_account.is_initialized = true;
        config_account.owner = *ctx.accounts.initializer.key;
        config_account.pending_owner = Pubkey::default();
        config_account.registration_fee = registration_fee;
        Ok(())
    }

    pub fn register_name(ctx: Context<RegisterName>, name: String) -> Result<()> {
        validate_name(&name)?;

        let config = &ctx.accounts.config_account;
        if ctx.accounts.registrant.to_account_info().lamports() < config.registration_fee {
            return err!(NameRegistryError::InsufficientFee);
        }

        // Transfer registration fee
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                ctx.accounts.registrant.key,
                ctx.accounts.treasury_or_config.key, // Assuming fees go to config or a treasury
                config.registration_fee,
            ),
            &[
                ctx.accounts.registrant.to_account_info(),
                ctx.accounts.treasury_or_config.to_account_info(), // Ensure this account can receive lamports
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let name_account = &mut ctx.accounts.name_account;
        name_account.is_initialized = true;
        name_account.owner = *ctx.accounts.registrant.key;
        name_account.name = name.clone();
        name_account.address = *ctx.accounts.registrant.key; // Name associated with registrant's address
        name_account.cooldown_until = Clock::get()?.unix_timestamp;

        let address_account = &mut ctx.accounts.address_account;
        address_account.is_initialized = true;
        address_account.name = name;

        Ok(())
    }

    pub fn request_address_update(
        ctx: Context<RequestAddressUpdate>,
        new_address: Pubkey,
    ) -> Result<()> {
        validate_address(&new_address)?;
        validate_cooldown(ctx.accounts.name_account.cooldown_until)?;
        // `has_one = owner` in Accounts struct handles owner check.

        let pending_update = &mut ctx.accounts.pending_update_account;
        pending_update.is_initialized = true;
        pending_update.new_address = new_address;
        Ok(())
    }

    pub fn complete_address_update(ctx: Context<CompleteAddressUpdate>) -> Result<()> {
        // Constraint `pending_update_account.new_address == new_owner.key()` handles check.
        let name_account = &mut ctx.accounts.name_account;
        name_account.address = *ctx.accounts.new_owner.key;
        name_account.owner = *ctx.accounts.new_owner.key; // Name ownership also transfers
        name_account.cooldown_until = Clock::get()?.unix_timestamp; // Reset cooldown

        // Update the AddressAccount for the new owner if it's being managed.
        // For simplicity, if AddressAccount is tied to the NameAccount's lifecycle:
        let _address_account = &mut ctx.accounts.address_account;
        // address_account.name = name_account.name.clone(); // Name doesn't change here

        // PendingUpdateAccount is closed by `close = new_owner`
        Ok(())
    }

    pub fn rename_name(ctx: Context<RenameName>, new_name_string: String) -> Result<()> {
        validate_name(&new_name_string)?;
        validate_cooldown(ctx.accounts.old_name_account.cooldown_until)?;
        // `has_one = owner` in Accounts struct handles owner check.

        let new_name_account = &mut ctx.accounts.new_name_account;
        new_name_account.is_initialized = true;
        new_name_account.owner = ctx.accounts.old_name_account.owner; // Owner remains the same
        new_name_account.name = new_name_string.clone();
        new_name_account.address = ctx.accounts.old_name_account.address; // Associated address remains
        new_name_account.cooldown_until = Clock::get()?.unix_timestamp;

        // Update the associated AddressAccount if it's keyed by owner/address
        let address_account = &mut ctx.accounts.address_account;
        address_account.name = new_name_string;

        // Close the old_name_account
        // `close = owner` in Accounts struct for old_name_account will handle this.
        Ok(())
    }

    pub fn set_registration_fee(ctx: Context<SetRegistrationFee>, new_fee: u64) -> Result<()> {
        // `has_one = owner` on config_account handles owner check
        ctx.accounts.config_account.registration_fee = new_fee;
        Ok(())
    }

    pub fn change_program_owner(ctx: Context<ChangeProgramOwner>, new_owner_pubkey: Pubkey) -> Result<()> {
        validate_address(&new_owner_pubkey)?;
        // `has_one = owner` on config_account handles owner check
        ctx.accounts.config_account.pending_owner = new_owner_pubkey;
        Ok(())
    }

    pub fn accept_program_ownership(ctx: Context<AcceptProgramOwnership>) -> Result<()> {
        // Constraint `config_account.pending_owner == pending_main_owner.key()` handles check.
        let config_account = &mut ctx.accounts.config_account;
        config_account.owner = *ctx.accounts.pending_main_owner.key;
        config_account.pending_owner = Pubkey::default();
        Ok(())
    }
}

// --------------- ACCOUNTS STRUCTS ---------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + ProgramConfig::LEN, // 8 for discriminator
        // seeds = [b"config"], bump // Example: if config is a PDA
    )]
    pub config_account: Account<'info, ProgramConfig>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)] // Make `name` available for PDA seeds if needed
pub struct RegisterName<'info> {
    // Example: NameAccount is a PDA keyed by the name string
    #[account(
        init,
        payer = registrant,
        space = 8 + NameAccount::LEN,
        seeds = [b"name_v1", name.as_bytes()], // PDA: one NameAccount per unique name
        bump
    )]
    pub name_account: Account<'info, NameAccount>,
    // Example: AddressAccount is a PDA keyed by the registrant's address,
    // ensuring one registration per address.
    #[account(
        init,
        payer = registrant,
        space = 8 + AddressAccount::LEN,
        seeds = [b"address_v1", registrant.key().as_ref()], // PDA: one AddressAccount per registrant
        bump
    )]
    pub address_account: Account<'info, AddressAccount>,
    #[account(mut)] // Assuming config account might be a treasury or needs update
    pub config_account: Account<'info, ProgramConfig>,
    #[account(mut)] // To receive lamports back if config is treasury
    pub treasury_or_config: SystemAccount<'info>, // Account to send fees to
    #[account(mut)]
    pub registrant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_address: Pubkey)] // Make accessible for seeds if needed
pub struct RequestAddressUpdate<'info> {
    #[account(mut, has_one = owner @ NameRegistryError::NotNameOwner)] // owner must be name_account.owner
    pub name_account: Account<'info, NameAccount>,
    #[account(
        init, // A new pending update is created
        payer = owner,
        space = 8 + PendingUpdateAccount::LEN,
        // Example: PDA keyed by the name_account, allowing one pending update per name
        seeds = [b"pending_update_v1", name_account.key().as_ref()],
        bump
    )]
    pub pending_update_account: Account<'info, PendingUpdateAccount>,
    #[account(mut)]
    pub owner: Signer<'info>, // This is the `owner` from name_account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteAddressUpdate<'info> {
    #[account(mut)]
    pub name_account: Account<'info, NameAccount>,
    // This AddressAccount should be the one associated with the *new_owner*.
    // If it's a PDA `seeds = [b"address_v1", new_owner.key().as_ref()]`,
    // it might need `init_if_needed`. For simplicity, assuming it exists or is handled by client.
    #[account(mut)]
    pub address_account: Account<'info, AddressAccount>, // This should be the NEW owner's address account
    #[account(
        mut,
        close = new_owner, // Close and refund lamports to new_owner
        // Constraint: The signer must be the one specified in the pending update
        seeds = [b"pending_update_v1", name_account.key().as_ref()], // Must match seeds from Request
        bump,
        constraint = pending_update_account.new_address == *new_owner.key @ NameRegistryError::NotPendingAddress
    )]
    pub pending_update_account: Account<'info, PendingUpdateAccount>,
    #[account(mut)]
    pub new_owner: Signer<'info>, // The one accepting the update
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(new_name_string: String)]
pub struct RenameName<'info> {
    #[account(
        mut,
        has_one = owner @ NameRegistryError::NotNameOwner, // owner must be old_name_account.owner
        close = owner // Close old name account, refund to owner
    )]
    pub old_name_account: Account<'info, NameAccount>,
    #[account(
        init, // New name account is created
        payer = owner,
        space = 8 + NameAccount::LEN,
        seeds = [b"name_v1", new_name_string.as_bytes()], // New PDA for the new name
        bump
    )]
    pub new_name_account: Account<'info, NameAccount>,
    // This AddressAccount is associated with the owner, its `name` field gets updated
    #[account(
        mut,
        seeds = [b"address_v1", owner.key().as_ref()], // Assuming PDA by owner
        bump,
        constraint = address_account.name == old_name_account.name @ NameRegistryError::InvalidAddress // Or some other check
    )]
    pub address_account: Account<'info, AddressAccount>,
    #[account(mut)]
    pub owner: Signer<'info>, // The owner initiating the rename
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SetRegistrationFee<'info> {
    #[account(mut, has_one = owner @ NameRegistryError::NotContractOwner)] // program_admin must be config_account.owner
    pub config_account: Account<'info, ProgramConfig>,
    pub owner: Signer<'info>, // Renamed from program_admin for clarity with has_one
}

#[derive(Accounts)]
#[instruction(new_owner_pubkey: Pubkey)]
pub struct ChangeProgramOwner<'info> {
    #[account(mut, has_one = owner @ NameRegistryError::NotContractOwner)]
    pub config_account: Account<'info, ProgramConfig>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptProgramOwnership<'info> {
    #[account(
        mut,
        constraint = config_account.pending_owner == *pending_main_owner.key @ NameRegistryError::NotPendingContractOwner
    )]
    pub config_account: Account<'info, ProgramConfig>,
    pub pending_main_owner: Signer<'info>, // The signer must be the pending_owner
}