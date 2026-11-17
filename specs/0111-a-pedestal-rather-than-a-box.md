# 0111 A pedestal rather than a box

**Status:** implemented
**Date:** 2026-11-16

## Goal

Jake: make the plinths in the gallery more ornate, Greco-Roman style.

## Behavior

**A plinth was one grey box**, 0.63 across and 1.08 high, drawn from a scaled
cube. The room skill's own words for that are that boxes are placeholders and
he will say so, and six of them stood in a row.

**It is six courses now**, bottom to top: a square base block, a turned
torus, a fluted shaft, an echinus, a square cyma and the abacus the shape
stands on. What makes a pedestal read as classical is the silhouette, which
is something wider at the foot, something wider again at the head, and a
shaft between them that is neither. One box has none of those.

The whole is still `stands_at()` high, so every shape sits exactly where it
sat and nothing above the pedestal moved. The courses are shares of that
height and come to one.

**The fluting is what the radius does with the angle.** `MeshData::surface`
is handed both ways round the mesh, so flutes cost nothing but the steps to
draw them. Boxes set into the face would be two surfaces in one plane, which
this building has written down seven times under other names. Sixteen flutes
rather than a Doric twenty: the shaft is half a metre across and read from a
metre away, and twenty at that size is a texture rather than a fluting.

**The shaft is narrower than its own courses, which is what makes it a
column.** The first build had it flush with the base at half a metre across
and 0.84 tall, which is a drum. A classical shaft is many diameters tall and
this one can only be two, so the slimness has to come from standing it in
from the base and the abacus rather than from its height. That build was
thrown away before Jake saw it.

**Then he saw the second and said the base block was too plate-like, and
asked for a moulding under the abacus.** Both are in. The block lost width
and gained height, 1.30 and 0.060 becoming 1.24 and 0.085, because a base
course is a block you could stub a toe on. The torus over it stopped being a
square step and went onto the wheel with the bulge a torus has, since a
square step under a round shaft is a box pretending to be a moulding, which
is the complaint that started this. And the cyma is a narrower band under the
abacus, 1.08 against 1.20, so the head steps out instead of ending in one
flat slab.

**The footprint is the base block.** `plinth_under` still hands out one box
for the collider and for what holds you off, and it is the widest course,
because what you would walk into is the thing at your feet. Two pedestals now
stand 0.27 apart where the plain boxes left 0.32 and the first build left
0.21.

## Acceptance criteria

- The six courses fill the pedestal exactly and in order, the turned part meets the boxes either side of it, and the base block is the widest. — `display::tests::the_courses_fill_the_pedestal`
- The gallery is drawn as it should be. — `tests::the_gallery_is_drawn`

Two controls. Dropping the shaft's share to 0.700 gives "the six courses come
to 0.965 of a pedestal". Widening the cyma past the base block gives "the
cyma is wider than the base block, so the footprint is wrong".

The arithmetic is spread over six constants and nothing else checks it. Take
one out of the middle and the shape is left standing on air or sunk into its
own abacus, and the only thing that would say so is a picture.

The picture test did say so, at 6.3 per cent of its pixels, and its
difference was red on the pedestals and nowhere else: the shapes, the wall,
the Mandelbrot and the floor are untouched. It was blessed after Jake
approved the look and not before.

### Verified by hand

By eye, three rounds of it, at the same pose each time.

## Out of scope

The colour. `aim::PLINTH` is what it was, and whether a classical pedestal in
this room wants to stop being the same grey is Jake's.

The benches in the nook and the cabinets in the hall, which are boxes for
their own reasons and were not what he asked about.

Edition 2018 caught this once on the way: `[T; N].into_iter()` yields
references, which the room skill already writes down and clippy had to say
again.
