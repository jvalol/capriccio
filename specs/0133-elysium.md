# 0133 Elysium

**Status:** draft
**Date:** 2026-11-23

## Goal

A second paradise, reached through the glass sphere of spec 0064, across
water, into a garden in daylight with nobody in it. Nothing in the building
says it is there.

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
fifteen metres over the nook with one object in it: a glass sphere traced
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

## What is undecided

**What opens the sphere.** It is a threshold with no door, which is the
point, and that leaves the question of when it is a threshold at all rather
than an exhibit. Three readings were put up and none chosen: only after
Valhalla, so it cannot compete with the trail and it rewards going back to a
finished room; on entering the water the building already has, which the
baths have the mechanics for; or on time spent rather than tasks done, which
is the one axis the trail does not use and which nothing in the project
counts today.

**How far it is across.** An expanse that reads as one is a cost nobody has
measured, and the field's own answer to this is spec 0040, a field with no
far side.

## Acceptance criteria

Nothing is built. These are the claims a build has to hold up, and the names
are what the tests will be called.

- The way in is the sphere, and nothing else in the building opens
  it. — `elysium::tests::only_the_sphere_lets_you_in`
- Nothing of the building is visible from inside it, and nothing of it from
  the building, which is what its own sky
  rests on. — `elysium::tests::neither_place_can_see_the_other`
- It is in daylight whatever hour the building is
  at. — `elysium::tests::it_is_morning_whenever_you_arrive`
- There is nobody in it. — `elysium::tests::nobody_is_in_it`
- You cross water to reach the garden, and cannot walk round
  it. — `elysium::tests::the_water_is_crossed_and_not_skirted`

### Verified by hand

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
