/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

type ErrorWithCode = Error & { code: number }
type MaybeErrorWithCode = ErrorWithCode | null | undefined

const createErrorFromCodeLookup: Map<number, () => ErrorWithCode> = new Map()
const createErrorFromNameLookup: Map<string, () => ErrorWithCode> = new Map()

/**
 * NotWriteable: 'Account should be writeable'
 *
 * @category Errors
 * @category generated
 */
export class NotWriteableError extends Error {
  readonly code: number = 0x0
  readonly name: string = 'NotWriteable'
  constructor() {
    super('Account should be writeable')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NotWriteableError)
    }
  }
}

createErrorFromCodeLookup.set(0x0, () => new NotWriteableError())
createErrorFromNameLookup.set('NotWriteable', () => new NotWriteableError())

/**
 * NoAccountLength: 'Account should not have 0 length data'
 *
 * @category Errors
 * @category generated
 */
export class NoAccountLengthError extends Error {
  readonly code: number = 0x1
  readonly name: string = 'NoAccountLength'
  constructor() {
    super('Account should not have 0 length data')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NoAccountLengthError)
    }
  }
}

createErrorFromCodeLookup.set(0x1, () => new NoAccountLengthError())
createErrorFromNameLookup.set(
  'NoAccountLength',
  () => new NoAccountLengthError()
)

/**
 * NonZeroData: 'Account should not have non-zero data'
 *
 * @category Errors
 * @category generated
 */
export class NonZeroDataError extends Error {
  readonly code: number = 0x2
  readonly name: string = 'NonZeroData'
  constructor() {
    super('Account should not have non-zero data')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NonZeroDataError)
    }
  }
}

createErrorFromCodeLookup.set(0x2, () => new NonZeroDataError())
createErrorFromNameLookup.set('NonZeroData', () => new NonZeroDataError())

/**
 * NotSigner: 'Account should be signer'
 *
 * @category Errors
 * @category generated
 */
export class NotSignerError extends Error {
  readonly code: number = 0x3
  readonly name: string = 'NotSigner'
  constructor() {
    super('Account should be signer')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NotSignerError)
    }
  }
}

createErrorFromCodeLookup.set(0x3, () => new NotSignerError())
createErrorFromNameLookup.set('NotSigner', () => new NotSignerError())

/**
 * InvalidSysProgram: 'Account should be valid system program'
 *
 * @category Errors
 * @category generated
 */
export class InvalidSysProgramError extends Error {
  readonly code: number = 0x4
  readonly name: string = 'InvalidSysProgram'
  constructor() {
    super('Account should be valid system program')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidSysProgramError)
    }
  }
}

createErrorFromCodeLookup.set(0x4, () => new InvalidSysProgramError())
createErrorFromNameLookup.set(
  'InvalidSysProgram',
  () => new InvalidSysProgramError()
)

/**
 * TooManyGuardians: 'There are too many guardians'
 *
 * @category Errors
 * @category generated
 */
export class TooManyGuardiansError extends Error {
  readonly code: number = 0x5
  readonly name: string = 'TooManyGuardians'
  constructor() {
    super('There are too many guardians')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, TooManyGuardiansError)
    }
  }
}

createErrorFromCodeLookup.set(0x5, () => new TooManyGuardiansError())
createErrorFromNameLookup.set(
  'TooManyGuardians',
  () => new TooManyGuardiansError()
)

/**
 * NotEnoughGuardians: 'There are too few guardians passed in'
 *
 * @category Errors
 * @category generated
 */
export class NotEnoughGuardiansError extends Error {
  readonly code: number = 0x6
  readonly name: string = 'NotEnoughGuardians'
  constructor() {
    super('There are too few guardians passed in')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NotEnoughGuardiansError)
    }
  }
}

createErrorFromCodeLookup.set(0x6, () => new NotEnoughGuardiansError())
createErrorFromNameLookup.set(
  'NotEnoughGuardians',
  () => new NotEnoughGuardiansError()
)

/**
 * NotEnoughAccounts: 'Specified amount of accounts are not passed in'
 *
 * @category Errors
 * @category generated
 */
export class NotEnoughAccountsError extends Error {
  readonly code: number = 0x7
  readonly name: string = 'NotEnoughAccounts'
  constructor() {
    super('Specified amount of accounts are not passed in')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NotEnoughAccountsError)
    }
  }
}

