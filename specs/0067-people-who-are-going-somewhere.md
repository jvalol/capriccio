# 0067 People who are going somewhere

**Status:** implemented
**Date:** 2026-11-13

## Goal

The figures at the top of the bridge live there rather than stand there.

Jake, 2026-11-13: the people should move around meaningfully. walking from the
house to the outside firepit, the wagons to the horses, getting food. it should
feel like a true and believable village.

## What was there

A full walk cycle, in `people.rs`, used by exactly one figure: you, in the
mirrors. Spec 0046 built the gait and spec 0061 is the only thing that ever ran
it. Everybody else in this building has stood still since they were drawn.

So the work is not the walking. It is giving somebody a reason to be somewhere
else, and then a way to say where they are at a given second.

## A round

**Arithmetic and not a simulation**, like the strikes, the koi and the moon.
`people::doing(round, since)` is handed the round and the clock and says where
that body is. Nothing is stepped forward and nothing is remembered, which is
what lets any frame be drawn at any `since` at all: every picture in this repo
does that, and a village that had to be run up to would have no pictures.

A `Stop` is a place, a facing to hold while there, and how long to stay. A
round is a list of them and it loops.

- **A stay of nought is a corner.** It is walked straight through, which is how
  a round turns a gable without the turn being a pause.
- **`walked` runs on across the whole round**, so the gait never jumps
  mid-stride, and starts again at nought when the round comes round. That is
  only invisible because a round begins standing; one that opened with a walk
  would snap there once a loop.
- **The turn is eased.** `people::TURNS_IN` is 0.7 seconds at each end of a
  stay, short way round. Without it a figure walks up to the fire facing the
  way it came and is facing the fire on the next frame, which is the one thing
  no amount of leg work survives.
- **`PACE` is 1.1 metres a second**, which is an errand rather than a journey
  and slower than either of your own speeds.

## Where they go

Three rounds over the steading's own geography, which is a hall 24 by 9 with
its door on the gable you arrive at, three outbuildings off the lane, and now
a fire and a woodpile.

- One who keeps the door: out of the hall, up the lane to the north store and
  back. The longest stay is at the door looking out at the climb, which is what
  spec 0055's figure was doing standing still.
- One who keeps the fire: from the woodpile against the south store's gable to
  the ring out in the lane. That is the errand Jake named, and both ends of it
  are things you can see.
- One round the back, past the gable to the third store. It is the only round
  that leaves the lane, and what it is for is that somebody is always arriving
  from somewhere you were not looking.

They are started 4.3 seconds apart, because three rounds of about a length are
otherwise three people moving together.

## The two things they walk to

An errand with nothing at either end is a figure on a track. So:

- **A fire in the lane**, eleven stones round a bed of coals with three flames
  in it. Three because one is a cone: the hall's trench learnt that at sixteen
  metres and a yard fire is the same lesson at two. It throws a light, which is
  what puts somebody walking past it in front of something.
- **A woodpile** against the south store's gable, thirty split logs in five
  courses with every other course laid back a little, which is how a stack of
  split wood sits and what stops it reading as brick.

Both are solid. A fire you can walk through is not a fire, and the ring takes
the trench's own kerb height for the reason the trench has one: below that
`walk` rides over it and you stand in the coals.

## Two things a walled room never had to say

Every other room in this building is a box with walls, so a stop in the wrong
place is a stop inside something and `nobody_walks_through_a_wall` catches it.
The steading is a slab in the sky, and the two faults it has that no room has
are both about what is not there.

**A stop can be over nothing.** The yard is the nine metres between the hall's
door and the east edge of the slab, and the ring was written as the door plus
seven of them: a metre from a twenty-six metre drop, with no side left to stand
on. The man who keeps it stood four and a half metres out in the air for six
and a half seconds of every round, and the walls test passed on him exactly
because there was nothing out there to be inside of. Jake, 2026-11-13, looking
west from the top of the climb: this person is walking on air.

So the ring is written off the yard and not off the door, there is room to
stand all the way round it by construction, and `nobody_walks_on_air` walks
every round and asks for ground under every step.

**A facing can be backwards and look like a number.** A facing of `f` sends a
body's forward to `(sin f, 0, -cos f)`. Three of the steading's four were
written by hand with the sign the other way, so the man at the woodpile, the
man at the north store and the man round the back all walked to their errand
and stood with their backs to it. Nothing in a stop's three numbers shows it,
and the one that was right was right because a test had already made somebody
look at it.

The fix is not a test. `people::facing` works a facing out from where the body
is and what it is there for, and the stops call it, so there is no sign left to
write.

## Acceptance criteria

