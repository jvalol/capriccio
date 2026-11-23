# 0084 The sack is in the wagon

**Status:** implemented
**Date:** 2026-11-15

## Goal

The loading round's first stop stands at a wagon doing a two-handed heave with
nothing in its hands, for five and a half seconds, and the feed appears the
instant its foot leaves the stop. Put the sack in the bed up to the moment the
hands are on it.

## Behavior

Spec 0078 said a load that vanishes when a foot leaves a stop is not a
transfer, and gave `lets_go_at` the moment the hand opens. Spec 0080 said the
same of the other end and gave `picks_up_at` and `Stop::takes_at`. Between
them they converted every stop that puts something down and the one at the
wellhead that takes something up. The wagon was the stop neither reached.

Two things were in the way, and both were the same mistake in different
clothes: `picks_up_at` answered only for `Errand::Stoop`, and its own words
said why, that a stoop is the only thing in this building that gets a hand
down to the floor. True, and an answer to a question nobody asked. What a
round needs to know is when the hands close, not how near the ground they
are. Where the thing sits is the round's business, and this round's thing
sits in a wagon.

So `picks_up_at` answers for `Errand::Lift` as well, at `LIFT_BOTTOM`, 1.7
seconds, which is halfway through a 3.4 second heave where the `down` term's
`(1 - cos(through * TAU)) * 0.5` peaks. Its own number rather than
`STOOP_BOTTOM`: the two are the same kind of moment on two different clocks,
and one number above both would move the wrong one the day either turn
changed.

The wagon stop then gets `takes_at` at the bed, taken from the cart rather
than from the man standing at it. He is `CLEAR_OF` clear of the near side, so
the bed starts that far ahead of him, and the sack sits a quarter of the
wagon's width into it at `WAGON_BED`. A sack on a bed is in the bed's place,
and the day the wagons move it moves with them.

`looks(facing)` is the forward vector of a facing, which `doors` and `fronts`
had each written out and this needed a third time.

**What it does not fix.** He still does not reach the sack. `CLEAR_OF` stands
everybody 1.6 metres clear of what their errand is at and an arm is 0.74, so
nobody in this street touches anything they work at: the drawer at the well
hauls a rope that is not there either. That is a decision above this one and
it is left alone here.

**And one field now means two things.** `doing.left` said "has been set down"
and now says "lying here", which covers waiting to be picked up as well.
`the_feed_gets_from_the_wagon_to_the_horses` read it the old way and took the
sack waiting in the bed for the sack laid at the rail, measuring nine metres
from the horses. It watches the step the load leaves his hands instead.

## Acceptance criteria

- The feed lies at the wagon's bed height and inside its footprint. — `village::tests::the_feed_is_in_the_wagon_until_it_is_lifted`
- It is in his hands from `LIFT_BOTTOM` into the lift and lying where the stop says before that, for a lift as well as a stoop. — `people::tests::a_load_comes_up_off_the_ground`
- A stop that takes something up leaves carrying it, with an errand whose hands close. — `people::tests::every_stop_that_takes_something_up_bends_for_it`
- The feed still gets from the wagon to a sack's length of the rail. — `village::tests::the_feed_gets_from_the_wagon_to_the_horses`

### Verified by hand

- The man at the east wagon has the sack in his hands partway through the heave rather than conjuring it as he walks. — `./target/release/xanadu --screenshot=village --picture-to /tmp/v.png` and look at the right of the frame.

## Out of scope

Standing anybody within arm's reach of what they are working at, which is
`CLEAR_OF` and every errand in the street rather than this one.

Letting go into something. `lets_go_at` still answers only for `Errand::Stoop`,
because nothing in this building puts a thing down into anything.

Restaging the village picture. The shot is at 300 fixed frames in the engine,
shared by all eighteen, and what was wrong with it was the pose rather than
the moment.
