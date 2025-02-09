import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VoteProgram } from "../target/types/vote_program";

describe("vote_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.VoteProgram as Program<VoteProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
