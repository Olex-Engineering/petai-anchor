import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, getAccount, createTransferInstruction, createAssociatedTokenAccountInstruction } from "@solana/spl-token";
import { expect } from "chai";
import { ASSET_TEST_MINT_SEED, MPL_TOKEN_METADATA_PROGRAM_ID, program, provider, secondUserProgram, secondUserProvider } from "./constants";
import { statePda, assetMint, assetState, assetMetadata, playerState, petState } from "./pdas";

describe("Assets logic", () => {
  anchor.setProvider(provider);

    it('asset can be created', async () => {
      const tokenAccount = await getAssociatedTokenAddress(
        assetMint,
        provider.wallet.publicKey
      )


      await program.methods.putAsset(
        {
          assetMint: assetMint,
          increaseFood: 1,
          increaseLoneliness: 0,
          increaseLove: 0,
          price: new anchor.BN(10),
          
        })
      .accounts({
        assetState: assetState,
        state: statePda,
      })
      .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 })])
      .rpc()

      const assetStatePda = await program.account.assetState.fetch(assetState);
      const account = await getAccount(provider.connection, tokenAccount);

      expect(assetStatePda.increaseFood).to.eq(1);
      expect(assetStatePda.increaseLoneliness).to.eq(0);
      expect(assetStatePda.increaseLove).to.eq(0);
      expect(assetStatePda.key.toBase58()).to.eq(assetMint.toBase58());
      expect(account.amount).to.eq(BigInt(10));
    });

    it('Can use asset', async () => {
      const tokenAccount = await getAssociatedTokenAddress(
        assetMint,
        provider.wallet.publicKey
      )

      const secondTokenAccount = await getAssociatedTokenAddress(
        assetMint,
        secondUserProvider.wallet.publicKey
      )

      // Transfer assets to second user
      const transferTransaction = new anchor.web3.Transaction().add(
        createAssociatedTokenAccountInstruction(
          provider.wallet.publicKey,
          secondTokenAccount,
          secondUserProvider.wallet.publicKey,
          assetMint
        ),
        createTransferInstruction(
          tokenAccount,
          secondTokenAccount,
          provider.wallet.publicKey,
          10,
        )
      )

      await provider.sendAndConfirm(transferTransaction);

      let account = await getAccount(provider.connection, secondTokenAccount);
      expect(account.amount).to.equal(BigInt(10));

      // Second user use 2 assets
      try {
        await secondUserProgram.methods.useAsset(ASSET_TEST_MINT_SEED, 2)
        .accounts({
          playerState: playerState,
          petState: petState,
          assetState: assetState,
          state: statePda,
          assetMint: assetMint,
          assetMetadataAccount: assetMetadata,
          ataAccount: secondTokenAccount,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
        })
        .rpc()
      } catch (error) {
        console.log(error);
      }
      

      const petStateAccount = await program.account.petState.fetch(petState);

      account = await getAccount(provider.connection, secondTokenAccount);
      
      expect(petStateAccount.condition).to.deep.eq({ super: {} });
      expect(account.amount).to.equal(BigInt(8));
    });
});