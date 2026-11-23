# 0130 A clock that keeps your time

**Status:** implemented
**Date:** 2026-11-22

## Goal

Jake: a large analog clock in the utility room that matches the real time of
day and also demonstrates the working gears that run it.

Three things, and the third is the one that makes it worth building. A clock
on a wall is a texture. A clock that says the right time is a readout. A
clock whose wheels are in plain sight, turning at the rates their own tooth
counts give them, is the only one of the three that can be read as an
argument: this is how a machine divides a day.

## Behavior

**In the load room.** Jake, on where it belongs: the load room, since it
also has to do with representing the overall mechanics of the building. Spec
0024's back room is where the arcade keeps its instruments, a bench and a
block of cubes and a board saying what a frame costs, and its own spec calls
it the room where you can watch the building work. A clock whose wheels are
in plain sight is that kind of thing, and not a thing that wants a room of
its own.

On the north wall, which is the one clear wall in there: the door is in the
west side, the bench runs down the east and the bay with the test rig in it
takes the whole south end. The ceiling is 2.8 where the rest of the
downstairs is 2.4, so the ring is 1.84 across.

**It was a utility room of its own for an hour.** Under the garden, west of
the baths, through a door cut in their back wall. Two things were wrong with
it and Jake found both by walking in: the baths' dado ran straight across the
new doorway, drawn and never collided, which is a wall you can walk through;
and a plant room is not where this belongs anyway. Both faults went with the
room.

**It keeps the machine's time and not the building's.** Jake asked for a
clock that matches the real time of day, and that is what this is: the hands
are at the hour the player's own machine says it is, down to the second.

**And the building's own time could not be read off a clock anyway.** A day
in the garden is `garden::DAY`, twenty seconds since 2026-11-12, per spec
0019. A minute hand on that would go round three times a second, an hour
hand twice every twenty seconds, and the escapement, whose whole point is
one tick a second, would beat a hundred and eighty times in one. Even at the
twenty minute day that was parked it is seventy two times real time, which
is still a blur. So the two clocks in this building disagree, and the
disagreement is the price of either of them being legible.

**A skeleton clock, because the gears are the point.** The dial is a chapter
ring a metre and a half across with the hours marked on it and nothing
behind them, so the whole movement stands inside the ring in plain sight: the
train below the hands, the escapement at the bottom of it, and the pendulum
hanging below the ring and swinging in front of the wall.

**And it is ornate, which took a second pass.** Jake, on the first build of
it: the clock could be ornate. What it was then was a flat white band with
twelve bars on it, two sawn posts and three tapered sticks, which is a
clock drawn as its own diagram. Five things changed, and every one of them
is a shape written as a profile rather than a box:

- *The ring is a moulding.* Its section steps up from the inside, runs flat
  across the chapter, and falls over a bead at the outer edge, so it catches
  the light in three places rather than one.
- *The chapter carries a minute track of sixty*, every fifth of them longer,
  which is the hour marks as well: one ring measured once rather than two
  lists of one thing.
- *And Roman numerals*, each standing on its own radius, so the twelve is
  upright and the six is upside down the way a chapter ring's are. Three
  letters is the whole alphabet a clock needs and all three are straight
  lines, so a numeral is a handful of strokes worked out from its letters.
  Four is `IIII` and not `IX`, which is what a clock dial does and has done
  since dials, so that the four balances the eight across from it.
- *The hands are shaped*: a tail behind the arbor, a boss, a slim shaft, a
  spade near the end and a point. One outline, three hands, what differs
  being how far each reaches and how heavily it is cut.
- *And the ring is the load room's own white, burning.* Jake: it would be
  fitting if the white glowed with the brightness of the cubes in the load,
  same room after all. The ring was a warm bone, which is a second white in
  a room that already had one. The hue is the block's now. The push over
  one is the part that is not simply the cubes' number: nothing in that bay
  glows, the block being 0.88 and reading as bright because a bulb stands
  over it and the lining behind it is near black. A ring on a grey wall
  with neither of those, given 0.88, is a grey ring. So it carries the
  colour and burns the way the gauge bars beside it do, which is what the
  cubes look like rather than what they are.