createErrorFromCodeLookup.set(0x7, () => new NotEnoughAccountsError())
createErrorFromNameLookup.set(
  'NotEnoughAccounts',
  () => new NotEnoughAccountsError()
)

/**
 * GuardianNotFound: 'The Guardian provided is not in the data'
 *
 * @category Errors
 * @category generated
 */
export class GuardianNotFoundError extends Error {
  readonly code: number = 0x8
  readonly name: string = 'GuardianNotFound'
  constructor() {
    super('The Guardian provided is not in the data')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, GuardianNotFoundError)
    }
  }
}

createErrorFromCodeLookup.set(0x8, () => new GuardianNotFoundError())
createErrorFromNameLookup.set(
  'GuardianNotFound',
  () => new GuardianNotFoundError()
)

/**
 * MissingGuardianSignatures: 'There are not enough guardian signatures to recover'
 *
 * @category Errors
 * @category generated
 */
export class MissingGuardianSignaturesError extends Error {
  readonly code: number = 0x9
  readonly name: string = 'MissingGuardianSignatures'
  constructor() {
    super('There are not enough guardian signatures to recover')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MissingGuardianSignaturesError)
    }
  }
}

createErrorFromCodeLookup.set(0x9, () => new MissingGuardianSignaturesError())
createErrorFromNameLookup.set(
  'MissingGuardianSignatures',
  () => new MissingGuardianSignaturesError()
)

/**
 * InvalidRecoveryThreshold: 'Recovery Threshold must be between 1 to 10'
 *
 * @category Errors
 * @category generated
 */
export class InvalidRecoveryThresholdError extends Error {
  readonly code: number = 0xa
  readonly name: string = 'InvalidRecoveryThreshold'
  constructor() {
    super('Recovery Threshold must be between 1 to 10')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidRecoveryThresholdError)
    }
  }
}

createErrorFromCodeLookup.set(0xa, () => new InvalidRecoveryThresholdError())
createErrorFromNameLookup.set(
  'InvalidRecoveryThreshold',
  () => new InvalidRecoveryThresholdError()
)

/**
 * InvalidAuthority: 'The pubkey is not authorized to act on behalf of the wallet'
 *
 * @category Errors
 * @category generated
 */
export class InvalidAuthorityError extends Error {
  readonly code: number = 0xb
  readonly name: string = 'InvalidAuthority'
  constructor() {
    super('The pubkey is not authorized to act on behalf of the wallet')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidAuthorityError)
    }
  }
}

createErrorFromCodeLookup.set(0xb, () => new InvalidAuthorityError())
createErrorFromNameLookup.set(
  'InvalidAuthority',
  () => new InvalidAuthorityError()
)

/**
 * NotAuthorizedToRecover: 'The pubkey is not authorized to recover the wallet'
 *
 * @category Errors
 * @category generated
 */
export class NotAuthorizedToRecoverError extends Error {
  readonly code: number = 0xc
  readonly name: string = 'NotAuthorizedToRecover'
  constructor() {
    super('The pubkey is not authorized to recover the wallet')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, NotAuthorizedToRecoverError)
    }
  }
}

createErrorFromCodeLookup.set(0xc, () => new NotAuthorizedToRecoverError())
createErrorFromNameLookup.set(
  'NotAuthorizedToRecover',
  () => new NotAuthorizedToRecoverError()
)

/**
 * MissingRecoveredAccounts: 'Required recovered accounts are not passed in'
 *
 * @category Errors
 * @category generated
 */
export class MissingRecoveredAccountsError extends Error {
  readonly code: number = 0xd
  readonly name: string = 'MissingRecoveredAccounts'
  constructor() {
    super('Required recovered accounts are not passed in')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, MissingRecoveredAccountsError)
    }
  }
}

createErrorFromCodeLookup.set(0xd, () => new MissingRecoveredAccountsError())
createErrorFromNameLookup.set(
  'MissingRecoveredAccounts',
  () => new MissingRecoveredAccountsError()
)

/**
 * InsufficientFundsForTransaction: 'There is insufficient SOL to transfer'
 *
 * @category Errors
 * @category generated
 */
export class InsufficientFundsForTransactionError extends Error {
  readonly code: number = 0xe
  readonly name: string = 'InsufficientFundsForTransaction'
  constructor() {
    super('There is insufficient SOL to transfer')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InsufficientFundsForTransactionError)
    }
  }
}

