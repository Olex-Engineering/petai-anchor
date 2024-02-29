import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, getAccount, createTransferInstruction, createAssociatedTokenAccountInstruction, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { expect } from "chai";
import { ASSET_TEST_MINT_SEED, MPL_TOKEN_METADATA_PROGRAM_ID, program, provider, secondUserProgram, secondUserProvider } from "./constants";
import { statePda, assetMint, assetState, assetMetadata, playerState, tokenMint, freeAssetState, getPetNftMint, getPetState, getPetMasterEdition, getPetMatadata } from "./pdas";

describe("Assets logic", () => {
  anchor.setProvider(provider);

  it('asset can be created', async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      assetMint,
      provider.wallet.publicKey
    )

    const effectName = 'Test effect';

    const [effectState] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(effectName)],
        program.programId
      );

    await program.methods.putAsset(
      {
        assetMint: assetMint,
        increaseFood: 100,
        increaseLoneliness: 100,
        increaseLove: 100,
        price: new anchor.BN(10),
        isCanBeCollected: true,
        collectableTimeDiff: new anchor.BN(5),
        removeEffect: null,
        addEffect: effectState
      })
    .accounts({
      assetState: assetState,
      state: statePda,
    })
    .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 })])
    .rpc()

    const assetStatePda = await program.account.assetState.fetch(assetState);
    const account = await getAccount(provider.connection, tokenAccount);

    expect(assetStatePda.increaseFood).to.eq(100);
    expect(assetStatePda.increaseLoneliness).to.eq(100);
    expect(assetStatePda.increaseLove).to.eq(100);
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

    const [petNftMint] = getPetNftMint(secondUserProvider.wallet.publicKey);

    const effectName = 'Test effect';

    const [effectState] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(effectName)],
        program.programId
      );


    // Second user use 2 assets
    try {
      await secondUserProgram.methods.useAsset(ASSET_TEST_MINT_SEED, 2)
      .accounts({
        petState: getPetState(petNftMint)[0],
        assetState: assetState,
        playerState: playerState,
        state: statePda,
        assetMint: assetMint,
        ataAccount: secondTokenAccount,
        addEffect: effectState,
        removeEffect: null,
        petNftMint: petNftMint,
        masterEdition: getPetMasterEdition(petNftMint)[0],
        metadataAccount: getPetMatadata(petNftMint)[0],
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
      })
      .rpc()
    } catch (error) {
      console.log(error);
    }
    

    const petStateAccount = await program.account.petState.fetch(getPetState(petNftMint)[0]);
    const playerStateAcount = await program.account.playerState.fetch(playerState);

    account = await getAccount(provider.connection, secondTokenAccount);
    
    expect(playerStateAcount.currentEffects.length).to.not.below(1);
    expect(petStateAccount.condition).to.deep.eq({ super: {} });
    expect(account.amount).to.equal(BigInt(8));
  });

  it('Can buy asset', async () => {
    const playerAccount = await program.account.playerState.fetch(playerState);

    const signerAssetAccount = await getAssociatedTokenAddress(
      assetMint,
      secondUserProvider.wallet.publicKey
    );

    const signerTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      secondUserProvider.wallet.publicKey
    );

    const treasuryTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      statePda,
      true
    );

    const realDogTreasureTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      playerAccount.realDogTreasury
    );

    const createTokenAccountsInstructions = [];

    
    try {
      const account = await getAccount(provider.connection, signerAssetAccount);
      expect(account.amount).to.equal(BigInt(8));
    } catch {
      createTokenAccountsInstructions.push(
        createAssociatedTokenAccountInstruction(
          secondUserProvider.wallet.publicKey,
          signerAssetAccount,
          secondUserProvider.wallet.publicKey,
          assetMint
        )
      )
    }

    try {
      const account = await getAccount(provider.connection, treasuryTokenAccount);
    } catch {
      createTokenAccountsInstructions.push(
        createAssociatedTokenAccountInstruction(
          secondUserProvider.wallet.publicKey,
          treasuryTokenAccount,
          statePda,
          tokenMint
        )
      )
    }

    try {
      const account = await getAccount(provider.connection, realDogTreasureTokenAccount);
    } catch {
      createTokenAccountsInstructions.push(
        createAssociatedTokenAccountInstruction(
          secondUserProvider.wallet.publicKey,
          realDogTreasureTokenAccount,
          playerAccount.realDogTreasury,
          tokenMint
        )
      )
    }

    try {
      await secondUserProgram.methods.buyAsset(2)
      .accounts({
        state: statePda,
        assetState: assetState,
        playerState: playerState,
        assetMint: assetMint,
        tokenMint: tokenMint,
        signerTokenAccount: signerTokenAccount,
        signerAssetAccount: signerAssetAccount,
        treasuryTokenAccount: treasuryTokenAccount,
        realDogTokenAccount: realDogTreasureTokenAccount,
        realDogTreasury: provider.wallet.publicKey,
      })
      .preInstructions(createTokenAccountsInstructions)
      .rpc()
    } catch (error) {
      console.log(error);
    }
    
    const account = await getAccount(provider.connection, signerAssetAccount);
    
    expect(account.amount).to.equal(BigInt(10));
  });

  it('Can collect asset', async () => {
    const signerAssetAccount = await getAssociatedTokenAddress(
      assetMint,
      provider.wallet.publicKey
    );

    try {
      const tx = await program.methods
        .collectAsset()
        .accounts({
          state: statePda,
          assetState: assetState,
          freeAssetState: freeAssetState,
          ataAccount: signerAssetAccount,
          assetMint: assetMint,
        })
        .rpc()

      const account = await getAccount(provider.connection, signerAssetAccount);
      expect(account.amount).to.equal(BigInt(1));
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }

    try {
      const tx = await program.methods
        .collectAsset()
        .accounts({
          state: statePda,
          assetState: assetState,
          freeAssetState: freeAssetState,
          ataAccount: signerAssetAccount,
          assetMint: assetMint,
        })
        .rpc()

      expect(tx).not.exist;
    } catch (error) {
      expect(error).exist;
    }

    await new Promise(resolve => setTimeout(resolve, 5500));

    try{
      const tx = await program.methods
        .collectAsset()
        .accounts({
          state: statePda,
          assetState: assetState,
          freeAssetState: freeAssetState,
          ataAccount: signerAssetAccount,
          assetMint: assetMint,
        })
        .rpc()

      const account = await getAccount(provider.connection, signerAssetAccount);
      expect(account.amount).to.equal(BigInt(2));
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }
  });
});