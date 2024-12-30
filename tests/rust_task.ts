// import { RustTask } from "../target/types/rust_task";

import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe("Rust_task", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.task_2;
  const nftAccount = anchor.web3.Keypair.generate();

  it("Initializes the NFT account", async () => {
    await program.methods
      .initialize()
      .accounts({
        nftAccount: nftAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([nftAccount])
      .rpc();
  });

  it("Mints an NFT", async () => {
    const tx = await program.methods
      .mintNft()
      .accounts({
        nftAccount: nftAccount.publicKey,
        owner: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Mint transaction signature:", tx);
  });

  it("Transfers NFT ownership", async () => {
    const newOwner = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
      newOwner.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );

    const tx = await program.methods
      .transferNft()
      .accounts({
        nftAccount: nftAccount.publicKey,
        owner: provider.wallet.publicKey,
        newOwner: newOwner.publicKey,
      })
      .rpc();
    console.log("Transfer transaction signature:", tx);
  });

  it("Fetches NFTs owned by a specific owner", async () => {
    const owner = provider.wallet.publicKey;
    const nfts = await program.account.nftAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: owner.toBase58(),
        },
      },
    ]);
    console.log("NFTs owned by", owner.toBase58(), nfts);
  });
});
