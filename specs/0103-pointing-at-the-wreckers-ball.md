# 0103 Pointing at the wrecker's ball

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: the wrecker's ball next. The fourth toy after specs 0098, 0099 and
0102.

## Behavior

**Two things on one bench, and now a key each.** Return at the bench rebuilds
the wall, which is what you do to the wall. Return at the ball catches it,
which is what you do to a ball swinging on a chain. Both are worth having and
neither was reachable before without pointing at the table.

The arrows haul it as they always have, found through its own bench by name,
and shift with up or down still winds the chain. The plate says what its
bench says.

**And `rebuild_wall` is not a way home**, which is what earns this one an R.
It returns at once if the wall is whole, so a ball swung through nothing and
a chain wound out to 1.25 stay exactly where they were left, and nothing in
the building put either back. `home` lays the wall whatever state it is in,
stills the ball, and winds the chain back to the length it came on.

That is the second time this has turned up. Spec 0101 ruled both the globe
and this out of needing a reset on the grounds that their own Return already
did the job, and both times the ruling was wrong in the same way: Return
does one of the things, and the rest of what a hand changed had no way back.
The lesson is in 0101 now, and it is that the question is answered by
pointing at the thing rather than by reading the function that looked like
the answer.

## Acceptance criteria

- The sight finds the ball, Return catches it, and R winds the chain back where rebuilding the wall does not. — `tests::the_sight_finds_the_wreckers_ball_and_r_puts_the_rest_back`

The test asserts that rebuilding the wall leaves the chain wound out before
it asks R to wind it in, so a `rebuild_wall` that quietly grew that job would
say so rather than making the rest of the test meaningless.

Two controls. Never asking about the ball gives "pointing at the ball finds
None". An R that only rebuilds gives "R left the chain wound out".

### Verified by hand

Not by eye yet.

## Out of scope

The wall as a thing to point at. Pointing at it to rebuild it is the obvious
next one and it wants the wall's own bounds, which are five by six bricks and
a gap where they have been knocked out.

The metronome's weight and the puzzle's cubies. Still one at a time.
