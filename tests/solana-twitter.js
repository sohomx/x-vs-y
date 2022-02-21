const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { Transaction } = require("@solana/web3.js");
const { SystemProgram } = anchor.web3;
describe("ronaldo-vs-messi", () => {
  // Configure the client
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.RonaldoVsMessi;
  const voteAccount = anchor.web3.Keypair.generate();
  it("Initialized with 0 votes for ronaldo and messi", async() => {
    console.log("Testing Initialize...")
    // The last element passed to RPC methods is always the Transaction options. Because voteAccount is being created here, we are required to pass it as a signers array

    await program.rpc.initialize({
      accounts: {
        voteAccount: voteAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [voteAccount],
    });
    const account = await program.account.voteAccount.fetch(voteAccount.publicKey);
    console.log("Ronaldo: ", account.ronaldo.toString());
    console.log("Messi: ", account.messi.toString());
    assert.ok(
      account.ronaldo.toString() == 0 && account.messi.toString() == 0
    );
  });
  it("Votes correctly for Ronaldo", async () => {
    console.log("Testing voteRonaldo...");
    await program.rpc.voteRonaldo({
      accounts: {
        voteAccount: voteAccount.publicKey,
      },
    });
    const account = await program.account.voteAccount.fetch(
      voteAccount.publicKey
    );
    console.log("Ronaldo: ", account.ronaldo.toString());
    console.log("Messi: ", account.messi.toString());
    assert.ok(
      account.ronaldo.toString() == 1 && account.messi.toString() == 0
    );
  });
  it("Votes correctly for messi", async() => {
    console.log("Testing voteMessi...");
    await program.rpc.voteMessi({
      accounts: {
        voteAccount: voteAccount.publicKey,
      },
    });
    const account = await program.account.voteAccount.fetch(voteAccount.publicKey);
    console.log("Ronaldo: ", account.ronaldo.toString());
    console.log("Messi: ", account.messi.toString());
    assert.ok(
      account.ronaldo.toString() == 1 & account.messi.toString() == 1
    );
  });
});