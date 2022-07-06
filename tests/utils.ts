import * as anchor from "@project-serum/anchor";

const LAMPORTS_PER_SOL = 1_000_000_000;

export const airDropSol = async (
    connection: anchor.web3.Connection, 
    pk: anchor.web3.PublicKey, 
    sol: number
) => {
    try {
      const airdropSignature = await connection.requestAirdrop(
        pk,
        sol * LAMPORTS_PER_SOL
      );

      const latestBlockHash = await connection.getLatestBlockhash();

      await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
      });
    } catch (error) {
      console.error(error);
    }
  };
