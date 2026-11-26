# 0133 Elysium

**Status:** implemented
**Date:** 2026-11-23

## Goal

A second paradise, reached by walking into the glass sphere of spec 0064,
which turns the room about so its own stair now goes down to a shore, across
water, into a garden in daylight with nobody in it. Walking back up the stair
and into the sphere puts you back. Nothing in the building says it is there.

## Why a second one at all

Jake, on the day the arcade took the name xanadu: valhalla is something to be
reached, and we could add an elysium. i could see the mirror ball room being
the gateway after some other kind of journey or whatever is suitable for that
legendary place.

The name settled the shape of it. Xanadu is a pleasure palace on earth and
not an afterlife, so it sits beside the places it contains rather than
competing with them: you wander the pleasure dome, and the heavens are
through their own doors. Valhalla is one. This is the other, and the two have
to differ in more than their furniture or the building has said the same
thing twice.

**They differ in what they are for.** Valhalla is earned. Five deeds, a
bridge visible from the first minute, a watchman at the top who opens it. It
is the public prize and the building announces it. This one is not announced
and not earned, and spec 0036's rule is the whole reason: somebody who never
finds the trail should not be able to tell it is there. That rule was traded
away once, for the site, on 2026-11-07. Nothing here trades it again.

## Where it is, which is nowhere

**Through the sphere, and not across the field.** Spec 0064's room stands
a storey over the nook, 3.5 metres by `glass::UP`, with one object in it: a glass sphere traced
against the building's own geometry. You already see the whole building
through it, bent. Going in is the one threshold this building has that needs
no door, no key and no sign.

**And this is what makes the rest of it possible.** The building has one sky,
and that is deliberate: spec 0055, "There were two, the garden's and the
field's, each plainly visible from under the other, and they were made one
deliberately." Spec 0056 parked Valhalla's aurora on those grounds and spec
0060 repeats them. Valhalla has to share the sky because you can stand in the
field and see both.

Reached through the sphere, this place is nowhere you can see the building
from, and nowhere the building can see. The rule protects against two skies
in one view, and there is no view that holds both. So it may have its own.

**And its own is cheaper than this spec first assumed.** The sky is not a
dome standing in the world. `Scene::backdrop` hangs it off the eye, gated by
`sees_outdoors`, so a second sky is the backdrop drawn with other colours
while you are here rather than a second piece of geometry at a safe distance.
Distance is still wanted, so that nothing of one place is in frame with the
other, but it is not what makes the sky possible.

**That is a decision and not a detail.** Spec 0055 asks that whoever builds a
second sky do it on purpose. This is that, taken on the one ground that makes
it safe, and it is the first thing to re-examine if this place ever becomes
visible from anywhere.

## What is in it

**Water to cross, and the crossing is the journey.** The Greek way in is over
water, which is also the way this one avoids being the Norse one again: a
bridge is Valhalla's. The building's water today is the baths' pool and the
garden's pond, both small, so an expanse is new geometry.

**A garden with nobody in it.** Spec 0033 draws people and spec 0038 is a
room whose whole point is other people using it. Valhalla has figures in it,
none of them fighting. This has none at all, and the emptiness is the thing
it is saying.

**In daylight.** The building runs a day in `garden::DAY`, twenty seconds,
and whatever hour you arrive at is whatever hour it is. Here it is morning
and stays morning.

**It is not the zen garden again.** The building already has a garden off the
hall with a pond in it. Three things have to be different in the first frame
or this is that room with the figures taken out: it is in daylight when the
building may be at night, there is nobody in it, and you crossed water to
reach it. If a first build does not read as somewhere else within a second of
arriving, the answer is not more planting.

## What guards it

**Nothing.** Spec 0063 took the watchman's riddle away because the bridge
only rises once every station is given, and a riddle on top of that was a
second lock on a door that was locked. The same reasoning holds here: if the
way in is a sphere nobody has been told about, a test at the threshold is a
second lock. Finding it is the whole of it.

**And nothing counts at you.** No panel, no "3 of 8 found", per spec 0036.

## How you get in