- *The pillars are turned and the bob is a lens.* A pillar has a foot, a
  torus, a shaft, a swell, a cap and a finial, which is the one part of a
  skeleton clock that is there to be looked at rather than to do something.
  A bob is a lens because it has to be heavy and thin, and the dome is the
  whole of why one hangs rather than sticking to the wall.

**The train is a real train, and the ratios come out of the teeth.**

| wheel | teeth | drives | one turn in |
| --- | --- | --- | --- |
| centre wheel | 64 | third pinion | an hour |
| third wheel | 60 | escape pinion | eight minutes |
| escape wheel | 30 | the pallets | a minute |
| cannon pinion | 10 | minute wheel | an hour |
| minute wheel | 30, on a pinion of 8 | hour wheel | three hours |
| hour wheel | 32 | the hour hand | twelve hours |

The pinions are 8 leaves each where the table does not say otherwise. Sixty
four over eight is eight, sixty over eight is seven and a half, and eight
times seven and a half is sixty: the escape wheel turns sixty times while
the centre wheel turns once, which is a minute against an hour.

**And the motion work's two meshes have to span the same distance**, which
is what picks its numbers. The cannon pinion rides the minute hand's own
arbor and the hour wheel is concentric with it, so the minute wheel between
them is the same distance from the centre in both meshes: ten into thirty
and eight into thirty two is three times four, and both pairs sum to forty.
Twelve into thirty six and ten into forty is the same twelve to one and
sums to forty eight and fifty, which is a wheel that cannot reach.

Nothing in the drawing is told how fast to turn. Each wheel's rate is worked
out from the one that drives it, and the hands are put on the arbors they
belong to, so a wrong tooth count comes out as a clock that keeps the wrong
time rather than as a wheel that looks odd.

**And the wheels mesh because of their teeth.** One module across the whole
train, so the distance between two arbors is half the module times the sum
of the two tooth counts, rather than a number anybody chose. A wheel is a
mesh with real teeth on it: the count you can see is the count the arithmetic
uses.

**The pendulum beats seconds**, which is what makes the escapement readable:
one tick per second, one tooth of the escape wheel per two ticks, thirty
teeth to the minute. Its length is the length a seconds pendulum has, which
is very nearly a metre, and that is a fact about this planet rather than a
number chosen to fit the room.

**Where the time comes from.** `chrono`, which is capriccio's first
dependency outside the engine and its own shapes. The machine's local time
needs the machine's time zone, and there is no way to ask for that from the
standard library: the engine's `web_time` gives the same answer as
`std::time` in a browser and both of them are UTC. One crate, with default
features off, `clock` for the native build and `wasmbind` for the browser,
where it reads the same `Date` the page would.

## What the drawing of it found

**Where a hand sits and which way it lies are one number, and they were
written twice.** A hand is a bar as long as its own reach, slid half that
along the way it points: the offset came off the sine and cosine of the
turn, and the rotation off a quarter turn written out separately with the
sign the other way about. The two then disagreed, and a bar whose body
points one way and whose middle has been slid the other is not a hand at
all. Its ends land at the top of the dial and out at the side of it, so
every hand came out a chord with neither end on the arbor it turns on.
Jake, on the first picture of it: the hands are all over the place.

The wheels were right throughout, which is why it was not obvious: they
read the turn the one way round and only the hands and the hour marks read
it the other. There is one `along` now, and the rotation is the same answer
asked for as a quaternion, so a thing on this face cannot sit in one place
and point at another.

