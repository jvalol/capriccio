# 0076 A neck and a collar

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0075 made the torsos better and left the close-up alone, on the
grounds that what was left was taste. The mirrors are the only place in
this building you see a body at arm's length, and from there every figure
wore a pale disc on its shoulders with a thin tan peg standing in the
middle of it. Jake, 2026-11-14: the collar.

It was not taste. It was three sizes handed to the wrong thing.

## What the disc was

**The top of a lathe is a hole.** The torso's profile ended 0.21 across, so
the shoulders had a 0.21 opening in them, and what shows through an opening
in a single surface is the inside of its own far wall. That is the disc, and
it is lit face-on by whatever is above.

The neck was supposed to fill it, and could not, for three reasons found in
that order by rendering the same frame with one thing changed at a time.

**The neck was half the width it reads as.** It is drawn from the torso's own
mesh, and `NECK_WIDE`, which reads as a width and is `HEAD_WIDE * 0.58`, was
handed straight to `with_scale`. A scale multiplies what the lathe already
is, and the lathe is 0.45 across, so every neck in this building came out
0.052 rather than 0.116.

**And two thirds the height, starting above the shoulders.** The same
mistake in y: `NECK_TALL + 0.05` multiplied by a mesh already `TORSO_TALL`
tall gave 0.08 instead of 0.13, and the placement put its foot four
centimetres above the torso's top rather than inside it. The comment beside
it said it runs a little into both ends. It ran into neither.

**And its base was a point.** A neck made from the torso's mesh has the
torso's shape, and the torso closes to nothing at its foot, so the neck
tapered to a point and sat in the hole like a peg in a socket two sizes too
big.

## What it does

**The neck is its own mesh**, off the same lathe with a profile of its own:
a column, a little wider where it meets the shoulders, closed at both ends.
Its width and height are metres now, not scales.

**It runs `NECK_SUNK` into the shoulders and the same into the head**, so
there is no seam at either end, which is what the old comment claimed and
did not do.

**And the shoulders shut to 0.10**, which is under the neck's own base, so
there is no opening left to see through. The last of the profile is a slope
rather than the two-point step it was, because a step a fiftieth of the
torso's height deep is a horizontal ring pointing straight up at the
ceiling.

## What it does not do

- **No collar as a thing.** There is no band, no lapel and no change of
  colour at the neck. What there is now is a body with a hole in it mended.
- **The head is where it was.** `HEAD_FROM` is spec 0033's and this did not
  touch it.
- **A lathe is still a lathe.** See spec 0075's own last section.

## 2026-11-14: and the fix left the same trap behind

The neck's own profile went in normalised, taking its size from the scale
it was drawn with. Every other profile in that file is in metres:
`torso_profile` gives 0.45, `cloak_profile` gives `CLOAK_TALL`, and what
the drawing multiplies them by is how big this particular person is, a
number near one. One profile of a different kind is the next version of
the bug this spec is about, and it would have arrived the first time
somebody scaled the neck the way all the others are scaled: 1.18 metres
across.

It is in metres now, scaled by `thick` like the rest, and `NECK_LONG`
names the length so the mesh and the gap it spans cannot drift. Nothing
moved: every picture reference passed untouched, which is what says this
was a refactor and not a change.

**And the rule is held rather than written down.** `a_figure_is_not_nothing`
walks the profiles, so it gained the neck, the cloak and the helm, and it
now asks each of them to be a part of a body rather than a unit shape.

## Acceptance criteria

- A neck is wider than the hole it fills. — `people::tests::a_neck_fills_the_hole_in_the_shoulders`
- And it runs into the shoulders below and the head above. — `people::tests::a_neck_reaches_the_shoulders_and_the_head`
- The figure still draws close up. — `tests::the_figure_is_drawn_close`
- Every profile in the file is a part of a body, in metres. — `people::tests::a_figure_is_not_nothing`

### Verified by hand

- No pale disc on anybody's shoulders. — run the arcade, go into the hall of
  mirrors and stand at the glass.
