# 0080 Picking it up

**Status:** implemented
**Date:** 2026-11-15

## What this is for

Spec 0078 gave a load a way out of the hand and said outright what it had
not done: nothing is ever picked up off the ground, the transfer runs one
way, and a round that took something another round had left would need the
two to share a clock. Jake, 2026-11-15: picking things up.

The shared clock turns out not to be the thing that was missing. Hauling a
bucket up a well is both hands on a rope, and the bucket lands at the
wellhead: the load the water round carries across the street has been on
the ground since before anybody touched it, and the hand closing on it is
one round's own business.

## What it does

**`Stop::takes_at` says where a load comes up off the ground**, next to
spec 0073's `carries`, which already says what the stop leaves holding. The
pair is the mirror of spec 0078's: `puts_at` is where a load the stop
arrived with is set down, `takes_at` is where the load it leaves with was
lying. A stop with neither takes what it leaves with off a shelf, a wagon
bed or a well rope, which is every other stop in this building.

**And the hand closes at the same moment it would have opened.**
`gesture::picks_up_at` is `gesture::lets_go_at`'s other end and both read
`gesture::STOOP_BOTTOM`. Letting go and taking hold are one instant of one
motion seen from either side, so two numbers here would be two accounts of
where the bottom of a stoop's dip is, and a change to the stoop's clock
would move one of them and not the other.

**Beside them, and not in front.** Spec 0079 laid a sack most of the way
from the stop to the thing the stop was at, because at the stop itself it
went down under the body and was never once visible. The other answer to
the same problem is to one side: a body is 0.9 across and an arm is 0.74
long, so `village::SETS_DOWN` is 0.52, which clears the shoulder and is
still inside a stoop's reach. `village::aside` is the one account of which
way across a facing points.

**And the wellhead gets two stops.** Hauling and lifting are two things and
a stop has one errand, which is the shape spec 0079 arrived at at the
hitching rail. `Draw` is six seconds of rope, then a `Stoop` of two and a
half with the pail on the ground beside them for the first three quarters
of a second of it.

## The reach is a bend and not a hand on the floor

Spec 0077 halved the stoop because 45 degrees and knees was a dive, and
nothing here changes that number. What the picture shows is somebody bent
forward over a pail at their feet with the near arm reaching down, which is
as far as a rig whose only fold is a lean can go: the hand stops around hip
height and the load is beside it rather than in it. The load moving from
the ground to the fist across that moment is what says the rest.

## What it does not do

- **Still nothing takes what another round put down.** The limit spec 0078
  named is real and unchanged. What has gone is the claim that it was the
  only way to pick anything up.
- **A load waiting to be taken up is not lying there all loop.** It appears
  at the start of the stay that lifts it, for the same reason spec 0078's
  stays only until its carrier walks away: `Doing` is one body, and a thing
  on the ground with nobody standing over it is a pile growing off a list
  with no memory.
- **Nothing is set down to free a hand and taken up again.** One round can
  say it, since the put and the take are both on its own list, and nowhere
  in this building wants it yet.

## Acceptance criteria

- A load lies at the stop that lifts it, goes into the hand partway through the stay, and is not lying there a whole loop. — `people::tests::a_load_comes_up_off_the_ground`
- No round takes something off the ground with an errand that never reaches it. — `people::tests::every_stop_that_takes_something_up_bends_for_it`
- The pail at the wellhead is clear of the body that lifts it and inside an arm's reach. — `village::tests::the_pail_comes_up_off_the_wellhead`
- And it still leaves the well and comes back empty. — `village::tests::the_pail_leaves_the_well_and_comes_back_empty`
- The wellhead is drawn with the pail on the ground and a hand going down to it. — `tests::the_village_well_is_drawn`

### Verified by hand

- The pail reads as being picked up rather than as standing next to
  somebody. — run the arcade, walk out to the village and watch the well.
