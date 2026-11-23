# 0090 The tesseract turns itself

**Status:** implemented
**Date:** 2026-11-16
**Replaces the input half of spec 0089**, which is a day old. Everything that
spec built about the shape stands: `corners_at`, the one plane it turns in,
the scaling that holds it to the row's width, and the mesh written over
rather than drawn from parts.

## Goal

Jake, after standing in front of it: the tesseract should continually and
automatically turn, and then the plaque can just say "click on or press enter
to enter it".

## Behavior

**The phase is time, like every other display in the row.** Spec 0089 gave
the arrows a direction to wind while they were held, on the reasoning that
the shape has no phases to step between so what it wants is a direction. That
reasoning was about the control and never about the person. What somebody
wants from this object is to watch the inner cube swell out through the outer
one, and a key they have to hold for it puts a chore in front of the only
trick the thing has. So `tesseract_turning` is gone and `tesseract_phase`
advances every frame.

The arrows go back to turning the person, which is what they do everywhere
else in the gallery. The branch at the top of `work_a_toy` that had claimed
them for this one display came out whole.

**Slower than a held arrow, and off the row's own rate.** `TURNS_BY` goes 0.7
to 0.26, a whole turn in about twenty four seconds rather than nine. Nobody
is waiting on it now, so it has to read as something the object is doing
rather than a machine running. And at 0.7 it was exactly twice
`display::TURNS`, so the turn about the upright and the turn in the fourth
came back to the same pose every eighteen seconds and the pair read as one
coupled motion. Off that ratio they are two things happening to the same
object, which is what they are.

**The plate says what Return does.** "Click or press enter to turn it" was
wrong for this display from the day it went in, since Return at it goes
inside, and spec 0089 left it alone and said so in its own out of scope. It
reads "Click or press enter to enter it" now. One case in the display arm,
not a new string family: the other five keep the line they had, and holding
is still impossible here because the tesseract never reaches `spin.take`.

**The guard on the mesh write came out with the hand.** It asked whether the
phase had moved since the last write, which was worth asking when the answer
was usually no. It is now always yes. `shape_turned` was measured at 7.6
microseconds, a twentieth of a percent of a frame at sixty, so the mesh is
written every frame and two of the three fields this needed are gone.

## Acceptance criteria

- Turning it never makes it outgrow the place the row leaves it, and the unheld cast would. — `tesseract::tests::turning_it_in_the_fourth_never_makes_it_bigger`, unchanged from spec 0089 and still the thing that matters about `corners_at`

**And the turning itself has none**, which is the honest position. What was
deleted here was a branch with a test behind it; what replaced it is one line
of `phase += rate * dt` in `advance`, reachable only through an `Arcade`,
which is the same wall spec 0087 hit with the flight toggle. A test of it
would be a test of addition.

### Verified by hand

No keys at all, which is the whole point:

```
./target/release/xanadu --pose "4.295 0 -13.0 1.55 0.0 -5.7 120.0" \
  --picture-for <t> --picture-to /tmp/a.png
```

- At 0.1 seconds it is the cube within a cube it has always been.
- At 6 it is a quarter turn on, the inner cube out at the outer one's width, and reads as a single cube with its diagonals drawn.
- At 12 the two have swapped and the small one sits at the back.
- The plate under it reads "Click or press enter to enter it" in all three.

## Out of scope

Naming the room, as in spec 0089. Jake calls it the museum, the code and the
specs call it the gallery, and nothing in the building says either.

A second plane of rotation. Spec 0089 settled that at one and nothing here
reopens it.