- Nobody on a round walks through a wall, or the fire, or the woodpile. — `valhalla::tests::nobody_walks_through_a_wall`
- Nobody on a round is ever standing on nothing. — `valhalla::tests::nobody_walks_on_air`
- Nobody stands in the lane you arrive along, at any stop on any round. — `valhalla::tests::nothing_stands_in_the_way_in`
- A round loops, and where it puts somebody is the same at `t` and at `t` plus a whole round. — `people::tests::a_round_comes_round`
- Somebody standing has no gait and somebody walking has one. — `people::tests::the_legs_run_only_while_walking`
- A turn is taken the short way round. — `people::tests::a_turn_goes_the_short_way`
- Nobody teleports: where somebody is moves no faster than they walk. — `people::tests::nobody_moves_faster_than_they_walk`
- The steading with its people out on it matches its reference. — `tests::the_steading_is_lived_in`

### Verified by hand

- It reads as a place somebody lives rather than as figures on tracks. Walk up
  the bridge and stand in the lane.

## Out of scope

- **Carrying, bending and sitting.** The hands hold nothing and nobody gets
  off their feet. What they do at a stop is the errand's own motion, over an
  idle, which is 2026-11-14 below.
- **Going indoors.** Every round is outside. The hall's door is passable and a
  round through it is the obvious next thing, and it wants the figure to not
  be drawn while it is in there or it is a body inside a lit room seen through
  a window.
- **The hall's long fire in daylight**, which stays written against the dark
  because indoors it is. The two yard fires are damped now, which is
  2026-11-14 below.

## 2026-11-14: what they do while they are standing

**Jake: the gesture at a stop, and it only lands if it reads as natural and
not robotic, as though the motion was recorded by a human actor.** Nothing
here is recorded and nothing here will be mistaken for it. What is written
down is the handful of things that separate a person standing from a machine
paused, and they are rules rather than keyframes.

**The robot was the moment they arrived.** A round walked somebody to the
fire and then set every joint to nought. A figure that reads as a person
while it moves becomes a mannequin the instant it gets where it was going,
and standing is most of what anybody out here is doing at any second, so it
was most of what they were.

Four rules, in `gesture.rs`, each of them a thing an animator would say:

- **Nothing is ever still.** Under every errand there is an idle that does
  not stop: breath at four millimetres, a sway off two periods that do not
  divide, and a weight shift every six seconds or so that moves the hips,
  both knees and the whole body's facing at once. The weight shift is the
  big one. Standing evenly on two feet is a thing people manage for a few
  seconds at a time.
- **The joints do not start together.** A shoulder leads and the elbow
  follows a tenth of a cycle later, which is the difference between an arm
  and a lever.
- **No two repetitions match.** Each turn of a cycle takes its own length
  and its own reach off a hash of who is doing it and which turn it is. The
  jitter is in the length as well as the size, because a motion that is
  always the same length is a metronome whatever its amplitude.
- **And it starts and stops at rest.** The gesture fades up over the first
  moments at a stop and back down before they leave, so nobody begins
  mid-stroke or walks away with an arm out.

**Six errands**, named for the errand and not for the motion: `Stir`,
`Lift`, `Draw`, `Tend`, `Knock` and `Wait`. `Wait` is its own errand rather
than the absence of one: somebody at the end of a street with nothing to do
looks at things, held for a beat and brought back, which is what makes it a
glance rather than a sweep. `Tend`, at the horses, stops to think every few
turns, which no amount of smoothing does for a cycle that never pauses.

**What the rig cannot do**, and it is still most of a body. The arms swing
forward and back only, because a limb turns about the walker's own right, and
the head does not turn on the neck. The whole figure turns a little instead,
which is cruder and is what there is.

**The torso leans, which it did not an hour ago.** It was written down here
as the thing that could not be done, on the grounds that every height above
the hips is a separate push in `main.rs`. They are, and all ten of them go
through one accessor, so the lean went in `up_to`: below the hips a height is
plumb and above them it is turned about the hips, which is where a body
folds. The arms hang off it, the legs do not, and the face points along it,
because a nose set off a level forward on a bowed head comes out of the
temple.

It is the difference between a cook and an arm on a post, and it was the
largest single thing left reading as a machine.

### Three things the pictures caught, which no test would have

- **The cook was waving at the fire from across the common.** `CLEAR_OF` is
  1.6 metres and keeps a body out of what it walks past, so a stop at the
  ring put him 2.8 metres from a pot, with an arm 0.7 long. `TENDS_FROM` is
  the ring's own edge and half a stride, which is where somebody stands to
  reach what is over it.
