# 0126 The way there is walked and not paved

**Status:** implemented
**Date:** 2026-11-22

## Goal

Jake: the path to the village could use work, it looks like a series of
rectangles and not a true curved path and it could use some gravel like
texture.

Two faults in one sentence, and they are separate. Spec 0086 wore the line
and cleared the view down it, which was the whole of what it set out to do.
What it left is how the line is drawn and what it is drawn in.

## Behavior

**The rectangles.** The way there was forty boxes laid end to end, each
carrying the run's one heading and each thrown a tenth of a metre sideways
of the last. Every join notched both edges and showed an end face, and the
whole of it was one flat colour, so what it read as from the gap was a
concrete ramp someone had dealt out in slabs.

It is one ribbon now. `path_mesh` walks `path_at` in ninety-six steps, takes
the heading from the line itself at each of them, and lays two vertices
across. The edges turn where the line turns, there is no join to notch, and
it costs a tenth of the triangles the boxes did.

**The straightness.** That was the line and not the drawing of it. `path`
ran straight from the gap to the ruts with a wander of a seventh of a metre
over fifty, which is a road somebody surveyed with a tremor in his hand.
`PATH_BOW` is three and a half metres, over a bow that is nought at both
ends because both ends are fixed: the gap is a hole in a wall and the far
end is where the street's own ruts run out.

It bows toward the village rather than away from it, which is a claim about
the sightline and not about the look of it. `PATH_CLEAR` keeps the view from
the gap open by lying near it rather than by aiming at it, in spec 0086's
own words, so a bow the other way would have walked the cleared strip off
the view it exists to keep open.

**And the gravel.** `grit` scatters twelve hundred and fifty chips into a
tile 1.1 metres square, each with its own size, its own grey and its own
height, and `grit_bumps` gives the engine the shape of them to light. The
chips are sown into the field rather than the field asked of every chip, so
the work is the area of the chips and not the area of the tile times their
number: five milliseconds for the colour and two for the shape, against the
street's hundred and twenty-seven and thirty-six.

A tile and not a face. The street's surface is one texture laid on it once,
because its ruts follow a curve down it in the world's own numbers and
nothing repeating can carry that. Nothing about this path varies along its
length, so a tile of it repeated is the same picture for a thousandth of the
memory.

**The tile knows where the path's edges are.** Its v runs nought to one edge
to edge, whatever the edge is doing there, so the grit wears itself thin at
nought and one and the worn part lands on the side of the path rather than
near it. Left alone, the band of stone stopped dead at a line and read as a
carpet runner laid on a field. Its u is the metres walked in tiles of them,
because a texture laid in shares of the run stretches where the run bends.

**Three fifths and not four.** Two thousand chips covered four fifths of the
tile and the path came out the palest thing in the frame, a ribbon of salt
on a green field. What makes it read as a track is the earth between the
stones, and at four fifths there was none of it left to see.

## What it cost to find

The seam guard is the part worth reading. The first one compared the two
borders of the tile against two texels inside it, on the argument that a
clamped border reads its own height twice, halves its own span and leans
twice as hard. The negative control passed: on a field this smooth a
one-sided difference is within a byte of a centred one, and the comment was
a cause nobody had checked.

What actually keeps the tile seamless is that the chips are sown across the
border, and that is what the test guards now: the two outermost columns hold
0.76 of the tile's own mean as this is written and 0.53 with the wrap taken
out. The first number is not one because twelve hundred chips thrown at a
tile clump, and a single column of it runs from 0.72 to 1.36 of the mean
wherever you read it. The threshold separates two measured cases and is not
a measure of anything on its own.

## Acceptance criteria

- The way there is a curve, with both ends still where spec 0086 pinned them, no corner in it and no step back, leaning toward the village and not away. — `village::tests::the_way_there_bends_toward_the_village`
- It is one ribbon, every triangle of it facing the sky, neither edge a ruled line. — `village::tests::the_ribbon_lies_face_up_with_no_notch_in_it`
- The grit is a direction and not a colour, nothing in it leans into the earth, and its chips cross the tile's border. — `village::tests::the_grit_tiles_without_a_seam_and_leans_out_of_the_earth`
- What the field keeps clear is the line that is walked, which is three and a half metres from the line between its ends. — `village::tests::the_keep_out_follows_the_curve`
- Nothing the field sows stands on it, which is spec 0086's own criterion swept along the curve. — `village::tests::nothing_grows_across_the_way_to_the_village`
- The field is drawn as the blessed reference has it. — `tests::the_field_is_drawn`

### Verified by hand

- From the gap the way there bends off to the village instead of ruling a line at it, and up close it is stone: `./target/release/capriccio --done --pose "-18.85 0 15.15 1.55 309.4 -3.0 130"`, and the same pose at a pitch of -22 for the near field.
- The whole run from above, where the bow is the thing you can see: `--fly --pose "-30 48 -18 1.55 0 -89.0 130"`.
- Colour, and whether a gravel track is what it reads as. The night pose spec 0086 was written at is still the one the references carry; every picture named here is daylight, which is where grit shows.

## Out of scope

- **The far end of it.** The way there meets the ruts and stops, as spec
  0086 left it. The junction is a path arriving at a paved street and
  nothing is laid to join them.
- **Grass over the edge.** The grit wears thin at its sides and then stops
  at a line, because the ribbon has an edge and the field's cover does not
  know about it. What would answer this is the field sowing tufts along the
  path rather than being kept off it, which is a change to `field` and not
  to this.
- **A path anywhere else.** The bridge's foot still has no worn approach,
  which spec 0086 also said.
