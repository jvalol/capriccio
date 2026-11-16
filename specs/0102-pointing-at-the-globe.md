# 0102 Pointing at the globe

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: the globe next. The third toy after specs 0098 and 0099, one at a time
as promised.

## Behavior

**The one part so far that does not move.** A ball on the cradle swings and a
rotor walks its frame, so neither could be in `Room`'s list; the globe turns
in place on a fixed stand, so it could. It goes with the others anyway. What
`Room` holds is the building, a ball on a toy is the toy's, and where the
parts are asked about is one place or it is several.

**Return is a hand on it, and R is the stand it came on.** That difference is
what earns the globe a second key, which spec 0101 said it had not: at the
bench, Return sets the spin to nought and leaves the ball showing whatever
longitude it had got to, and nothing in the building could put the Atlantic
back in front of you. R does. A reset that only stilled it would be the same
key twice, which is why the wrecker's wall still has none.

The arrows flick it as they always have, found through its own bench by name
rather than copied, which is spec 0099's arrangement with a second toy in it.
The plate says what its bench says.

## Acceptance criteria

- The sight finds the globe, Return stills it where it stands, and R takes it home. — `tests::the_sight_finds_the_globe_and_r_takes_it_home`

Two controls. Never asking about the globe gives "pointing at the ball finds
None". An R that only stills it gives "R left it where it had got to", which
is the whole of why it has one.

Both controls were written to fail if their own edit did not apply, after
spec 0101's second control passed twice against an unchanged function.

### Verified by hand

Not by eye yet.

## Out of scope

The metronome's weight, the puzzle's cubies and the wrecker's ball. Still one
at a time.
