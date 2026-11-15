# 0082 Lights in the pool walls

**Status:** implemented
**Date:** 2026-11-15

## What this is for

Jake, 2026-11-15: subsurface pool lights that shine up through the water
surface from the pool walls.

The baths had light from the ceiling and nothing else, so the pool was a
dark blue rectangle in a lit room. A pool at night is lit from inside it,
and the thing that reads is not the fittings, it is water with light in it.

## What it does

**Six lamps in the two long walls of the basin**, three a side, 0.40 under
the still surface. The long walls and not the short ones: the short ones
are two and a half metres apart and one of them is the flight of steps.

**And each one is in a lens.** `spa::pool_lens_mesh` is a short bezel with
a lens doming out of it, turned like everything round in this building,
two-sided because it is an open shell. `spa::pool_lamps` is derived from
`spa::pool_lights` and not listed beside it, which is the rule this room
learnt the hard way when its lamps were on the ceiling and its fittings on
the wall.

**Short and bright rather than long and soft.** The first build gave each
lamp three metres over a pool eight by five, which is six lamps washing the
basin evenly: the water came up a shade paler and nothing read as a light.
2.1 metres and 1.5 of intensity gives each one somewhere of its own.

**The lens stands proud of the lining**, by its own bezel, because a lens
level with the wall is two surfaces in one plane, which is the fault this
room has had twice already.

## What the water does with it

The water is translucent, so what the room sees is the basin lit from
within, through the surface. That is most of the effect and it was there as
soon as the lamps were.

**The lens itself is only legible near the surface.** Seen from across the
pool the sight line runs two and a half metres through water and Beer's law
takes the disc out entirely, while the pool of light it throws on the wall
still reads because it is large and the lens is small. Drawn at five
heights thirteen centimetres apart, it is bright above the water line and
gone about twenty centimetres under. Nothing is wrong with it; that is what
a small bright thing under water looks like from a distance.

## What it does not do

- **The surface is not lit from beneath.** A lamp below it is behind it
  from the room: the lambert on the face you can see is negative and the
  engine returns nothing on a negative lambert. Light coming up through the
  surface itself is an engine change and is not in this.
- **Nothing is reflected in the water.** What the pool gives back is still
  the fill and a share of the sun, per the engine's own water shader.
- **The lamps do not cast.** Two lamps in the building may, and these are
  not them.

## Acceptance criteria

- The baths are drawn with the pool lit from inside it. — `tests::the_baths_are_drawn`
- Nothing in the room moved but the water and the basin. — the same reference, which is the only picture that changed.

### Verified by hand

- The lens reads as a light set into the wall rather than a sticker on it.
  — run the arcade, go down to the baths and stand at the long side.
