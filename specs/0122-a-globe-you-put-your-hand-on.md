# 0122 A globe you put your hand on

**Status:** implemented
**Date:** 2026-11-21
**Built:** 2026-11-21, the same day, in three rounds off three screenshots,
and a fourth for the fault they left behind.

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
solid. An electrode of 0.045 at its middle, which is the one solid thing in
there.

**The filaments.** Five, each a chain of six thin boxes from the electrode to
the shell, 0.014 across and drawn past one so they give off rather than take.
Each has its own slow wander, so they writhe, and each is pulled toward the
point on the shell nearest the eye. All of it is worked out from the time and
the eye, so there is nothing to keep and nothing to save: spec 0120 keeps the
trail and nothing else, and a toy is a toy as it was built.

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
- And the room still cannot be walked into and not out of, which spec 0043
  asks of every room at once. — `room::tests::no_room_lets_you_in_and_not_out`

### Verified by hand

- Whether it reads as a plasma globe or as a glowing marble. Run the arcade,
  climb to the glass room and stand at it.
- Whether the filaments following you reads as a thing answering you or as a
  thing twitching. Walk a circle round it.
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
