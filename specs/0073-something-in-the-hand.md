# 0073 Something in the hand

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Jake's ask for the village, in spec 0068, was a place where the people move
around meaningfully: from the house to the outside firepit, the wagons to the
horses, getting food. Everything in that sentence was built except the last
part of it. Nobody's hands held anything, so every errand was somebody
walking to a place, doing a motion at it, and walking back with exactly what
they came with, which is a patrol rather than an errand.

Spec 0067 listed carrying under what it does not do, and said the hands hold
nothing. I twice told Jake the reason was that the rig has no hand. That was
wrong and a look at the file says so: `people::HAND_WIDE` is drawn at the end
of every forearm, and Gjallarhorn has hung off the watchman's fist since spec
0055, with the comment beside it saying why it hangs from the wrist rather
than floating beside it.

## What it does

**Two loads, and both of them named by an errand that already existed.** Spec
0067's `Draw` is a bucket up a well hand over hand, and its `Lift` at the
steading is an armful off a woodpile. What is in the hand at the end of
either is not a decision anybody has to make. What is in a wagon or in a
store is, so the wagon, the store and the cook fire carry nothing until
somebody says what is in them.

**A stop says what you leave it holding**, not what you arrive holding. A
load is picked up somewhere, and the place it is put down is just wherever
the round goes next. So `Stop::carries` sits on the well and on the woodpile,
and every other stop says nothing.

**And a corner is not a stop.** A stop stayed nought is a bend in a leg, which
is what every one of them in this building already is. Read without that, the
pail was picked up at the well and put down two metres later at the corner
that aims the walk across the street. The lookback walks past anything with
no stay.

**Hung off the fist, turned by the forearm.** No turn of its own: both meshes
run from their origin down their own -y, which is the convention every limb
here follows, so a pail on a hanging arm hangs plumb and on a swinging arm it
swings. In the right hand, where the horn is in the left, so the watchman
could hold both.

**Both off the lathe, and both closed.** A pail is straight-sided and a
little wider at the rim; a log is a plain billet with its ends cut flat.
Open at the top means two-sided or it is a tube you see the inside of the far
wall of, and neither has an inside worth showing at this size. The log takes
the woodpile's own colour, because it came off that pile.

## What it does not do

- **Nobody carries with two hands.** An arm here swings fore and aft about
  the walker's own right, so there is no cradling an armful and no carrying
  a thing between two people. One fist is what there is.
- **The load appears when they step off.** There is no moment of closing a
  hand round it: the `Lift` and `Draw` motions over the stay are what cover
  the change.
- **It costs nothing in the stance.** Somebody carrying a weight in one hand
  leans away from it, and this does not. That lives in `people::walking`
  rather than in `gesture`, which is a change to the gait and not to this.
- **You carry nothing.** The figure of you in spec 0061's mirrors gets
  `None`, because it is you and you are not holding a pail.
- **The wagon, the store and the cook fire are still empty-handed**, for the
  reason above.

## 2026-11-14: the store has firewood

Jake, asked what was in it. So the door-keeper's round carries a log from
the north store, through the yard, in at the door and onto the hall's long
fire, which is what the comment at the head of that round has promised since
spec 0067: back with whatever was in it.

**It needed the yard to say so too.** A load is put down at the first stop
with a stay that does not name one, and the yard is a stop with a stay
between the store and the door. Without `carries` on it as well the log is
set down out in the lane, which is what
`valhalla::tests::the_firewood_gets_from_the_store_to_the_hall` catches when
the line is taken out.

That is the cost of saying a load on the stop it comes from rather than
naming a journey, and it is the right cost: a journey would be a second
account of the round's own order.

**And the stop it ends at is a stoop**, per spec 0077, because the trench is
a bed of coals at ankle height and what was there was `Stir`, which is a hand
over a pot.

The wagon at the east end of the village street and what the cook brings out
of her cabin are still nobody's decision.

## Acceptance criteria

- What is in the hand is what the last stop with a stay said, through all four moments of a round. — `people::tests::a_load_is_carried_from_the_stop_that_gives_it`
- The pail leaves the well, goes somewhere, and comes back put down. — `village::tests::the_pail_leaves_the_well_and_comes_back_empty`
- The wood gets from the pile to the fire. — `valhalla::tests::the_wood_gets_from_the_pile_to_the_fire`
- And the firewood gets from the store all the way into the hall. — `valhalla::tests::the_firewood_gets_from_the_store_to_the_hall`

### Verified by hand

- A pail reads as a pail at walking distance. — run the arcade, walk out to
  the village and watch the well.
