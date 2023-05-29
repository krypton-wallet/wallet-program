export * from './GuardAccount'
export * from './ProfileHeader'

import { ProfileHeader } from './ProfileHeader'
import { GuardAccount } from './GuardAccount'

export const accountProviders = { ProfileHeader, GuardAccount }
