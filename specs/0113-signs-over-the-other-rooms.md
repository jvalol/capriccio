# 0113 Signs over the other rooms

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: signs over the other rooms.

## Behavior

**Three rooms already had one.** The hall's own ARCADE neon, the nook's
hanging plank, and THE BATHS on the wall over its door on the cellar's side.
The deep has EXIT. What this adds is the two that were left.

**Three rooms get none, on purpose.** The cellar is behind the bookcase that
swings, the hall of mirrors behind the other one, and the room behind the
wall is called that for a reason. A sign over any of those is the building
giving away its own secret, and a found room that is signposted was never
found. If they are ever to have one it is because Jake says the finding has
stopped being worth it, not because this spec swept them up.

**So the garden and the gallery.** `sign.rs` held one plank and now holds a
list of them, each with its word, its place and which way its point is cut.
A plank with no point is a room you walk straight on into, which is the one
direction an arrow cannot draw: the point is the whole of the direction and
"onward" is not a way to cut a plank.

**The gallery's hangs lower than the nook's.** Both are over the middle of
the aisle, and at one height the nearer covered the farther exactly, for the
whole length of the hall. Jake, shown it: hang the gallery lower.

It cannot clear the nook's altogether. The nook's reaches down to 2.06, and a
plank whose top is under that hangs at 1.78, which is a plank you walk into.
2.10 is as low as it goes with a head's room under it, and what that buys is
the lower edge showing below the nearer plank rather than the whole of it.

**The garden's is a board on the wall and not a plank over the aisle.** Its
opening has no head to hang a name over: the lintel was taken out on purpose
and the opening stops at the hall's own ceiling. Hung at its own z a plank
landed among the entrance's neon, and since the lettering is on one face it
showed you its blank back as you walked in. Jake: the wall pattern for the
garden. It is beside the opening, on the aisle side, which is the baths'
pattern one room along.

## Acceptance criteria

- Every plank clears a head and hangs under the ceiling, the gallery's hangs below the nook's, and the garden's board is beside its opening rather than over it. — `sign::tests::every_sign_clears_a_head_and_none_hides_another`
- The hall is drawn as it should be. — `tests::the_hall_is_drawn`

Two controls. Putting the gallery's plank back to the nook's height gives
"the gallery's plank hangs at 2.34 and the nook's at 2.34, so the nearer
hides the farther". Sliding the board back over the opening gives "the board
runs 6.95 to 8.45 and the opening 7.55 to 9.25, so it is hanging in the
doorway".

The test also asserts the two planks are over the same line, because the
height is the only thing separating them: moved apart in x the heights would
stop mattering and the first assertion would be guarding nothing.

The hall's picture test caught the change, and its difference is red on the
two planks and the board's edge and nowhere else.

### Verified by hand

By eye, two rounds: the first hung the garden's as a plank and put both
planks at one height, and Jake sent back both.

## Out of scope

The three found rooms, above.

Lettering on both faces. The plank's own comment says it reads from both and
the drawing places one quad, which is why the garden's plank showed a blank
back. It did not matter for the two that are left, since both are read from
the way you walk in, and it would have to be settled before a plank is ever
hung where you meet it from behind.
