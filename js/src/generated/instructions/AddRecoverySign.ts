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
 * @category AddRecoverySign
 * @category generated
 */
export const AddRecoverySignStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number
}>([['instructionDiscriminator', beet.u8]], 'AddRecoverySignInstructionArgs')
/**
 * Accounts required by the _AddRecoverySign_ instruction
 *
 * @property [_writable_] profileInfo PDA of Krypton Program to be recovered
 * @property [] authorityInfo Pubkey of keypair of PDA to be recovered
 * @property [] newProfileInfo PDA to be recovered into
 * @property [] newAuthorityInfo Pubkey of the keypair to be recovered into
 * @property [**signer**] guardianInfo Pubkey of recovery guardian
 * @category Instructions
 * @category AddRecoverySign
 * @category generated
 */
export type AddRecoverySignInstructionAccounts = {
  profileInfo: web3.PublicKey
  authorityInfo: web3.PublicKey
  newProfileInfo: web3.PublicKey
  newAuthorityInfo: web3.PublicKey
  guardianInfo: web3.PublicKey
}

export const addRecoverySignInstructionDiscriminator = 8

/**
 * Creates a _AddRecoverySign_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category AddRecoverySign
 * @category generated
 */
export function createAddRecoverySignInstruction(
  accounts: AddRecoverySignInstructionAccounts,
  programId = new web3.PublicKey('2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL')
) {
  const [data] = AddRecoverySignStruct.serialize({
    instructionDiscriminator: addRecoverySignInstructionDiscriminator,
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
      isSigner: false,
    },
    {
      pubkey: accounts.newProfileInfo,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.newAuthorityInfo,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.guardianInfo,
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
