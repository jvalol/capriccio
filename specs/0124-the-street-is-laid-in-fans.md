# 0124 The street is laid in fans

**Status:** implemented
**Date:** 2026-11-21
**Built:** 2026-11-21, in four rounds, one of which was taken back out.

## Goal

Jake, with two photographs: the stones should arc in pattern like the first,
and the second is how the texture, depth and colouring should imitate. And
before that: make the cobble stone more impressive by adding depth, thickness,
mipmaps, with the lighting making the grooves between the stones deep.

## Behavior

**Segmentbogenpflaster.** A German street is not laid in courses across it. It
is laid in fans: the stones run on concentric arcs about a point on the centre
line, `ARC` apart down the street, and where one fan runs out the next begins.
`sett_at` is the one account of which stone a spot belongs to and how far it
is from that stone's own edge, so the joint and the dome are the same
question asked twice.

**A hand across.** The setts came down from 0.28 to 0.20 and the street's face
went up from 2048 by 512 to 4096 by 768, so a stone is fifteen texels and can
carry a dome and a joint. Twelve megabytes, built once at load.

**Each stone its own.** A grey of its own and a hue of its own, warm or cool,
rather than one grey varied in brightness, which reads as one stone cut up.
And a joint of its own width, a third either way, because a fan laid off one
arithmetic is a moulding rather than stones somebody set by hand.

**The depth is in the colour.** A stone falls away toward its own joint. This
engine has neither normal maps nor occlusion of any kind that this building
uses, so that is the closest it gets, and the cost is that it does not move
when the sun does.

**And a kerb.** Sandstone, a hand proud of the paving, either side. What it is
for is that a groove says a sett has a top and a kerb says the whole street is
a course of stone laid on the ground rather than a pattern printed on it.

## What was tried and taken back out

The street was laid as a `MeshData::surface` at three cells to a sett with
every joint sunk into it, so the sun would shade the grooves itself the way it
shades the garden's water. It came back as bright streaks down the street.

One real fault came out of that and is worth keeping: `MeshData::surface`
takes `u` crossed into `v`, and x crossed into z is minus y, so laid the
obvious way round every normal on the street pointed at the ground. Fixing it
did not fix the streaks. A groove of four millimetres then drew pixel
identical streaks to one of thirty five, which rules out the angle the walls
are lit at, since that differs by a factor of seven, and points at the mesh's
own normals or at how its texture registers on it. The street is a flat quad
again and the attempt is written into `sett_at`'s doc so the next person does
not repeat it.

**Mipmaps were already there.** `TextureData::from_pixels` builds the chain on
the way in, so the street's face has had them since spec 0096 laid it.

## Acceptance criteria

- The polished lines the wheels run in are darker than the stone beside
  them. — `village::tests::the_track_is_worn_where_the_wheels_run`
- And the street reads as stone rather than as the earth it was paved over,
  by its hue, since the earth and the joint are within a thousandth of each
  other in red. — the same test

### Verified by hand

- Whether it reads as the first photograph's fans. It does from close and
  less well down the street, where the ring joints cross the view and the
  radial ones run away along it, so the arcs read and the stones inside them
  read weakly.
- Whether the stones read as the second photograph's. Not yet, and the reason
  above is most of why.

## Out of scope

- **Normal maps and occlusion**, which are the two things that would answer
  the depth properly and are the engine's to add rather than this room's.
