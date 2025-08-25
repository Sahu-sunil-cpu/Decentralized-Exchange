import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Contract } from "../target/types/contract";
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

describe("contract", async () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.Contract as Program<Contract>;

  let  mint_a;
  let  mint_b;

  before(async () => {
 mint_a = await createMint(
    connection,
    wallet.payer,
    wallet.publicKey,
    null,
    6
  );

  mint_b = await createMint(
    connection,
    wallet.payer,
    wallet.publicKey,
    null,
    6
  );

  
    const tx = await program.methods.initializePool(
      Number(23)
    )
      .accounts({
        reserveAMint: mint_a,
        reserveBMint: mint_b,

      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("initialize pool --> ", tx);
  })

  it("initialize pool", async () => {

  });

  it("initialize reserves", async () => {

    console.log(mint_a, mint_b)
     const tx1 = program.methods.initializeReserveB()
     .accounts({
        reserveBMint: mint_b
        
     })
     .rpc({ skipPreflight: true, commitment: "confirmed"});

    //   const tx2 = await program.methods.initializeReserveB()
    //  .accounts({
    //     reserveBMint: mint_b,
    //  })
    //  .rpc({ skipPreflight: true, commitment: "confirmed"});


    console.log("reserve a --> ", tx1);
  })

  // it("intialize lp token account",async () => {
  //    const tx = await program.methods.initializeLpTokenAccount()
  //    .accounts({
     
  //    })
  //    .rpc({skipPreflight: true, commitment: "confirmed"})

  //   console.log("initialize lp token account --> ", tx);

  // })

  // it("intialize liquidity provider ", async () => {
  //   const tx = await program.methods.initializeLiquidityProvider()
  //   .accounts({

  //   })
  //   .rpc({skipPreflight: true, commitment: "confirmed"})
  // })

  //   it("fail deposit liquidity with zero tokens in ata", async () => {

  //       // derive buyer ATA
  // const depositorReserveAAta = getAssociatedTokenAddressSync(
  //   mint_a,
  //   wallet.publicKey,
  //   false,
  //   TOKEN_2022_PROGRAM_ID
  // );

  // const depositorReserveBAta = getAssociatedTokenAddressSync(
  //   mint_a,
  //   wallet.publicKey,
  //   false,
  //   TOKEN_2022_PROGRAM_ID
  // );

  // // create ATA on-chain
  // await createAssociatedTokenAccount(
  //   connection,
  //   wallet.payer, // payer
  //   mint_a,
  //   wallet.publicKey,
  //   undefined,
  //   TOKEN_2022_PROGRAM_ID
  // );

  // await createAssociatedTokenAccount(
  //   connection,
  //   wallet.payer, // payer
  //   mint_b,
  //   wallet.publicKey,
  //   undefined,
  //   TOKEN_2022_PROGRAM_ID
  // );

  //   const tx = await program.methods.deposit(
  //     new anchor.BN(12),
  //     new anchor.BN(12)
  //   )
  //   .accounts({
  //     depositorReserveAAta,
  //     depositorReserveBAta,
  //   })
  //   .rpc({skipPreflight: true, commitment: "confirmed"})
  // })
});
