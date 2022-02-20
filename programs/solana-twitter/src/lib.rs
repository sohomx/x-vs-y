use anchor_lang::prelude::*;

declare_id!("EcPBzAJVchiVnnZJHGbkQHHfMPCbyfVTFetkp9rPA8DC");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
