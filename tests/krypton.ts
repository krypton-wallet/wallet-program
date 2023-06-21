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

  // inspect profile
  const profileAccount = await connection.getAccountInfo(profileAddress);
  if (profileAccount) {
    const [profile] = krypton.UserProfile.fromAccountInfo(profileAccount);
    console.log("initial user profile:", profile.pretty());
  } else {
    console.log("profile not found");
  }

  // add two recovery guardians
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

  // fund authority for creating guard account
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
        transferAmount: LAMPORTS_PER_SOL,
      },
    }
  );
  const createGuardTx = new Transaction();
  createGuardTx.add(createGuardIx);
  const txId = await sendAndConfirmTransaction(
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
