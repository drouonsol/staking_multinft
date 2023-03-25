import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js"
import { createMint, getAssociatedTokenAddress } from "@solana/spl-token"
import * as anchor from "@project-serum/anchor"

export const setupNft = async (program, payer) => {
  const metaplex = Metaplex.make(program.provider.connection)
    .use(keypairIdentity(payer))
    .use(bundlrStorage())

  const nft = await metaplex
    .nfts()
    .create({
      uri: "https://media.discordapp.net/attachments/890670720705777785/1084419960484397177/Untitled_Artwork.png?width=878&height=658",
      name: "Test nft",
      sellerFeeBasisPoints: 500,
    })
    .run()

  console.log("nft metadata pubkey: ", nft.metadataAddress.toBase58())
  console.log("nft token address: ", nft.tokenAddress.toBase58())
  const [delegatedAuthPda] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("authority")],
    program.programId
  )


  

  console.log("delegated authority pda: ", delegatedAuthPda.toBase58())

  const [mintAuth] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("mint")],
    program.programId
  )

  const [stakedList] = await anchor.web3.PublicKey.findProgramAddress(
    [payer.publicKey.toBuffer(),Buffer.from("")],
    program.programId
  )

  const mint = await createMint(
    program.provider.connection,
    payer,
    mintAuth,
    null,
    2
  )
  console.log("Mint pubkey: ", mint.toBase58())

  const tokenAddress = await getAssociatedTokenAddress(mint, payer.publicKey)

  return {
    nft: nft,
    delegatedAuthPda: delegatedAuthPda,
    mint: mint,
    mintAuth: mintAuth,
    tokenAddress: tokenAddress,
  }
}
