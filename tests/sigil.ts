import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sigil } from "../target/types/sigil";
import { Connection } from "@solana/web3.js";
import assert from "assert";
import * as buffer from "node:buffer";
import { BN } from "@coral-xyz/anchor";

const RPC_URL = "http://127.0.0.1:8899";

describe("sigil", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const connection = new Connection(RPC_URL, "confirmed");
  const program = anchor.workspace.Sigil as Program<Sigil>;

  const [NETWORK_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("sigil")],
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
      .catch((err: any) => {
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
      .catch((err: any) => {
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
      .catch((err: any) => {
        console.log(err);
      });

    const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

    console.log("updated identity", identity);

    assert.ok(identity.owner.equals(payer.publicKey));
  });

  it("Creates an asset", async () => {
    const assets = await program.account.asset.all([
      {
        memcmp: {
          offset: 8, // This offset corresponds to the 'authority' field in the Asset struct
          bytes: payer.publicKey.toBase58(),
        },
      },
    ]);

    const nonce_string = assets.length.toString();
    const metadata_uri = "https://example.com";

    const [ASSET_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("asset"),
        payer.publicKey.toBuffer(),
        Buffer.from(nonce_string),
      ],
      program.programId
    );

    await program.methods
      .createAsset(nonce_string, payer.publicKey, metadata_uri)
      .accounts({
        network: NETWORK_PDA,
        asset: ASSET_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err: any) => {
        console.log(err);
      });

    const asset: any = await program.account.asset.fetch(ASSET_PDA);

    assert.ok(asset.owner.equals(payer.publicKey));
  });

  it("Transfers an asset", async () => {
    const assets = await program.account.asset.all([
      {
        memcmp: {
          offset: 8, // This offset corresponds to the 'authority' field in the Asset struct
          bytes: payer.publicKey.toBase58(),
        },
      },
    ]);

    const asset = assets[0];

    const recipient = anchor.web3.Keypair.generate().publicKey;

    await program.methods
      .transferAsset(recipient)
      .accounts({
        asset: asset.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err: any) => {
        console.log(err);
      });

    const updatedAsset: any = await program.account.asset.fetch(
      asset.publicKey
    );
  });

  it("Creates a second asset", async () => {
    const assets = await program.account.asset.all([
      {
        memcmp: {
          offset: 8, // This offset corresponds to the 'authority' field in the Asset struct
          bytes: payer.publicKey.toBase58(),
        },
      },
    ]);

    const nonce_string = assets.length.toString();
    const metadata_uri = "https://example.com";

    const [ASSET_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("asset"),
        payer.publicKey.toBuffer(),
        Buffer.from(nonce_string),
      ],
      program.programId
    );

    await program.methods
      .createAsset(nonce_string, payer.publicKey, metadata_uri)
      .accounts({
        network: NETWORK_PDA,
        asset: ASSET_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err: any) => {
        console.log(err);
      });

    const asset: any = await program.account.asset.fetch(ASSET_PDA);

    assert.ok(asset.owner.equals(payer.publicKey));
  });

  const recovery_account = anchor.web3.Keypair.generate();

  // add recovery account
  it("Adds a recovery account", async () => {
    await program.methods
      .addRecoveryAccount(recovery_account.publicKey)
      .accounts({
        network: NETWORK_PDA,
        identity: IDENTITY_PDA,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err: any) => {
        console.log(err);
      });

    const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

    console.log("identity", identity);

    assert.ok(identity.recoveryAccounts.length === 1);
  });

  // it("Removes a recovery account", async () => {
  //   await program.methods
  //     .removeRecoveryAccount(recovery_account.publicKey)
  //     .accounts({
  //       network: NETWORK_PDA,
  //       identity: IDENTITY_PDA,
  //       payer: payer.publicKey,
  //     })
  //     .rpc()
  //     .catch((err: any) => {
  //       console.log(err);
  //     });

  //   const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

  //   console.log("identity", identity);
  // });

  it("Recovers an identity", async () => {
    await program.methods
      .recover()
      .accounts({
        network: NETWORK_PDA,
        identity: IDENTITY_PDA,
        payer: recovery_account.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([recovery_account])
      .rpc()
      .catch((err: any) => {
        console.log(err);
      });

    const identity: any = await program.account.identity.fetch(IDENTITY_PDA);

    console.log("identity", identity);
  });
});
