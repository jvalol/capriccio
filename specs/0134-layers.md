# 0134 Layers

**Status:** draft
**Date:** 2026-11-23

## Goal

A fourth coordinate, `w`, so that two places can share three of their
coordinates and still be two places. The arcade is one layer and spec
0133's place is another. What stops you, what is drawn and what lights you
is what shares your layer, and nothing else is considered.

**Not the same ground in two states.** Jake, on a picture of the two and
a sloppy sentence under it: *what does "the same ground in the morning"
mean, they should differ by the w value.* A point at `w = 0` and a point
at `w = 1` are different points, as far apart as any two, and the ground
at one is not the ground at the other wearing different light. Thinking of
it the other way is what leads to wanting `w` to blend, or to asking what
a body at `w = 0.4` stands on. There is no such body.

## Why there is no other way

**Two unbounded worlds cannot share a direction.** The field is unbounded on
purpose, and says so: *a region cannot help being a region. This is not one:
if nothing of the building is under your feet, the field is, and there is
nowhere left for an edge to be.* Spec 0133 put a second world 600 metres
south of the first, in the same three coordinates, as a box 150 metres each
way. So it is a region, and it has an edge, and on 2026-11-23 Jake walked
over that edge and stood in the arcade's field at night with its trees and
its blue hills.

Every fix available inside three dimensions is the same compromise at a
different size. A bigger box moves the edge. A wall at the rim is an edge you
are stopped at rather than one you fall through. Giving this place its own
half plane was tried and reverted the same evening:
`tests::you_cannot_walk_behind_the_building` and
`tests::the_clip_walks_the_whole_route` both failed, because the field's own
walks go out to z 1019 and a half plane takes its wall away out there.

Jake's reading, which is the one that holds: *we're thinking in 3d, since
that's what the engine was built for, but there's no reason a 4th spacial
dimension can't be added to support spaces existing on top of each other.
The whole building can be a tesseract.* With a layer apiece, neither world
needs a boundary, because they are not in each other's way.

## What this building already calls w

**Spec 0021 has a tesseract in it and it is implemented.** `tesseract.rs`
owns `Axis::W` and Hinton's words for the two ways along it, ana and kata.
That room is eight cells of one cube and its `w` picks which cell you are
standing in; this spec's `w` picks which world. They are the same idea at
two scales, and a reader who meets both should be told so rather than
work it out.

Nothing is shared between them in code. The tesseract room's cells are
inside one room on one layer, and this coordinate does not reach into it.

**And spec 0021 paid for a lesson that applies here.** Its cells were alike
in everything but a word on a wall, so walking between them told you
nothing: *a cell is a closed box with no landmark in it, and everything you
can see inside one is stated relative to the frame you happen to be
standing in.* Three fixes failed before a map worked.

Two layers are not that, because they do not look alike: one is the arcade
at night and the other is a morning with nobody in it, which is the whole
of spec 0133. But if a third layer ever arrives that resembles another,
0021's note is what it will cost.

## Behavior

**Every body, collider, drawn thing and lamp is on exactly one layer**, named
by an integer. The arcade is layer 0. Spec 0133's place is layer 1. A layer
has no extent and no edge: it is a whole copy of the world's three
coordinates.

**Integers, not a continuum.** You are on a layer or you are not, so every
question about one is an equality and there is nothing in between two of
them to get wrong. Jake, deciding it: *it can be integers because that makes
more sense than irrationals blending between things.* Nothing fades, nothing
is half on a layer, and no code has to say what a body at `w = 0.4` collides
with.

**`w` sits beside a position and not inside it.** A four-component position
would change every transform, every collider, every vertex and every shader
signature in blitzkit, which is a published crate with ten games on it, and
none of them would use the fourth number. The engine stays three
dimensional and is never told. What hands it a scene is what knows the
layer.

**One place answers whether a thing is yours.** The fault this building
keeps making is two lists of one thing, and a layer is a question asked by
four lists at once: what you walk into, what is drawn, what lights you, and
what you hear. Each of those asking in its own way is how a lamp ends up
lighting a room on another layer. One predicate, called by all four.

**You change layer only where something says so.** Today that is spec
0133's sphere and nothing else. Walking does not change it, falling does
not change it, and there is no edge that changes it.

## What this takes away

The value here is subtractive, and the list is the argument.

- `elysium::holds`, a 150 metre box asked by five callers, becomes `w == 1`.
- The field's trees are kept off spec 0133's ground by an early return
  written on 2026-11-23. The field is layer 0; nothing is needed.
- `field::afoot(&out, at) && !elysium::holds(reaches, at)`, same day, same
  reason, goes the same way.
- `Arcade::sky_clock`'s branch becomes the layer's own hour.
- The block that moves spec 0064's lamps 580 metres so the copy of that room
  is lit stops being a move at all.
- `elysium::drawn` and `elysium::holds`, which had to be reconciled because
  they disagreed over part of the ground, become one question.
- `elysium::OUT`, `BROAD` as a bound, and `REACHES_BACK` all go, along with
  the back door they were trying to place well.

## Acceptance criteria

Part of this is built. The ones marked unbuilt have no test yet.

- Anything that does not ask about a layer is in the
  arcade. — `layers::tests::what_does_not_ask_is_in_the_arcade`
- The glass takes you across and back and nowhere
  else. — `layers::tests::across_and_across_again_is_where_you_started`
- Spec 0133's sphere changes which layer you are on, and walking in any
  direction for a minute does
  not. — `tests::only_the_sphere_changes_the_layer`
- Each layer keeps its own hour, so the building's clock does not move spec
  0133's morning. — `tests::the_hour_is_the_layers_and_not_the_buildings`
- The field is unbounded on its own layer, exactly as it was before this
  spec. — `tests::you_cannot_walk_behind_the_building`

Unbuilt:

- A body on one layer does not collide with anything on
  another. — `layers::tests::nothing_on_another_layer_stops_you`
- Nothing on another layer is
  drawn. — `layers::tests::nothing_on_another_layer_is_drawn`
- No lamp lights across layers. — `layers::tests::no_lamp_reaches_another_layer`

### Verified by hand

- Walking out of spec 0133's place in any direction, for a minute, never
  arrives anywhere else. — run xanadu, go through the sphere, pick a
  direction and hold W.
- Nothing of the arcade is visible from spec 0133's place and nothing of it
  from the arcade. — look both ways from both.

## Open questions

**Does spec 0133's place move onto the arcade's own ground?** It is 600
metres south today and its geometry is placed off that. On a layer it could
stand at the same x and z, which is the whole of what a tesseract means
here, and it would make *neither place can see the other* true by
construction rather than by distance. Spec 0133 argues that distance at
length, so this supersedes an argument rather than filling a gap. It also
makes `glass::Placed` simpler, since the shift falls away and only the turn
is left.

**What does the minimap do?** Spec 0132 draws the floor you are on. A layer
is a second axis of the same question.

**Is sound a fourth list or does it come for free?** The three visible ones
are certain. The sound system has not been read with this in mind.

## Out of scope

**A fourth axis you can move along.** Not a tesseract you rotate through,
with three dimensional cross sections changing as you slide. That means a
fourth number on every position, transform, collider and vertex, which is a
different engine. Nothing here wants it: you do not slide along `w`, you
step through a sphere.

**Anything in blitzkit.** The engine draws what it is handed. If the layer
ever has to be *seen* rather than merely lived on, that is its own spec.
