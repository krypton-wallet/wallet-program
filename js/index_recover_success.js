const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
  NONCE_ACCOUNT_LENGTH,
  NonceAccount,
  NonceInformation,
} = require("@solana/web3.js");

const {
  getOrCreateAssociatedTokenAccount,
  createTransferCheckedInstruction,
  getMint,
  createMint,
  mintTo,
  approveChecked,
  TOKEN_PROGRAM_ID,
  AccountLayout,
  transferChecked,
  transfer,
  closeAccount,
} = require("@solana/spl-token");

const BN = require("bn.js");

const feePayer_sk = new Uint8Array([
  106, 183, 255, 76, 63, 41, 0, 108, 81, 92, 85, 207, 151, 2, 145, 66, 145, 177,
  132, 136, 141, 200, 248, 66, 173, 223, 196, 153, 51, 39, 208, 214, 154, 171,
  172, 84, 99, 243, 214, 204, 12, 127, 222, 208, 61, 67, 210, 83, 79, 95, 197,
  175, 206, 149, 28, 40, 125, 231, 159, 236, 145, 16, 93, 193,
]);

const dele_sk = new Uint8Array([
  180, 254, 129, 52, 4, 139, 16, 122, 221, 204, 223, 101, 11, 252, 82, 19, 140,
  133, 180, 26, 182, 149, 104, 122, 211, 122, 74, 99, 137, 15, 116, 166, 184,
  241, 24, 190, 112, 85, 209, 108, 174, 86, 31, 58, 3, 95, 104, 4, 122, 188,
  142, 4, 103, 3, 201, 210, 240, 179, 11, 135, 235, 3, 94, 138,
]);

const newFeePayer_sk = new Uint8Array([
  13, 76, 45, 159, 244, 117, 228, 247, 245, 226, 224, 138, 54, 179, 7, 229, 172,
  13, 115, 206, 144, 255, 193, 31, 43, 228, 167, 247, 101, 109, 198, 67, 89,
  213, 222, 218, 245, 110, 134, 222, 168, 77, 131, 147, 1, 36, 63, 239, 100, 29,
  104, 1, 254, 140, 175, 232, 248, 232, 24, 35, 28, 234, 142, 250,
]);

const guard1_sk = new Uint8Array([
  242, 196, 63, 33, 10, 154, 96, 153, 165, 84, 185, 28, 66, 38, 166, 46, 245,
  69, 251, 222, 174, 236, 224, 34, 30, 170, 195, 103, 181, 19, 120, 85, 64, 68,
  9, 152, 112, 37, 199, 9, 67, 74, 245, 78, 195, 170, 183, 207, 185, 94, 56,
  154, 6, 137, 29, 216, 113, 24, 180, 207, 96, 127, 124, 145,
]);

const guard2_sk = new Uint8Array([
  79, 245, 176, 109, 251, 79, 198, 158, 35, 60, 239, 212, 84, 135, 141, 154,
  145, 208, 242, 2, 87, 202, 2, 124, 6, 230, 119, 73, 199, 47, 9, 135, 44, 19,
  2, 147, 214, 229, 252, 244, 15, 212, 223, 79, 79, 240, 141, 227, 131, 225, 29,
  192, 239, 233, 70, 9, 117, 110, 216, 163, 163, 148, 203, 89,
]);

const guard3_sk = new Uint8Array([
  188, 247, 137, 132, 218, 110, 27, 210, 165, 127, 75, 190, 27, 3, 211, 53, 195,
  215, 210, 181, 233, 192, 19, 146, 113, 51, 0, 59, 126, 207, 18, 20, 182, 89,
  82, 12, 217, 139, 235, 49, 52, 204, 127, 241, 215, 161, 1, 251, 137, 103, 249,
  50, 142, 47, 173, 143, 17, 159, 64, 103, 91, 146, 218, 141,
]);

const mintAuthority_sk = new Uint8Array([
  183, 128, 61, 203, 3, 3, 54, 190, 151, 3, 136, 153, 3, 114, 49, 70, 142, 225,
  130, 2, 141, 122, 215, 213, 199, 243, 244, 6, 149, 216, 175, 153, 145, 7, 134,
  247, 29, 146, 85, 103, 46, 6, 26, 10, 71, 225, 94, 205, 73, 230, 107, 172, 99,
  159, 195, 85, 94, 102, 35, 59, 28, 184, 243, 71,
]);

const customMint = new PublicKey(
  "Aj7HtywN3kaCHw4KTKCthMNWRAZvYE79vw7tm7YhtkpG"
);