- **And his arm hung down with a twitch in it.** The first build swung the
  shoulder a quarter of a radian, which is not a hand over a pot. A gesture
  is mostly a held pose with a small movement on top of it, and the held
  pose is the part that says what the errand is.
- **A hash with `fract` in it returns -3 to 1, not -1 to 1**, because Rust
  keeps the sign. A quarter of the cycle lengths came out negative, which ran
  the cycles backwards, and a quarter of the reaches came out inside out. It
  presented as five separate faults in five separate errands.

## Acceptance criteria, added

- Nobody standing still is actually still. — `gesture::tests::nobody_standing_still_is_actually_still`
- And no two turns of a cycle are the same. — `gesture::tests::no_two_turns_of_a_cycle_are_the_same`
- A gesture fades up and down rather than switching on. — `gesture::tests::a_gesture_fades_up_and_down_rather_than_switching_on`
- The shoulder and the elbow are two signals and not one. — `gesture::tests::the_shoulder_and_the_elbow_do_not_turn_together`
- And nothing bends the wrong way or too far. — `gesture::tests::nothing_bends_the_wrong_way_or_too_far`
- Somebody at work looks like it, close enough to see. — `tests::the_village_cook_is_drawn`
- Nobody ever stands inside anybody else. — `people::tests::nobody_ever_stands_inside_anybody_else`
- And being moved out of each other puts nobody in a wall. — `village::tests::being_moved_apart_never_puts_anybody_in_a_wall`

### Verified by hand

- Whether it reads as a person rather than as a part oscillating, which is
  the whole of what was asked and the one thing in here no test can see.

## 2026-11-14: the fire at noon

**It was a white blob and now it is orange.** `aim::HEARTH_FIRE` asks for 3.6
in the red and 1.65 in the green, and two channels past one is white. Against
the dark that reads as a fire because nothing near it is bright. At noon it is
a hole in the lane, and nobody had taken a picture at that hour, which is why
it sat in Out of scope.

`garden::firelight` brings the want down as the sun comes up and the two yard
fires multiply it in. The hall's long fire does not: indoors it is against the
dark whichever hour it is.

### Two things got this wrong before it was right

**The first damping was on the wrong fire.** `aim::FIRE` and the flame loop I
found first are the cellar's hearth, three floors down and never in daylight.
It was committed to nothing and caught by the picture not changing.

**And the number was on the wrong side of the arithmetic.** `gives_off`
divides out the ambient so a colour lands on screen as written, which made it
look as though bringing the want under one was enough. It divides out the
ambient and nothing else: at noon the sun multiplies on top of that, so a want
of 0.58 in the green still rendered white. The share of sun on a flame depends
on which way the flame is facing, so what fixed it was rendering the common at
noon three times rather than working it out. A tenth, at noon.

## 2026-11-14: the picture that was missing

**Every reference in the suite passed when the lean went in.** Not because
the lean was wrong but because nothing in them shows a standing figure big
enough to read: the steading's picture is pinned at a second where somebody
is mid-stride, and the village's two are forty metres of street. So the whole
of what a body does while it is standing could have been written and not
applied, and nothing would have said so.

`tests::the_village_cook_is_drawn` is that picture: the cook at the pot, five
metres off, at a second found by walking his own round until he is well into
the stay under a fair afternoon. Writing the lean and leaving it unapplied
fails it, which was checked by doing exactly that.

## 2026-11-14: they do see each other, after all

**This was out of scope on the grounds that nobody had seen it happen.** The
arithmetic says it has to. The rounds in one place run on their own lengths,
23, 32, 45 and 106 seconds in the village, and no two of those divide, so
every phase relationship between two rounds comes round eventually and any
two paths that cross in space cross in time sooner or later. Measured over
twenty minutes, the closest two villagers come is three centimetres, at 633
seconds. That is one person wearing another.

**Both ways out that need nobody to notice anybody fail.** One clock for the
village makes the phases fixed and the offsets a one-off choice, and it means
padding a round of 23 seconds out to 106, which is somebody standing at a
wagon for forty-seven seconds in a building whose day is twenty. Paths that
never cross is not a village, because a street is a thing people cross.

So `people::apart` moves them. One symmetric pass per pair closer than
`ROOM`, twice over so a knot of three settles, each pushed half the
shortfall. It is not steering, nobody looks where they are going, and it is
the smallest thing that is never wrong. Positions stay a pure function of the
time, which is what keeps a picture able to pin a second.

**And it knows nothing about walls**, which is the question worth asking of a
push that only knows about people. Ten minutes of every round, nudged, held
against the same boxes the walk test uses: nobody lands anywhere solid. That
is a measurement rather than a guarantee, and it is written here so the next
person who moves a cabin knows to run it again.
