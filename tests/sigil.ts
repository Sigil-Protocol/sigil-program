import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sigil } from "../target/types/sigil";
import { Connection } from "@solana/web3.js";
import assert from "assert";
import * as buffer from "node:buffer";

const RPC_URL = "http://127.0.0.1:8899";

describe("sigil", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const connection = new Connection(RPC_URL, "confirmed");
  const program = anchor.workspace.Sigil as Program<Sigil>;

  const [NETWORK_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("network")],
    program.programId
  );

  const [IDENTITY_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("did:sigil"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized", async () => {
    await program.methods
      .init()
      .accounts({
        network: NETWORK_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err) => {
        throw new Error(err);
      });

    const network: any = await program.account.network.fetch(NETWORK_PDA);

    console.log("network", network);

    assert.ok(network.admin.equals(payer.publicKey));
  });

  it("Creates an identity", async () => {
    const metadata = {
      age: 21,
      name: "Alice",
      email: "me@me.com",
    };
    const metadata_uri = "https://example.com";
    const metadata_merkle_root = Buffer.from("did:example:123");

    await program.methods
      .createIdentity(metadata_uri, metadata_merkle_root)
      .accounts({
        network: NETWORK_PDA,
        identity: IDENTITY_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err) => {
        console.log(err);
      });

    const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

    console.log("identity", identity);

    assert.ok(identity.owner.equals(payer.publicKey));
  });

  it("Updates an identity", async () => {
    const metadata_uri = "https://example.com";
    const metadata_merkle_root = Buffer.from("did:example:123");

    await program.methods
      .updateIdentity(metadata_uri, metadata_merkle_root)
      .accounts({
        network: NETWORK_PDA,
        identity: IDENTITY_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err) => {
        console.log(err);
      });

    const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

    console.log("updated identity", identity);

    assert.ok(identity.owner.equals(payer.publicKey));
  });
});