const main = async () => {
  const args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  const connection = new Connection("https://api.devnet.solana.com/");

  //   const feePayer = new Keypair();
  //   const newFeePayer = new Keypair();
  //   const dele = new Keypair();
  //   const guard1 = new Keypair();
  //   const guard2 = new Keypair();
  //   const guard3 = new Keypair();
  //   const nonceAccount = new Keypair();

  const feePayer = Keypair.fromSecretKey(feePayer_sk);
  const newFeePayer = Keypair.fromSecretKey(newFeePayer_sk);
  const dele = Keypair.fromSecretKey(dele_sk);
  const guard1 = Keypair.fromSecretKey(guard1_sk);
  const guard2 = Keypair.fromSecretKey(guard2_sk);
  const guard3 = Keypair.fromSecretKey(guard3_sk);
  const nonceAccount = new Keypair();
  const mintAuthority = Keypair.fromSecretKey(mintAuthority_sk);

  console.log(`feePayer: ${feePayer.publicKey.toBase58()}`);
  console.log(`newFeePayer: ${newFeePayer.publicKey.toBase58()}`);
  console.log(`delegate: ${dele.publicKey.toBase58()}`);
  console.log(`mint: ${customMint.toBase58()}`);

  const profile_pda = PublicKey.findProgramAddressSync(
    [Buffer.from("profile", "utf-8"), feePayer.publicKey.toBuffer()],
    programId
  );
  const new_profile_pda = PublicKey.findProgramAddressSync(
    [Buffer.from("profile", "utf-8"), newFeePayer.publicKey.toBuffer()],
    programId
  );

  console.log(`profile_pda: ${profile_pda[0].toBase58()}`);
  console.log(`new_profile_pda: ${new_profile_pda[0].toBase58()}\n`);

  //   console.log("Requesting Airdrop of 2 SOL...");
  //   const signature = await connection.requestAirdrop(feePayer.publicKey, 1e9);
  //   await connection.confirmTransaction(signature, "finalized");
  //   console.log("Airdrop received");
  //   const balance = await connection.getBalance(feePayer.publicKey);
  //   console.log(`feePayer Balance: ${balance}\n`);

  //   console.log("Requesting Airdrop of 2 SOL to new fee payer...");
  //   const signature1 = await connection.requestAirdrop(
  //     newFeePayer.publicKey,
  //     1e9
  //   );
  //   await connection.confirmTransaction(signature1, "finalized");
  //   console.log("Airdrop received\n");

  //   console.log("Requesting Airdrop of 2 SOL to delegate...");
  //   const signature3 = await connection.requestAirdrop(dele.publicKey, 1e9);
  //   await connection.confirmTransaction(signature3, "finalized");
  //   console.log("Airdrop received\n");

  // Mint new token
  //   console.log("Minting new token...");
  //   const mintAuthority = Keypair.generate();
  //   const freezeAuthority = Keypair.generate();
  //   const customMint = await createMint(
  //     connection,
  //     feePayer,
  //     mintAuthority.publicKey,
  //     freezeAuthority.publicKey,
  //     9
  //     // Keypair.generate(),
  //     // { skipPreflight: true, commitment: "finalized" }
  //   );
  //   console.log("Mint: " + customMint.toBase58() + "\n");

  //   console.log("timer started");
  //   await new Promise((resolve) => setTimeout(resolve, 20000));
  //   console.log("waited for 20s\n");

  // instr 1: initialize social recovery wallet
  const idx = Buffer.from(new Uint8Array([0]));
  const acct_len = Buffer.from(new Uint8Array(new BN(3).toArray("le", 1)));
  const recovery_threshold = Buffer.from(
    new Uint8Array(new BN(3).toArray("le", 1))
  );

  const initializeSocialWalletIx = new TransactionInstruction({
    keys: [
      {
        pubkey: profile_pda[0],
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: guard1.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: guard2.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: guard3.publicKey,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: Buffer.concat([idx, acct_len, recovery_threshold]),
  });

  // Transaction 1: setup nonce
  let tx = new Transaction();
  tx.add(
    // create nonce account
    SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: nonceAccount.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        NONCE_ACCOUNT_LENGTH
      ),
      space: NONCE_ACCOUNT_LENGTH,
      programId: SystemProgram.programId,
    }),
    // init nonce account
    SystemProgram.nonceInitialize({
      noncePubkey: nonceAccount.publicKey, // nonce account pubkey
      authorizedPubkey: feePayer.publicKey, // nonce account auth
    })
  );
  tx.feePayer = feePayer.publicKey;

  console.log("Sending nonce transaction...");
  let txid = await sendAndConfirmTransaction(
    connection,
    tx,
    [feePayer, nonceAccount],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet\n`);

  let nonceAccountData = await connection.getNonce(
    nonceAccount.publicKey,
    "confirmed"
  );

  // Transaction 3: Initialize wallet
  //   console.log("Initializing social wallet...");
  //   tx = new Transaction();
  //   tx.add(
  //     SystemProgram.nonceAdvance({
  //       noncePubkey: nonceAccount.publicKey,
  //       authorizedPubkey: feePayer.publicKey,
  //     })
  //   ).add(initializeSocialWalletIx);
  //   tx.recentBlockhash = nonceAccountData.nonce;

  //   txid = await sendAndConfirmTransaction(connection, tx, [feePayer], {
  //     skipPreflight: true,
  //     preflightCommitment: "confirmed",
  //     confirmation: "confirmed",
  //   });
  //   console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet\n`);

  // Create Token Account
  console.log("Creating token account...");
  const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    feePayer,
    customMint,
    profile_pda[0],
    true
  );
  console.log(
    "token account created: " + senderTokenAccount.address.toBase58() + "\n"
  );

  // Mint to token account
