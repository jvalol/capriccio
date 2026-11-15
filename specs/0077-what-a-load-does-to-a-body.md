# 0077 What a load does to a body

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0073 put things in people's hands and left three notes behind: that the
stance does not change when somebody is carrying a weight, that nobody bends,
and that carrying and sitting had never been put together. Jake, 2026-11-14:
the three figure ones.

They are one thing. A load has weight, and weight has to go somewhere.

## What it does

**A load leans the body away from itself.** `gesture::laden` is added on top
of whatever else the body is doing, which is what makes it work while
walking, standing and sitting alike: the loaded arm hangs long and straight,
the free one comes out to balance it, and the whole figure tips a little the
other way. `Load::how_heavy` says how much of that each one earns, because a
pail of water is heavy and a billet of firewood is not, and one number for
both would bend somebody sideways under a stick.

**The gait takes the stance on top of itself.** While walking, the arms and
the rise came from `people::walking` alone and the stance was thrown away, so
a lean written into the stance reached the torso and not the arms. It is
added now, and is nought for anybody carrying nothing.

**And a load hangs plumb rather than along the forearm.** Spec 0073 took the
arm's rotation, on the argument that it should swing with the arm rather than
float beside it. That argument is about where it is, which the fist still
says. Which way it points is gravity's, and a log on an arm reaching down to
a fire came out level, carried like a lance.

**Somebody bends.** `Errand::Stoop` folds at the hips with the knees taking
about a third of it, because the only fold this rig has is `lean` and
reaching the ground with that alone needs about 1.2 radians, which puts a
head past its own toes. It goes down once and spends the rest of the turn
upright: the shape the rest of this module uses holds its middle for most of
a cycle, which is the wrong way round here, so the dip runs on a clock of its
own that finishes a third of the way through.

**And it has somewhere to happen that needed nothing new.** The steading's
fire-keeper already walks a log from the pile to the ring, per spec 0073, and
the ring is eleven stones at ankle height. That stop was `Stir`, which is a
hand over a pot, and there is no pot there. It is a stoop now.

**A seated body puts its load down.** A seated hand is at the height of the
seat, so a thing hanging from it hangs inside the bench. It goes on the floor
where the hand already is in plan, which is what anybody does with a pail
when they sit.

## What it does not do

- **No round sits holding anything yet.** The behaviour is there and was
  looked at by giving the sitter a pail for one render, which is not
  committed. What the north store has in it is still a decision nobody has
  made, and that is where a load the door-keeper could sit with would come
  from.
- **Nothing is set down by bending.** The stoop is a motion, not a transfer:
  the log stays in the hand through the stay. A load with a position of its
  own, separate from a hand, is what that would take.
- **Nobody bends anywhere else.** One stoop, at one fire.

## Acceptance criteria

- A stoop goes down once and spends most of its turn upright, at a depth that is neither a nod nor a dive. — `gesture::tests::a_stoop_is_a_thing_you_do_once_and_get_up_from`
- A load leans the body away from itself, and nothing at all when there is none. — `gesture::tests::a_load_leans_a_body_away_from_it`
- The rounds still walk through no wall with the new errand. — `valhalla::tests::no_round_walks_through_a_wall`

### Verified by hand

- Somebody carrying a pail reads as carrying something heavy. — run the
  arcade, walk out to the village and watch the well.
- The fire-keeper bends to the ring rather than reaching over it. — run it
  with `--done`, walk up the bridge and watch the lane.
