import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";
import { expect } from "chai";
import { ASSET_COLLECTION_MINT_SEED, ASSET_TEST_MINT_SEED, MPL_TOKEN_METADATA_PROGRAM_ID, PET_COLLECTION_MINT_SEED, PET_NFT_MINT_SEED, program, provider, secondUserProgram, secondUserProvider } from "./constants";
import { petCollectionMint, statePda, petCollectionMetadata, petCollectionMasterEdition, petNFTMint, petMetadata, petMasterEdition, assetCollectionMint, assetMint, assetCollectionMetadata, assetCollectionMasterEdition, assetMetadata } from "./pdas";
import { createNft } from "@metaplex-foundation/mpl-token-metadata";

describe("Token logic", () => {
  anchor.setProvider(provider);

  it('Pet collection can be minted', async () => {
      const tokenAccount = await getAssociatedTokenAddress(
        petCollectionMint,
        provider.wallet.publicKey
      )
  
      try {
        await program.methods.createToken(
          PET_COLLECTION_MINT_SEED,
          new anchor.BN(1),
          {
            name: 'Test collection',
            symbol: 'TTT',
            uri: 'https://test.com',
            tokenStandart: {
              nonFungible: {}
            },
            primarySaleHappened: false,
            collection: null,
            collectionDetails: {
              v1: { size: new anchor.BN(0) }
            },
            decimals: null,
            printSupply: {
              zero: {}
            },
            creators: [{
                address: statePda,
                verified: true,
                share: 100
            }]
          },
        )
          .accounts({
            state: statePda,
            mint: petCollectionMint,
            metadataAccount: petCollectionMetadata,
            masterEdition: petCollectionMasterEdition,
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            ataAccount: tokenAccount,
            collectionMint: null,
            collectionMetadata: null,
            collectionMasterEdition: null
          })
          .signers([])
          .rpc()
    
        const state = await program.account.programState.fetch(statePda);
        const account = await getAccount(provider.connection, tokenAccount);
    
        console.log(state.petCollection.toBase58());
        expect(account.amount).to.equal(BigInt(1));
        expect(state.petCollection.toBase58()).to.equal(petCollectionMint.toBase58());
      } catch (error) {
        console.log(error);
        expect(error).not.exist;
      }
      
    });
  
    it ('Pet collection can be updated', async () => {
      const tokenAccount = await getAssociatedTokenAddress(
        petCollectionMint,
        provider.wallet.publicKey
      );
  
      try {
        await program.methods.updateToken({
          name: 'Test updated collection',
          symbol: 'TTTT',
          uri: 'https://test.com',
          tokenStandart: {
            nonFungible: {}
          },
          primarySaleHappened: false,
          collection: null,
          collectionDetails: {
            v1: { size: new anchor.BN(0) }
          },
          decimals: null,
          printSupply: {
            zero: {}
          },
          creators: [{
              address: statePda,
              verified: true,
              share: 100
          }]
        })
          .accounts({
            state: statePda,
            mint: petCollectionMint,
            metadataAccount: petCollectionMetadata,
            masterEdition: petCollectionMasterEdition,
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            ataAccount: tokenAccount,
            sysvarInstructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY
          })
          .rpc()
      } catch (error) {
        expect(error).not.exist;
      }
      
    });

  it("Pet NFT can be minted", async () => {
    const airddropTx = await provider.connection.requestAirdrop(secondUserProvider.wallet.publicKey, 1000 * anchor.web3.LAMPORTS_PER_SOL);
    
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airddropTx
    });

    const tokenAccount = await getAssociatedTokenAddress(
      petNFTMint,
      secondUserProvider.wallet.publicKey
    )

    try {
      await secondUserProgram.methods.createToken(
        PET_NFT_MINT_SEED,
        new anchor.BN(1),
        {
          name: 'Test collection',
          symbol: 'TTT',
          uri: 'https://test.com',
          tokenStandart: {
            nonFungible: {}
          },
          primarySaleHappened: false,
          collection: {
            key: petCollectionMint,
            verified: false
          },
          collectionDetails: null,
          decimals: null,
          printSupply: {
            zero: {}
          },
          creators: [{
              address: statePda,
              verified: true,
              share: 100
          }]
        },
      )
        .accounts({
          state: statePda,
          mint: petNFTMint,
          metadataAccount: petMetadata,
          masterEdition: petMasterEdition,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          ataAccount: tokenAccount,
          collectionMint: petCollectionMint,
          collectionMetadata: petCollectionMetadata,
          collectionMasterEdition: petCollectionMasterEdition
        })
        .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
        .rpc()
  
      const account = await getAccount(provider.connection, tokenAccount);
  
      expect(account.amount).to.equal(BigInt(1));
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }
  });

  it('Asset collection can be created', async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      assetCollectionMint,
      provider.wallet.publicKey
    )

    try {
      await program.methods.createToken(
        ASSET_COLLECTION_MINT_SEED,
        new anchor.BN(1),
        {
          name: 'Asset collection',
          symbol: 'BBB',
          uri: 'https://test.com',
          tokenStandart: {
            nonFungible: {}
          },
          primarySaleHappened: false,
          collection: null,
          collectionDetails: {
            v1: { size: new anchor.BN(0) }
          },
          decimals: null,
          printSupply: {
            zero: {}
          },
          creators: [{
              address: statePda,
              verified: true,
              share: 100
          }]
        },
      )
        .accounts({
          state: statePda,
          mint: assetCollectionMint,
          metadataAccount: assetCollectionMetadata,
          masterEdition: assetCollectionMasterEdition,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          ataAccount: tokenAccount,
          collectionMint: null,
          collectionMetadata: null,
          collectionMasterEdition: null
        })
        .rpc()
  
      const account = await getAccount(provider.connection, tokenAccount);
  
      expect(account.amount).to.equal(BigInt(1));
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }
    
  });

  it('Asset NFT can be created', async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      assetMint,
      provider.wallet.publicKey
    )

    try {
      await program.methods.createToken(
        ASSET_TEST_MINT_SEED,
        new anchor.BN(10),
        {
          name: 'Asset fungible',
          symbol: 'ASS',
          uri: 'https://test.com',
          tokenStandart: {
            fungibleAsset: {}
          },
          primarySaleHappened: false,
          collection: {
            key: assetCollectionMint,
            verified: false
          },
          collectionDetails: null,
          decimals: 0,
          printSupply: {
            zero: {}
          },
          creators: [{
              address: statePda,
              verified: true,
              share: 100
          }]
        },
      )
        .accounts({
          state: statePda,
          mint: assetMint,
          metadataAccount: assetMetadata,
          masterEdition: null,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          ataAccount: tokenAccount,
          collectionMint: assetCollectionMint,
          collectionMetadata: assetCollectionMetadata,
          collectionMasterEdition: assetCollectionMasterEdition
        })
        .rpc()
  
      const account = await getAccount(provider.connection, tokenAccount);
  
      expect(account.amount).to.equal(BigInt(10));
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }
    
  });
});
