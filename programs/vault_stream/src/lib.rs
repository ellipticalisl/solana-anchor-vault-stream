use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("StrmV11111111111111111111111111111111111111");

#[program]
pub mod vault_stream {
    use super::*;

    pub fn initialize_stream(
        ctx: Context<InitializeStream>,
        stream_id: u64,
        total_amount: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
    ) -> Result<()> {
        require!(end_time > start_time, StreamError::InvalidTimeRange);
        require!(cliff_time <= end_time, StreamError::InvalidCliffTime);
        require!(total_amount > 0, StreamError::ZeroDeposit);

        let stream = &mut ctx.accounts.stream_account;
        stream.sender = ctx.accounts.sender.key();
        stream.recipient = ctx.accounts.recipient.key();
        stream.token_mint = ctx.accounts.token_mint.key();
        stream.stream_id = stream_id;
        stream.total_amount = total_amount;
        stream.withdrawn_amount = 0;
        stream.start_time = start_time;
        stream.end_time = end_time;
        stream.cliff_time = cliff_time;
        stream.is_canceled = false;
        stream.bump = ctx.bumps.stream_account;

        // Transfer funds from sender to vault PDA
        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), total_amount)?;

        emit!(StreamCreatedEvent {
            stream_id,
            sender: ctx.accounts.sender.key(),
            recipient: ctx.accounts.recipient.key(),
            total_amount,
            start_time,
            end_time,
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream_account;
        require!(!stream.is_canceled, StreamError::StreamAlreadyCanceled);

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(current_time >= stream.cliff_time, StreamError::CliffNotReached);

        let available_to_withdraw = compute_vested_amount(
            stream.total_amount,
            stream.start_time,
            stream.end_time,
            current_time,
        ) - stream.withdrawn_amount;

        require!(available_to_withdraw > 0, StreamError::NothingToWithdraw);

        stream.withdrawn_amount += available_to_withdraw;

        let seeds = &[
            b"stream".as_ref(),
            stream.sender.as_ref(),
            &stream.stream_id.to_le_bytes(),
            &[stream.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: stream.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer),
            available_to_withdraw,
        )?;

        emit!(StreamWithdrawnEvent {
            stream_id: stream.stream_id,
            recipient: stream.recipient,
            amount: available_to_withdraw,
            remaining: stream.total_amount - stream.withdrawn_amount,
        });

        Ok(())
    }
}

pub fn compute_vested_amount(
    total_amount: u64,
    start_time: i64,
    end_time: i64,
    current_time: i64,
) -> u64 {
    if current_time <= start_time {
        0
    } else if current_time >= end_time {
        total_amount
    } else {
        let elapsed = (current_time - start_time) as u128;
        let duration = (end_time - start_time) as u128;
        ((total_amount as u128 * elapsed) / duration) as u64
    }
}

#[derive(Accounts)]
#[instruction(stream_id: u64)]
pub struct InitializeStream<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: Recipient wallet address
    pub recipient: AccountInfo<'info>,
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = sender,
        space = 8 + StreamAccount::INIT_SPACE,
        seeds = [b"stream", sender.key().as_ref(), &stream_id.to_le_bytes()],
        bump
    )]
    pub stream_account: Account<'info, StreamAccount>,

    #[account(
        init,
        payer = sender,
        token::mint = token_mint,
        token::authority = stream_account,
        seeds = [b"vault", stream_account.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawStream<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(
        mut,
        has_one = recipient,
        seeds = [b"stream", stream_account.sender.as_ref(), &stream_account.stream_id.to_le_bytes()],
        bump = stream_account.bump
    )]
    pub stream_account: Account<'info, StreamAccount>,

    #[account(
        mut,
        seeds = [b"vault", stream_account.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct StreamAccount {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub token_mint: Pubkey,
    pub stream_id: u64,
    pub total_amount: u64,
    pub withdrawn_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub is_canceled: bool,
    pub bump: u8,
}

#[event]
pub struct StreamCreatedEvent {
    pub stream_id: u64,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub total_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct StreamWithdrawnEvent {
    pub stream_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub remaining: u64,
}

#[error_code]
pub enum StreamError {
    #[msg("End time must be strictly after start time")]
    InvalidTimeRange,
    #[msg("Cliff time cannot exceed stream end time")]
    InvalidCliffTime,
    #[msg("Total deposit amount must be greater than zero")]
    ZeroDeposit,
    #[msg("Stream cliff timestamp has not been reached yet")]
    CliffNotReached,
    #[msg("Stream is already canceled")]
    StreamAlreadyCanceled,
    #[msg("Available claimable amount is zero")]
    NothingToWithdraw,
}
