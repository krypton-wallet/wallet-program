/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import {
  NativeSolTransferInterval,
  nativeSolTransferIntervalBeet,
} from './NativeSolTransferInterval'
import { Context, contextBeet } from './Context'
export type NativeSolTransferGuard = {
  guarded: web3.PublicKey
  transferAmountRemaining: beet.bignum
  transferLimit: beet.bignum
  transferInterval: NativeSolTransferInterval
  lastTransferred: beet.bignum
  context: beet.COption<Context>
}

/**
 * @category userTypes
 * @category generated
 */
export const nativeSolTransferGuardBeet =
  new beet.FixableBeetArgsStruct<NativeSolTransferGuard>(
    [
      ['guarded', beetSolana.publicKey],
      ['transferAmountRemaining', beet.u64],
      ['transferLimit', beet.u64],
      ['transferInterval', nativeSolTransferIntervalBeet],
      ['lastTransferred', beet.i64],
      ['context', beet.coption(contextBeet)],
    ],
    'NativeSolTransferGuard'
  )
