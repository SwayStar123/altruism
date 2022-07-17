// DO NOT EDIT - automatically generated file
pub mod altruism_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        35u8, 183u8, 27u8, 177u8, 57u8, 43u8, 226u8, 115u8, 174u8, 139u8, 238u8, 43u8, 210u8,
        249u8, 80u8, 140u8, 90u8, 43u8, 28u8, 245u8, 123u8, 30u8, 232u8, 183u8, 164u8, 180u8, 36u8,
        235u8, 225u8, 241u8, 61u8, 189u8,
    ]);
    pub async fn create_mint(
        client: &Client,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_payer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                altruism::instruction::CreateMint {},
                altruism::accounts::CreateMint {
                    mint: a_mint,
                    payer: a_payer,
                    system_program: a_system_program,
                    token_program: a_token_program,
                    rent: a_rent,
                },
                signers,
            )
            .await?)
    }
    pub fn create_mint_ix(
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_payer: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: altruism::instruction::CreateMint {}.data(),
            accounts: altruism::accounts::CreateMint {
                mint: a_mint,
                payer: a_payer,
                system_program: a_system_program,
                token_program: a_token_program,
                rent: a_rent,
            }
            .to_account_metas(None),
        }
    }
    pub async fn create_token_account(
        client: &Client,
        a_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                altruism::instruction::CreateTokenAccount {},
                altruism::accounts::CreateTokenAccount {
                    token_account: a_token_account,
                    mint: a_mint,
                    authority: a_authority,
                    system_program: a_system_program,
                    token_program: a_token_program,
                    rent: a_rent,
                },
                signers,
            )
            .await?)
    }
    pub fn create_token_account_ix(
        a_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: altruism::instruction::CreateTokenAccount {}.data(),
            accounts: altruism::accounts::CreateTokenAccount {
                token_account: a_token_account,
                mint: a_mint,
                authority: a_authority,
                system_program: a_system_program,
                token_program: a_token_program,
                rent: a_rent,
            }
            .to_account_metas(None),
        }
    }
    pub async fn mint_tokens(
        client: &Client,
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                altruism::instruction::MintTokens {},
                altruism::accounts::MintTokens {
                    token: a_token,
                    mint: a_mint,
                    mint_authority: a_mint_authority,
                    token_program: a_token_program,
                },
                signers,
            )
            .await?)
    }
    pub fn mint_tokens_ix(
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: altruism::instruction::MintTokens {}.data(),
            accounts: altruism::accounts::MintTokens {
                token: a_token,
                mint: a_mint,
                mint_authority: a_mint_authority,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
}
