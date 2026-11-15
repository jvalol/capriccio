# 0074 Sitting down

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0060 put a bench down both flanks of the hall at the top of the bridge
and a high seat at its far gable, and spec 0072 sent somebody into that hall
for the first time. They stood at the fire. Every seat in this building has
been furniture.

Spec 0067 put sitting out of scope in the same breath as carrying, and
carrying turned out to be an afternoon once anybody looked at the rig rather
than at the sentence about it. Jake, 2026-11-14: sitting.

## What it does

**A seated pose is a `Stance` like any other.** `gesture::sitting` is its own
function rather than a flag on `standing`, because the big thing about
standing is the weight going from one foot to the other and somebody on a
bench has put theirs down, and because the errands are no use seated: `Lift`
dips both knees and `Draw` works two arms over a well.

**The thigh takes whatever angle puts the feet on the floor.** Not a right
angle, which was the first build: the steading's benches are 0.52 and a shin
is 0.447, so a level thigh leaves the feet seven centimetres in the air,
which is near enough to read as a mistake and not far enough to read as
dangling. The knee keeps the shin plumb under the thigh. Where a seat is
lower than a shin the same arithmetic comes out negative and the knees rise,
which is what a low stool does to you.

**And the shift is on the seat rather than between the feet.** Nobody holds
one position on a bench either, so there is a lean, a knee out further than
the other, and a turn of the whole figure, every eight seconds or so. The
looking-about from `Wait` goes over the top of it, because somebody sitting
in a hall looks at things.

**The drop onto the seat is the drawing's and the pose is the gesture's.**
Only the drawing knows how tall this particular body is, so `sitting` takes
the seat height and the body's height and gives angles, and the drawing turns
the seat height into the metres the hips come down by.

## What the hall turned out to be like

**You cannot stand in front of these benches.** The colonnade stands 2.6
either side of the centre line and the benches reach 1.1 in from the walls,
which leaves 0.19 between a post and a bench. `benches` own comment has said
so since spec 0060: a strip narrower than a body and never anywhere to walk.

So a bench here is reached by stepping between two posts and sitting on it,
and the stop is on the bench rather than in front of it. The round takes the
bay nearest the door, worked out from `post_line` rather than written down,
so the walk threads the gap instead of a post.

That leaves a body whose box overlaps the bench it is sitting on, which both
of this module's wall tests called a fault. They skip the benches now, with
the reason written in both: what those tests are for is walls, and a bench
can only be reached by occupying it.

**And the way to the bench goes round the fire.** The fire is stood at on the
door's own line and the bench is down one flank, so the straight line between
the two cuts the corner of a trench that `FIRE_KERB` exists to stop anybody
walking into. A corner either side of the sit takes it round the end, and the
same point serves going and coming back.

## What it does not do

- **Nobody sits anywhere else.** The high seat at the west gable is the one
  thing in that room that is somewhere rather than somewhere to sit, per
  spec 0060, and it stays that way. The village has nothing to sit on.
- **No getting up.** The pose fades in and out at the ends of the stay like
  every other gesture here; there is no rising motion between standing and
  sitting, and at a walking pace nobody has seen one yet.
- **Nothing is held while seated.** Spec 0073's loads and this have not been
  put together, and the round that sits is not carrying anything when it
  does.
- **Bending still stands as out of scope**, which is the third thing spec
  0067 listed.

## Acceptance criteria

- A seated shin hangs plumb and the foot reaches the floor, across seat heights and body sizes. — `gesture::tests::a_seated_foot_reaches_the_floor`
- Somebody sitting is never still. — `gesture::tests::nobody_sitting_is_actually_still`
- Somebody sits down in the hall, on the bench rather than beside it. — `valhalla::tests::somebody_sits_down_in_the_hall`
- And the round still walks through no wall. — `valhalla::tests::no_round_walks_through_a_wall`

### Verified by hand

- It reads as somebody sitting rather than somebody kneeling in the air. —
  run the arcade with `--done`, walk up the bridge, go into the hall and
  wait.
