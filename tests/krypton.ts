import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  ConfirmOptions,
} from "@solana/web3.js";
import * as krypton from "../js/src/generated";

const run = async () => {
  const feePayerKeypair = Keypair.generate();
  const connection = new Connection("http://localhost:8899", "confirmed");
  let airdropSig = await connection.requestAirdrop(
    feePayerKeypair.publicKey,
    LAMPORTS_PER_SOL
  );
  let recentBlockhash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    { ...recentBlockhash, signature: airdropSig },
    "confirmed"
  );

  // initialize wallet
  console.log("TX: Initializing Wallet");
  const [profileAddress] = findProfileAddress(feePayerKeypair.publicKey);
  const initInstruction = krypton.createInitializeWalletInstruction(
    {
      profileInfo: profileAddress,
      authorityInfo: feePayerKeypair.publicKey,
    },
    {
      initializeWalletArgs: {
        recoveryThreshold: 2,
      },
    }
  );
  const tx = new Transaction();
  tx.add(initInstruction);
  recentBlockhash = await connection.getLatestBlockhash();
  await sendAndConfirmTransaction(connection, tx, [feePayerKeypair], {
    skipPreflight: true,
    preflightCommitment: "confirmed",
    confirmation: "confirmed",
  } as ConfirmOptions);
  const profileAccount = await connection.getAccountInfo(profileAddress);
  if (profileAccount) {
    const [profile] = krypton.UserProfile.fromAccountInfo(profileAccount);
    console.log("initial user profile:", profile.pretty());
  } else {
    console.log("profile not found");
  }

  // add two recovery guardians
  console.log("TX: Adding 2 recovery guardians");
  const guardianKeypair = Keypair.generate();
  const guardianKeypair2 = Keypair.generate();
  const addGuardianIx = krypton.createAddRecoveryGuardiansInstruction({
    profileInfo: profileAddress,
    authorityInfo: feePayerKeypair.publicKey,
    guardian: guardianKeypair.publicKey,
  });
  addGuardianIx.keys.push({
    pubkey: guardianKeypair2.publicKey,
    isSigner: false,
    isWritable: false,
  });
  const addGuardianTx = new Transaction();
  addGuardianTx.add(addGuardianIx);
  await sendAndConfirmTransaction(
    connection,
    addGuardianTx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  let profileAccountAfter = await connection.getAccountInfo(profileAddress);
  if (profileAccountAfter) {
    const [profile] = krypton.UserProfile.fromAccountInfo(profileAccountAfter);
    console.log("user profile with guardian:", profile.pretty());
  } else {
    throw new Error("profile not found");
  }

  // remove a recovery guardian
  console.log("TX: Removing a guardian");
  const removeGuardianIx = krypton.createRemoveRecoveryGuardiansInstruction({
    profileInfo: profileAddress,
    authorityInfo: feePayerKeypair.publicKey,
    guardian: guardianKeypair.publicKey,
  });
  removeGuardianIx.keys.push({
    pubkey: guardianKeypair2.publicKey,
    isSigner: false,
    isWritable: false,
  });
  const removeGuardianTx = new Transaction();
  removeGuardianTx.add(removeGuardianIx);
  await sendAndConfirmTransaction(
    connection,
    removeGuardianTx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  profileAccountAfter = await connection.getAccountInfo(profileAddress);
  if (profileAccountAfter) {
    const [profile] = krypton.UserProfile.fromAccountInfo(profileAccountAfter);
    profile.guardians.forEach((has_signed, guardian) => {
      console.log("guardian:", guardian.toString(), " has_signed:", has_signed);
    });
  } else {
    throw new Error("profile not found");
  }

  // add a recovery guardian
  console.log("TX: Adding guardian back");
  const addGuardian2Ix = krypton.createAddRecoveryGuardiansInstruction({
    profileInfo: profileAddress,
    authorityInfo: feePayerKeypair.publicKey,
    guardian: guardianKeypair.publicKey,
  });
  const addGuardian2Tx = new Transaction();
  addGuardian2Tx.add(addGuardian2Ix);
  await sendAndConfirmTransaction(
    connection,
    addGuardian2Tx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  profileAccountAfter = await connection.getAccountInfo(profileAddress);
  if (profileAccountAfter) {
    const [profile] = krypton.UserProfile.fromAccountInfo(profileAccountAfter);
    profile.guardians.forEach((has_signed, guardian) => {
      console.log("guardian:", guardian.toString(), " has_signed:", has_signed);
    });
  } else {
    throw new Error("profile not found");
  }

  // fund PDA for creating guard account
  console.log("Funding PDA for guard account");
  airdropSig = await connection.requestAirdrop(
    profileAddress,
    LAMPORTS_PER_SOL
  );
  recentBlockhash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    { ...recentBlockhash, signature: airdropSig },
    "confirmed"
  );

  // initialize guard account
  console.log("TX: Initializing guard account");
  let [guardAddress] = findGuardAddress(profileAddress);
  console.log("guard address", guardAddress.toString());
  let createGuardIx = krypton.createInitializeNativeSolTransferGuardInstruction(
    {
      profileInfo: profileAddress,
      authorityInfo: feePayerKeypair.publicKey,
      guardInfo: guardAddress,
      systemProgram: SystemProgram.programId,
    },
    {
      initializeNativeSolTransferGuardArgs: {
        target: profileAddress,
        transferAmount: 2 * LAMPORTS_PER_SOL,
      },
    }
  );
  const createGuardTx = new Transaction();
  createGuardTx.add(createGuardIx);
  await sendAndConfirmTransaction(
    connection,
    createGuardTx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  const guardAccountAfter = await connection.getAccountInfo(guardAddress);
  if (guardAccountAfter) {
    const [profile] = krypton.GuardAccount.fromAccountInfo(guardAccountAfter);
    console.log(profile.pretty());
    console.log("guard:", profile.guard.fields[0]);
  } else {
    throw new Error("profile not found");
  }

  // fund PDA for testing guard account
  console.log("Funding PDA for testing guard");
  airdropSig = await connection.requestAirdrop(
    profileAddress,
    2 * LAMPORTS_PER_SOL
  );
  recentBlockhash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    { ...recentBlockhash, signature: airdropSig },
    "confirmed"
  );

  // transfer native sol with guard - success
  console.log("TX: Transfer 1 SOL success");
  const receiver = Keypair.generate();
  let profileLamps = (await connection.getAccountInfo(profileAddress))
    ?.lamports;
  console.log("profileLamps:", profileLamps);
  let receiverLamps = (await connection.getAccountInfo(receiver.publicKey))
    ?.lamports;
  console.log("receiverLamps:", receiverLamps);
  const transferNativeSOLIx = krypton.createTransferNativeSOLInstruction(
    {
      profileInfo: profileAddress,
      authorityInfo: feePayerKeypair.publicKey,
      destination: receiver.publicKey,
      guard: guardAddress,
    },
    {
      transferNativeSolArgs: {
        amount: LAMPORTS_PER_SOL,
      },
    }
  );
  const transferNativeSOLIxTx = new Transaction();
  transferNativeSOLIxTx.add(transferNativeSOLIx);
  await sendAndConfirmTransaction(
    connection,
    transferNativeSOLIxTx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  profileLamps = (await connection.getAccountInfo(profileAddress))?.lamports;
  console.log("profileLamps after successful transfer:", profileLamps);
  receiverLamps = (await connection.getAccountInfo(receiver.publicKey))
    ?.lamports;
  console.log("receiverLamps after successful transfer:", receiverLamps);

  // transfer native sol with guard - fail
  console.log("TX: Transfer 1 SOL fail");
  const failTransferNativeSOLIxTx = new Transaction();
  failTransferNativeSOLIxTx.add(transferNativeSOLIx);
  await sendAndConfirmTransaction(
    connection,
    failTransferNativeSOLIxTx,
    [feePayerKeypair],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    } as ConfirmOptions
  );
  profileLamps = (await connection.getAccountInfo(profileAddress))?.lamports;
  console.log("profileLamps after failed transfer:", profileLamps);
  receiverLamps = (await connection.getAccountInfo(receiver.publicKey))
    ?.lamports;
  console.log("receiverLamps after failed transfer:", receiverLamps);
};

run().then(() => console.log("done"));

const findProfileAddress = (authority: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), authority.toBuffer()],
    krypton.PROGRAM_ID
  );
};

const findGuardAddress = (profile: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("guard"), profile.toBuffer()],
    krypton.PROGRAM_ID
  );
};