Jake: walking up and into the sphere turns the room around with some kind of
wormhole or blackhole effect, so you walk into the same room but now the
stairs are in front of you. and then down them is elysium.

**The threshold and the journey are the same act.** Three ways in were put up
before this and all three were a lock by another name: only after Valhalla,
which makes it earned by the trail; on entering water elsewhere, which is a
key; on time spent, which needs a counter spec 0036 forbids pointing at you.
This is none of them. You walk at the glass, and the way back out of the room
is the way on.

**What it rewards is going back down.** Spec 0064's stair climbs a storey
from the nook's far wall to this room, fourteen risers and 3.5 metres, and the one thing nobody does
in a building is walk back down a stair they have just come up to see whether
it still goes where it went. Here it does not.

**The room is the same room to walk through, and two rooms to build.** The
glass room stands over the nook and its stair comes down into the nook, so
one piece of world cannot give that stair two bottoms. What is built is a
second copy of the room, elsewhere, whose stair goes down to the shore, and
the sphere moves you between them while the effect is covering the screen.
Valhalla is the precedent: a whole place built somewhere else and walked to.

Said plainly because the first draft of this spec claimed nothing was built
for the other state, which was a claim about what you experience written as
though it were a claim about the geometry. You experience one room turned
about. What exists is two.

**The effect is the sphere's own index.** `Scene::lens` takes one, which is
how spec 0064 makes glass and how spec 0122 reads `glass::MIRROR`. Driving it
is the room folding into the glass and back out, and it costs no new pass,
no second render target and no full-screen shader. This is a proposal rather
than Jake's: he asked for a wormhole or a black hole, and this is the
cheapest thing in the engine that looks like one.

**And it is the same walk back.** Jake: and walking back up the stairs and
into the sphere puts you back. So the route out of Elysium is the route in,
read backwards: up the stair, into the glass, and the room turns again with
the nook at the bottom of it. One door, used twice, and the building is never
further away than the way you came.

That is also what keeps it from being a trap. There is no second mechanism to
find, nothing to work out, and nothing that can be missed: whoever got in
already knows the way out, because it is the way they got in.

**Arriving does not take your hands off the keys.** The crossing let go of
everything held, which is what `focus_changed` does and is right there: a key
down when the window goes away is down for ever. Nothing goes away here, so
what it did was stop you still in the room you had just reached, with the key
down and nothing to do but lift it and press it again. Jake: after moving
through the sphere the first time if i'm still holding forward my motion
stops.

**One sphere each, and only yours takes you.** There are two of them: spec
0064's and the copy of it here, standing 4.2 apart inside one room. Reading
both made the walk above impossible. Arriving leaves you facing the sphere
you came from with that room's stair beyond it, so walking to the stair
walked you back into it, and holding forward for twenty seconds crossed
thirteen times without reaching anything. Which sphere is yours is the layer,
which is the one thing that already knows. Only yours is drawn as well, so
the other was a door standing at a piece of empty air.

## The three that were open, and how they settled

**What counts as walking into it is settled.** You cannot literally enter.
The plinth is 0.9 across and a body is 0.45, so you stop 0.90 out from the
sphere's axis; at an eye height of 1.55 against a centre at 1.62 the glass is
0.616 out, which holds you 28 centimetres short. `ball()`'s own note that
"walking at it is walking into it" is about it filling your view, not about
entry.

So the trigger is that you are *still walking* into it: your wish is forward,
the plinth is not letting you go anywhere, and the ball is what you are
looking at. Leaning your forehead on the glass and pushing.

**It cannot fire while you stand and look**, which is the whole reason it is
this and not proximity. Spec 0064's room exists to show refraction and
anybody who walks up to see it would trip a proximity trigger every time.
Standing is not walking, so the exhibit is untouched and no geometry moves.

Narrowing the plinth to about 0.2 was the other answer and would have put
your eye 7 centimetres inside the glass, which is truer to the image. It was
turned down for what it costs: spec 0064 argues at length about where the
plinth stands and never about how wide it is, so the number is free, but the
room has a reference picture of the ball on its plinth from 2026-11-12 and
this is a draft. Jake's call, 2026-11-23.

