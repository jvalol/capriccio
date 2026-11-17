# 0112 The panel is modal

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: the Mandelbrot should be modal, meaning the arrows shouldn't interact
with it until the user wants them to. Until then they should just be motion
in the room.

## Behavior

**Standing in front of it was enough.** `diving` was the sight and nothing
else, so walking the gallery's centre line took the arrows off you the moment
the panel filled the sight, and what had been turning you became a dive. It
is the complaint spec 0106 was written for, one wall along from the toys and
with no reach to pull in: a panel on a wall is a thing you look at from
across the room.

**So it is taken hold of, the way a shape is.** This building already had the
pattern and did not use it here: a shape in the row does nothing to your
arrows until Return takes it, and `Spin::holding` is what the arrows ask.
Return on the panel takes it or lets it go, and `diving` is that mode and the
sight together.

**Both, and not the mode alone.** Holding it and walking away would leave the
arrows diving a wall behind you, which is the stranding this was meant to
stop rather than move. The dive itself keeps its place while you are away:
what ends is the hold, not the depth.

**Return stops resetting the dive, and R goes on doing it.** Spec 0101 gave
R that job on the grounds that Return already had it and a second key cost
nothing. What cost something was the arrows. Return has the hold now and R
has the reset, which is one key an idea rather than two keys for one.

**And the plate says which of the two you are in**, because a plate that
promises the arrows while they are walking you is the fault spec 0106 went in
under. `mandel::says` takes the hold and picks its line.

## Acceptance criteria

- A route that turns in front of the panel is a route, and the same route after Return is one whose turn was eaten. — `tests::a_turn_the_panel_eats_is_a_fault_when_the_route_carries_on`

**That test was already here and was about the opposite.** Spec 0042 wrote it
to prove the route checker notices a turn the panel swallowed, and it staged
the fault by walking up to the panel, which used to be enough. It fails on
this change, correctly, because the staged fault stops happening.

It has both halves now. The first is the modal gate and the second is the
fault the checker is for, and the second is what keeps the first honest:
without it the test would pass against a panel that had stopped eating turns
altogether, and the checker would be guarding nothing.

The control is the old rule. Putting `diving` back to the sight alone gives
"the panel ate a turn from somebody who had not taken hold of it".

### Verified by hand

Not by eye yet.

## Out of scope

The toys, which still take the arrows from the sight alone at an arm's
length. That is spec 0106's answer and Jake has not said it is wrong.

What the mouse does, which was never the complaint: it worked throughout.
