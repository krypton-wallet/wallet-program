/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category AddRecoveryGuardians
 * @category generated
 */
export const AddRecoveryGuardiansStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number
}>(
  [['instructionDiscriminator', beet.u8]],
  'AddRecoveryGuardiansInstructionArgs'
)
/**
 * Accounts required by the _AddRecoveryGuardians_ instruction
 *
 * @property [_writable_] profileInfo PDA of Krypton Program
 * @property [**signer**] authorityInfo Pubkey of keypair of PDA
 * @property [] guardian Pubkey that will act as guardian to recover profile_info
 * @category Instructions
 * @category AddRecoveryGuardians
 * @category generated
 */
export type AddRecoveryGuardiansInstructionAccounts = {
  profileInfo: web3.PublicKey
  authorityInfo: web3.PublicKey
  systemProgram?: web3.PublicKey
  guardian: web3.PublicKey
}

export const addRecoveryGuardiansInstructionDiscriminator = 4

/**
 * Creates a _AddRecoveryGuardians_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category AddRecoveryGuardians
 * @category generated
 */
export function createAddRecoveryGuardiansInstruction(
  accounts: AddRecoveryGuardiansInstructionAccounts,
  programId = new web3.PublicKey('2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL')
) {
  const [data] = AddRecoveryGuardiansStruct.serialize({
    instructionDiscriminator: addRecoveryGuardiansInstructionDiscriminator,
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
    {
      pubkey: accounts.guardian,
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
