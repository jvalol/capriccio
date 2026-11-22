# 0127 Grass over the edge of the way there

**Status:** implemented
**Date:** 2026-11-22

## Goal

Jake: now do the grass over the edge.

Spec 0126 named this one and put it out of scope in the same breath: the
grit wears thin at its sides and then stops at a line, because the ribbon
has an edge and the field's cover does not know about it.

## Behavior

**The one thing that could break the line was the one thing not allowed
near it.** `field::cover` is the flowers, the tufts and the fallen leaves,
and it honoured `along_the_path`, which is four metres either side of a path
1.2 wide. So the path came with three metres of mown verge down each side of
it, and the edge of the grit met grass that had been kept back from it.

`on_the_way` is the footprint instead: the ribbon itself, less `PATH_OVER`
at each side. A tuft may stand on that outer hand's breadth, which is how
grass meets a path, and anything further out is simply the field.

**The trees are unchanged and that is the point of having two.**
`along_the_path` is the sightline spec 0086 wore this run for, and a birch
standing beside the path can still stand in front of the village. A tuft
cannot. One of them is a question about what you see over the field and the
other is a question about what you tread on, so they are two keep-outs and
not one knob.

The bridge's abutment is the same lesson, written down two specs before this
one and in `field::cover`'s own comment: its own footprint and not
`bridge::shades`, or there is a bald stripe down the field.

**And the edge moved to where anything can ask about it.** `path_edge` was
keyed on the metres walked, which the ribbon accumulates as it is built and
which a spot standing in a field has no way to know. It is keyed on the
share of the run now. The uv still runs in metres walked, because a texture
laid in shares stretches where the line bends, so those are two numbers that
nearly agree and are kept apart on purpose: the grass line and the drawn
line coming apart is exactly the fault this would have.

## Acceptance criteria

- The middle of the way there is on it, a hand's breadth past its edge is not, and what the field sows comes within a third of a metre of its edge in more than twenty places. — `village::tests::the_grass_comes_over_the_edge_of_the_way_there`
- Nothing the field sows stands on the worn part, and no tree stands in the view down the run: the two keep-outs, each asked of what honours it. — `village::tests::nothing_grows_across_the_way_to_the_village`
- The field is drawn as the blessed reference has it. — `tests::the_field_is_drawn`

### Verified by hand

- The near field at the gap, where the verge was: `./target/release/capriccio --done --pose "-18.85 0 15.15 1.55 309.4 -22.0 130"`.

## Out of scope

- **Grass growing out of the path.** Nothing stands on the trodden middle,
  which is the fault `field::cover`'s own comment was written about: flowers
  through a cart's bed is what a field does to anything standing on it that
  it was not told about.
- **The grit's own edge.** It still ends where the mesh ends. What this
  changes is what stands beside it, not what it is.
