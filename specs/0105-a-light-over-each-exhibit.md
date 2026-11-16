# 0105 A light over each exhibit

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: galleries often have lights directly over their exhibits, one over each
would look elegant.

## Behavior

There were three, spread evenly along the row on spec 0022's argument that
three covers six without a fitting per shape. What that bought was a lamp
over a gap and a shadow thrown from a foot to one side of whatever you had
walked up to. There are six now, one over each plinth.

**Off the row's own places and not a division of its span.** `gallery_lamps`
read `display::row()` and cut it in three; it reads `display::along(n)` now,
which is where the nth shape stands, factored out of `all_of_them` so that
the ceiling and the floor cannot come apart. `all_of_them` could not answer
it: finding where a lamp goes would have built six meshes, one of them a
Menger sponge, every frame.

**And the same two always cast.** The engine carries two shadowing lamps
against six here, so two have to be chosen. The first go chose the two
nearest the eye, and Jake walked the room and sent back a light sweeping
across the floor: four metres of sidestep changes that pair three times,
measured, and each change re-points a shadow map at a different lamp, so a
shadow leaves one plinth and appears under another while you walk. A shadow
is the highest contrast thing in a room and the worst possible thing to
decide per frame.

It is the middle lamp of each group of three, which is a place and not a
distance. Each is directly over one exhibit and within a step of two more,
and nothing about it moves.

**What a light directly overhead fixes is sharpness**, which is what Jake
asked about before he asked for this. A lamp a foot to one side rakes its
shadow across the plinth and the penumbra is spread over all of it; overhead,
the throw is short and square and the sponge's shadow lands under the sponge
with its own hole in the middle. The engine's spec 0104 doubled the map for
the same complaint and the geometry was the larger half of it.

**Spots were built and backed out.** A cone is one projection where a lamp is
six, so six spots would have been cheaper and sharper, and `MAX_SPOT_LIGHTS`
was raised to six to try it. They light and they cast nothing: six cones
straight down produced the lighting in these pictures and no shadow at all,
and tilting them off vertical changed nothing, so it is not the degenerate
straight-down case that `spot_view_projection` already guards. The engine's
own `cubes` example takes its shadows from the sun and its white lamp. That
is worth chasing and it is not this spec.

## Acceptance criteria

- One lamp over each exhibit, at the exhibit's own place. — `room::tests::a_lamp_hangs_over_each_exhibit`

### Verified by hand

- The row from the doorway: six fittings, in two groups of three with the panel between.
- The sponge's shadow lands square under it: `./target/release/capriccio --pose "2.395 0 -12.6 1.55 0.0 -12.0 120.0" --picture-for 2 --picture-to /tmp/a.png`.
- The casting pair no longer changes under a sidestep, which was checked by rendering the same four moments of `down D` before and after.

## Out of scope

A shadow under all six. The engine carries two and six would be thirty six
faces of map and six passes over the scene.

Why a spot casts nothing, which is an engine question.
