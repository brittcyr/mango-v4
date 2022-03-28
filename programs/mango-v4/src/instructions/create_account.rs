use anchor_lang::prelude::*;

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(account_num: u8)]
pub struct CreateAccount<'info> {
    pub group: AccountLoader<'info, Group>,

    #[account(
        init,
        seeds = [group.key().as_ref(), b"MangoAccount".as_ref(), owner.key().as_ref(), &account_num.to_le_bytes()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<MangoAccount>(),
    )]
    pub account: AccountLoader<'info, MangoAccount>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_account(ctx: Context<CreateAccount>, account_num: u8) -> Result<()> {
    let mut account = ctx.accounts.account.load_init()?;
    // TODO: dont init on stack
    *account = MangoAccount {
        group: ctx.accounts.group.key(),
        owner: ctx.accounts.owner.key(),
        delegate: Pubkey::default(),
        token_account_map: TokenAccountMap::new(),
        serum3_account_map: Serum3AccountMap::new(),
        perp_account_map: PerpAccountMap::new(),
        order_market: [PerpMarketIndex::MAX; MAX_PERP_OPEN_ORDERS],
        order_side: [Side::Bid; MAX_PERP_OPEN_ORDERS],
        orders: [i128::MAX; MAX_PERP_OPEN_ORDERS],
        client_order_ids: [u64::MAX; MAX_PERP_OPEN_ORDERS],
        being_liquidated: false,
        is_bankrupt: false,
        account_num,
        bump: *ctx.bumps.get("account").ok_or(MangoError::SomeError)?,
        reserved: [0; 5],
    };

    Ok(())
}