**Where the water is is settled.** The stair lands on the near shore and the
garden is across the water. Jake's call, 2026-11-23.

**Because the phrase only reads if the water is between you and it.** A
garden in daylight across water, seen from the head of a stair a storey up,
is a thing you walk down towards. Landing on the far side spends the
water as scenery you have already passed, and the descent then descends
towards nothing in particular.

**Crossing is wading, and the baths already have it.** `spa::pace` takes you
to `spa::WADE`, 0.42 of walking, while your feet are under the surface. Shin
deep and bright, slowed to less than half, for as long as it takes. Nothing
new is needed and nothing is being asked of you: it is not a puzzle, it is a
distance.

**It is not the pool and it is not the pond.** Those are the two pieces of
water this building has and both are things you look into. This is the first
one that is a distance rather than an object, which is also the first
difference that will be visible in the opening frame.

**How far it is across is settled.** About thirty-five metres, which at
`spa::WADE` is a touch over twenty seconds at a walk. Jake's call,
2026-11-23.

**Measured in time rather than in metres**, because the number that matters
is how long you are in the water. `SPEED` is 4.2 and `spa::WADE` is 0.42 of
it, so wading is 1.76 a second: thirty-five metres is twenty seconds across,
or ten at a run if running works in water, which nobody has checked.

**Twice the hall and a third of the bridge.** The hall is 19 along, the
garden 16, and the bridge to Valhalla 100. Rivalling the bridge would be
competing with the one long walk this building already has, and the two are
meant to feel unlike each other: one is a climb you labour up, the other is
a slow walk across a flat bright thing.

**And it is a round trip.** You cross it again every time you leave, which is
the argument against anything longer. Sixty metres is a better first arrival
and a toll on every departure after it.

**This is not the field's problem.** Spec 0040 made a field with no far side,
generated as you walk into it and thinned past `GROUND_REACH` at 160. This
has a far side and you can see it from the top of the stair, so none of that
machinery is wanted: it is a measured crossing, not an horizon.

## Acceptance criteria

Much of this is built now, so these name the tests that hold it. The ones
still marked unbuilt have no test yet and are what is left.

- Standing beside the glass, or looking at it from across the room, is not
  going through it. — `elysium::tests::standing_beside_it_is_not_going_through_it`
- Walking at it while it is what you are looking at
  does. — `tests::walking_at_the_sphere_takes_you_through_it`
- And walking straight at it with a hand on the mouse, which is how anybody
  would. — `tests::it_goes_through_walking_straight_at_it_while_drifting`
- The fold starts on the step you take at it, not when the plinth stops
  you. — `tests::the_fold_starts_on_the_step_and_not_on_the_plinth`
- And it survives a frame that goes the wrong way, which is what a body
  jittering against the plinth
  does. — `tests::the_crossing_survives_a_frame_of_jitter`
- Once started it finishes on its own, with nothing held
  down. — `tests::walking_into_it_takes_you_with_nothing_held_down`
- The room swaps at the top of the fold, with the frame
  covered. — `tests::the_room_swaps_behind_the_black`
- And having swapped it lets you walk on: one press of forward crosses once,
  keeps carrying you, and reaches the stair. It used to cross thirteen times
  in twenty seconds, and it used to stop you dead on arrival with the key
  still down. — `tests::one_walk_at_the_glass_crosses_once`
- The glass winds from a mirror into a lens on the way
  across. — `elysium::tests::the_glass_turns_from_a_mirror_into_a_lens`
- And is black before that lens bends hard enough to gather the room behind
  it into a picture, while the frame itself is still mostly
  open. — `elysium::tests::it_is_black_before_it_gathers_the_room`
- And the frame is wholly black across every frame the swap can land on, at
  any rate down to twenty a second, without being black for the whole
  crossing. — `elysium::tests::the_swap_is_in_the_dark_whatever_frame_it_lands_on`
- And is its own tint at rest, so none of this dims spec 0064's
  room. — `elysium::tests::the_glass_is_itself_until_you_walk_into_it`
