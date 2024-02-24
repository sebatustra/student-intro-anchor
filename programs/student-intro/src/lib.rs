use anchor_lang::prelude::*;

declare_id!("EZVdGWwx3FTorwSFJFWw12BZRdevcqro4vs1Vm9NTg7Q");

#[program]
pub mod student_intro {
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
    pub system_program: Program<'info, System>
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

#[account]
pub struct StudentIntroState {
    pub name: String,
    pub message: String
}