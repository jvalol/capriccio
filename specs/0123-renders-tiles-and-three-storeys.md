# 0123 Renders, tiles and three storeys

**Status:** implemented
**Date:** 2026-11-21

## Goal

Jake, looking at the staged shot: the village is really drab, let's steer in
the direction of the style of Heidelberg Germany.

## Why it is drab, which is arithmetic rather than taste

Nine colours are named for this place and eight of them are brown. The wall
is 0.46, 0.385, 0.285; the roof is a darker brown; the timber is a browner
brown; the street is brown and its ruts are a darker brown; the door is brown.
The one thing out there that is not brown is the water in the trough. A
picture of it is a brown street between brown buildings under a blue sky, and
no amount of boarding and shingling detail fixes a palette with one hue in it.

The second half is height. The cabins stand 2.4 to 2.8 to the eaves, which is
one storey. A street ten metres wide between one-storey buildings is a
frontier main street, and that is what this reads as, because that is what it
is.

## What Heidelberg's Altstadt actually is

Rendered facades, each house a different colour, in ochres, creams, roses and
pale greens. Clay tile roofs, steep, in a warm red. Red sandstone at the
doorways and the ground course, which is the stone the castle and the bridge
are built of. Three to five storeys, narrow, standing shoulder to shoulder.
Shutters. Not one brown anywhere.

## Behavior, in this pass

**A render apiece.** A palette of six, one picked per building, named once so
the drawing and any test ask the same thing. The point is that no two
neighbours match, which is the whole look: a street of one render is a
terrace, and a terrace is not what this is.

**Three storeys and four.** Eaves go from 2.4 to 2.8 up to 8 to 12, each still
different from its neighbour, since spec 0068's own note is that none of them
being the same size is what tells a village from a row of sheds. The street
stays ten wide: it is not squeezed, it is given something to be a street
between. The barn keeps its own proportion.

**Tiles rather than shingles.** The courses stay, since laying them is what
keeps a roof from being one sheet, and what changes is their size and their
colour: a clay tile is smaller than a timber shingle and red rather than
brown.

**Windows by the storey.** One row per floor rather than one row at 1.25,
which is the other half of what says how tall a building is.

**A stone course at the foot.** Red sandstone, a storey's worth at the ground,
which is what the Altstadt has and what stops a rendered wall running into the
street.

**And the street is paved.** Setts of 0.28 in a running bond, each its own
tenth either side of one grey, with a darker joint round it. The two lines the
wheels run in stay: a paved street does not rut but it polishes, which is the
same two lines down the same curve and keeps the picture's own account of
itself true. The common is still beaten earth, because a village square is
where the fire and the cooking are.

## Acceptance criteria

- No two buildings that stand next to each other take the same
  render. — `village::tests::no_two_neighbours_are_the_same_colour`
- Every building that fronts the street is at least three storeys to its
  eaves. — `village::tests::the_street_has_something_to_be_a_street_between`

**Next to each other is the street's own width**, which is the one number
that separates a pair somebody reads as a pair from the two ends of a
village: fronts facing across the street stand ten apart, two in a row four
to five and a half, and the gap left for the common fourteen. The test also
asks that every building has a neighbour at all, because a palette with a
colour per house passes any rule that reaches no pairs.

**And it is the fronts and not the barn.** The criterion said every building
when it was written. The barn closes the west end from beyond the street
rather than standing on it, and it is a working building at six metres:
raising it to three storeys says the place has none. It keeps its own brown
for the same reason.
- The street is still clear, and the rounds people walk still
  walk. — the tests spec 0068 already has

### Verified by hand

- Whether it reads as Heidelberg or as a paint job on a frontier town, which
  is the only question that matters and no test can answer it.
- Whether the renders are too clean: an Altstadt facade is weathered and this
  engine has no dirt.

## What the building found while this was done

**Every window in this village had been buried since it was built.** A pane
was drawn `SET_BACK` inside its own wall, which is a pane inside a solid box,
which the engine draws nowhere at all. Nobody saw it because a one-storey
cabin at thirty metres is a shape and not a facade, and because nothing asks a
picture whether a thing it names is in it. The panes are on the wall's face
now with a painted surround round them.

**And then they blazed at noon.** A pane is drawn through
`garden::gives_off`, which is never under one, so once they could be seen
every house on the street had its lamps lit under a blue sky. Dark glass by
day and lit by night, on `garden::sun_up`.

**One cabin was two storeys and nobody had counted.** Written later, by
the test above on the day it was written: the smallest of the six stood 8.4
to its eaves, which is 2.9 storeys and floors to two, so it carried two rows
of windows where its neighbours carried three or four. It is 8.9 now. Three
`STOREY`s exactly, 8.7, floors to two as well, which was tried rather than
worked out, so a height that means three storeys has to clear the number and
not meet it.

**The joints fell under a texel.** The street's whole face is one texture,
because the ruts follow a curve down it in the world's own numbers and a tiled
pattern cannot carry that. At 256 across a street 10 wide a texel is 3.9
centimetres and a joint is 5: the joints aliased away and the street read as a
chequer of bare stones. 512 across gives a joint two and a half texels.

## Out of scope, in this pass

- **The sandstone fittings.** The well, the trough and the door surrounds are
  still timber and brown.
- **The skyline.** Heidelberg has a castle over it and a church tower in it,
  and this has a barn.
