import * as anchor from "@coral-xyz/anchor";
import { ASSET_COLLECTION_MINT_SEED, ASSET_STATE_SEED, ASSET_TEST_MINT_SEED, FREE_ASSETS_STATE_SEED, MPL_TOKEN_METADATA_PROGRAM_ID, PET_COLLECTION_MINT_SEED, PET_NFT_MINT_SEED, PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED, REAL_DOG_STATE_SEED, TOKEN_MINT_SEED, clockworkProvider, program, provider, secondUserProvider, threadId } from "./constants"

export const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(PROGRAM_STATE_SEED)],
    program.programId
  )

export const [realDogsState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(REAL_DOG_STATE_SEED)],
    program.programId
  );

export const [CollectableAssetState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(FREE_ASSETS_STATE_SEED)],
    program.programId
  );


export const [playerState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(PLAYER_STATE_SEED), secondUserProvider.wallet.publicKey.toBuffer()],
    program.programId
  )


export const [tokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(TOKEN_MINT_SEED), secondUserProvider.wallet.publicKey.toBuffer()],
    program.programId
  )

export const [tokenMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from('metadata'),
    MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    tokenMint.toBuffer()
  ],
  MPL_TOKEN_METADATA_PROGRAM_ID
  )

// Pet pda's
export const [petCollectionMint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(PET_COLLECTION_MINT_SEED), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

export const [petCollectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      petCollectionMint.toBuffer()
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  )

export const [petCollectionMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      petCollectionMint.toBuffer(),
      Buffer.from('edition')
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

export const [petNFTMint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(PET_NFT_MINT_SEED), secondUserProvider.wallet.publicKey.toBuffer()],
    program.programId
  );

export const [petState] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from(PET_STATE_SEED), petNFTMint.toBuffer()],
  program.programId
);

export const [petMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      petNFTMint.toBuffer()
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  )

export const [petMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      petNFTMint.toBuffer(),
      Buffer.from('edition')
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

export const [threadAddress, threadBump] = clockworkProvider.getThreadPDA(statePda, threadId);

// Asset pda's
// Pet pda's
export const [assetCollectionMint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(ASSET_COLLECTION_MINT_SEED), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

export const [assetCollectionMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      assetCollectionMint.toBuffer()
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  )

export const [assetCollectionMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      assetCollectionMint.toBuffer(),
      Buffer.from('edition')
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

export const [assetMint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(ASSET_TEST_MINT_SEED), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

export const [freeAssetState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(FREE_ASSETS_STATE_SEED), assetMint.toBuffer(), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

export const [assetState] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from(ASSET_STATE_SEED), assetMint.toBuffer()],
  program.programId
);

export const [assetMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      assetMint.toBuffer()
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  )
