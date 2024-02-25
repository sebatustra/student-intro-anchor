use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("EZVdGWwx3FTorwSFJFWw12BZRdevcqro4vs1Vm9NTg7Q");

#[program]
pub mod student_intro {
    use anchor_spl::token::{mint_to, MintTo};

    use super::*;

    pub fn initialize_intro(
        ctx: Context<InitializeIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Initializing student intro...");
        msg!("name: {}", name);
        msg!("message: {}", message);

        let intro = &mut ctx.accounts.student_intro;
        intro.name = name;
        intro.message = message;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info()
                }, 
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.mint]
                ]]
            ), 
            10 * 10 ^ 6
        )?;

        Ok(())
    }

    pub fn update_intro(
        ctx: Context<UpdateIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("Updating student intro...");
        msg!("new name: {}", name);
        msg!("new message: {}", message);

        let intro = &mut ctx.accounts.student_intro;
        intro.name = name;
        intro.message = message;

        Ok(())
    }

    pub fn delete_intro(
        _ctx: Context<DeleteIntro>
    ) -> Result<()> {
        msg!("Deleted intro");
        Ok(())
    }

    pub fn initialize_mint(
        _ctx: Context<InitializeMint>
    ) -> Result<()> {
        msg!("Initialized mint");

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct InitializeIntro<'info> {
    #[account(
        init,
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 4 + name.len() + 4 + message.len()
    )]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct UpdateIntro<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        realloc = 8 + 4 + name.len() + 4 + message.len(),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteIntro<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct StudentIntroState {
    pub name: String,
    pub message: String
}
