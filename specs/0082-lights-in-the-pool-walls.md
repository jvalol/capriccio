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

**At 0.40 the lens itself was only legible near the surface.** Seen from
across the pool the sight line ran two and a half metres through water and
Beer's law took the disc out entirely, while the pool of light it threw on
the wall still read because it is large and the lens is small. Drawn at
five heights thirteen centimetres apart, it was bright above the water line
and gone about twenty centimetres under.

**So it is at 0.20, and the lens is 0.12 across the radius.** Jake,
2026-11-15, once the surface was lit over each one. Half as far down is a
quarter of the water in the way at the angle anybody looks from, and from
the room the lights now read as points along both long edges rather than as
a wash.

The two numbers had to be settled together, which is what
`spa::tests::every_pool_light_is_under_the_water` is for. At 0.20 the lens as it
was stood two centimetres out of the water, because `POOL_LENS` is a radius
and the dome was 0.44 across. Narrowed to 0.16 it cleared the still surface
by four centimetres and an ordinary trough uncovered it; the surface was
measured reaching 0.082 above still before engine spec 0103 held it at the
coping, and it drops about as far the other way. 0.12 puts the top of it
`FREEBOARD` under, which is the one number in this file saying how far the
surface stands from where it is held.

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
- And every lens is under the water by the margin the surface moves. — `spa::tests::every_pool_light_is_under_the_water`
- Nothing in the room moved but the water and the basin. — the same reference, which is the only picture that changed.

### Verified by hand

- The lens reads as a light set into the wall rather than a sticker on it.
  — run the arcade, go down to the baths and stand at the long side.
