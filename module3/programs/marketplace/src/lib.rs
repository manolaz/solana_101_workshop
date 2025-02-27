use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod marketplace {
    use super::*;

    pub fn create_listing(
        ctx: Context<CreateListing>,
        price: u64,
        quantity: u64,
        name: String,
    ) -> Result<()> {
        // Validate inputs
        require!(price > 0, ErrorCode::InvalidPrice);
        require!(quantity > 0, ErrorCode::InvalidQuantity);
        require!(name.len() > 0, ErrorCode::InvalidName);
        require!(name.len() <= 32, ErrorCode::NameTooLong);

        let listing = &mut ctx.accounts.listing;
        let seller = &ctx.accounts.seller;

        // Initialize listing account
        listing.seller = seller.key();
        listing.price = price;
        listing.quantity = quantity;
        listing.name = name;
        listing.active = true;
        listing.bump = *ctx.bumps.get("listing").ok_or(ErrorCode::BumpSeedNotInHashMap)?;

        // Initialize escrow account
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.seller_token_account.to_account_info(),
                    to: ctx.accounts.escrow_token_account.to_account_info(),
                    authority: seller.to_account_info(),
                },
            ),
            quantity,
        )?;

        msg!("Listing created successfully!");
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>, quantity: u64) -> Result<()> {
        // Validate inputs
        let listing = &mut ctx.accounts.listing;
        require!(listing.active, ErrorCode::ListingNotActive);
        require!(quantity > 0, ErrorCode::InvalidQuantity);
        require!(quantity <= listing.quantity, ErrorCode::InsufficientQuantity);

        let total_price = listing.price.checked_mul(quantity)
            .ok_or(ErrorCode::NumericalOverflow)?;

        // Transfer payment from buyer to seller
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.buyer_payment_account.to_account_info(),
                    to: ctx.accounts.seller_payment_account.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            total_price,
        )?;

        // Transfer items from escrow to buyer
        let seeds = &[
            b"listing",
            listing.seller.as_ref(),
            listing.name.as_bytes(),
            &[listing.bump],
        ];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow_token_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.listing.to_account_info(),
                },
                signer,
            ),
            quantity,
        )?;

        // Update listing
        listing.quantity = listing.quantity.checked_sub(quantity)
            .ok_or(ErrorCode::NumericalOverflow)?;
        
        if listing.quantity == 0 {
            listing.active = false;
        }

        msg!("Purchase completed successfully!");
        Ok(())
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.active, ErrorCode::ListingNotActive);
        
        // Transfer items from escrow back to seller
        let seeds = &[
            b"listing",
            listing.seller.as_ref(),
            listing.name.as_bytes(),
            &[listing.bump],
        ];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow_token_account.to_account_info(),
                    to: ctx.accounts.seller_token_account.to_account_info(),
                    authority: ctx.accounts.listing.to_account_info(),
                },
                signer,
            ),
            listing.quantity,
        )?;

        // Mark listing as inactive
        listing.active = false;
        listing.quantity = 0;

        msg!("Listing cancelled successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(price: u64, quantity: u64, name: String)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        init,
        payer = seller,
        space = Listing::LEN,
        seeds = [b"listing", seller.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key() @ ErrorCode::InvalidOwner
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = seller,
        seeds = [b"escrow", listing.key().as_ref()],
        bump,
        token::mint = item_mint,
        token::authority = listing,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub item_mint: Account<'info, token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(quantity: u64)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", listing.seller.as_ref(), listing.name.as_bytes()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: This is the seller's address from the listing
    #[account(
        mut,
        constraint = seller.key() == listing.seller @ ErrorCode::InvalidSeller
    )]
    pub seller: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = escrow_token_account.owner == listing.key() @ ErrorCode::InvalidEscrowOwner
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key() @ ErrorCode::InvalidOwner
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer_payment_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub seller_payment_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(
        mut,
        constraint = seller.key() == listing.seller @ ErrorCode::InvalidSeller
    )]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", listing.seller.as_ref(), listing.name.as_bytes()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(
        mut,
        constraint = escrow_token_account.owner == listing.key() @ ErrorCode::InvalidEscrowOwner
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key() @ ErrorCode::InvalidOwner
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub quantity: u64,
    pub name: String,
    pub active: bool,
    pub bump: u8,
}

impl Listing {
    const LEN: usize = 8 + // discriminator
        32 + // seller pubkey
        8 +  // price
        8 +  // quantity
        4 + 32 + // name (String with max length of 32)
        1 +  // active
        1;   // bump
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid price, must be greater than zero")]
    InvalidPrice,
    
    #[msg("Invalid quantity, must be greater than zero")]
    InvalidQuantity,

    #[msg("Invalid name, must not be empty")]
    InvalidName,
    
    #[msg("Name is too long, must be 32 characters or less")]
    NameTooLong,

    #[msg("Listing is not active")]
    ListingNotActive,
    
    #[msg("Insufficient quantity available")]
    InsufficientQuantity,

    #[msg("Invalid seller")]
    InvalidSeller,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Invalid escrow owner")]
    InvalidEscrowOwner,

    #[msg("Numerical overflow")]
    NumericalOverflow,

    #[msg("Bump seed not in hash map")]
    BumpSeedNotInHashMap,
}
