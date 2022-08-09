import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorBpfTemplate } from "../target/types/anchor_bpf_template";

describe("anchor_bpf_template", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorBpfTemplate as Program<AnchorBpfTemplate>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
