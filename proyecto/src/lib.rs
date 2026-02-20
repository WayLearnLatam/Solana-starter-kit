use anchor_lang::prelude::*;

declare_id!("");

#[program]
mod modulo {
    use super::*;

    pub fn mi_primer_instruccion(ctx: Context<Saludo>) -> Result<()> {
        msg!("Mi instruccion, funciona !!! :D");
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Saludo {}