createErrorFromCodeLookup.set(
  0xe,
  () => new InsufficientFundsForTransactionError()
)
createErrorFromNameLookup.set(
  'InsufficientFundsForTransaction',
  () => new InsufficientFundsForTransactionError()
)

/**
 * Overflow: 'Operation overflowed'
 *
 * @category Errors
 * @category generated
 */
export class OverflowError extends Error {
  readonly code: number = 0xf
  readonly name: string = 'Overflow'
  constructor() {
    super('Operation overflowed')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, OverflowError)
    }
  }
}

createErrorFromCodeLookup.set(0xf, () => new OverflowError())
createErrorFromNameLookup.set('Overflow', () => new OverflowError())

/**
 * InvalidAccountAddress: 'Invalid Account Address'
 *
 * @category Errors
 * @category generated
 */
export class InvalidAccountAddressError extends Error {
  readonly code: number = 0x10
  readonly name: string = 'InvalidAccountAddress'
  constructor() {
    super('Invalid Account Address')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidAccountAddressError)
    }
  }
}

createErrorFromCodeLookup.set(0x10, () => new InvalidAccountAddressError())
createErrorFromNameLookup.set(
  'InvalidAccountAddress',
  () => new InvalidAccountAddressError()
)

/**
 * InvalidDateTime: 'Invalid date time'
 *
 * @category Errors
 * @category generated
 */
export class InvalidDateTimeError extends Error {
  readonly code: number = 0x11
  readonly name: string = 'InvalidDateTime'
  constructor() {
    super('Invalid date time')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidDateTimeError)
    }
  }
}

createErrorFromCodeLookup.set(0x11, () => new InvalidDateTimeError())
createErrorFromNameLookup.set(
  'InvalidDateTime',
  () => new InvalidDateTimeError()
)

/**
 * TargetAccountNotFound: 'Target account not found'
 *
 * @category Errors
 * @category generated
 */
export class TargetAccountNotFoundError extends Error {
  readonly code: number = 0x12
  readonly name: string = 'TargetAccountNotFound'
  constructor() {
    super('Target account not found')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, TargetAccountNotFoundError)
    }
  }
}

createErrorFromCodeLookup.set(0x12, () => new TargetAccountNotFoundError())
createErrorFromNameLookup.set(
  'TargetAccountNotFound',
  () => new TargetAccountNotFoundError()
)

/**
 * InvalidGuardTarget: 'Invalid target for guard'
 *
 * @category Errors
 * @category generated
 */
export class InvalidGuardTargetError extends Error {
  readonly code: number = 0x13
  readonly name: string = 'InvalidGuardTarget'
  constructor() {
    super('Invalid target for guard')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, InvalidGuardTargetError)
    }
  }
}

createErrorFromCodeLookup.set(0x13, () => new InvalidGuardTargetError())
createErrorFromNameLookup.set(
  'InvalidGuardTarget',
  () => new InvalidGuardTargetError()
)

/**
 * GuardContextNotFound: 'Guard Context not found'
 *
 * @category Errors
 * @category generated
 */
export class GuardContextNotFoundError extends Error {
  readonly code: number = 0x14
  readonly name: string = 'GuardContextNotFound'
  constructor() {
    super('Guard Context not found')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, GuardContextNotFoundError)
    }
  }
}

createErrorFromCodeLookup.set(0x14, () => new GuardContextNotFoundError())
createErrorFromNameLookup.set(
  'GuardContextNotFound',
  () => new GuardContextNotFoundError()
)

/**
 * ArithmeticOverflow: 'Arithmetic Overflow'
 *
 * @category Errors
 * @category generated
 */
export class ArithmeticOverflowError extends Error {
  readonly code: number = 0x15
  readonly name: string = 'ArithmeticOverflow'
  constructor() {
    super('Arithmetic Overflow')
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, ArithmeticOverflowError)
    }
  }
}

createErrorFromCodeLookup.set(0x15, () => new ArithmeticOverflowError())
createErrorFromNameLookup.set(
  'ArithmeticOverflow',
  () => new ArithmeticOverflowError()
)

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 * @category generated
 */
export function errorFromCode(code: number): MaybeErrorWithCode {
  const createError = createErrorFromCodeLookup.get(code)
  return createError != null ? createError() : null
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 * @category generated
 */
export function errorFromName(name: string): MaybeErrorWithCode {
  const createError = createErrorFromNameLookup.get(name)
  return createError != null ? createError() : null
}
