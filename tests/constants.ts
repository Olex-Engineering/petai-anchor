import { ClockworkProvider } from "@clockwork-xyz/sdk";
import { Program } from "@coral-xyz/anchor";
import { Petai } from "../target/types/petai";
import * as anchor from "@coral-xyz/anchor";

export const PROGRAM_STATE_SEED = 'state';
export const REAL_DOG_STATE_SEED = 'real-dogs-state';
export const FREE_ASSETS_STATE_SEED = 'free-assets-state';
export const TOKEN_MINT_SEED = 'token-mint'
export const PET_COLLECTION_MINT_SEED = 'pet-collection-mint-seed';
export const ASSET_COLLECTION_MINT_SEED = 'asset-collection-mint-seed';
export const PLAYER_STATE_SEED = 'player-state';
export const PET_STATE_SEED = 'pet-state';
export const ASSET_STATE_SEED = 'asset-state';
export const DECOR_STATE_SEED = 'decor-state';
export const PET_NFT_MINT_SEED = 'pet-nft-mint';
export const ASSET_TEST_MINT_SEED = 'asset-test-mint';
export const MPL_TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
export const secondUser = anchor.web3.Keypair.generate();

export const program = anchor.workspace.Petai as Program<Petai>;

// Configure the client to use the local cluster.
export const provider = anchor.AnchorProvider.env();
export const secondUserProvider = new anchor.AnchorProvider(provider.connection, new anchor.Wallet(secondUser), {});
export const secondUserProgram = new anchor.Program(program.idl as anchor.Idl, program.programId, secondUserProvider);
export const clockworkProvider = ClockworkProvider.fromAnchorProvider(provider);
export const threadId = "counter-" + new Date().getTime() / 1000;