# SolMate Wallet

Introducing SolMate, a Solana smart contract wallet with multisig social recovery, eliminating fear of losing your private key and improving usability and security with advanced features.

## Key Features

- Self custody without seed phrases (not good user experience)
- Social recovery with guardians (people, secondary wallet)
- Account locking & freezing
- Setting transaction limit to prevent wallets being emptied
- Multi-factor authentication with authApp
- Ability to interact with arbitrary smart contracts
- Optional whitelist checking

## Flow Diagram

![flow-diagram](./media/flow.png)

## Instructions

### Initialize Social Wallet

Purpose: initialize social wallet program with pda corresponding to authority and store all information in pda

- Get PDA, authority and guardian info from client
- Invoke CPI to create profile account with pda that stores a ProfileHeader
- initialize ProfileHeader with given info and serialize into pda data

ProfileHeader format:
<table>
  <tr>
   <td>
    <strong>name</strong>
   </td>
   <td><strong>Byte position in pda data </strong>
   </td>
   <td><strong>description</strong>
   </td>
   <td><strong>Default value</strong>
   </td>
  </tr>
  <tr>
   <td>recovery_threshold
   </td>
   <td>0
   </td>
   <td>Minimum number of guardians needed to recover
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td>executor
   </td>
   <td>1~32
   </td>
   <td>Executes all transfers on behalf of authority during recovery mode
   </td>
   <td>Pubkey::default()
   </td>
  </tr>
  <tr>
   <td>Size of vector of guardians
   </td>
   <td>33~36
   </td>
   <td>Preset space to store the size of vector
   </td>
   <td>n
   </td>
  </tr>
  <tr>
   <td>Vector of guardians
   </td>
   <td>37~(n*32+37)
   </td>
   <td>Vector of guardian pubkeys
   </td>
   <td>-
   </td>
  </tr>
</table>

### Add to recovery list

add a new guardian to guardian list

### Modify recovery list

modify/replace existing guardians with new guardians

### Delete from recovery list

delete existing guardians from recovery list, while maintaining recovery_threshold

### Modify Recovery Threshold

change recovery threshold in pda

### Recover wallet

Purpose: enter recovery mode by populating executor field in pda and moving all info from old pda to new pda seeded with new authority

- Get PDA of old authority and stores the executor inside it
- Create new profile account with new pda
- Deserialize ProfileHeader from PDA and store it in the new pda
- Executor field of new pda is initialized to Pubkey::default()

### Transfer to New Token Account

Purpose: transfer to a destination token account and break into two cases: recovery and non-recovery; if in recovery mode, also need to close sender/src token account

- Executor_info is read and executor key is set iff we are in recovery mode (recovery_mode == 1); in this case, executor must be a signer, since its responsibility is to sign for the program to transfer, as original authority is gone and cannot sign during recovery
- Transfer from src to dest token account with given amount and sign the transaction with original pda
- Close the src token account iff recovery_mode == 1 and executor in pda matches executor_info.key
