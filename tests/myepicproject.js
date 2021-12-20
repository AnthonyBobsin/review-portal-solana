const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('myepicproject', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it('Is initialized!', async () => {
    const program = anchor.workspace.Myepicproject;
    // Create an account keypair for our program to use.
    const baseAccount = anchor.web3.Keypair.generate();

    const tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log("Your transaction signature", tx);

    // Fetch data from the account.
    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 Product Count', account.totalProducts.toString())

    // Update product count
    await program.rpc.addProduct("https://media4.giphy.com/media/4JZB66hwV8xMOKKrKr/giphy.gif", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey
      },
    });
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 Product Count', account.totalProducts.toString())

    // Access product_list on the account!
    console.log('👀 Product List', account.productList);
  });
});
