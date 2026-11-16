# 0104 Pointing at the weight

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: metronome. The fifth toy after specs 0098, 0099, 0102 and 0103.

## Behavior

**The weight is the thing a hand goes to**, which is the whole of why this
toy is worth pointing at: sliding it up the needle is what a metronome is
for, and until now that meant aiming at the table it stands on.

**It moves two ways**, which settles where it is sighted: it slides along the
needle when a hand sets the tempo and it swings with the needle while the
thing is going. `Room` holds the places the building keeps.

**Return is its bench's, because the weight has no verb of its own that
Return could be.** What you do to a weight is slide it, and the arrows
already do that. So pressing it starts and stops the thing, and up and down
slide it, found through its own bench by name.

**And `stop` is not a way home.** It levels the arm and clears `going` and
leaves `notch` exactly where a hand put it, so a metronome set to its slowest
stays at its slowest and nothing put the weight back. `home` slides it to
`STARTS_AT` and then stops.

That is the third toy in a row with the same gap, after the globe and the
wrecker's ball, and the third time the rule in spec 0101 needed narrowing:
not "it has no other way home" but "nothing already puts back everything a
hand changed". Three is enough that the next toy should be assumed to have
the gap until it is shown not to.

## Acceptance criteria

- The sight finds the weight, Return starts it, and R slides it back to the notch it came on where stopping does not. — `tests::the_sight_finds_the_weight_and_r_puts_it_back`

The test asserts that stopping leaves the weight where it was slid, before it
asks R to put it back, so a `stop` that quietly grew that job would fail here
rather than make the rest meaningless. That shape came from spec 0103 and is
worth keeping.

Two controls. Never asking about the weight gives "pointing at the weight
finds None". An R that only stops gives "R left the weight at 6 of 8".

### Verified by hand

Not by eye yet.

## Out of scope

The puzzle's cubies, which is the last of them and the only one where a part
is a move in a game rather than a knob on a toy.
