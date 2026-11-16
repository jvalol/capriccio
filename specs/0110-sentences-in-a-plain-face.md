# 0110 Sentences in a plain face

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: even the font could be better. Offered the choice between
swapping the one font, adding a second, or leaving it, he took the second.
The rest of what he said that turn is spec 0109.

## Behavior

**A marquee and a sentence are two jobs.** The engine drew every word in
`PressStart2P`, which is eight by eight with no descenders. Over a cabinet
that is the point of this building. On the line under the sight, which is
where the arcade asks you to read which key turns which face, it is a
sentence set in a marquee.

Engine spec 0106 gave the engine a second face. This is the arcade using it.
`NAME_FACE` is the arcade one and `SAYS_FACE` is the plain one, named here
rather than chosen at each call, because which face a thing is set in is a
rule about what the thing is and not a decision three places make
separately.

The name of what you are pointing at keeps the arcade face. It is a marquee,
and this is an arcade.

**What moved to the plain face is every sentence on the screen**: the line
under the name, and the help along the top. The cost readout keeps the
arcade face, being a row of numbers.

**The sizes went up to stay the same size.** `PressStart2P` fills its em, so
a cap is the whole of the number you ask for. A plain face spends part of its
em on ascenders and descenders, so the same number is a smaller letter and
both sentences shrank the moment they changed face. `DETAIL_SIZE` is 19 from
15 and `SAYS_SIZE` is 18 from 14.

**And the plate had to learn the face too.** `plate_round` measured every
line with `room_for`, which is the arcade face, while the text under it was
drawn in the plain one. The two faces are not the same width at a size, so
the plate would have been cut for a line nobody draws. That is the fault
`said`'s own note describes about wrapping, one step along, so `plate_round`
takes a face a line now.

## Acceptance criteria

- Every line of a block falls inside its plate, measured in the face each line is drawn in. — `aim::tests::a_wrapped_line_sits_on_its_plate`
- The prompt is at the foot and clear of the sight, in both faces. — `aim::tests::the_prompt_sits_at_the_foot_and_clear_of_the_sight`

**`a_wrapped_line_sits_on_its_plate` caught this change the moment it
landed.** The line in it is Jake's own, from the browser build, kept because
a test written with a short line passes on the fault it is for. In the plain
face his line fits one row at 1600 across and wider, so the test's own guard
fired: "(1600, 1200) does not wrap the line at all". His words are still the
unit; the test now repeats them until they wrap, because a window the line
fits is a window the test checks nothing at.

### Verified by hand

By eye, on shots of the puzzle's plate and the help line, at the same pose as
spec 0109's.

## Out of scope

The licence of the font itself, which engine spec 0106 carries and which is
not recorded anywhere in that repository yet.

The other toys' plate copy, which comes from `about` and is Jake's.
