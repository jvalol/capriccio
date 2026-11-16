# 0108 The plate is a shade again

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing, looking at a toy's plate: the contrast of the text is hard to
read.

## Behavior

**The alpha was not the fraction of the room it looked like.** `PLATE` is
black at an alpha, and its own note reasons about it as a share taken off
whatever is behind: black "takes whatever is behind it down by the same
amount wherever you stand". At 0.62 that reads as two thirds gone. Measured
at one pixel of the nook's table, with the line quad off, the bare room is
158 and the plate is 119. A fifth, not two thirds.

Neither blend space explains it. In sRGB the answer would be 60 and in plain
linear it would be 101; the measured 119 is a multiplier of 0.54 where the
constant says 0.38. These quads are blended before the frame is tonemapped,
and a tonemap is not a power law, so nothing worked out from the constant was
going to be right. The range was measured instead: 0.80 gives 101, 0.90 gives
86, 0.96 gives 73.

**It is 0.90.** White on the plate goes from 3.7 to one to 6.2 to one, and at
0.90 the room is still visible through it, which 0.96 is close to ending. The
upper bound in the test moved with it, because the old `< 0.9` was guarding a
constant on the assumption that the constant meant what it looked like.

**And the line round the plate is under the whole plate, not round it.**
`plate` draws the line quad first, slightly larger, then the shade on top,
and the comment there said what is left showing round the edge is the line.
That is only true if the shade is opaque. It is not, so 22 per cent white
sits under the whole plate and lifts it: 119 to 132 at the old alpha, over a
pale table. Over a dark room it would have been the grey box `PLATE`'s note
says a plate must never be, which is the real cost and the reason the wording
mattered.

Raising the shade turns that leak into a floor of about a fiftieth, which is
why this spec does not also rebuild the line as a frame of four quads. It is
still a leak and the comment now says so, with the measurement, so the next
person to lower that alpha meets it.

## Acceptance criteria

- The plate is a shade and not a bar, at an alpha the measurements support. — `aim::tests::the_plate_is_a_shade_and_not_a_bar`

The control is the old number: putting `PLATE.w` back to 0.62 gives "a plate
at 0.62 alpha is either nothing or a bar".

**No test reads the screen, and this is a change nothing but the screen can
judge.** What the test holds is the constant; what settled it is six
screenshots of one pixel, listed above and in `PLATE`'s own note so they
cannot be separated from the number they chose.

### Verified by hand

By eye, on a shot of the puzzle's plate over the nook's table, before and
after.

## Out of scope

The line quad, for the reason above. Rebuilding it as four thin quads is a
separate change and a smaller one now than it was this morning.

The title sitting close to the plate's top edge, which is `PLATE_PAD` doing
what it says and was the same before.
