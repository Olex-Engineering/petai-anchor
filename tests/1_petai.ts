import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { provider, program, secondUserProvider,} from "./constants";
import { statePda, tokenMint, petCollectionMint, assetCollectionMint } from "./pdas";

describe("Initialization and state", () => {
  anchor.setProvider(provider);
  

  it("Is initialized!", async () => {
    try {
      const state = await program.account.programState.fetch(statePda);
    } catch {
      try {
        const tx = await program.methods.initialize().accounts({
          state: statePda,
        }).rpc();
      } catch(error) {
        console.log(error);
        expect(error).not.exist;
      }
    }
  });

  it('program state can be updated', async () => {
    const initialState = await program.account.programState.fetch(statePda);

    try {
      const tx = await program.methods.updateProgramState({
        bump: initialState.bump,
        authority: provider.wallet.publicKey,
        petCollection: petCollectionMint,
        assetCollection: assetCollectionMint,
        decorCollection: anchor.web3.Keypair.generate().publicKey,
        realDogsConfigs: [{
          wallet: secondUserProvider.wallet.publicKey,
          uri: 'http:test.ocm'
        }]
      }).accounts({
        state: statePda
      }).rpc();
  
      const state = await program.account.programState.fetch(statePda);
      expect(state.bump).to.exist;
    } catch (error) {
      console.log(error);
      expect(error).not.exist;
    }
    
  });
});
