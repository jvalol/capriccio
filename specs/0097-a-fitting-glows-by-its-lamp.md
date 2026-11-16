# 0097 A fitting glows by its lamp

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: the lamps and sconces themselves should all glow commensurate with the
highlighting and shadowing they give off.

## Behavior

`aim::SHADE` was one colour, `(2.6, 1.9, 1.1)`, and six fittings in six
rooms were drawn in it: the nook's sconces and its pendants, the baths', the
cellar's table lamp, the sign's, and the gallery's. The lamps inside them run
from 0.35 to 1.45, so the dimmest fitting in the building burned exactly as
hard as the brightest and none of them was the colour of its own bulb.

**The garden's lantern already did it right and is where the shape comes
from.** Its glow is `LANTERN_LIT * (0.35 + lit * 2.2)`, with its own comment
saying the window burns by the same number the lamp inside it uses rather
than a second one. `aim::fitting(colour, lit)` is that line for every other
fitting, and each of the six now passes the colour and the strength its own
lamp is pushed with.

**A floor and not a plain multiple.** The weakest lamp here is the mirrors'
at 0.35, and a fitting scaled straight by that lands at 0.37 in its brightest
channel, which in this engine is a thing with no light of its own: a grey box
screwed to the wall. `GLOWS_FROM` keeps the dimmest one glass and `GLOWS_BY`
keeps it dimmer than the gallery's.

**And the mirrors' lamp was borrowing a fitting's colour.** It was pushed
with `aim::SHADE.truncate()`, so the corridor was lit by whatever colour the
building happened to be drawing its pendants, which is this relationship the
wrong way round. `mirrors::LAMP_COLOUR` is its own now, and the fitting would
read from it rather than the reverse.

Five rooms moved and the nook is the one to look at: its sconces were glaring
white at a lamp of 0.55 and are warm now, which is what the room was always
lit like.

## Acceptance criteria

- A fitting glows by the lamp it holds, and the dimmest one still glows. — `aim::tests::a_fitting_glows_by_the_lamp_it_holds`

Both halves were put back as controls. One colour for every fitting fails the
first: the gallery's lamp is 1.45 against the mirrors' 0.35 and its fitting is
not brighter. A plain multiple with no floor fails the second at 0.37.

### Verified by hand

- The gallery from the doorway: its six fittings read as lit glass and each throws a pool on the wall under it.
- The nook, reference against now: the sconces come down from white to warm.

## Out of scope

The cabinets' neon and the hearths, which are already derived from what they
throw and were never drawn in `SHADE`.

Whether a fitting should also take the lamp's *range* into account. A long
reach is a brighter bulb in the world and this reads only its strength, which
is the half that varies here.
