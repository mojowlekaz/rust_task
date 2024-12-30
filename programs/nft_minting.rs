use anchor_lang::prelude::*;
use anchor_lang::system_program::ID;  

// Define the NFT struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NFT {
    pub id: u64,
    pub owner: Pubkey, 
    pub metadata: String,  
}

// Account struct to store NFTs
#[account]
pub struct NFTAccount {
    pub nft_count: u64,  
    pub nfts: Vec<NFT>,  
}

impl NFTAccount {
    pub fn mint_nft(&mut self, metadata: String, owner: Pubkey) {
        let nft = NFT {
            id: self.nft_count,
            owner,
            metadata,
        };

        self.nfts.push(nft);  
        self.nft_count += 1;
    }

    // Function to transfer NFT ownership
    pub fn transfer_nft(&mut self, nft_id: u64, new_owner: Pubkey) -> Result<()> {
        for nft in &mut self.nfts {
            if nft.id == nft_id {
                nft.owner = new_owner;  
                return Ok(());
            }
        }
        Err(error!(ErrorCode::NFTNotFound))  
    }


    pub fn get_nfts_by_owner(&self, owner: Pubkey) -> Vec<NFT> {
        self.nfts.iter().filter(|nft| nft.owner == owner).cloned().collect()
    }
}

// Define the minting context
#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, NFTAccount>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,  
}

// Define the transfer context
#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, NFTAccount>,
    pub owner: Signer<'info>, 
}

// Define the minting and transfer errors
#[error_code]
pub enum ErrorCode {
    #[msg("NFT not found.")]
    NFTNotFound,
}

// Program entrypoint
#[program]
pub mod task_2 {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, metadata: String) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        let owner = ctx.accounts.owner.key();

        nft_account.mint_nft(metadata, owner);

        Ok(())
    }


    pub fn transfer_nft(ctx: Context<TransferNFT>, nft_id: u64, new_owner: Pubkey) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        nft_account.transfer_nft(nft_id, new_owner)?;

        Ok(())
    }

    // Fetch NFTs by the owner's public key
    pub fn get_nfts_by_owner(ctx: Context<MintNFT>, owner: Pubkey) -> Result<Vec<NFT>> {
        let nft_account = &ctx.accounts.nft_account;
        
 
        let owned_nfts = nft_account.get_nfts_by_owner(owner);

        Ok(owned_nfts)
    }
}
