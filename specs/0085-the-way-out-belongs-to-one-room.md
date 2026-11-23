# 0085 The way out belongs to one room

**Status:** implemented
**Date:** 2026-11-15

## Goal

Jake flew out of the building and found the lit portal hanging on an exterior
wall, with the hall's near wall showing in pieces behind it. Draw both for the
room they belong to and nowhere else.

## Behavior

Two things are drawn on one question, and spec 0011 is why: the hall's near
wall is one piece from the hall, so the way out is a wall you have to walk
into, and two pieces from the room behind, so it is a lit rectangle you can
find. Spec 0015 added the portal in the gap for the same reason. The question
both ask is which side of that wall you are on.

It was asked as `self.at.z < self.room.reaches`, a half space with no top and
no sides. Everything past that plane got the room's treatment: the lawn, the
garden, and the open air over the roof. From outside the building that is a
glowing spiral on a blank wall and a wall in three pieces with a hole in it.

It asks `behind::in_the_room` now, which was already written and was
`#[cfg(test)]`, kept for the tests after the backdrop stopped asking it. Both
halves of it earn their place and neither alone is enough:

- The **footprint** is what keeps the garden out. `behind::inside` is a half
  space bounded above and not along, on purpose, and the garden sits past the
  hall's reaches under the same lid, which
  `behind::tests::the_garden_is_under_the_lid_and_not_in_the_room` already says in as
  many words. Gating on `inside` alone would hand the portal to anybody
  standing among the trees, on foot, with no flight involved.
- The **lid** is what keeps out a flight over the roof. That is inside the
  footprint, which runs 159 by 159 from the near wall, and only the ceiling
  tells you that you are over the building rather than in it.

**The eye and not the feet**, which is the whole of the margin. Jake's pose
was `5.85 1.99 24.93` with the eye at 3.54, the number his own plate prints
beside it, against a lid at `HIGH + THICK`, 3.5. Read at the feet he is in the
room and the portal is right to be there. Four centimetres of head through the
ceiling is the difference, and it is the right difference: what this decides
is what you can see, so it is the eye that has to be asked.

Nothing else changed. The view from inside the room renders byte for byte what
it did before.

## Acceptance criteria

- The way out shows from inside the room and from none of the hall, the garden or a flight over the roof. — `behind::tests::the_way_out_is_lit_for_this_room_and_nowhere_else`
- The garden is under the lid and outside the footprint, which is what the footprint half is for. — `behind::tests::the_garden_is_under_the_lid_and_not_in_the_room`

### Verified by hand

- From Jake's own pose there is no portal on the exterior wall and the wall is one piece: `./target/release/xanadu --fly --pose "5.85 1.99 24.93 3.54 310.0 -13.0 209.9" --picture-to /tmp/a.png`.
- From inside the room the portal and the EXIT sign are still there: the same pose without `--fly`, which falls to the floor and puts the eye back under the lid.

## Out of scope

Where the room is. Its footprint overlaps the garden and the lawn in plan and
is told apart from them by the lid, which is how the building has always been
arranged and is not a thing this spec touches.

Anything else drawn on a half space. This fixed the near wall and the way out
because that is what Jake could see; nothing here audits the rest.
