# 0129 A pond early in the field

**Status:** implemented
**Date:** 2026-11-22

## Goal

Jake, with a photograph of one: put a pond early in the field that looks like
this, large with islands trees and ducks.

The photograph is of a made pond and not a lake. Cobbles set in the bank hold
the water, an island inside the same stone carries bare trees standing in a
winter's leaf litter, and the whole of the far half of it is sky and branches
lying on the water.

## Behavior

**North of the way, and the first thing out of the gap.** The gap is at
x -18.85 and the way there leaves it heading south west, so the field's whole
north side is open ground that nothing crosses: the bridge's deck is a band
six metres wide and `bridge::shades` keeps trees off a little past it, and
north of that there is nothing at all until the ground runs out. The pond's
middle stands 21.5 metres west of the gap and 22.35 north of it.

Early means early on foot, not early along the way. You come out of the gap,
the path pulls you left toward the village, and this is what is on your right
if you look.

**Set below the ground round it, by raising the ground.** A pond is lower
than its own bank, and the field's ground is a slab that follows you with no
hole in it to sink one into. So the land comes up instead: the grass stands
0.34 higher at the lip than it does out in the field and falls back to it
over three and a half metres, the bank runs down from the lip through the
waterline to the silt, and the water sits 0.06 over the field's own level and
a hand's breadth under the grass beside it.

The whole of that is one section, written once as a height at a distance
either side of the lip. The bank mesh, the verge, the rows of stone, where
the water's sheet is cut and what you walk up are all read off it, so there
is no second account of where the shore is.

**An outline that wanders, said as a radius.** A rounded oblong rather than
an oval, 12.2 by 8.3 about its middle, with three waves of different lengths
running round it at a twentieth of its size. Said in polar form, so asking
whether a spot is in the pond is one comparison: the water is cut by it, the
bank is built on it, the stones are set along it, the field is kept off it
and the ducks are held inside it, all from the one line.

**The water is cut to its own shore.** `blitzkit::water::Water` is a
rectangle and will not be anything else, so the rectangle is bigger than the
pond and what is drawn is the triangles of it whose three corners are all
inside. The count does not change from frame to frame because the outline
does not, which is what `update_mesh` wants.

**Cobbles in rows up the bank**, from two rows under the waterline to one
outside the lip, every 0.3 metres of slope. The rows are reckoned off the
shore and not off the lip, so one of them is exactly where the water ends
however the bank is cut: a row near the waterline and not on it leaves a ring
of bare bank, which is a tide mark. Bigger stones lie over them every few
feet, which is what a bank like this actually has.

**Two islands, each a lump with a wandering edge**, eight metres by five and
a half and four by three, neither in the middle and neither lined up with the
other. Each stands a hand over the water with the same two rows of stone
round it, a winter's leaf fall on it and trees in it.

**Grand trees round the water.** Eleven of them standing on the verge,
oak and aspen and one fir, at up to two and a half times the size the field
stands its own: eleven metres against four and a half. The field is meant to
be a field, which is why its scatter keeps them small; a pond is the one
place a tree is let grow. Their feet are outside the lip and their crowns are
over the water, which is what the water has to give back.

**And the water gives it back.** The surface wears a mirror, per engine spec
0091: a pass over the scene from the camera reflected in the water's own
plane, which the glass of spec 0092 then samples by screen position. Drawn
twice, because that glass takes no light at all: the reflection first, and
the water's own colour two millimetres over it, which is what carries the
chop and the dark. `field::LITTER_LAID` does the same job for a patch of
leaves.

The asking is the game's, because the engine draws every mirror it is handed.
What is asked is whether a sphere half the pond across is in view: a frustum
holds a sphere it is standing inside, so asking about the whole pond and its
verge bought a reflection of the water behind your head. Half the pond is a
sphere nobody can stand inside, because the water is the thing you cannot
walk into.

**Ducks.** Six of them, each going round a slow ellipse of its own at its own
pace, riding the water's own height rather than a line drawn at nought: a
duck on still water sits where the water is. Each pushes the surface as it
goes, which is `Water::push`, and that is the whole of why the pond is not a
mirror.

**What stops you is the water.** There is no wall. You come up the grass and
stand at the lip; what is past the lip is a thing you cannot cross, because a
pond you can stride over is a pond nobody believes.

The verge is given something to stand on all the way up, in bands that follow
the section. Not because it could not be climbed otherwise: the whole rise is
0.34 and `walk::STEP` is 0.42, so one box would let anybody up it. It is so
that your feet are on the grass rather than a third of a metre over it, which
is what the first test written here asked the wrong question about.

**The field is told the pond is there.** Two keep-outs, each its own
footprint, which is the lesson spec 0127 wrote down after the way there grew
a mown verge: `field::cover` keeps the grass and the flowers off the water,
the bank and the mown part of the verge, and `field::trees` asks about the
crown and keeps a tree from leaning over any of it.

