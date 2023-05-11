/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  ModifyRecoveryThresholdArgs,
  modifyRecoveryThresholdArgsBeet,
} from '../types/ModifyRecoveryThresholdArgs'

/**
 * @category Instructions
 * @category ModifyRecoveryThreshold
 * @category generated
 */
export type ModifyRecoveryThresholdInstructionArgs = {
  modifyRecoveryThresholdArgs: ModifyRecoveryThresholdArgs
}
/**
 * @category Instructions
 * @category ModifyRecoveryThreshold
 * @category generated
 */
export const ModifyRecoveryThresholdStruct = new beet.BeetArgsStruct<
  ModifyRecoveryThresholdInstructionArgs & {
    instructionDiscriminator: number
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['modifyRecoveryThresholdArgs', modifyRecoveryThresholdArgsBeet],
  ],
  'ModifyRecoveryThresholdInstructionArgs'
)
/**
 * Accounts required by the _ModifyRecoveryThreshold_ instruction
 *
 * @property [_writable_] profileInfo PDA of Krypton Program
 * @property [**signer**] authorityInfo Pubkey of keypair of PDA
 * @category Instructions
 * @category ModifyRecoveryThreshold
 * @category generated
 */
export type ModifyRecoveryThresholdInstructionAccounts = {
  profileInfo: web3.PublicKey
  authorityInfo: web3.PublicKey
}

export const modifyRecoveryThresholdInstructionDiscriminator = 6

/**
 * Creates a _ModifyRecoveryThreshold_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category ModifyRecoveryThreshold
 * @category generated
 */
export function createModifyRecoveryThresholdInstruction(
  accounts: ModifyRecoveryThresholdInstructionAccounts,
  args: ModifyRecoveryThresholdInstructionArgs,
  programId = new web3.PublicKey('2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL')
) {
  const [data] = ModifyRecoveryThresholdStruct.serialize({
    instructionDiscriminator: modifyRecoveryThresholdInstructionDiscriminator,
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
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
