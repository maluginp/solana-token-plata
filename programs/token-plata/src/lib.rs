use anchor_lang::prelude::*;

declare_id!("7fQFifcVnr4gkSHwkp21ggCKVSrhLdSccDZ2gdUHoy25");

#[program]
pub mod token_plata {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        let mint = &mut ctx.accounts.mint;

        // msg!("Mint key {}", mint.key());        

        mint.authority = ctx.accounts.payer.key();
        mint.supply = 0;
       
        Ok(())
    }

    pub fn initialize_token_account(ctx: Context<InitializeTokenAccount>) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        
        token_account.owner = ctx.accounts.payer.key();
        token_account.mint = ctx.accounts.mint.key();
        token_account.amount = 0;

        Ok(())
    }

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let dst = &mut ctx.accounts.dst;
        let mint = &mut ctx.accounts.mint;
        let authority = &ctx.accounts.authority;

        assert!(mint.authority == authority.key());
        assert!(dst.mint == mint.key());
        
        mint.supply += amount;
        dst.amount += amount;    
        
        msg!("total supply {}", mint.supply);
        msg!("dst amount {}", dst.amount);

        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let src = &mut ctx.accounts.src;
        let owner = &ctx.accounts.owner;

        assert!(src.owner == owner.key());
        assert!(src.mint == mint.key());
        assert!(src.amount >= amount);

        // if token_account.amount < amount {
        //     return Err(error!(ErrorCode::InsufficientFunds));
        // }
        
        src.amount -= amount;
        mint.supply -= amount;   
        
        msg!("total supply {}", mint.supply);
        msg!("src amount {}", src.amount);
        
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let src = &mut ctx.accounts.src;
        let dst = &mut ctx.accounts.dst;
        let owner = &ctx.accounts.owner;

        assert!(src.amount >= amount);
        assert!(src.owner == owner.key());
        assert!(src.mint == dst.mint);
        
        src.amount -= amount;
        dst.amount += amount;

        msg!("src supply {}", src.amount);
        msg!("dst amount {}", dst.amount);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init, 
        seeds = [
            payer.key().as_ref(),
        ],
        payer = payer, 
        bump,
        space = 8 + 40
    )]
    mint: Account<'info, MintData>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(
        init,
        seeds = [
            payer.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        space = 8 + 72,
        payer = payer,
    )]
    token_account: Account<'info, TokenAccountData>,
    mint: Account<'info, MintData>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    mint: Account<'info, MintData>,
    #[account(mut)]
    dst: Account<'info, TokenAccountData>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    mint: Account<'info, MintData>,
    #[account(mut)]
    src: Account<'info, TokenAccountData>,
    owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub src: Account<'info, TokenAccountData>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccountData>,
    owner: Signer<'info>,
}

#[account]
pub struct MintData {
    pub authority: Pubkey, // 32
    pub supply: u64, // 8
}

#[account]
pub struct TokenAccountData {
    pub owner: Pubkey, // 32
    pub mint: Pubkey, // 32
    pub amount: u64, // 8
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}