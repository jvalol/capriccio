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
