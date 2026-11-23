# 0083 The building does not move

**Status:** implemented
**Date:** 2026-11-15

## What this is for

Engine spec 0101 took xanadu's suite from 62 seconds to 13.3 by
noticing that one test built a render mesh once a frame to read one number
off it. The same shape was still here, in the next test along.

`field::tests::a_body_let_loose_outdoors_stays_in_the_world` drops a body
at eight places and walks it for thirty simulated seconds at sixty frames
a second, asserting every frame that it has not fallen through the world.
Every one of those 14,400 frames called `standing_on`, whose first line
was `crate::room::Room::of(crate::games()).solid()`: the whole arcade,
laid out from nothing, to ask where the ground is.

Measured: 446 microseconds and 878 boxes a call, so 6.4 seconds of that
test's 8.7. Two other walks in the same module did it too.

## What it does

**The building is handed in.** `standing_on` takes the solid boxes rather
than making them, and `the_building` makes them once at the top of each
walk. The only part of that function which depends on where the body is
was always the field's own boxes.

**And `field::tests::the_building_is_the_same_every_time` is why that is
safe.**
Asking once instead of per frame is the same thing exactly as long as the
answer does not change, so that is the thing a test says. If a room ever
starts laying itself out from a clock or a fresh seed, it is said there
rather than in a walk that quietly starts testing a different building
from the one it used to.

## What it bought

| | |
| --- | --- |
| the `field` module, on its own | 11.6 s to 0.22 s |
| the whole suite | 13.3 s to 11.2 s |

The suite moves less than the module because `cargo test` runs across every
core: these were two long poles among several, and what is left is bounded
by `tests::the_clip_walks_the_whole_route` at 5.7 seconds with the rest
tapering below it. There is no third fifty-second win sitting here.

## What the control said, which is not what it was asked

Breaking `the_building` to return nothing at all leaves all three walks
passing. That is not a weak test: they are field tests, and what holds a
body up out there, and what stops it walking east through the wall, are the
field's own boxes rather than the arcade's. The building is in the list
because a body let loose can wander into its footprint, and none of these
eight starts does.

Worth writing down because it looked for a moment like six seconds a test
spent on something nothing could detect. It is in the list for a case the
list does not reach yet.

## What it does not do

- **It does not change a walk.** The same boxes reach `walk` in the same
  order; they are copied rather than rebuilt.
- **It does not touch `Arcade::standing_on`**, which is the real one, and
  which has the building to hand already.
- **It does not make the room cheaper to build.** 446 microseconds is what
  laying out the arcade costs, and the game pays it once at load.

## Acceptance criteria

- The building is the same every time it is built, box for box. — `field::tests::the_building_is_the_same_every_time`
- And the walks still hold: nobody falls through the world, nobody walks out of the field, and you can still get out of the garden. — `field::tests::a_body_let_loose_outdoors_stays_in_the_world`, `field::tests::you_cannot_walk_out_of_the_field`, `field::tests::you_can_walk_out_of_the_garden`
