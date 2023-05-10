import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import * as krypton from "../js/src/generated";

const run = async () => {
  const feePayerKeypair = Keypair.generate();
  const connection = new Connection("http://localhost:8899");
  const airdropSig = await connection.requestAirdrop(
    feePayerKeypair.publicKey,
    LAMPORTS_PER_SOL,
  );
  await connection.confirmTransaction(airdropSig);

  const [profileAddress] = findProfileAddress(feePayerKeypair.publicKey);

  const initInstruction = krypton.createInitializeWalletInstruction({
    profileInfo: profileAddress,
    authorityInfo: feePayerKeypair.publicKey,
  }, {
    initializeWalletArgs: {
      recoveryThreshold: 2,
      privScan: Buffer.alloc(32),
      privSpend: Buffer.alloc(32),
    },
  });

  const tx = new Transaction();
  tx.add(initInstruction);

  const txSig = await connection.sendTransaction(tx, [feePayerKeypair], {
    skipPreflight: true,
  });

  console.log("txSig", txSig);
};

run().then(() => console.log("done"));

const findProfileAddress = (authority: PublicKey) => {
  return PublicKey.findProgramAddressSync([
    Buffer.from("profile"),
    authority.toBuffer(),
  ], krypton.PROGRAM_ID);
};
