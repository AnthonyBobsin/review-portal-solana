use anchor_lang::prelude::*;

declare_id!("2GN1pKddDAx75fAAuz5owVgWE7CUMqEjXz14sYtgwoyW");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_products = 0;

    Ok(())
  }

  pub fn add_product(ctx: Context<AddProduct>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let product = ProductStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };

    base_account.product_list.push(product);
    base_account.total_products += 1;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddProduct<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProductStruct {
  pub gif_link: String,
  pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
  pub total_products: u64,
  // Attach a Vector of type ProductStruct to the account.
  pub product_list: Vec<ProductStruct>,
}
