# 0125 The street is told what shape it is

**Status:** implemented
**Date:** 2026-11-22
**Built:** 2026-11-22, the day after the engine learned how.

## Goal

Jake: use it on the village street. The engine's spec 0110 gave a draw a
second texture that bends its normal per fragment, and spec 0124 left the
street's depth baked into its colour with the cost written down. This spends
that.

## Behavior

**A height field, and the normal off its neighbours.** `proud_at` says how
high the street stands at a spot, off `sett_at`'s one answer, so the shape
the sun shades and the joint the colour draws cannot come apart. The joint is
cut `SUNK` deep over its own width and the stone domes `DOMED` over that.
Fourteen millimetres and four: spec 0124 reached for thirty five when it was
cutting grooves as geometry, which is a gutter, and a sett is set in sand with
a centimetre of it showing.

`bumps` builds that field at the face's own size and in register with it,
then differences each texel against its neighbours to get a slope, and writes
the normal as raw bytes. **Not through `encoded`**, which is the one thing
about this file that is not obvious: every other texture in this building
carries the sRGB curve and this one must not, the bump binding reading the
upload as numbers rather than as a colour.

**The dome stays in the albedo.** Taking it out was the obvious move and was
wrong: it falls away in every direction at once rather than toward the sun,
which is what occlusion looks like and what a slot between two stones does to
the light reaching the bottom of it. Taken out, the street washed pale. What
the bump adds is the part the dome cannot have, which is a stone lit on the
side the sun is on.

**What it costs to build.** 35 milliseconds beside the face's 126, measured
three times at load on this machine, for twelve more megabytes of texture.
The field is built whole and differenced after rather than asking `sett_at`
five times a texel, four of the five answers being ones it would throw away.

## Three joints that were never there

The normal map found them, each as a line of texels leaning hard in the middle
of what `sett_at` called a stone. All three were already in the colour and all
three passed for stonework there.

**The keystone.** Every radial joint of a fan meets at its middle, so reckoned
like the rest the middle is a point with three joints running out of it and
`atan2` swinging a whole turn across one texel. It leaned fifty three out of a
hundred and twenty eight where the flattest stone on the street should be. A
fan is laid from a stone, so there is one there now and the rings are reckoned
from its edge.

**The seam.** `fan` steps at the half way line between two centres, so the last
ring of one fan butted the first of the next at whatever height each of them
reckoned: a step in the street rather than a joint in it.

**The ray the angle wraps on.** `atan2` answers between minus pi and pi, so the
one cell in every ring straddling the far side of a centre was cut in two by
the arithmetic and its two halves sat at different heights.

## Acceptance criteria

- The middle of a stone lies nearly flat, which is also the check that the
  map was not written as a colour: an sRGB encode puts the flat texel at 188
  rather than 128 and leans every stone a quarter turn off the surface it is
  on. — `village::tests::the_street_leans_at_its_joints_and_lies_flat_on_its_stones`
- The two walls of a stone lean opposite ways, up out of their own joints,
  read on a radius so that a red and green swapped between them fails it
  too. — the same test
- Nothing on the street leans into the earth it is laid on. — the same test
- The street still reads as stone and not as the earth it was paved
  over. — `village::tests::the_track_is_worn_where_the_wheels_run`

### Verified by hand

- Three of the village's reference pictures draw through the bump map and
  moved when it went in, so a bend that died would move them back.
- Whether the grooves move when the sun does, which is the whole of what this
  buys over spec 0124. Run it and walk the street at two hours.

## Out of scope

- **The far end of the street.** The mips average a normal map toward flat,
  so what this buys is the near field; down the road the albedo's joints are
  what you see, as before. Named in the engine's 0110 as well.
- **The pattern itself.** Spec 0124 laid the fans and Jake has seen them. The
  three joints above are faults in that pattern rather than changes to it.
