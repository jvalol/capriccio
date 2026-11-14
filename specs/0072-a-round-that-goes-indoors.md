# 0072 A round that goes indoors

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0067 put the steading's three people on rounds and left this out, with a
reason:

> **Going indoors.** Every round is outside. The hall's door is passable and a
> round through it is the obvious next thing, and it wants the figure to not
> be drawn while it is in there or it is a body inside a lit room seen through
> a window.

The first sentence was the gap and the last one was wrong. One of the three is
called the one who keeps the door, and was the only person in this building
who never went through the door they keep.

Jake, 2026-11-14, picking it off a list: the round indoors.

## The reason for the cull does not hold

Checked before anything was built, because it is the whole of what 0067 said
this would cost.

**Nothing is seen through a window.** `valhalla::lit` draws an opening as a
lit panel set back in its wall, not a hole, and every one of them is on the
hall's east face along with the door. There is no sightline into the hall
except the doorway.

**And a figure deep in the hall is not in the doorway's cone.** The stop the
round now has is six metres in and two metres off the door's centre line.
Rendered from the lane, the door frames the fire and the figure is not in the
frame at all.

**Rendered from inside, it is the point of the whole thing.** The hall is a
room you walk into, per spec 0060, and until now it was a room with a fire in
it and nobody. So the figure is drawn, always, and there is no cull anywhere
in this spec.

## What it does

**The round goes in, stands at the fire, and comes out.** Out of the yard,
through the door, nine seconds at the foot of the long fire on `Stir`, and
back out to the store to the north, which is where it went before.

**Four corners on the door's own centre line.** A leg is a straight line
between two stops, so one stop inside the hall and one in the yard is a body
walking through a gable. Two corners were not enough either: from the yard to
the opening runs through the jamb, and the first build clipped the north
reveal by two centimetres. The last stretch outside and the first inside are
both square to the door, so the turn happens in the open where there is room
for it. Every other worked door in this building is approached the same way.

**The threshold stands on the hall's own floor**, which is a slab `FLOOR_UP`
above the steading's. A body walking in rises those six centimetres over the
leg, and clips the slab's east edge by three millimetres doing it, which is a
doorstep rather than a wall: `walk::RIDES` is the game's own account of the
difference and the test reads it.

**At the foot of the fire and not beside it.** The trench runs down the
length of the hall in line with the door, so a stop at its side is reached by
a diagonal that cuts its corner, which is a body standing in the coals.
`FIRE_KERB` exists so that you cannot ride over one. On the centre line there
is no corner to cut, and the view is the one spec 0060 built the room for:
you come in at the east gable and the fire runs away from you to the high
seat.

**A quarter of the time, one of the three is out of sight.** That is the
honest cost of this and it is measured rather than estimated. It also moved
the steading's own picture: the second that test looks for wanted somebody
walking, and a body walking the length of a hall nobody can see into answers
that while the picture shows a place with nobody in it. It asks for somebody
walking out in the lane now.

## What it does not do

- **No cull, and no fade.** See above.
- **Nobody sits.** The benches down both walls and the high seat at the far
  gable are furniture, not stops. Spec 0067 keeps sitting out of scope and
  this does not change that.
- **The other two rounds stay outside.** One hall with one person in it is
  the claim; three people indoors at once is an empty steading.
- **Nothing happens at the fire.** `Stir` is the errand, which is the motion
  of working a pot, and no coal moves.

## Acceptance criteria

- Somebody goes into the hall, comes out again, and is not in there for most of the round. — `valhalla::tests::somebody_goes_into_the_hall`
- No round walks through a wall, after being pushed apart. — `valhalla::tests::no_round_walks_through_a_wall`
- The steading still shows somebody out in its lane. — `tests::the_steading_is_lived_in`

### Verified by hand

- Standing in the hall, somebody comes in and works the fire. — run the
  arcade with `--done`, walk up the bridge and stand inside the hall.
