# 0122 A globe you put your hand on

**Status:** implemented
**Date:** 2026-11-21
**Built:** 2026-11-21, the same day, in three rounds off three screenshots,
a fourth for the fault they left behind, and six more on Jake's own
pictures of it running, then on a photograph and a video of the real thing.

## Goal

Jake, on spec 0064's mirror ball: it could glow from inside, or be like one of
those electricity exhibits at a planetarium. A plasma globe, standing beside
the mirror ball, whose filaments reach toward whoever is standing there.

## Why it stands beside the ball rather than being it

**The mirror ball has no inside to glow from.** It is `scene.lens` with the
index at `MIRROR`, and that path returns the room it traces multiplied by
`SILVER`. There is no transmission term in it, so there is nothing behind the
surface. Colour past one glows in this engine, so raising the tint would work
and what it would light up is the whole reflection: a floodlight shaped like a
ball.

**And spec 0064 is the only thing in the building that needs the tracer.**
A plasma globe needs none of it. Glow is a channel past one and see-through is
an alpha under one, and this building already draws both, in the neon over the
hall and in the koi pond. Spending the one refraction demo on an effect any
engine does with blending is a trade with nothing on the near side.

Standing them together pays twice. The tracer reads real mesh geometry since
the engine's spec 0107, so the mirror shows the glowing thing next to it, which
is the one thing a mirror is for.

## What makes it an exhibit and not a lamp

The reason anybody remembers these is that the filaments come to your hand.
Memory of this project's own demos says the same thing twice: show what nothing
else shows, and give the player something to decide. A ball that glows is a
brighter ball and there is nothing in it to decide.

So the filaments reach for where the player is. Walk round it and they follow.
That is the whole input: no key, no mode, nothing held. Spec 0112's lesson is
about a panel you look at from across a room, and this is the opposite kind of
thing, something you stand at.

**There is no hand to reach for.** The arcade draws people but it has never
drawn the player, which is spec 0061's own hard part. What the filaments have
is the eye, which is where a face would be rather than a palm. One step off
from the real exhibit and the only honest target available.

## Behavior

**The pedestal.** 0.52 square and 0.95 tall, standing 0.30 east of the mirror
ball's plinth on that plinth's own z centre line, so the two read as a pair
across the walk. Cut in the courses `plinth_pieces` already cuts, which takes
a box now so that both plinths in this room are cut once rather than twice.
The east lane past it keeps 1.65, over this building's 1.05.

**It is lower on purpose.** The mirror ball's middle is at eye height because
walking at it is walking into it. This one's is 1.17 over the floor, which is
where a hand falls, because reaching is how it is met.

**The globe.** A shell of 0.22 radius at the pedestal's top, see-through, with
a faint cool tint: alpha under one, so the engine draws it after everything
solid. The electrode is a glowing ball of 0.04 at its middle, nearly a quarter
of the globe across, which is what the real thing has and what the filaments
leave the face of.

**The filaments.** Sixteen, each a chain of eighteen thin boxes from the
electrode's face to the shell, hairline and drawn past one so they give off
rather than take. Each one thins and dims along its run. Each is white where
it leaves the electrode and violet well before the glass, because what is
hottest is whitest. Each wanders slowly and crinkles quickly at the same
time, which is two terms and not one. And each forks: a thinner, dimmer
branch leaves it in the outer half and lands on the glass like its parent.

**The sparks.** Where a filament meets the glass it spreads, which is the one
place along an arm that is wider than the thread rather than narrower. Two
spheres: a small bright core and a see-through bloom round it, which is how a
soft edge gets built here, both pulled in by the bloom so nothing of them
stands outside the glass. They breathe a little, since the one place on an arm
that is a point rather than a line reads as a bead stuck to the glass if it is
steady.

**How they move.** Each arm has two clocks of its own, and they are not the
same clock. One is its life: every beat is a throw for whether it is lit at
all, under a fifth of them dark, which leaves eight of the ten burning on
average. The other is where it stands. It crawls round the inside of the
glass at its own rate and in its own direction, from where its current lit run
struck, and about once a second it jumps instead, each arm on its own era.
So what you mostly see is creeping and what you sometimes see is a strike
landing somewhere else, and no two arms are doing either at the same time.

An arm cannot go out if it is the last one: independent throws do come up all
dark, and a plasma globe that blinks off is a broken one, so when nothing at
all is lit the arm that came nearest to staying strikes. All of it is worked out from the time and the eye, so there is nothing
to keep and nothing to save: spec 0120 keeps the trail and nothing else, and
a toy is a toy as it was built.

**The light.** One point light at the globe's middle, in the plasma's own
colour, reaching 1.6 metres. It is in a fitting by the room skill's rule
because the fitting is the globe: the lamp's own body is the thing that glows.

## Acceptance criteria

