use anchor_lang::prelude::*;
use anchor_spl::token;

declare_id!("2vteEBtJopYp8Shc1kbD29WCjAZezp8mJMoDkmX5xGca");

#[program]
pub mod staker {
    //stake //beef
    pub const REWARD_MINT_ADDRESS: &str = "5yCiYccC6xiv7s4yPHo4ESgHjBsXh1ySuwZr9Z1oL5v7";
    pub const MYSPL_MINT_ADDRESS: &str = "6oban7Xk5hk58NngWWyajhM9pQZej2akxUBSkAKwGJPF";
    use super::*;
    pub fn create_myspl_ata(_ctx: Context<CreateMysplATA>) -> Result<()> {
        Ok(())
    }

    pub fn stake(
        ctx: Context<Stake>,
        authority_of_reward_mint_bump: u8,
        _myspl_ata_for_program_bump: u8,
        myspl_amount: u64,
    ) -> Result<()> {
        // ************************************************************
        // 1. Ask SPL Token Program to mint REWARD to the user.
        // ************************************************************
        // findPDA(programId + seed)
        // rewardMintPDA, rewardMintPDABump = findPDA(programId + rewardMint.address)
        // and get signer
        let reward_amount = myspl_amount; // TODO: Change the formula
        let reward_mint_address = ctx.accounts.reward_mint.key();
        let seeds = &[
            reward_mint_address.as_ref(),
            &[authority_of_reward_mint_bump],
        ];
        let signer = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.reward_ata_for_user.to_account_info(),
                authority: ctx.accounts.authority_of_reward_mint.to_account_info(),
            },
            &signer,
        );
        token::mint_to(cpi_ctx, reward_amount)?;

        // ************************************************************
        // 2. Ask Token Program to transfer MYSPL from the user.
        // ************************************************************
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.myspl_ata_for_user.to_account_info(),
                to: ctx.accounts.myspl_ata_for_program.to_account_info(),
                authority: ctx
                    .accounts
                    .authority_of_myspl_ata_for_user
                    .to_account_info(),
            },
        );
        token::transfer(cpi_ctx, myspl_amount)?;

        Ok(())
    }

    pub fn unstake(
        ctx: Context<UnStake>,
        myspl_ata_for_program_bump: u8,
        reward_amount: u64,
    ) -> Result<()> {
        // ************************************************************
        // 1. Ask Token Program to burn user's REWARD.
        // ************************************************************
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.reward_mint.to_account_info(),
                from: ctx.accounts.reward_ata_for_user.to_account_info(),
                authority: ctx
                    .accounts
                    .authority_of_reward_ata_for_user
                    .to_account_info(),
            },
        );
        token::burn(cpi_ctx, reward_amount)?;

        // ************************************************************
        // 2. Ask Token Program to transfer back MYSPL to the user.
        // ************************************************************
        let myspl_mint_address = ctx.accounts.myspl_mint.key();
        let seeds = &[myspl_mint_address.as_ref(), &[myspl_ata_for_program_bump]];
        let signer = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.myspl_ata_for_program.to_account_info(),
                authority: ctx.accounts.myspl_ata_for_program.to_account_info(),
                to: ctx.accounts.myspl_ata_for_user.to_account_info(),
            },
            &signer,
        );

        let myspl_amount = reward_amount; // TODO: Change the formula
        token::transfer(cpi_ctx, myspl_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMysplATA<'info> {
    // 1. PDA (pubkey) for myspl ATA for our program.
    // seeds: [myspl_mint + current program id] => "HashMap[seeds+bump] = pda"
    // token::mint: Token Program wants to know what kind of token this ATA is for
    // token::authority: It's a PDA so the authority is itself!
    #[account(
        init,
        payer = payer,
        space = 165,
        seeds = [ MYSPL_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref() ],
        bump,
    )]
    pub myspl_ata_for_program: UncheckedAccount<'info>,

    // 2. The MYSPL token address used as token::mint = [...]
    #[account(
        address = MYSPL_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub myspl_mint: UncheckedAccount<'info>,

    // 3. The rent payer
    #[account(mut)]
    pub payer: Signer<'info>,

    // 4. Anchor needed  for the creation of an Associated Token Account
    pub system_program: Program<'info, System>,
    pub token_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(authority_of_reward_mint_bump: u8, myspl_ata_for_program_bump: u8)]
pub struct Stake<'info> {
    // Token Program (can be either SPL Token or Token 2022)
    pub token_program: UncheckedAccount<'info>,

    // ***********
    // MINTING REWARD TO USERS
    // ***********

    // User ATA for receive REWARD
    #[account(mut)]
    pub reward_ata_for_user: UncheckedAccount<'info>,

    /// CHECK: only used as a signing PDA for mutate the above
    #[account(
    seeds = [ reward_mint.key().as_ref() ],
    bump = authority_of_reward_mint_bump,
    )]
    pub authority_of_reward_mint: UncheckedAccount<'info>,

    // Address of the REWARD mint allowed as mutate for mint new for user
    #[account(
    mut,
    address = REWARD_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub reward_mint: UncheckedAccount<'info>,

    // ***********
    // TRANSFERING MYSPL FROM USERS
    // ***********

    // User ATA which holds MYSPL.
    #[account(mut)]
    pub myspl_ata_for_user: UncheckedAccount<'info>,

    // User ATA authority allowed for mutate the above
    pub authority_of_myspl_ata_for_user: Signer<'info>,

    // Program ATA to receive MYSPL from users
    #[account(
        mut,
        seeds = [ myspl_mint.key().as_ref() ],
        bump = myspl_ata_for_program_bump,
    )]
    pub myspl_ata_for_program: UncheckedAccount<'info>,

    // Require for the PDA above
    #[account(
        address = MYSPL_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub myspl_mint: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(myspl_ata_for_program_bump: u8)]
pub struct UnStake<'info> {
    // Token Program (can be either SPL Token or Token 2022)
    pub token_program: UncheckedAccount<'info>,

    // ***********
    // BURNING USER'S REWARD
    // ***********

    // user ata which hold REWARD use in `token::Burn.to`
    #[account(mut)]
    pub reward_ata_for_user: UncheckedAccount<'info>,

    // The authority allowed for mutate the above
    pub authority_of_reward_ata_for_user: Signer<'info>,

    // REWARD address use in `token::Burn.mint`
    #[account(
        mut,
        address = REWARD_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub reward_mint: UncheckedAccount<'info>,

    // ***********
    // TRANSFER MYSPL TO USERS
    // ***********

    // Program ATA use for `token::Transfer.from`
    #[account(
        mut,
        seeds = [ myspl_mint.key().as_ref() ],
        bump = myspl_ata_for_program_bump,
    )]
    pub myspl_ata_for_program: UncheckedAccount<'info>,

    // user ATA use for `token::Transfer.to`
    #[account(mut)]
    pub myspl_ata_for_user: UncheckedAccount<'info>,

    // Require for the PDA above
    #[account(
        address = MYSPL_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub myspl_mint: UncheckedAccount<'info>,
}
