import * as anchor from "@coral-xyz/anchor";
import { MPL_TOKEN_METADATA_PROGRAM_ID, clockworkProvider, program, provider, secondUserProgram, secondUserProvider, threadId } from "./constants";
import { statePda, playerState, petCollectionMint, petCollectionMetadata, petCollectionMasterEdition, threadAddress, realDogsState, getPetNftMint, getPetState, getPetMatadata, getRandomTreadIdWithAddress, getThreadAddressById } from "./pdas";
import { print_thread } from "./utils";
import { expect } from "chai";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("Player logic", () => {
    anchor.setProvider(provider);
    const [petNftMint] = getPetNftMint(secondUserProvider.wallet.publicKey);
    const [firstUserPetNftMint] = getPetNftMint(provider.wallet.publicKey);

    it('player can initialize account', async () => {
        const airddropTx = await provider.connection.requestAirdrop(secondUserProvider.wallet.publicKey, 1000 * anchor.web3.LAMPORTS_PER_SOL);
    
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airddropTx
        });

        const effectName = 'Test effect';

        const [effectState] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(effectName)],
          program.programId
        );

        const tokenAccount = getAssociatedTokenAddressSync(petNftMint, secondUserProvider.wallet.publicKey);

        try {
          const tx = await secondUserProgram.methods.initPlayerState(
            provider.wallet.publicKey
          )
          .accounts({
            playerState: playerState,
            realDogsConfigState: realDogsState,
          })
          .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
          .rpc()

          const txPet = await secondUserProgram.methods.initPet(
            [],
            Buffer.from(threadId),
          )
          .accounts({
            state: statePda,
            playerState: playerState,
            petState: getPetState(petNftMint)[0],
            petNftMint: petNftMint,
            petNftMintAta: tokenAccount,
            metadataAccount: getPetMatadata(petNftMint)[0],
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            clockworkProgram: clockworkProvider.threadProgram.programId,
            thread: threadAddress
          })
          .remainingAccounts([{
            pubkey: effectState,
            isSigner: false,
            isWritable: false
          }])
          .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
          .rpc()

          console.log(txPet);
          await print_thread(clockworkProvider, threadAddress);
        } catch (error) {
          console.log(error);
          expect(error).not.exist;
        }
        
      });
    
    
      it("pet account will be updated every 10 sec", (done) => {
        program.account.petState.fetch(getPetState(petNftMint)[0]).then((initialAccont) => {
          let initalFood = initialAccont.food;

          setTimeout(() => {
            const interval = setInterval(async () => {
              const petStateAccount = await program.account.petState.fetch(getPetState(petNftMint)[0]);
              initalFood = petStateAccount.food;
              console.log('PET_STATE_FOOD_DECREASING: -' + petStateAccount.food);
            }, 1000);
        
            setTimeout(() => {
              clearInterval(interval);
              done();
            }, 10000)
          }, 1200);
        });
      });

      it('User can init second pet state', async () => {
        try {
          const { id, threadAddress: thread } = getRandomTreadIdWithAddress();
          
          const secondUserTokenAccountWithFirstUserNft = getAssociatedTokenAddressSync(
            firstUserPetNftMint,
            secondUserProvider.wallet.publicKey
          )

          const txPet = await secondUserProgram.methods.initPet(
            [],
            Buffer.from(id),
          )
          .accounts({
            state: statePda,
            playerState: playerState,
            petState: getPetState(firstUserPetNftMint)[0],
            petNftMint: firstUserPetNftMint,
            petNftMintAta: secondUserTokenAccountWithFirstUserNft,
            metadataAccount: getPetMatadata(firstUserPetNftMint)[0],
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            clockworkProgram: clockworkProvider.threadProgram.programId,
            thread: thread[0]
          })
          .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
          .rpc()
        } catch (error) {
          console.log(error);
          expect(error).not.exist;
        }
      });

      it("Second pet account will be updated every 10 sec", (done) => {
        program.account.petState.fetch(getPetState(firstUserPetNftMint)[0]).then((initialAccont) => {
          let initalFood = initialAccont.food;

          setTimeout(() => {
            const interval = setInterval(async () => {
              const petStateAccount = await program.account.petState.fetch(getPetState(firstUserPetNftMint)[0]);
              initalFood = petStateAccount.food;
              console.log('PET_STATE_FOOD_DECREASING: -' + petStateAccount.food);
            }, 1000);
        
            setTimeout(() => {
              clearInterval(interval);
              done();
            }, 10000)
          }, 1200);
        });
      });

      it("User can change first pet nft to second", async () => {
        try {
          const tokenAccount = getAssociatedTokenAddressSync(petNftMint, secondUserProvider.wallet.publicKey);
          const petState = await program.account.petState.fetch(getPetState(petNftMint)[0]);

          const txPet = await secondUserProgram.methods.updatePlayerPet(
          )
          .accounts({
            state: statePda,
            playerState: playerState,
            petState: getPetState(petNftMint)[0],
            newPetNftMint: petNftMint,
            newPetNftMintAta: tokenAccount,
            metadataAccount: getPetMatadata(petNftMint)[0],
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            clockworkProgram: clockworkProvider.threadProgram.programId,
            thread: getThreadAddressById(petState.threadId.toString())[0]
          })
          .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
          .rpc()
        } catch (error) {
          console.log(error);
          expect(error).not.exist;
        }
      })
});