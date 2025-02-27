import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as fs from 'fs';
import * as path from 'path';
import * as borsh from 'borsh';

// Replace with your deployed program ID
const PROGRAM_ID = new PublicKey('REPLACE_WITH_YOUR_PROGRAM_ID');

// Class for message account data structure - should match the Rust structure
class MessageAccount {
  message: string;

  constructor(message: string) {
    this.message = message;
  }

  static schema = new Map([
    [
      MessageAccount,
      {
        kind: 'struct',
        fields: [['message', 'string']],
      },
    ],
  ]);
}

// Connect to the local Solana cluster
async function connectToCluster() {
  const connection = new Connection('http://localhost:8899', 'confirmed');
  console.log('Connected to local Solana cluster');
  return connection;
}

// Load or create keypair for the user
async function getKeypair() {
  try {
    const HOME = process.env.HOME as string;
    const keypairFile = path.join(HOME, '.config', 'solana', 'id.json');
    const secretKeyString = fs.readFileSync(keypairFile, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
  } catch (error) {
    console.log('Failed to load keypair, creating a new one');
    return Keypair.generate();
  }
}

// Create a new message account
async function createMessageAccount(
  connection: Connection,
  payer: Keypair,
  message: string
) {
  // Create a new keypair for the message account
  const messageAccount = Keypair.generate();
  console.log('Message account pubkey:', messageAccount.publicKey.toString());

  // Calculate space needed for the account
  const messageSize = borsh.serialize(
    MessageAccount.schema,
    new MessageAccount(message)
  ).length;
  const space = messageSize + 10; // Add some extra space for future updates

  // Calculate lamports needed
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  // Create transaction instruction to create account
  const createAccountIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: messageAccount.publicKey,
    lamports,
    space,
    programId: PROGRAM_ID,
  });

  // Create instruction data for initialization
  const initMessage = Buffer.from([0, ...Buffer.from(message)]);

  // Create instruction to initialize message
  const initIx = new TransactionInstruction({
    keys: [
      { pubkey: messageAccount.publicKey, isSigner: false, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: initMessage,
  });

  // Create and send transaction
  const tx = new Transaction().add(createAccountIx, initIx);
  const signature = await sendAndConfirmTransaction(connection, tx, [
    payer,
    messageAccount,
  ]);
  console.log(
    `Message account created and initialized with tx signature: ${signature}`
  );

  return messageAccount.publicKey;
}

// Update message
async function updateMessage(
  connection: Connection,
  payer: Keypair,
  messageAccountPubkey: PublicKey,
  newMessage: string
) {
  // Create instruction data for update
  const updateMessageData = Buffer.from([1, ...Buffer.from(newMessage)]);

  // Create instruction
  const updateIx = new TransactionInstruction({
    keys: [
      { pubkey: messageAccountPubkey, isSigner: false, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: updateMessageData,
  });

  // Create and send transaction
  const tx = new Transaction().add(updateIx);
  const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
  console.log(`Message updated with tx signature: ${signature}`);
}

// Read message from account
async function readMessage(
  connection: Connection,
  messageAccountPubkey: PublicKey
) {
  const accountInfo = await connection.getAccountInfo(messageAccountPubkey);
  if (!accountInfo) {
    throw new Error('Message account not found');
  }

  // Deserialize account data
  const messageData = borsh.deserialize(
    MessageAccount.schema,
    MessageAccount,
    accountInfo.data
  );
  console.log(`Stored message: "${messageData.message}"`);
  
  return messageData.message;
}

async function main() {
  try {
    // Connect to cluster
    const connection = await connectToCluster();
    
    // Get or create keypair
    const payer = await getKeypair();
    console.log('Using keypair:', payer.publicKey.toString());

    // Ensure the payer account has sufficient funds
    const balance = await connection.getBalance(payer.publicKey);
    console.log(`Account balance: ${balance / 1_000_000_000} SOL`);
    
    if (balance < 1_000_000_000) {
      console.log('For airdrop, run: solana airdrop 1');
      throw new Error('Insufficient balance. Please airdrop some SOL first.');
    }

    // Create message account with initial message
    const initialMessage = "Hello, Solana!";
    const messageAccountPubkey = await createMessageAccount(
      connection, 
      payer, 
      initialMessage
    );

    // Read the message back
    await readMessage(connection, messageAccountPubkey);

    // Update the message
    const newMessage = "Solana is amazing!";
    await updateMessage(connection, payer, messageAccountPubkey, newMessage);

    // Read the updated message
    await readMessage(connection, messageAccountPubkey);

    console.log('Demo completed successfully!');
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