**And a picture test could not have caught it.** The reference was taken at
ten past ten, where the two hands happen to fall either side of upright,
and a dial with two bars across it at ten and two reads as ten past ten
whatever either bar is actually attached to. Same lesson as the engine's
spec 0110, two days apart: a reference says what it looked like when
somebody was happy with it.

## What the ornament found

**Everything written on the ring was laid at a depth of its own, and the
moulding rose over it.** The marks stood twenty-two millimetres off the
dial and the bezel's crest thirty-one, so the minute track and all twelve
numerals were buried: what a chapter ring carrying its own numbers came out
as was a ring with a few faint extra ticks on it. The ring says where its
own face is now, the flat part of its section is named rather than drawn in
passing, and the marks are laid a few millimetres over that.

**And the dial ran backwards.** The hour marks were written as `n` twelfths
of a turn and the hands as a rate times an hour, with the signs opposite,
so the ring and the hands went round opposite ways. Nothing could say so
for as long as the marks were twelve identical bars, because a dial
mirrored about noon has a bar wherever a dial has a bar. It took putting
numbers on them, and then it was obvious in one frame: the hour hand stood
at eleven, pointing at a numeral reading I. An hour mark is now defined as
where the hour hand points at that hour, which is also what one is.

**And the bob came down into a bucket.** Jake: the pendulum intersects an
object. The back room's clutter was placed by hand along this wall when the
wall was bare, and the bob hangs a quarter of a metre off the floor, which
is bucket height. The clock says what floor its pendulum needs and the test
asks the room; the bucket moved three quarters of a metre along the wall.
Two questions rather than one number, so neither has to know about the
other.

## Acceptance criteria

- Every meshing pair's arbors stand the distance their tooth counts put them
  at. — `clock::tests::the_wheels_mesh_because_of_their_teeth`
- The train divides an hour into a minute: the escape wheel turns sixty
  times while the centre wheel turns
  once. — `clock::tests::the_train_divides_the_hour`
- The hands say the time they are given, round the clock, and the hour hand
  creeps between the hours rather than
  jumping. — `clock::tests::the_hands_say_the_time`
- The pendulum beats seconds and the escape wheel lets one tooth by every
  two beats. — `clock::tests::the_pendulum_beats_seconds`
- A thing laid in the face of the clock at a turn points the way the face
  says it points, so where it sits and which way it lies cannot come
  apart. — `clock::tests::a_turn_lays_a_thing_the_way_the_face_says`
- And the face runs clockwise for somebody standing in the room, which is a
  different claim: it is the one that knows the clock is on the north wall
  and that a player at it has east on their
  left. — `clock::tests::the_face_runs_the_way_the_room_sees_a_clock_run`
- The numeral at an hour stands where the hour hand points at that hour,
  which twelve identical bars could not have
  told. — `clock::tests::an_hour_mark_stands_where_the_hour_hand_points`
- Nothing in the room stands where the pendulum
  swings. — `clock::tests::nothing_stands_where_the_pendulum_swings`
- The clock is drawn as the blessed reference has it, stopped at ten past
  ten: a picture of a thing whose point is the time cannot be taken from the
  machine's own. — `tests::the_clock_is_drawn`

### Verified by hand

- Whether it says what your watch says. Walk through the gallery's unmarked
  door, look at it, look at your wrist.
- Whether it reads as running. The hour hand turns half a degree a minute
  and the minute hand six, so neither says anything about itself in the time
  anybody stands there: what does is the red seconds hand going round once a
  minute on the escape arbor, and the pendulum under it.
- Whether the wheels read as driving each other rather than as turning
  nearby, which is a thing only motion can say.

## Out of scope

- **Winding it.** A real train has a mainspring or a weight at one end and
  this one has the machine's clock instead. There is nothing to wind and
  nothing runs down.
- **A strike.** A striking train is a second train, a count wheel and a
  bell, and it is its own spec.
- **The time zone changing while you stand there.** Read once a frame from
  the machine, so it follows if the machine's does, and nothing here cares.
