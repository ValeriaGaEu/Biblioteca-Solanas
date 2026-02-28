use anchor_lang::prelude::*;

declare_id!();

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca() -> Result<()>{
        //codigo...
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca {
    ower: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    libros: Vec<Libro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro{
    #[max_leg(60)]
    nombre:String,

    paginas: u16,

    disponibles: bool,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca{
    pub ower: Signer<'info>,
    pub biblioteca: Account<'info, Biblioteca>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoLibro{
    #[account(mut)]
    pub ower: Signer<'info>,

    #[account(
        init,
        payer = ower,
        space = Biblioteca:: INITI_SPACE + 8,
        seeds = [b"biblioteca", ower.key().as_ref()]
    )]
    pub biblioteca: Account<'info, Biblioteca>,
    pub system_program: Program<'info, System>,
}
