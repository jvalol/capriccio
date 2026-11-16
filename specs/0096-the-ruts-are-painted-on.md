# 0096 The ruts are painted on

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake, playing: I saw weird lines move across the path and the ground of the
village as the day progressed. And, on being offered a flag that would stop
them casting: textured surfaces should have small shadows, mipmaps are a
thing for a reason.

## Behavior

The street's two wheel ruts were 116 slabs, 58 a side, each five millimetres
thick and standing two above the track. A slab standing proud of the ground
casts a shadow, so as the sun went round, 116 small shadows swung with it and
the street grew lines that moved. At distance they stopped being ruts and
became a crawl, because a strip of geometry narrower than a pixel either
lands on one or misses.

**A worn line in the ground is a texture's job.** Jake's objection is the
whole of this spec: the engine builds mips for every texture it is given and
`TextureData::build_mips` says why in its own words, that without them a
textured floor seen at an angle shimmers. Geometry gets none of that. So the
ruts are painted into the track's own face, the two wandering bands drawn
per texel from the same curve the wheels wore, and there is nothing standing
proud of anything to cast.

**One curve and not two.** `rut_wander` was lifted out of the slab loop
before the loop went, so the paint and the pieces read the same arithmetic
for the one sitting they both existed. `rut_runs` puts the middle of a rut
across the street. This building has learnt what two accounts of one line do.

**2048 by 256 over 72 metres by 10**, which is a texel every 35 millimetres
along and 39 across, and nine texels across a rut. Powers of two because the
mips halve to a pixel and odd sizes round differently at every level. The
edge is soft over a tenth of the width each side, so it is worn rather than
cut.

**Laid two millimetres over the track**, which is `behind::TRACK_HIGH` and
its reason word for word: in the track's own plane it would be two exposed
faces sharing one, which this building calls its commonest fault. The quad
itself is flat, so what it casts is its own outline at the street's edge and
nothing across the middle.

**And the first painting came out nearly black.** `texture::FORMAT` is
`Rgba8UnormSrgb`, so the sampler decodes every texel on the way in, where an
instance's colour is handed over as it stands. Written straight, the earth's
0.235 came back as 0.045. `encoded` does the sRGB transfer, and the test
below holds it.

It also deletes 116 instances from the busiest view in the building, and
`ruts` and `RUT_PIECES` with them.

## Acceptance criteria

- The track's face is worn where the wheels run, and the earth between comes back the colour it went in. — `village::tests::the_track_is_worn_where_the_wheels_run`

Both halves were put back as controls. Painting the ruts straight, ignoring
the wander, reads 133 against earth at 133 at the quarter mark. Writing the
colour without the sRGB gives the fault exactly: in at 0.235, back at 0.045.

### Verified by hand

```
./target/release/capriccio --screenshot=village --day 20 \
  --picture-for 2 --picture-to /tmp/a.png
```

- The street reads as it did: the blessed picture moved 0.5 per cent.
- The ruts fade into the distance rather than aliasing into lines.
- The hard step across the foreground, where two slabs met, is gone.

## Out of scope

The path out to the village, which is 40 more slabs of the same kind and the
other half of what Jake saw. It crosses ground that is not one quad, so it
is either its own strip with a cutout alpha or it goes into whatever the
field draws its ground with, and that choice wants looking at rather than
guessing.

A flag that would let an instance skip the shadow pass. It was the first
answer and it is the wrong one: it would have left the ruts aliasing at
distance, which is the half mips fix.
