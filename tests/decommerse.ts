import * as anchor from "@coral-xyz/anchor";
import fs from 'fs';
const { PublicKey, SystemProgram } = anchor.web3;
import { Decommerse } from "../target/types/decommerse";

const idl = require("../target/idl/decommerse.json");


describe("decommerse", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = new anchor.Program<Decommerse>(idl, provider);

  it('creates new user', async () => {
    const creator = provider.wallet;

    const USERNAME = "USRNAME";

    const [userProfileStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_profile"), creator.publicKey.toBuffer()],
      program.programId,
    )
    try {
      const acc = await program.account.userProfile.fetch(userProfileStatePda);
      console.log("usr already initialized");
      console.log(acc);
    } catch (error) {
      const tx = await program.methods.createUserProfile(USERNAME).accountsPartial(
        {
          userProfile: userProfileStatePda,
          user: creator.publicKey,
          systemProgram: SystemProgram.programId,
        }
      ).rpc();
      console.log("user initialized successful signature: ", tx);
    }
  })
  it('creates new product', async () => {
    const [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId,
    );
    const state = await program.account.programState.fetch(programStatePda);
    let pid = state.productCount.add(new anchor.BN(1));
    const seller = provider.wallet;
    const [productPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("product"),
        seller.publicKey.toBuffer(),
        pid.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )
    const PRODUCT_NAME = "I PHONE";
    const PRODUCT_DESCRIPTION = "This is the newly launched I PHONE";
    const PRODUCT_METADATA = "METADATA.JSON";
    const PRODUCT_PRICE = new anchor.BN(3 * 1_000_000_000);
    const PRODUCT_STOCK = new anchor.BN(10);
    const tx = await program.methods.createProduct(PRODUCT_NAME, PRODUCT_DESCRIPTION, PRODUCT_METADATA, PRODUCT_PRICE, PRODUCT_STOCK)
      .accountsPartial({
        product: productPda,
        seller: seller.publicKey,
        programState: programStatePda,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const product = await program.account.product.fetch(productPda);
    console.log("PRODUCT CREATED, SIGNATURE: ", tx);
    console.log(product);
  })
  it('update the stocks', async () => {
    const seller = provider.wallet;
    const [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId,
    );
    const state = await program.account.programState.fetch(programStatePda);
    let pid = state.productCount;
    console.log("TRANSACTION COUNT: ",state.transactionCount);
    const [productPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("product"),
        seller.publicKey.toBuffer(),
        pid.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )
    const PRODUCT_STOCK = new anchor.BN(4);
    const tx = await program.methods.updateProductStock(PRODUCT_STOCK)
      .accountsPartial({
        product: productPda,
        programState: programStatePda,
        seller: seller.publicKey,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const product = await program.account.product.fetch(productPda);
    console.log("PRODUCT CREATED, SIGNATURE: ", tx);
    console.log(product);
  })

  it('purchases product', async () => {
    const seller = provider.wallet;
    const keypairPath = './user.json'
    const keyPairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
    const wallet = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(keyPairData));
    const [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId,
    );
    const state = await program.account.programState.fetch(programStatePda);
    let pid = state.productCount;
    const [productPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("product"),
        seller.publicKey.toBuffer(),
        pid.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )
    let tid = state.transactionCount.add(new anchor.BN(1));
    const [transactionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("transaction"),
        wallet.publicKey.toBuffer(),
        seller.publicKey.toBuffer(),
        tid.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )

    const STOCK = new anchor.BN(2);
    const tx = await program.methods.purchaseProduct(pid, STOCK)
      .accountsPartial({
        buyer: wallet.publicKey,
        seller: seller.publicKey,
        product: productPda,
        transaction: transactionPda,
        programState: programStatePda,
        systemProgram: SystemProgram.programId,
      }).signers([wallet]).rpc();
    const product = await program.account.product.fetch(productPda);
    console.log("TX SUCCESSFUL SIGNATURE: ", tx);
    console.log(product);
    const transaction = await program.account.transaction.fetch(transactionPda);
    console.log(transaction);
  })
});
