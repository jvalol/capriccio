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

**Both of those were wrong, and specs 0102 and 0103 took them back.**

The globe's Return stills the spin and leaves the ball showing whatever
longitude it had got to, so "no other way home" was the one thing it did not
have: nothing in the building could put the Atlantic back in front of you.
The wrecker's `rebuild_wall` returns at once if the wall is whole, so a ball
swung through nothing and a chain wound out to 1.25 stay exactly where they
were left.

The fault is the same both times and it is in the reasoning rather than in
either toy. A Return that does one of the things a hand can change looks like
a reset while you are reading it, and is not one. Return is a hand on the
thing; R is where it came from; two verbs and not one said twice. Neither
clause above was reasoned from a test, because no test was written for a toy
being ruled out. What found both was pointing at the thing.

So the rule the four bullets are really stating is narrower than "it has no
other way home": it is that nothing already puts back *everything* a hand
changed. The wrecker's wall is still not on the list for its own sake, and
the globe's spin still is not. What they needed was the rest.

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