**Drawn as an ordinary surface and not as the engine's water.** `fs_water`
answers a glancing angle with the ambient plus a share of the sun, standing
in for a reflection this engine cannot draw. Indoors that is a dim room and
the baths read as water. Out here it is a bright sky, so the whole pond came
back the colour of it: from the gap it was a concrete apron. The surface mesh
is the same mesh with its chop in the normals; what changes is the pipeline.
The cost is that water turned away from you no longer brightens, which is a
thing to want back the day something can draw what is above it.

## What it took three builds to get

**The first was a tank.** A rectangle of water in a band of concrete standing
46 centimetres out of the grass, because that is what clears `walk::STEP` and
it is how the garden's koi pond is built. Indoors, in a walled garden, that
reads. In a field it is a reservoir.

**The second was the same with cobbles on it.** Jake: it is very angular, the
islands are rectangular, and nothing is bordered with rounded random stones
like the photo. The islands became lumps and the rims became stone, and the
concrete was still there holding the whole thing up.

**The third is this.** Jake: ponds are usually set below ground, this one
should be with the stones sloping up from under the water to slightly above
the grass. That one sentence is the section above, and it took the wall out:
once the ground rises to the pond rather than the pond standing on the
ground, there is nothing to make out of boxes and nothing left that is
straight.

## What the reflection costs

**Nothing, and rather less than nothing.** Measured 2026-11-22 from the
south bank with the pond filling most of the frame, six runs of each arm in
both orders, the median of the last 240 frames of each run:

| | the whole GPU frame |
| --- | --- |
| with the reflection | 4.10 ms |
| without it, the water see-through as it was | 4.76 ms |
| without it, the water opaque | 4.60 ms |

The frame with a whole extra pass in it is the fastest of the three, by two
thirds of a millisecond, in every one of the runs and in both orders. The
reflection's own pass reads 3.5 ms on the clock, which is most of a frame
that comes to 4.1: the passes are not disjoint stretches of the timeline,
which spec 0110 of the engine found as well, and only the total is a frame
cost.

What is not established is the mechanism. The likely one is that
`fs_glass` does a texture read and a multiply where the water's own shading
does the sun, the lamps, a Fresnel and the chop over a third of the screen,
and the mirror pass draws into 960 by 540 with everything below the water
clipped away by spec 0093's near plane. That is a guess with a measurement
beside it, not a finding.

## Acceptance criteria

- The pond is north of the way there, and nothing of it is on the way or
  under the bridge's
  deck. — `pond::tests::the_pond_keeps_clear_of_the_way_and_the_bridge`
- You cannot walk into the water, from any of eight directions, and the walk
  gets to the pond to be stopped by
  it. — `pond::tests::you_cannot_walk_into_the_water`
- What stands at the lip is unbroken and higher than a
  step. — `pond::tests::nothing_gets_over_the_lip`
- And what you stand on across the verge is the ground that is drawn
  there. — `pond::tests::the_verge_is_what_you_walk_on`
- The outline wanders rather than being one smooth
  curve. — `pond::tests::the_outline_wanders`
- The water is cut to its own shore, so no corner of the rectangle it comes
  from is ever drawn. — `pond::tests::the_water_is_cut_to_its_own_shore`
- Both islands stand in the water, clear of its edge and of each
  other. — `pond::tests::the_islands_stand_in_the_water`
- Nothing the field sows stands on the pond, and no tree leans over
  it. — `pond::tests::nothing_of_the_field_grows_in_the_pond`
- The water is seen to end in stone the whole way
  round. — `pond::tests::the_water_ends_in_stone`
- A duck is on the water all the way round its circuit, and never on an
  island. — `pond::tests::a_duck_rides_the_water_it_is_on`
- The grand trees stand on the verge and not in the water, they are grand,
  and their crowns reach over
  it. — `pond::tests::the_grand_trees_stand_round_the_water`
- The water gives back what is in front of you and pays for nothing
  else. — `pond::tests::the_water_gives_back_only_what_is_looked_at`
- The pond is drawn as the blessed reference has it, from a pace back from
  its south east shoulder. — `tests::the_pond_is_drawn`

### Verified by hand

- Whether it reads as the photograph does, which is the thing this is for.
  There is no staged shot of it: `--pose` is how it was looked at, and the
  numbers are the room's own, so they move with `room::Room::of`. From the
  south bank, with the room as it stands:
  `./target/release/xanadu --at 0.28 --sky fair --pose "-42 0 26.2 1.5 170 -11 70"`.
- Whether the ducks look like ducks and move like them, which a still picture
  cannot say. Walk out of the gap and watch one.
- Whether the reflection holds together as you walk round the water, which is
  the one thing a single frame cannot say: a mirror is drawn from one camera
  and what it gives back moves with yours.
- Whether the verge reads as ground rising rather than as a mound laid on it.
  The grass on it is the field's own texture, tiled off the world so the
  tiles line up across the join, and the join is the thing to look at.

## Out of scope

- **Water you can get into.** Wading is the baths.
- **Fish.** The garden's koi are a pond you look into. This one is a pond you
  look across, and what is under the surface of it is silt.
- **A path round it.** The way there is the only made line in this field and
  giving the pond one of its own would make two. Walking the grass round the
  bank is the walk.
