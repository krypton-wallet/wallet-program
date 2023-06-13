export * from './GuardAccount'
export * from './ProfileHeader'
export * from './UserProfile'

import { UserProfile } from './UserProfile'
import { ProfileHeader } from './ProfileHeader'
import { GuardAccount } from './GuardAccount'

export const accountProviders = { UserProfile, ProfileHeader, GuardAccount }
