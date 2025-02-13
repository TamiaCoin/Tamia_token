import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tamiacoin } from "../target/types/tamiacoin";
import * as splToken from "@solana/spl-token";

describe("tamiacoin", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tamiacoin as Program<Tamiacoin>;
  let mint = null;
  let userTokenAccount = null;

  it("Initializes the token", async () => {
    // Create the mint account (mint authority is the wallet payer)
    mint = await splToken.createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey, // Mint authority
      null,
      9 // Decimals
    );

    console.log("Mint created:", mint.toBase58());
  });

  it("Adds a single distribution account", async () => {
    // Create the associated token account for the user (recipient)
    userTokenAccount = await splToken.getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint, // The mint we just created
      provider.wallet.publicKey // User's public key
    );

    console.log("User Token Account:", userTokenAccount.address.toBase58());
  });

  it("Mints tokens to a user", async () => {
    // This is where we mint tokens to the user
    await program.rpc.mintTokens(new anchor.BN(1000), {
      accounts: {
        mint: mint, // Pass the full mint object, not just the public key
        recipient: userTokenAccount.address, // Recipient's associated token account
        authority: provider.wallet.publicKey, // The authority (signer)
        tokenProgram: splToken.TOKEN_PROGRAM_ID, // Token program ID
      },
      signers: [provider.wallet.payer], // Add the wallet payer as a signer
    });

    console.log("Minted 1000 tokens to", userTokenAccount.address.toBase58());
  });
});