- The way back is the way in read backwards, so nobody can be stranded
  there. — `elysium::tests::the_glass_takes_you_through_and_back`
- The sun is up over it, whatever hour the building is
  at. — `elysium::tests::the_sun_stands_well_up_over_elysium`
- And that hour is the layer's, so the building's clock does not move
  it. — `tests::the_hour_is_the_layers_and_not_the_buildings`
- You cross water to reach the garden, and cannot walk round
  it. — `elysium::tests::the_water_is_crossed_and_not_skirted`
- Crossing slows you to a wade the way the baths
  do. — `elysium::tests::the_crossing_is_waded_at_the_baths_own_pace`
- The crossing is about twenty seconds of wading, walked from the stair's
  foot to the garden. — `elysium::tests::the_crossing_takes_about_twenty_seconds`
- The stair comes down on the near shore and not in the
  water. — `elysium::tests::the_stair_comes_down_on_the_near_shore`
- The field is not here at all, being on another layer, so nothing it
  sows can land here. — `tests::only_the_sphere_changes_the_layer`
- The colonnade stands across the way in, on dry
  ground. — `elysium::tests::the_colonnade_stands_across_the_way_in_on_dry_ground`
- It reads as a ruin rather than as a pattern of
  gaps. — `elysium::tests::the_colonnade_is_a_ruin_and_not_a_pattern`
- Its second row is down in different places from its first, so the gaps
  have something behind them. — `elysium::tests::you_cannot_see_through_both_rows_at_once`
- Its horizon stands inside the sky, where it can be
  drawn. — `elysium::tests::the_horizon_stands_no_further_out_than_the_fields`

- The way in is the sphere, and nothing else in the building opens
  it. — `elysium::tests::only_the_sphere_lets_you_in`
- Coming through leaves you with the stair in front of
  you. — `elysium::tests::the_stair_is_ahead_of_you_on_the_far_side`
- The stair goes down to the nook on one side of it and to here on the
  other, and it is one stair. — `elysium::tests::the_same_stair_has_two_bottoms`
- The garden is in sight from the head of the stair, with the water between
  you and it. — `elysium::tests::you_see_it_across_the_water_from_the_top`

- Nothing of this place is drawn in the
  building. — `tests::neither_place_can_see_the_other`
- And nothing of the building here. — `tests::nothing_on_another_layer_is_drawn`,
  which is spec 0134's
- There is nobody in it. — `tests::nobody_is_in_it`

**Those three are in `main.rs` and not in `elysium.rs`**, where this spec
first put them, because each needs a frame: the question is what a scene
holds, which wants the game and a renderer.

**And each reads a colour rather than a position.** The first go at the
colonnade counted instances inside its own boxes and found two of them on
the arcade's layer, which is not a leak: those are the arcade's own field,
standing at coordinates the two places have shared since spec 0134 made
the separation a fourth coordinate rather than six hundred metres. What is
being asked is whose thing is in the frame, and `aim::RUIN` and
`aim::COATS` are palettes nothing else in the building wears.

Where somebody is standing is `people::doing` over a round, offset per
round and then moved apart, so a test that worked out a position again
would be a second account of that. A coat in the frame is somebody in the
frame wherever they are standing. A palette invented for Elysium later
would walk past it, which is the limit of what a colour can say.

### Verified by hand

Neither of these is answered. They are what the status does not cover: the
place is built and held, and whether it is worth walking to is Jake's.

- **Whether it reads as somewhere else within a second of arriving**, which
  is the only thing that decides whether it is a second garden or a second
  paradise.
- **Whether going through a sphere reads as going anywhere**, or as a screen
  wipe with a glass ball in front of it.

## Out of scope

**Telling anybody it is there.** No sign, no line on the site, no bridge on
the horizon. Valhalla is the announced prize and carries that for both.

**A second trail.** Five more deeds is the trail again with the numbers
changed.

**Anything in the sky but the sky.** The aurora is spec 0056's parked
question and parking it was right; this spec gives a second sky a place to
exist and does not paint anything on it.
