/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  TransferTokenArgs,
  transferTokenArgsBeet,
} from '../types/TransferTokenArgs'

/**
 * @category Instructions
 * @category TransferToken
 * @category generated
 */
export type TransferTokenInstructionArgs = {
  transferTokenArgs: TransferTokenArgs
}
/**
 * @category Instructions
 * @category TransferToken
 * @category generated
 */
export const TransferTokenStruct = new beet.BeetArgsStruct<
  TransferTokenInstructionArgs & {
    instructionDiscriminator: number
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['transferTokenArgs', transferTokenArgsBeet],
  ],
  'TransferTokenInstructionArgs'
)
/**
 * Accounts required by the _TransferToken_ instruction
 *
 * @property [] profileInfo PDA of Krypton Program
 * @property [**signer**] authorityInfo Pubkey of authority keypair of PDA
 * @property [_writable_] tokenAccountInfo ATA of the PDA
 * @property [_writable_] destTokenAccountInfo Destination Token Account
 * @category Instructions
 * @category TransferToken
 * @category generated
 */
export type TransferTokenInstructionAccounts = {
  profileInfo: web3.PublicKey
  authorityInfo: web3.PublicKey
  tokenAccountInfo: web3.PublicKey
  destTokenAccountInfo: web3.PublicKey
  tokenProgram?: web3.PublicKey
}

export const transferTokenInstructionDiscriminator = 1

/**
 * Creates a _TransferToken_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category TransferToken
 * @category generated
 */
export function createTransferTokenInstruction(
  accounts: TransferTokenInstructionAccounts,
  args: TransferTokenInstructionArgs,
  programId = new web3.PublicKey('2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL')
) {
  const [data] = TransferTokenStruct.serialize({
    instructionDiscriminator: transferTokenInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.profileInfo,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.authorityInfo,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.tokenAccountInfo,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.destTokenAccountInfo,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
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
