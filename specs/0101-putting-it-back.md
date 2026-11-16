# 0101 Putting it back

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: for those that are appropriate, there should be a reset option with the
r key or something else if that's taken.

## Behavior

**R was not taken, quite.** It put you back at the door, and only while
flying: `KeyboardKey::R if held && !input.repeat && self.flying`. On foot it
did nothing at all.

**So it puts back whatever the sight is on, and keeps the door for when the
sight is on nothing.** That is the arrows' own shape, which this building
states in its own words: they work the toy the sight is on and walk you about
when it is on nothing. One key, one idea, and the thing it did before is what
it does when there is nothing else to do.

**Only what can drift and has no other way home.** A cabinet, a door and the
watchman have no state to lose. The wrecker's wall and the globe's spin are
already what their own Return does, and a second key for them would be two
ways to say one thing. What is left is four:

- A row still swinging, which `cradle::settle` hangs still. Left to the drag it takes the better part of half a minute, which is right while you watch it and wrong when you want to start again.
- A rotor still up, which `gyro::stop` squares and stills, and which answers from the wheel or from its bench.
- A needle still ticking.
- A shape somebody turned and walked away from, squared to the row again by `Spin::let_it_go`. The row's own slow turn goes on; what comes off is the hand.

And the panel, which is not a toy and whose Return already goes back to the
top of the dive. It gets R as well because that is the one place a second key
costs nothing: your hands are on the arrows and Return is a long way from
them.

## Acceptance criteria

- R puts back a swinging row, a spinning rotor, a turned shape and a dive, and finds nothing to put back when the sight is on nothing. — `tests::r_puts_back_what_the_sight_is_on`

Two controls. Forgetting the cradle gives "a ball had nothing to put back".
Answering for everything gives "R found something to put back while the sight
was on nothing", which is the arm that keeps the door.

**That second control passed twice before it was right.** `work_a_toy` ends
with the same `_ => return false,` and the same `true`, so editing by line
number put the fault in the wrong function and the test went green against an
unchanged `put_it_back`. A control that cannot fail is the thing this project
keeps catching itself on. It was found by the control passing.

### Verified by hand

Not by eye yet.

## Out of scope

Saying anywhere that R does this. It joins the building's undocumented
inspection keys, which spec 0087 already argued is the convention here rather
than an exception to it, and saying so costs a signing pass on the copy.

The puzzle, which has no one state to go back to that is not also a move in
the game it is.
