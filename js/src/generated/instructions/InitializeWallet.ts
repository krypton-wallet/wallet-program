/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  InitializeWalletArgs,
  initializeWalletArgsBeet,
} from '../types/InitializeWalletArgs'

/**
 * @category Instructions
 * @category InitializeWallet
 * @category generated
 */
export type InitializeWalletInstructionArgs = {
  initializeWalletArgs: InitializeWalletArgs
}
/**
 * @category Instructions
 * @category InitializeWallet
 * @category generated
 */
export const InitializeWalletStruct = new beet.BeetArgsStruct<
  InitializeWalletInstructionArgs & {
    instructionDiscriminator: number
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['initializeWalletArgs', initializeWalletArgsBeet],
  ],
  'InitializeWalletInstructionArgs'
)
/**
 * Accounts required by the _InitializeWallet_ instruction
 *
 * @property [_writable_] profileInfo PDA of Krypton Program
 * @property [**signer**] authorityInfo Pubkey of keypair of PDA
 * @category Instructions
 * @category InitializeWallet
 * @category generated
 */
export type InitializeWalletInstructionAccounts = {
  profileInfo: web3.PublicKey
  authorityInfo: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const initializeWalletInstructionDiscriminator = 0

/**
 * Creates a _InitializeWallet_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitializeWallet
 * @category generated
 */
export function createInitializeWalletInstruction(
  accounts: InitializeWalletInstructionAccounts,
  args: InitializeWalletInstructionArgs,
  programId = new web3.PublicKey('2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL')
) {
  const [data] = InitializeWalletStruct.serialize({
    instructionDiscriminator: initializeWalletInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.profileInfo,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authorityInfo,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
