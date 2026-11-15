# 0079 Feed for the horses

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Jake, asked what was in the wagon at the east end of the street: feed for
the horses. Spec 0073 left that stop empty-handed because what a wagon holds
is not a thing arithmetic can answer, and this round has walked from a wagon
to a rail of horses carrying nothing since spec 0068.

## What it does

**A third load, a sack.** Full almost to the top and gathered hard at the
throat in the last fifth. The first build tapered evenly from the foot,
which off a lathe is an urn, and a sack of feed standing by a rail looked
like a piece of pottery. It is the heaviest thing anybody here carries, so
it leans its carrier further than a pail does, and it is the one thing in
this village that is cloth rather than timber, earth or iron, which is why
it has a colour of its own.

**Two stops in one place at the rail.** A stop has one errand and this is
two things: you set the sack down, and then you see to the horse. So the
`Tend` that was there is now a `Stoop` that puts the feed down, followed by
the `Tend`.

**And it goes most of the way to the rail**, not half. Half is at the
carrier's own feet: somebody stands `RAIL_LONG * 0.36` off the rail and is
0.9 across, so the sack went down under them and was never once visible.

## Every profile in this file is in metres

Spec 0076 is about a size handed to a scale. This spec made the same mistake
twice in one sitting, in both directions.

**The fourth load profile was written in metres** to match the rule 0076
set, while the three before it were shapes that `Load::size` scaled. The
drawing multiplied it by its own size a second time and drew a sack a tenth
of a metre across.

**And the fix went the other way.** Putting all three on the file's rule
meant the drawing stopped scaling them, which it did before the other two
profiles had been converted, so for one build a pail and a log were drawn a
metre across. `a_figure_is_not_nothing` caught that on the first run after
the loads were added to the profiles it walks, which is why they are in it
now.

## What it does not do

- **Nothing eats it.** The sack sits where it was put until its carrier
  walks away, per spec 0078. The horses are spec 0070's and do not know it
  is there.
- **The cook's cabin is still nobody's decision.**

## Acceptance criteria

- The feed gets from the wagon to within a head's reach of the rail. — `village::tests::the_feed_gets_from_the_wagon_to_the_horses`
- Every profile in this file, the loads included, is a part of a body or a thing in metres. — `people::tests::a_figure_is_not_nothing`
- The village's rounds still walk through nothing. — `village::tests::nobody_walks_through_anything`

### Verified by hand

- The sack reads as a sack and not as an urn. — run the arcade, walk out to
  the village and watch the rail.
