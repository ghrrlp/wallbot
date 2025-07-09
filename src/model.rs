// NOTE TO SELF: Functionalities to Implement
//
// ### Covered by commands
//
// - (none)
//
// ### Covered by code
//
// - (none)
//
// ### Covered by Schema
//
// - Every user or guild can have 1 active wall; there is a resolution limit
// - Every user can have up to 2 rollbacks *aside* from their active wall
// - Every guild can have up to 5 rollbacks *aside* from their active wall
// - Every wall version tracks a list of contributors (user IDs) and an immediate previous version
// - Users can clear, rollback, or configure their own walls; guild admins can do so on guild walls
// - Members can undo a guild wall version to the previous IFF it's the latest version, they're the
//   only contributor, and the immediate previous version still exists
//
// - Each member can query guild wall and rollbacks from any guild channel
// - Each user can query personal wall and rollbacks from any channel
// - User can view other users' / guilds' walls from any channel, provided the said wall is public
//
// - Every guild can set 1 credit gain-and-cap pair, which is per minute
// - Every member builds up `gain` credits up to `cap`; they may consume 1 credit per pixel altered
// - Apart from setting 1 pixel at once, one may also upload RGBA8-compatible images
// - Changes that happen in the same minute shall be collated into a single version
//
// - Every user can have up to 3 drafts; each draft is attached to a guild wall version
// - Drafts can be created, appended to, or deleted, but there's no change-tracking
// - Drafts can be set to auto-apply when there are enough credits, and optionally only when
//   certain version information remain unchanged
//
// - A user can adopt a guild wall as their public wall instead of making their own wall public
//
// ### Missing
//
// - Each guild can designate 0 to 1 channel to receive daily / weekly wall updates always /
//   on-update
// - Wall versions should be pinnable; a pinned wall version will not automatically become dropped
// - Each guild can designate a timezone to perform daily version pinning and a partitioning of
//   version slots into daily pinned versions and minute versions
//
// - Drafts can be scheduled

use crate::schema::*;
