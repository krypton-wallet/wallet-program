import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  SystemProgram,
} from "@solana/web3.js";
import * as krypton from "../js/src/generated";

const run = async () => {
  const feePayerKeypair = Keypair.generate();
  const connection = new Connection("http://localhost:8899", "confirmed");
  const airdropSig = await connection.requestAirdrop(
    feePayerKeypair.publicKey,
    LAMPORTS_PER_SOL,
  );
  const recentBlockhash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    { ...recentBlockhash, signature: airdropSig },
    "confirmed",
  );

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
    },
  );

  const tx = new Transaction();
  tx.add(initInstruction);

  const txSig = await connection.sendTransaction(tx, [feePayerKeypair], {
    skipPreflight: true,
  });

  console.log("txSig", txSig);
  await connection.confirmTransaction(txSig);

  // inspect profile
  const profileAccount = await connection.getAccountInfo(profileAddress);

  if (profileAccount) {
    const [profile] = krypton.ProfileHeader.fromAccountInfo(profileAccount);
    console.log(profile);
  } else {
    console.log("profile not found");
  }

  // // add a recovery guardian
  // const guardianKeypair = Keypair.generate();
  // const addGuardianIx = krypton.createAddRecoveryGuardiansInstruction({
  //   profileInfo: profileAddress,
  //   authorityInfo: feePayerKeypair.publicKey,
  //   guardian: guardianKeypair.publicKey,
  // }, {
  //   addRecoveryGuardianArgs: {
  //     numGuardians: 1,
  //   },
  // });

  // const addGuardianTx = new Transaction();
  // addGuardianTx.add(addGuardianIx);

  // const addGuardianSig = await connection.sendTransaction(addGuardianTx, [
  //   feePayerKeypair,
  // ], { skipPreflight: true });

  // await connection.confirmTransaction(addGuardianSig);

  // const profileAccountAfter = await connection.getAccountInfo(profileAddress);
  // if (profileAccountAfter) {
  //   const [profile] = krypton.ProfileHeader.fromAccountInfo(
  //     profileAccountAfter,
  //   );
  //   console.log(profile);

  //   profile.guardians.forEach((guardian) => {
  //     console.log("guardian");
  //     console.log(guardian.pubkey.toString());
  //   });
  // } else {
  //   throw new Error("profile not found");
  // }

  let [guardAddress] = findGuardAddress(profileAddress);
  console.log("guard address", guardAddress.toString());
  let createGuardIx = krypton.createInitializeNativeSolTransferGuardInstruction({
    profileInfo: profileAddress,
    authorityInfo: feePayerKeypair.publicKey,
    guardInfo: guardAddress,
    systemProgram: SystemProgram.programId
  }, {
    initializeNativeSolTransferGuardArgs: {
      target: profileAddress,
      transferAmount: LAMPORTS_PER_SOL
    }
  });

  const createGuardTx = new Transaction();
  createGuardTx.add(createGuardIx);

  const createGuardSig = await connection.sendTransaction(createGuardTx, [
    feePayerKeypair,
  ], { skipPreflight: true });

  await connection.confirmTransaction(createGuardSig);

  const guardAccountAfter = await connection.getAccountInfo(guardAddress);
  if (guardAccountAfter) {
    const [profile] = krypton.GuardAccount.fromAccountInfo(
      guardAccountAfter,
    );
    console.log(profile);

    console.log("target: ")
    console.log(profile.target)
    console.log("guard: ")
    console.log(profile.guard.fields[0])
  } else {
    throw new Error("profile not found");
  }
};

run().then(() => console.log("done"));

const findProfileAddress = (authority: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), authority.toBuffer()],
    krypton.PROGRAM_ID,
  );
};

const findGuardAddress = (profile: PublicKey) => {
  return PublicKey.findProgramAddressSync([Buffer.from("guard"), profile.toBuffer()], krypton.PROGRAM_ID)
}