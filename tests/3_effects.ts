import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { statePda } from "./pdas";
import { program, provider } from "./constants";

describe("Effects logic", () => {
  anchor.setProvider(provider);

  it('Effect can be created', async () => {
    const effectName = 'Test effect';

    const [effectState] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(effectName)],
        program.programId
      );

    
    await program.methods.putEffect(
      {
        name: effectName,
        effectType: {
            increase: {}
        },
        lonelinessImpact: 1,
        foodImpact: 2,
        loveImpact: 1,
        autoSet: {
          chanceOfAutoSet: 2,
          targetPetConditions: [
            {
              super: {}
            },
          ]
        }, // 50% = 100 / 2
        duration: 20, // 20 seconds
      })
    .accounts({
      effectState: effectState,
      state: statePda,
    })
    .preInstructions([anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 })])
    .rpc()

    const effectStatePda = await program.account.effectState.fetch(effectState);

    expect(effectStatePda.name).to.eq(effectName);
  });
});