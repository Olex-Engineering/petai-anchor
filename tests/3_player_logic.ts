import * as anchor from "@coral-xyz/anchor";
import { MPL_TOKEN_METADATA_PROGRAM_ID, clockworkProvider, program, provider, secondUserProgram, secondUserProvider, threadId } from "./constants";
import { petNFTMint, statePda, playerState, petCollectionMint, petCollectionMetadata, petCollectionMasterEdition, petMetadata, petMasterEdition, threadAddress, petState, realDogsState } from "./pdas";
import { print_thread } from "./utils";
import { expect } from "chai";

describe("Player logic", () => {
    anchor.setProvider(provider);

    it('player can initialize account', async () => {
        const airddropTx = await provider.connection.requestAirdrop(secondUserProvider.wallet.publicKey, 1000 * anchor.web3.LAMPORTS_PER_SOL);
    
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airddropTx
        });
    
        try {
          const tx = await secondUserProgram.methods.initPlayerState(
            [],
            Buffer.from(threadId),
            {
              wallet: provider.wallet.publicKey,
            },
          )
          .accounts({
            state: statePda,
            playerState: playerState,
            realDogsConfigState: realDogsState,
            petState: petState,
            petNftMint: petNFTMint,
            metadataAccount: petMetadata,
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            clockworkProgram: clockworkProvider.threadProgram.programId,
            thread: threadAddress
          })
          .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 })])
          .rpc()
      
          await print_thread(clockworkProvider, threadAddress);
        } catch (error) {
          console.log(error);
          expect(error).not.exist;
        }
        
      });
    
    
      it("player account will be updated every 10 sec", async () => {
        const initialAccont = await program.account.petState.fetch(petState);
        let initalFood = initialAccont.food;

        setTimeout(() => {
          const interval = setInterval(async () => {
            const petStateAccount = await program.account.petState.fetch(petState);
            initalFood = petStateAccount.food;
            console.log('PET_STATE_FOOD_DECREASING: -' + petStateAccount.food);
          }, 1000);
      
          setTimeout(() => {
            clearInterval(interval);
          }, 10000)
        }, 1200);
        
      });
});