- The pedestal stands clear of the mirror ball's plinth and leaves a body room
  to pass on the east side, and the lane
  walks. — `glass::tests::the_pedestal_leaves_the_walk_open`
- Every filament runs from the electrode to the shell and nowhere outside it,
  over a sweep of times and eye
  positions. — `plasma::tests::every_filament_runs_from_the_core_to_the_glass`
- The filaments favour the side the eye is on, all the way
  round. — `plasma::tests::the_filaments_reach_for_whoever_is_there`
- The shell is see-through, the plasma glows, and one channel of it stays
  under one so that it has a
  colour. — `plasma::tests::the_glass_is_see_through_and_the_plasma_is_not`
- The lamp is inside the globe. — `plasma::tests::the_lamp_is_in_the_globe`
- Nothing of it stands inside the room's walls, the colonnade or the mirror
  ball's plinth. — `plasma::tests::the_globe_stands_inside_the_room`
- And nothing drawn round the globe shares a plane with anything else drawn
  there. — `plasma::tests::nothing_round_the_globe_is_in_one_plane_with_anything`
- Each arm creeps, each arm sometimes jumps, jumping is the exception, no two
  of them jump together, and the quickest creeps several times faster than
  the slowest. — `plasma::tests::the_arms_crawl_at_their_own_rates_and_jump_apart`
- Arms come and go, and the globe is never wholly
  dark. — `plasma::tests::the_arms_come_and_go_and_the_globe_is_never_dark`
- And the room still cannot be walked into and not out of, which spec 0043
  asks of every room at once. — `room::tests::no_room_lets_you_in_and_not_out`

### Verified by hand

- Whether it reads as a plasma globe or as a glowing marble. Run the arcade,
  climb to the glass room and stand at it.
- Whether the filaments following you reads as a thing answering you or as a
  thing twitching. Walk a circle round it.
- Whether it reads as electricity rather than as something alive, which is the
  one thing here that has been wrong twice and that no test can settle.
- Whether the mirror ball showing the glow beside it is worth the two standing
  together, which is the whole argument for the siting.
- Whether the glow fights the bay lamps at 0.95.

## What the first three builds got wrong

**Past one in all three channels is white.** The filaments went in at 3.6,
1.5 and 4.4 and came out as a white starfish, because the screen clips at one
and a colour over it everywhere is the same colour as every other colour over
it everywhere. The only place the violet survived was the ball's reflection,
where it is dimmer. The neon over the hall had the answer already, at 0.70
against 3.04 and 3.2: one channel under one is what keeps the hue, and the
test asks for that rather than for three channels past one.

**A link as thick as it is long is a cube.** Six links over 0.22 at a
thickness of 0.014 is a chain of cubes, and the bow turned 1.9 radians from
one to the next, so each filament came out as a run of hard right angles: a
comic's lightning bolt. Nine links at 0.006 with 0.6 of turn is a gentle
helix, which is what a thread of gas in a globe is.

**Nearly clear and drawn plain, the shell is not there at all.** The
filaments hung in the air and the thing read as a sparkler rather than
something under a dome. What makes glass is the highlight: `aim::MATTE` is
this building's hot spot and the bay lamps give it a rim.

**The courses were in metres.** `plinth_pieces` oversails its base by 0.13,
written when there was one plinth in this room and it was 0.9 across. On a
die of 0.52 the same metres are half again as much oversail and what it drew
was a base wider than the thing standing on it. They are a share of the die
now, divided by `PLINTH` so the ball's own plinth is the course it has always
been.

**The pedestal's box was drawn twice and nobody saw why.** `glass::draw`
draws every box in that room's `solid` list bar a handful it names, the
plinth among them. The pedestal went into `solid` so a body would walk into
it and nothing added it to that handful, so the room drew its box as a plain
cube and `plasma::draw` drew its courses over it. The die's sides landed
exactly on the cube's: two exposed faces in one plane, the eighth time in
this building.

What it looked like was a checkerboard of lit and shadowed squares with a
stair-stepped diagonal through it, which reads like a coarse shadow map and
is not one. That cost eight eliminations, each a build and a picture: the
lamp, the shell, the stem, the filaments, the ball and the plinth beside it
(moved three metres away and it followed), the courses' oversail, the sun's
own bounds, and the pedestal's place in the room. One box of the same size
and colour came back perfectly clean, which is what finally pointed at it,
because exactly coincident geometry does not fight and nearly coincident
geometry does.

Two things come out of it. The handful is a list now, `drawn_otherwise`,
rather than a line of `&&`s, because leaving one off is invisible. And the
check is the engine's own, asked the way spec 0060's hall asks it and
narrowed to this corner, so the next thing stood in here is held the day it
goes in.

