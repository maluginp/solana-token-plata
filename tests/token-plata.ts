import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TokenPlata } from "../target/types/token_plata";

describe("token-plata", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TokenPlata as Program<TokenPlata>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
