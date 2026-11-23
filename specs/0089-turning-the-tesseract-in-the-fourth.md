# 0089 Turning the tesseract in the fourth

**Status:** implemented, with its input half replaced by spec 0090 on
2026-11-16. The shape is all still here: `corners_at`, the plane it turns in,
the scaling that holds it to the row's width, and the mesh written over. What
went is the pair of arrows below, which became time.
**Date:** 2026-11-15

## Goal

Jake: the tesseract model on the pedestal, while it can be entered into that
room and be viewed internally, with some arrows or other keys could be
rotated through its phases for the external viewer.

## Behavior

The sixth plinth in the gallery already held a tesseract. What it did not
have was a fourth dimension to turn in: `tesseract::shape()` was built from
`corners()`, which takes no angle, so the object was one fixed projection of
the sixteen corners, the cube-within-a-cube, for ever. It turned, slowly like
every display and faster under a hand, but only in the three. It never did
the thing a tesseract is worth standing in front of.

`corners_at(phase)` turns the corners in the plane of x and the fourth before
flattening them. One plane and it has to be one holding the fourth: a
rotation in any plane of the three is an ordinary turn and the plinth already
has two of those. Not the plane of y and the fourth, which swells the shape
up and down against a room where up is fixed and reads as a squash.

**Worked by the arrows, because Return is spoken for.** Every other worked
thing in the building stands on a bench and is found by name; this one is a
display, and Return at a display either takes hold of it or, for this one,
goes inside. Going inside is not spare: `WentInsideTheTesseract` is one of
the trail's questions. So `work_a_toy` grew a branch for the one display that
is a toy, and left and right turn it for as long as they are held, the way
leaning on the gyro walks its axis. Left and right only, since up and down
would be a second plane.

**Held to one size, which is the row's rule rather than an exception to it.**
The cast really grows: 1.32 across at rest, 1.76 at phase 0.77, a third more.
`display::ACROSS` is `APART` over the root of three precisely so a shape's
turning sweep fails to reach its neighbour, so left alone this would have
reached into the hilbert tube and down through its own plinth, which is the
fault spec 0004 fixed once already for the Sierpinski tetrahedron. Every
shape in that row is scaled to a common width by its own bounds; `corners_at`
does the same at every phase rather than once. The object stays its
neighbours' size and the inner cube still swells through the outer one, that
being a change in the proportions and not in the size.

Jake chose this over drawing it at three quarters the others' size, and over
spreading the row.

**The mesh is written again rather than drawn from parts.** Turning in the
fourth moves all sixteen corners and changes the length of every one of the
thirty-two bars, so none of it is a transform: the object is a different
shape at each phase. `Renderer::update_mesh` writes over the one mesh, and
only on frames where the phase has actually moved. The alternative was
thirty-two bars pushed as instances, which costs nothing to turn and would
have made them boxes rather than the tubes the rest of the row is drawn with.

## Acceptance criteria

- Turning it never makes it outgrow the place the row leaves it, and the unheld cast would. — `tesseract::tests::turning_it_in_the_fourth_never_makes_it_bigger`

### Verified by hand

Driven with a played key log rather than pressed:

```
printf '0.5000 down Right\n9.0000 up Right\n' > /tmp/turn.log
./target/release/xanadu --pose "4.295 0 -13.0 1.55 0.0 -5.7 120.0" \
  --play /tmp/turn.log --picture-for 4 --picture-to /tmp/a.png
```

- Held, the inner cube swells out through the outer one and the frame skews.
- It stays inside its plinth's footprint at every phase.

## Out of scope

The plate under it still reads "Click or press enter to turn it", which was
already wrong for this one before today, since Return at it goes inside. It
is copy, so it is Jake's. It is more misleading now that the arrows do turn
it.

Naming the room. Jake called it the museum and asked; it is the gallery
everywhere in the code and in the specs, and nothing in the building displays
that, unlike the nook and the arcade which both have a sign over the door.
Giving it one is its own job and its own copy.

A second plane of rotation, which the question above settled at one.