**It was a glowing tentacled monster.** Jake's words, on the first one that
ran. Three things made it one, and all three are about the difference
between a thread of gas and a limb. A filament of one thickness end to end
is a limb, so each one thins to an eighth and dims by two thirds along its
run, which is what a discharge does. A smooth glowing curve is a limb, and a
regular sawtooth is a comic's lightning, so the wander is two terms now, a
wide slow one that bends the whole thread and a small fast one laid on it:
one turn rate gives a curve or a sawtooth and nothing in between. And a big
bright ball where they meet is a head, so the electrode is a third of what it
was and dimmer, and the stem that carries it is a rod rather than a bar.

**And they all moved together.** Jake, watching that one: the arms should
move erratically, not in sync with each other. Every arm was a sine of the
time at a rate a few hundredths off its neighbour's, so all seven turned the
same way at nearly the same speed, which is one movement shared out rather
than seven of them. They hold still and strike somewhere else now, each on
its own beat, each at its own offset into it, with the thread's own shape
re-struck at the same moment so a jump reads as a new discharge rather than
the old one swinging. Where it stands comes out of a scatter of which arm and
which jump, since the globe keeps nothing and a function of the two is what
stands in for a dice.

**And then they were seven things snapping about.** Jake, on the version
that did: there should on average be more arms, and they should crawl more
than jump across the inner surface, though some jumping is good too, and in
general more variety in how they move. The answer to the note before had
been to hold each arm still for a beat and snap it somewhere else, which is
all jump and no crawl, and every arm snapping the same size of stride.

An arm crawls now, at its own share of a base rate and in its own direction,
so some creep and some travel and two beside each other are not going the
same way. The jump is a rare kick laid over the crawl, a few seconds apart on
each arm's own era. The lean sways on its own rate and its own amplitude, so
one arm breathes in and out of the middle while another sits near the rim.
And an arm that strikes after a dark stretch starts somewhere new rather than
where it went out, which is the one place the old snapping was right.

Ten arms rather than seven, and under a fifth of beats dark rather than
nearly a third, which is eight burning on average against five.

**And before that they were always all there.** Jake again: there should be arms that pop
in and out of existence in addition to jumping around the sphere. Seven arms
that only ever move are seven arms however erratically they move, and the
count is the other half of what a globe does. Each beat is now a throw for
whether that arm is lit at all, with short ramps at either end of a lit run,
and the dying ramp asks the *next* beat rather than this one, since an arm
that is lit again next beat does not go out in between, it jumps.

Writing the test found the thing worth finding. Seven independent throws come
up all dark sooner than it feels like they should: inside ten minutes of
running, at nearly a third each. So an arm cannot go out if it is the last
one, and the globe is never dark by construction rather than by luck.

**And then a photograph and a video of the real thing.** Jake sent both.
What they changed, in the order the difference mattered.

The electrode is a ball, not a point: nearly a quarter of the globe across,
and the filaments leave its face rather than its middle, where a filament
that starts at the middle of one spends its widest and hottest quarter buried
inside it. A filament is two colours and not one, white where it leaves and
violet by the time it reaches the glass, because what is hottest is whitest,
and one colour dimmed along its run is a violet thread turned down, which
reads as paint. There are sixteen of them rather than ten.

And every filament in the video forks. Not one of them runs from the
electrode to the glass without splitting and several split twice, which is
the one thing that still read as drawn rather than struck. Each arm has a
branch now, leaving in its outer half, thinner and dimmer because it is what
is left of the discharge after the fork, landing on the glass like its parent
and carrying its own spark.

The video also settled the rate, which the note before it had only guessed
at. A third of a second apart the arrangement is largely re-struck; a sixth
of a second apart it is recognisably the same. So the jump era is a second
and a half rather than four, the life beat is shorter, and the thread's own
wander runs at about twice what it did. That moved the bound in
`the_arms_crawl_at_their_own_rates_and_jump_apart`, which is honest: a jump
takes its own fixed time out of an era, so shortening the era raises the
share by arithmetic, and what the test should hold is that crawling is what
an arm is mostly doing rather than a particular number.

**And `you_can_walk_round_it` ran through the new pedestal and passed.** Its
ring stood at `middle.x + 1.3`, three centimetres off that pedestal's east
face, and `walked` allows 0.8 at the corner: the body was shoved round the
thing and still arrived. The same blind spot as the back room's crates, where
a walk somebody chose with a tolerance on the end of it said nothing about
what it squeezed past. The ring goes round the pair now, which is what the
claim was ever about.

## Out of scope

- **No hand.** See above: the target is the eye.
- **No key and no mode.** Standing there is the input.
- **No sound.** Spec 0119's note is the trail's and this is not one of the
  trail's questions.
- **No dial on the filaments.** Spec 0064 refused an index on the arrow keys
  for the same reason, which is the mistake aleph is parked for.
