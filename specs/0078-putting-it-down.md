# 0078 Putting it down

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0077 gave somebody a stoop and a reason to use it: the fire-keeper
walks a log to a ring of stones at ankle height and bends over it. Then they
straighten, walk away, and the log stops existing at the moment their foot
leaves the stop. The same at the hall's trench once the store turned out to
hold firewood.

That is a transfer that does not transfer. Jake, 2026-11-14: the transfer.

## What it does

**A load leaves the hand partway through the stay.** Spec 0073's rule says
what you leave a stop holding, which is enough to say that a load is given
up here and not enough to say when. The errand says when, because the errand
is the motion that does it: `gesture::lets_go_at` gives the bottom of a
stoop's first dip, 0.78 of a second, and `None` for every other errand,
which is what they all did before.

**And it lies where it was put.** `Stop::puts_at` is the place, and it is
the thing the stop is at rather than a reach in front of it, because a stop
that works at something already faces it and `people::facing` is the one
account of which way round that goes. Written as a reach, a log would land
between somebody and the fire they were putting it on.

**Where you look and where you reach are two points.** You look at the fire
and lay the wood on the edge of it nearest you. Laid at the middle, a log
goes into the flames and is a brown sliver behind a stone, which is what the
first build of this was, and the hall's trench is sixteen metres long so its
middle is eight metres out of reach.

**A log lies and a pail stands.** Both hang plumb from a fist, which is
gravity on a thing held at one end. The moment it is on the ground that
stops being true of the log: a billet standing on end in a fire is a post.

**And it is gone when they go.** What happens to a log after somebody walks
away from a fire is the fire's business, and a round has no memory: a load
that stayed would be a pile growing for ever off a list that cannot count.

## What it does not do

- **Nothing is ever picked up off the ground.** The transfer runs one way.
  A round that took something from where another round left it would need
  the two to share a clock, which nothing here does.
- **No load outlives the person who put it down.** See above.
- **Only a stoop lets go.** Every other errand holds what it has until it
  walks away, which is what they all did.

## Acceptance criteria

- A load leaves the hand partway through the stay, lies where it was put, and is gone once they leave. — `people::tests::a_load_leaves_the_hand_partway_through_the_stay`
- Both fires actually take the wood somebody carried to them. — `valhalla::tests::the_wood_ends_up_on_the_fires`
- The rounds still walk through no wall. — `valhalla::tests::no_round_walks_through_a_wall`

### Verified by hand

- The log is on the fire and not in the flames. — run the arcade with
  `--done`, walk up the bridge, go into the hall and watch the near end of
  the trench.
