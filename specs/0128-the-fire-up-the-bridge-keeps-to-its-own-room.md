# 0128 The fire up the bridge keeps to its own room

**Status:** implemented
**Date:** 2026-11-22

## Goal

Jake, reading a line the hall's staged shot printed: why is there a limit,
and then which lamp in the hall is losing the cast.

The line was `a scene asked 1 more point lights to cast than the 2 it
carries`, and it had been printing under every picture of the arcade
nobody looked at the log for.

## Behavior

**The limit is six passes, not arithmetic.** A lamp shines every way at
once, so covering what it can see is six renders of the scene into six
layers of a depth array, which is engine spec 0022. Two casters is twelve
passes, already more than the rest of a frame put together. Beside it
`MAX_POINT_LIGHTS` is a hundred and twenty eight, because a lamp that only
lights is a few lines of shader that quit early when its range does not
reach the surface. Lighting is cheap and shadowing is geometry drawn again.

**The lamp losing it was the gallery's right-hand one**, and the lamp
taking its slot was the steading's hearth, a hundred and sixty four metres
west and twenty seven up. The engine gives the two slots out in the order
they are asked for. `draw_the_hall` asked for the fire whether or not any
of what it lights was on screen, and the only gate above it is
`bridge::whole`, so finishing the trail put a shadow out in the nook.

That is spec 0094 arriving by another road. It chose the gallery's two
casters by place rather than by distance, so a shadow would stop moving as
you walk, and the place held. What it could not hold was whether the second
of them got a slot at all.

**So the fire asks the same question the gallery already asks.** One line:
the frustum holds a sphere of `HEARTH_REACH` about it, or the lamp goes in
without its shadow. The gallery's casters have taken that cull since spec
0094 and the comment above them says why, that what the engine drops must
never be a lamp a room is built around. The fire was simply never asked.

**The camera goes down four signatures to get there.** `draw_the_field`,
`draw_bridge`, `draw_valhalla`, `draw_the_hall`, each of them private and
each already taking the scene. The other way was a place test on the eye,
which needs nothing passed and is wrong for a shadow: a lamp that cannot
reach you can still cast something you can see from the doorway. The
frustum is the test the building already uses for this, so it is one rule
and not a second kind of cull.

## What it cost to find

Nothing, once it was measured. A guess would have said the gallery's own
two were fighting each other, since they are the only pair in the building
that both ask. A temporary print in `Scene::push_light` named all three
asks in their order in one run, and the first of them was 164 metres out of
frame. Five minutes against an afternoon of reading draw order.

## Acceptance criteria

- From the arcade doorway with the trail answered, the hall's fire does not cast and both the gallery's casters do. — `tests::no_fire_up_the_bridge_takes_a_shadow_from_the_gallery`
- Standing in the hall, the fire still casts, which is the half a cull quietly loses. — `tests::the_hall_up_the_bridge_still_casts_its_own_fire`
- The hall inside is drawn as the blessed reference has it, which is the picture that would go flat. — `tests::the_hall_inside_is_drawn`

### Verified by hand

- The warning is gone from `--screenshot=hall`, and both staged pictures come back byte for byte what they were, the gallery being far enough down the aisle for its second shadow to fall under the tolerance a picture test uses.

## Out of scope

- **The order lamps are asked in.** First come first served is still the
  rule, and a building with three rooms in sight that each want a shadow
  would still hand them out by draw order. Nothing in this building has
  three.
- **A warning that says which lamp.** The engine counts what it turned away
  and not what it was, which is why this took a print. Worth doing in the
  engine rather than here.
