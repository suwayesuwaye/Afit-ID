use anchor_lang::prelude::*;
use crate::error::NameRegistryError;
use crate::state::MAX_NAME_LENGTH;

pub fn validate_name(name: &str) -> Result<()> {
    if name.len() < 3 || name.len() > MAX_NAME_LENGTH {
        return err!(NameRegistryError::InvalidNameFormat);
    }
    if name.starts_with('-') || name.ends_with('-') {
        return err!(NameRegistryError::InvalidNameFormat);
    }
    for c in name.chars() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-' {
            return err!(NameRegistryError::InvalidNameFormat);
        }
    }
    Ok(())
}

pub fn validate_address(address: &Pubkey) -> Result<()> {
    if address == &Pubkey::default() {
        return err!(NameRegistryError::InvalidAddress);
    }
    Ok(())
}

pub fn validate_cooldown(cooldown_until: i64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    if current_time < cooldown_until {
        return err!(NameRegistryError::CooldownNotOver);
    }
    Ok(())
}
