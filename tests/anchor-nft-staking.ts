import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { AnchorNftStaking } from "../target/types/anchor_nft_staking"
import { setupNft } from "./utils/setupNft"
import { PROGRAM_ID as METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import { expect } from "chai"
import { getAccount } from "@solana/spl-token"
import { findCandyMachinesByPublicKeyFieldOperation } from "@metaplex-foundation/js"
import { Keypair, SystemProgram } from "@solana/web3.js"

describe("anchor-nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.AnchorNftStaking as Program<AnchorNftStaking>

  const wallet = anchor.workspace.AnchorNftStaking.provider.wallet
  let  stakeList = Keypair.generate()
  let delegatedAuthPda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let tokenAddress: anchor.web3.PublicKey
  let userFixedPoolKey: anchor.web3.PublicKey
  let NftListPda: anchor.web3.PublicKey


  // LOOPS FOR CERTAIN  SCRITPS


  before(async () => {
    ;({ nft, delegatedAuthPda, mint, mintAuth, tokenAddress, userFixedPoolKey,NftListPda } =
      await setupNft(program, wallet.payer))
  })

  it("New Account", async () => {

    // Add your test here.
    const stake = await program.methods
      .createStakingAccount()
      .accounts({
        

        // stakeList: userFixedPoolKey,

      })
      .rpc({
        skipPreflight: true
      })

      console.log(stake)

  })

  // it("Increases Space", async () => {
  //   const incrspace = await program.methods.increaseStakeCapacity().accounts({stakeAccountList: NftListPda}).rpc({skipPreflight: true})
  //   console.log(incrspace)
  // })

  it("Stakes", async () => {
    const [staked_nft_address] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("staked-nft"), nft.mintAddress.toBuffer()],
      program.programId
    );
      console.log(staked_nft_address.toBase58())
    // Add your test here.
    const stake = await program.methods
      .stake()
      .accounts({
        
        nftTokenAccount: nft.tokenAddress,
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        nftMetadataAccount: nft.metadataAddress,
        stakeAccountList: NftListPda,
        // stakeList: userFixedPoolKey,
        metadataProgram: METADATA_PROGRAM_ID,
         
      })
      .rpc({
        skipPreflight: true
      })

      console.log(stake)

  })

  it("Withdrawing Fees", async () => {

    // Add your test here.
    const stake = await program.methods
      .withdrawFees()
      .accounts({
         
      })
      .rpc({
        skipPreflight: true
      })

      console.log(stake)

  })

  it("Redeems", async () => {
    const redeem = await program.methods
      .redeem()
      .accounts({
        
        stakeMint: mint,
        userStakeAta: tokenAddress,
      })
      .rpc({
        skipPreflight: true
      })

    console.log(redeem)
    const tokenAccount = await getAccount(provider.connection, tokenAddress)
  })



  
  it("Unstakes", async () => {

    const unstakeprep = await program.methods.prepunstake().accounts({
      nftTokenAccount: nft.tokenAddress,
      nftMint: nft.mintAddress,
      stakeAccountList: NftListPda,
      nftEdition: nft.masterEditionAddress,
      metadataProgram: METADATA_PROGRAM_ID,
    }).rpc()
      console.log(unstakeprep)
    const unstake = await program.methods
      .unstake()
      .accounts({
        nftTokenAccount: nft.tokenAddress,
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        metadataProgram: METADATA_PROGRAM_ID,
        stakeMint: mint,
        userStakeAta: tokenAddress,
      })
      .rpc({skipPreflight: true})

      console.log(unstake)

  })
})
