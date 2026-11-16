# 0091 The arrows turn the shape

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, in the gallery: free rotation doesn't seem to work, all I can do is
press enter to toggle their auto rotation.

## Behavior

Spec 0004 gave the row a verb. Return takes hold of a shape, which freezes
it, and the mouse then turns it about the upright and about whichever way is
right on the screen. That half works and has worked since the day it went in.

The arrows did not. At a display `work_a_toy` wanted a `Seen::Bench` and
found a `Seen::Display`, so it said no and the arrows went on turning the
player. Turning the player takes the sight off the thing in your hands, and
`update` lets go of a shape the sight has left, so an arrow pressed at a
plinth swung the view away and started the shape turning again within a
frame. Which is exactly what Jake read off the screen: the only thing the
keyboard does at a plinth is stop the row's turn and start it again.

**This is the panel's fault, found again.** The comment above the arrow
handling already describes it, in spec 0039's words: left and right went into
the panel and kept turning you as well, and turning takes the sight off the
panel, so a pan stopped within a frame of starting. The fix is the same one.
All four arrows go into a shape you are holding, and the turn and tilt they
would have given the player come out as nought.

**Held and not pressed.** The mouse gives a distance moved and an arrow gives
a direction held, so `Spin::held` takes a wish and a `dt` where `Spin::drag`
takes pixels, and both go through one `turn_of` so that what the axes are is
written once. `ARROW_TURNS` is 1.5 radians a second, half a turn in about two
seconds: an arrow cannot be flicked, so the holding has to get you to the far
side without becoming a wait.

**Taking hold first is kept.** Look at a shape, press Return, and now the
arrows and the mouse both turn it. The alternative was an arrow taking hold
on its own, which reads better for one shape and makes the whole gallery a
room you cannot look around with the keyboard.

## Acceptance criteria

- An arrow turns the shape in your hands, does not turn you, and does not let go of it. — `tests::an_arrow_turns_the_shape_in_your_hands_and_not_you`
- The same arrow with empty hands still turns you, which is the control, in the same test.

### Verified by hand

Driven with a played log, since the fault needs a real press at a real
plinth:

```
printf '0.500 press Left\n1.000 down Return\n1.033 up Return\n1.500 down Right\n2.300 up Right\n' > /tmp/arrow.log
./target/release/capriccio --pose "3.345 0 -13.0 1.55 0.0 -5.7 120.0" \
  --play /tmp/arrow.log --picture-for 3 --picture-to /tmp/a.png
```

- Before: the camera has swung off the row and is looking into the cellar.
- After: the camera has not moved, the plate still reads "to let go", and the hilbert tube has turned about the upright.
- The same log with `Up` tips it toward you, and it rises on its plinth as `sits_at` says a tilted shape must.

## Out of scope

Looking away while holding one. You cannot, and could not before: spec 0004
says in its own words that the mouse turns the thing rather than the view.
Return lets go.

Everything else Jake asked for in the same breath, which is a knob on each
shape: the hole size on the three fractals and solid against transparent on
the teapot and the Klein bottle. Those are shapes being rebuilt and a
material being changed, and they need a gesture of their own now that all
four arrows are spoken for.
