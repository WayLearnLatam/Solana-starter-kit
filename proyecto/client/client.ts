// Client
console.log("My address:", pg.wallet.publicKey.toString());
console.log("Program ID:", pg.PROGRAM_ID.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
const txHash = await pg.program.methods.saludo().rpc();
console.log(txHash);