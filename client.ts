const client   = anchor.web3.Keypair.generate();
const shop   = pg.wallet.publicKey;

 const [receipts_account, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      shop.toBuffer(),
      client.publicKey.toBuffer(),
    ],
    pg.program.programId
  );

console.log("start");
  await pg.program.methods
  .initialize()
  .accounts({
    pdaAccount: receipts_account,
    signer: pg.wallet.publicKey,
    client: client.publicKey,
    systemProgram:  anchor.web3.SystemProgram.programId
  })
  .rpc();
  console.log("abcdef");

for (let i = 0;i < 2;++i){
await pg.program.methods
  .mint()
  .accounts({
    pdaAccount: receipts_account,
    signer: pg.wallet.publicKey,
    client: client.publicKey,
    systemProgram:  anchor.web3.SystemProgram.programId
  })
  .rpc();

const info = await pg.connection.getAccountInfo(receipts_account);
const receipt = pg.program.coder.accounts.decode("Receipt", info!.data);
const asNumbers = receipt.timestamps.map((bn: BN) => bn.toNumber());
console.log("timestamps (numbers):", asNumbers);
}
