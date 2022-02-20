use anchor_lang::prelude::*;

declare_id!("EcPBzAJVchiVnnZJHGbkQHHfMPCbyfVTFetkp9rPA8DC");

// each method inside here defines an RPC request handler / instruction handler which can be handled by the clients
#[program]
pub mod solana_twitter {
    use super::*;
    // context is always the first parameter passed in any method when we create with anchor
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Context is a container over the struct Initialize which is the defined at the bottom of the code.
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.ronaldo = 0;
        vote_account.messi = 0;
        Ok(())
    }

    pub fn vote_ronaldo(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.ronaldo += 1;
        Ok(())
    }

    pub fn vote_messi(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.messi += 1;
        Ok(())
    }
}

/* 
init -> creation of the new account
payer -> funds the acc creation space which defines how large the acc should be and system_program which is required by it's runtime
*/

#[derive(Accounts)] // we are able to define transactions instruction and handle validation logic outside our main program
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

/*
We define a struct with 2 public properties : ronaldo and messi,
this property will keep a track of it's vote in terms of unsigned 64 bit integers
this VoteAccount will pass through each transaction instruction to record votes as they occur
*/

#[account]
pub struct VoteAccount {
    pub ronaldo: u64,
    pub messi: u64,
}