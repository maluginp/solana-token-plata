import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TokenPlata } from "../target/types/token_plata";
import * as assert from "assert";
import { BN } from "bn.js";
import { airDropSol } from "./utils";
import { rpc } from "@project-serum/anchor/dist/cjs/utils";

const { SystemProgram } = anchor.web3;

const logTx = async (provider, tx) => {
  await provider.connection.confirmTransaction(tx, "confirmed");
  console.log(
    (await provider.connection.getConfirmedTransaction(tx, "confirmed")).meta
      .logMessages
  );
};


describe("token", () => {
  // Configure the client to use the local cluster.
  // const provider = anchor.AnchorProvider.local();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TokenPlata as Program<TokenPlata>;

  console.log(program.programId.toBase58());
  const mint_auth = anchor.web3.Keypair.generate();
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();



  it("Initialize start state", async () => {

    await airDropSol(
      program.provider.connection,
      mint_auth.publicKey,
      10
    );

    await airDropSol(
      program.provider.connection,
      user1.publicKey,
      10
    );

    await airDropSol(
      program.provider.connection,
      user2.publicKey,
      10
    );


  });

  it("Simple Test", async () => {

    // PDA - details - https://solanacookbook.com/core-concepts/pdas.html
    // shortly, PDA able 
    let [mint, num] = (await anchor.web3.PublicKey.findProgramAddress(
      [ mint_auth.publicKey.toBuffer() ],
      program.programId,
    ));

    assert.ok(!anchor.web3.PublicKey.isOnCurve(mint));
    console.log(`Mint number = ${num}`);
    console.log(`Mint key = ${mint.toBase58()}`);

    // Creating new token
    let txid = await program.methods.initializeMint()
      .accounts({
        mint: mint,
        payer: mint_auth.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([mint_auth])
      .rpc()
    ;
    await logTx(program.provider, txid);
    
    let mintData = await program.account.mintData.fetch(mint)
    assert.ok(mintData.supply.eq(new BN(0)));
    assert.equal(mintData.authority.toBase58(), mint_auth.publicKey.toBase58());

    // assert.ok(mintData.amount.eq(new BN(0)));
        // assert.equal(mintData.mint.toBase58(), mint.toBase58());

    // Generate PDA  
    let [user1TA, user1Num] = (await anchor.web3.PublicKey.findProgramAddress(
      [
        user1.publicKey.toBuffer(), 
        mint.toBuffer()
      ], program.programId
    ));

    console.log(`User 1 number = ${user1Num}`);

    txid = await program.methods.initializeTokenAccount()
      .accounts({
        tokenAccount: user1TA,
        mint: mint, // PDA to Token
        payer: user1.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    await logTx(program.provider, txid);

    // Minting is able for owner
    txid = await program.methods.mint(new anchor.BN(10))
     .accounts({
        dst: user1TA,
        mint: mint,
        authority: mint_auth.publicKey,
      })
      .signers([mint_auth])
      .rpc()

    await logTx(program.provider, txid);

    mintData = await program.account.mintData.fetch(mint)
    assert.ok(mintData.supply.eq(new BN(10)));
    assert.equal(mintData.authority.toBase58(), mint_auth.publicKey.toBase58());



    let user2TA = (await anchor.web3.PublicKey.findProgramAddress(
      [user2.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];
    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: user2TA,
        mint: mint,
        payer: user2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.mint(new anchor.BN(5), {
      accounts: {
        dst: user1TA,
        mint: mint,
        authority: mint_auth.publicKey,
      },
      signers: [mint_auth],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.mint(new anchor.BN(10), {
      accounts: {
        dst: user2TA,
        mint: mint,
        authority: mint_auth.publicKey,
      },
      signers: [mint_auth],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.transfer(new anchor.BN(7), {
      accounts: {
        src: user2TA,
        dst: user1TA,
        owner: user2.publicKey,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.burn(new anchor.BN(3), {
      accounts: {
        src: user2TA,
        mint: mint,
        owner: user2.publicKey,
      },
      signers: [user2],
    });
    await logTx(program.provider, txid);
  });
});

