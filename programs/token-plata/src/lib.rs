use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_plata {
    use super::*;

    pub fn initializeMint(ctx: Context<InitializeMint>) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        
        mint.tag = AccountTag::Mint;
        mint.authority = ctx.accounts.authority;
        mint.supply = 0;
       
        Ok(())
    }

    pub fn initializeTokenAccount(ctx: Context<InitializeTokenAccount>) -> Result<()> {
        let tokenAccount = &mut ctx.accounts.tokenAccount;
        
        tokenAccount.tag = AccountTag::TokenAccount;
        tokenAccount.owner = ctx.accounts.owner;
        tokenAccount.mint = ctx.accounts.mint;
        tokenAccount.amount = 0;

        Ok(())
    }

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let tokenAccount = &mut ctx.accounts.tokenAccount;
        let mint = &mut ctx.accounts.mint;

        mint.supply += amount;
        tokenAccount.amount += amount;        
        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let mut mint = &mut ctx.accounts.mint;
        let mut tokenAccount = &mut ctx.accounts.tokenAccount;

        if tokenAccount.amount < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        tokenAccount.amount -= amount;
        mint.supply -= amount;        
        
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let mut srcTokenAccount = &mut ctx.accounts.srcTokenAccount;
        let mut dstTokenAccount = &mut ctx.accounts.dstTokenAcount;

        if srcTokenAccount.amount < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        srcTokenAccount.amount -= amount;
        dstTokenAccount.amount += amount;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, payer = authority, space = 8 + 41)]
    pub mint: Account<'info, MintData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(init, payer = owner, space = 8 + 73)]
    pub tokenAccount: Account<'info, TokenAccountData>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, MintData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut, owner = tokenAccountOwner)]
    pub tokenAccount: Account<'info, TokenAccountData>,
    pub tokenAccountOwner: Signer<'info>,
    #[account(mut, has_one=mintAuthority)]
    pub mint: Account<'info, MintData>,
    pub mintAuthority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut, owner = tokenAccountOwner)]
    pub tokenAccount: Account<'info, TokenAccountData>,
    pub tokenAccountOwner: Signer<'info>,
    #[account(mut, has_one=mintAuthority)]
    pub mint: Account<'info, MintData>,
    pub mintAuthority: Signer<'info>
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, owner = srcTokenAccountOwner)]
    pub srcTokenAccount: Account<'info, TokenAccountData>,
    pub srcTokenAccountOwner: Signer<'info>,
    #[account(mut)]
    pub dstTokenAcount: Account<'info, TokenAccountData>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount,
}


#[account]
pub struct MintData {
    pub tag: AccountTag, // 1
    pub authority: Pubkey, // 32
    pub supply: u64, // 8
}

#[account]
pub struct TokenAccountData {
    pub tag: AccountTag, // 1b
    pub owner: Pubkey, // 32
    pub mint: Pubkey, // 32
    pub amount: u64, // 8
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}