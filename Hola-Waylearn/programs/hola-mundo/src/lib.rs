use anchor_lang::prelude::*;

declare_id!("");

#[program]
mod hola_mundo {
    use super::*;

    pub fn saludo(ctx: Context<Saludo>) -> Result<()> {
        msg!("Hola Solana con Waylearn!!!");
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Saludo {}

