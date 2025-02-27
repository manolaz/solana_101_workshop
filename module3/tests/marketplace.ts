import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";
import { 
  TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
  createAccount,
  getAccount
} from "@solana/spl-token";
import { assert } from "chai";

describe("marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Marketplace as Program<Marketplace>;
  const seller = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  let itemMint: anchor.web3.PublicKey;
  let paymentMint: anchor.web3.PublicKey;
  let sellerItemAccount: anchor.web3.PublicKey;
  let buyerItemAccount: anchor.web3.PublicKey;
  let sellerPaymentAccount: anchor.web3.PublicKey;
  let buyerPaymentAccount: anchor.web3.PublicKey;
  let listingPda: anchor.web3.PublicKey;
  let listingBump: number;
  let escrowPda: anchor.web3.PublicKey;
  let escrowBump: number;
  
  const itemName = "Test Item";
  const itemPrice = 100;
  const itemQuantity = 5;
  const purchaseQuantity = 2;

  before(async () => {
    // Airdrop SOL to seller and buyer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(seller.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(buyer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );

    // Create item mint
    itemMint = await createMint(
      provider.connection,
      seller,
      seller.publicKey,
      null,
      0
    );

    // Create payment mint
    paymentMint = await createMint(
      provider.connection,
      buyer,
      buyer.publicKey,
      null,
      0
    );

    // Create token accounts
    sellerItemAccount = await createAccount(
      provider.connection,
      seller,
      itemMint,
      seller.publicKey
    );

    buyerItemAccount = await createAccount(
      provider.connection,
      buyer,
      itemMint,
      buyer.publicKey
    );

    sellerPaymentAccount = await createAccount(
      provider.connection,
      seller,
      paymentMint,
      seller.publicKey
    );

    buyerPaymentAccount = await createAccount(
      provider.connection,
      buyer,
      paymentMint,
      buyer.publicKey
    );

    // Mint tokens
    await mintTo(
      provider.connection,
      seller,
      itemMint,
      sellerItemAccount,
      seller.publicKey,
      itemQuantity
    );

    await mintTo(
      provider.connection,
      buyer,
      paymentMint,
      buyerPaymentAccount,
      buyer.publicKey,
      itemPrice * itemQuantity * 2 // Enough to buy all items
    );

    // Derive PDAs
    [listingPda, listingBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        seller.publicKey.toBuffer(),
        Buffer.from(itemName),
      ],
      program.programId
    );

    [escrowPda, escrowBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        listingPda.toBuffer(),
      ],
      program.programId
    );
  });

  it("Creates a listing", async () => {
    await program.methods
      .createListing(
        new anchor.BN(itemPrice),
        new anchor.BN(itemQuantity),
        itemName
      )
      .accounts({
        seller: seller.publicKey,
        listing: listingPda,
        sellerTokenAccount: sellerItemAccount,
        escrowTokenAccount: escrowPda,
        itemMint: itemMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([seller])
      .rpc();

    // Verify the listing was created
    const listing = await program.account.listing.fetch(listingPda);
    assert.equal(listing.seller.toBase58(), seller.publicKey.toBase58());
    assert.equal(listing.price.toString(), itemPrice.toString());
    assert.equal(listing.quantity.toString(), itemQuantity.toString());
    assert.equal(listing.name, itemName);
    assert.isTrue(listing.active);

    // Check if escrow has the items
    const escrowBalance = await getAccount(provider.connection, escrowPda);
    assert.equal(escrowBalance.amount.toString(), itemQuantity.toString());
  });

  it("Purchases items from a listing", async () => {
    const beforeBuyerItemBalance = (await getAccount(provider.connection, buyerItemAccount)).amount;
    const beforeSellerPaymentBalance = (await getAccount(provider.connection, sellerPaymentAccount)).amount;

    await program.methods
      .purchase(new anchor.BN(purchaseQuantity))
      .accounts({
        buyer: buyer.publicKey,
        listing: listingPda,
        seller: seller.publicKey,
        escrowTokenAccount: escrowPda,
        buyerTokenAccount: buyerItemAccount,
        buyerPaymentAccount: buyerPaymentAccount,
        sellerPaymentAccount: sellerPaymentAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();

    // Verify the listing was updated
    const listing = await program.account.listing.fetch(listingPda);
    assert.equal(listing.quantity.toString(), (itemQuantity - purchaseQuantity).toString());
    assert.isTrue(listing.active);

    // Verify tokens were transferred
    const afterBuyerItemBalance = (await getAccount(provider.connection, buyerItemAccount)).amount;
    const afterSellerPaymentBalance = (await getAccount(provider.connection, sellerPaymentAccount)).amount;

    assert.equal(
      afterBuyerItemBalance.toString(),
      beforeBuyerItemBalance.add(new anchor.BN(purchaseQuantity)).toString()
    );
    
    assert.equal(
      afterSellerPaymentBalance.toString(),
      beforeSellerPaymentBalance.add(new anchor.BN(itemPrice * purchaseQuantity)).toString()
    );
  });

  it("Cancels a listing", async () => {
    const beforeSellerItemBalance = (await getAccount(provider.connection, sellerItemAccount)).amount;
    const remainingQuantity = itemQuantity - purchaseQuantity;

    await program.methods
      .cancelListing()
      .accounts({
        seller: seller.publicKey,
        listing: listingPda,
        escrowTokenAccount: escrowPda,
        sellerTokenAccount: sellerItemAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([seller])
      .rpc();

    // Verify the listing was cancelled
    const listing = await program.account.listing.fetch(listingPda);
    assert.isFalse(listing.active);
    assert.equal(listing.quantity.toString(), "0");

    // Verify tokens were returned
    const afterSellerItemBalance = (await getAccount(provider.connection, sellerItemAccount)).amount;
    assert.equal(
      afterSellerItemBalance.toString(),
      beforeSellerItemBalance.add(new anchor.BN(remainingQuantity)).toString()
    );
  });
});
