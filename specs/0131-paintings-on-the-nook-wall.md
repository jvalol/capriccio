# 0131 Paintings on the nook wall

**Status:** implemented
**Date:** 2026-11-23

## Goal

Eight famous paintings, hung in a line on the nook's north wall, in gilt
frames. The nook is a library and the building as a whole is a museum of
curiosities, and until now there was not a picture anywhere in it.

## Why that wall, and why not the plan

Jake, looking at spec 0115's relief on this wall: I don't know if the concept
works. Maybe that wall would be better used for well known and popular
classical art, renaissance era, the scream, etc, since this is kind of a
library / museum of curiosities.

He is right about the wall and the plan is not the thing that was wrong with
it. Ten metres is a mural's wall and a plan is a diagram, which is most of
why it never sat there however it was dressed: it was three pale shapes
scattered on panelling, then three framed sheets that landed and still only
used a third of the run. The plan is in the cellar now, between the wine
racks, which is a wall its size.

The one argument for keeping it here was that it is the only thing in the
building that reads itself back: cut from the colliders, so a wall that moves
moves the drawing. That is an argument for keeping the plan, which is what
moving it does, and not for keeping it on a wall it cannot fill.

## Behavior

**They are the works themselves and not pastiche.** Botticelli, Leonardo,
Bosch, Vermeer, Goya, Hokusai, van Gogh, Munch. Every one is out of copyright
and every scan is open access off Wikimedia Commons, fetched at five hundred
pixels and re-encoded: three hundred kilobytes for the eight of them, against
three megabytes of footstep sounds already in `res`. A painted-looking
abstraction of a famous painting is a thing that reads as placeholder, which
is what this building calls a box.

**The hang is asked of the paintings.** How tall each one is on the wall
comes from how tall it really is, as the holding museums give it, and how
wide it is comes from the scan's own pixels rather than a number written
beside it. So the row has a Bosch two metres across and a Hokusai print a
third of that, which is what those two are, and cropping a scan moves its
frame rather than leaving the frame round the wrong thing.

**Compressed and not scaled.** Bosch's three and nine tenths of a metre is
nine times Hokusai's quarter of one, and a row that honoured that would be
one picture and seven stamps on a wall ten metres long. Raised to two fifths
the spread comes down to about two and a half, which is the spread a real
hang has: you can see at a glance which are the big ones and none of them is
unreadable.

**One line through the middles**, a little over eye height. That is how a
room hangs pictures and it is the only thing holding a row of eight different
shapes together. The dado rail is at waist and nothing crosses it.

**Fitted rather than trusted to fit.** What the natural widths add up to is
not a number anybody chose, so the row is worked out at its natural size and
then scaled until it fits the wall. The frames and the gaps are kept out of
that scaling, a moulding being a moulding whatever it is round; scaling them
with the rest is what put the last frame a centimetre past the end of the
wall on the first build.

**The canvas is drawn at one in every channel.** Colour here is a multiplier,
so any tint at all is a reproduction with a filter over it, and a Bosch wants
to be the colour Bosch painted it. What makes them sit in the room rather
than glow out of it is the nook's own two lamps.

**A scan that will not decode is left out.** Not drawn as a white rectangle,
which is what a missing texture otherwise is, and not drawn as a frame round
nothing either: the frame still hangs, because the frame is geometry and the
failure is the picture.

## Acceptance criteria

- Every scan decodes and is a picture rather than a
  thumbnail. — `art::tests::every_scan_decodes_and_is_a_picture`
- The row fits the wall it is on, nothing overlaps its neighbour, nothing
  crosses the dado rail or the cornice, and each canvas sits inside its own
  frame. — `art::tests::the_row_fits_the_wall_and_nothing_overlaps`
- Their middles are all on one line, and they are not all one
  size. — `art::tests::their_middles_are_all_on_one_line`
- Nothing of them stands inside the nook's walls or
  furniture. — `art::tests::nothing_hangs_inside_anything`

### Verified by hand

- **Whether it reads as a gallery wall or as eight posters.** Run the arcade,
  walk into the nook and turn round.
- **Whether the nook's two lamps light it.** They were hung for a blank wall
  and they wash the middle of it, so the ends of the row are darker than the
  middle.
- **Whether the paintings want naming.** There are no labels: a painter and a
  title are facts rather than drafted copy, as the plan's room names are, but
  eight more lines of lettering on this wall is a second thing and may be
  worse than none.

## Out of scope

- **No labels.** Named above.
- **No second wall.** The nook's other walls are bookcases and the way in.
- **Nothing you can work.** They are hung, not toys. Spec 0064's refusal of a
  dial on a thing you stand and look at is the same argument.
