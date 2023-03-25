import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { AnchorNftStaking } from "../target/types/anchor_nft_staking"
import { setupNft } from "./utils/setupNft"
import { PROGRAM_ID as METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import { expect } from "chai"
import { getAccount } from "@solana/spl-token"
import { findCandyMachinesByPublicKeyFieldOperation } from "@metaplex-foundation/js"
import { SystemProgram } from "@solana/web3.js"

describe("anchor-nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.AnchorNftStaking as Program<AnchorNftStaking>

  const wallet = anchor.workspace.AnchorNftStaking.provider.wallet

  let delegatedAuthPda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let tokenAddress: anchor.web3.PublicKey
  let userFixedPoolKey: anchor.web3.PublicKey

  before(async () => {
    ;({ nft, delegatedAuthPda, mint, mintAuth, tokenAddress, userFixedPoolKey } =
      await setupNft(program, wallet.payer))
  })

  it("Stakes", async () => {

    // Add your test here.
    const stake = await program.methods
      .stake()
      .accounts({
        nftTokenAccount: nft.tokenAddress,
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        // stakeList: userFixedPoolKey,
        metadataProgram: METADATA_PROGRAM_ID,

      })
      .rpc()

      console.log(stake)

  })

  it("Redeems", async () => {
    const redeem = await program.methods
      .redeem()
      .accounts({
        nftTokenAccount: nft.tokenAddress,
        stakeMint: mint,
        userStakeAta: tokenAddress,
      })
      .rpc()

    console.log(redeem)
    const tokenAccount = await getAccount(provider.connection, tokenAddress)
  })

  it("Unstakes", async () => {
    await program.methods
      .unstake()
      .accounts({
        nftTokenAccount: nft.tokenAddress,
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        metadataProgram: METADATA_PROGRAM_ID,
        stakeMint: mint,
        userStakeAta: tokenAddress,
      })
      .rpc()


  })
})