//   console.log("Minting to token account...");
//   await mintTo(
//     connection,
//     feePayer,
//     customMint,
//     senderTokenAccount.address,
//     mintAuthority,
//     6e9
//     //[],
//     //{skipPreflight: true},
//   );
//   console.log("Minted!\n");

//   // set delegate
//   console.log("Setting delegate...");
//   await approveChecked(
//     connection, // connection
//     feePayer, // fee payer
//     customMint, // mint
//     senderTokenAccount.address, // token account
//     dele.publicKey, // delegate
//     profile_pda[0], // owner of token account
//     10e9, // amount, if your deciamls is 8, 10^8 for 1 token
//     9 // decimals
//   );
//   console.log("Delegate set!\n");

  const senderTokenAccountBalance = await connection.getTokenAccountBalance(
    senderTokenAccount.address
  );
  console.log(
    `Sender Token Account Balance: ${senderTokenAccountBalance.value.amount}\n`
  );

  // Transaction 3: recover wallet
  const idx1 = Buffer.from(new Uint8Array([5]));
  const new_acct_len = Buffer.from(new Uint8Array(new BN(3).toArray("le", 1)));
  transfer

  const recoverWalletIx = new TransactionInstruction({
    keys: [
      {
        pubkey: profile_pda[0],
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: new_profile_pda[0],
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: newFeePayer.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: guard1.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: guard2.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: guard3.publicKey,
        isSigner: true,
        isWritable: false,
      },
    ],
    programId,
    data: Buffer.concat([idx1, new_acct_len]),
  });

  let res = await connection.getTokenAccountsByOwner(profile_pda[0], {
    programId: TOKEN_PROGRAM_ID,
  });
  //   console.log("timer started");
  //   await new Promise((resolve) => setTimeout(resolve, 6000));
  //   console.log("waited for 6s\n");

  let transferCloseTx = new Transaction();
  let delegateList = [];

  res.value.forEach(async (e) => {
    const oldTokenAccount = e.pubkey;
    console.log(`pubkey: ${oldTokenAccount.toBase58()}`);
    const accountInfo = AccountLayout.decode(e.account.data);

    const mint = new PublicKey(accountInfo.mint);
    const amount = accountInfo.amount;
    const delegate = accountInfo.delegate;
    console.log(`mint: ${mint}`);
    console.log(`amount: ${amount}`);
    console.log(`delegate: ${delegate}\n`);

    delegateList.push(delegate);
    const newTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      newFeePayer,
      mint,
      new_profile_pda[0],
      true
    );

    console.log("Storing instruction into transaction...");
    const idx2 = Buffer.from(new Uint8Array([6]));
    const amountBuf = Buffer.from(
      new Uint8Array(new BN(amount).toArray("le", 8))
    );
    const transferAndCloseIx = new TransactionInstruction({
      keys: [
        {
          pubkey: profile_pda[0],
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: new_profile_pda[0],
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: feePayer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: newFeePayer.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: oldTokenAccount,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: newTokenAccount.address,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: delegate,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId,
      data: Buffer.concat([idx2, amountBuf]),
    });

    transferCloseTx.add(transferAndCloseIx);

    // console.log(
    //   `Transfer from old ${new PublicKey(
    //     oldTokenAccount
    //   ).toBase58()} to new ${newTokenAccount.address.toBase58()}`
    // );

    // await transfer(
    //   connection,
    //   feePayer,
    //   new PublicKey(oldTokenAccount),
    //   newTokenAccount.address,
    //   profile_pda[0],
    //   amount
    // );

    // await closeAccount(
    //   connection,
    //   feePayer,
    //   new PublicKey(oldTokenAccount),
    //   newTokenAccount.address,
    //   profile_pda[0]
    // );
  });

  // recover wallet
  tx = new Transaction();
  tx.add(recoverWalletIx);

  console.log("Recover Wallet...");
  let recover_txid = await sendAndConfirmTransaction(
    connection,
    tx,
    [feePayer, newFeePayer, guard1, guard2, guard3],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(
    `https://explorer.solana.com/tx/${recover_txid}?cluster=devnet\n`
  );

  // transfer and close
  console.log("Transfer and close...");
  txid = await sendAndConfirmTransaction(
    connection,
    transferCloseTx,
    [feePayer, newFeePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet\n`);
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
