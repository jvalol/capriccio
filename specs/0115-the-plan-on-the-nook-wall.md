# 0115 The plan on the nook wall

**Status:** draft
**Date:** 2026-11-17

## Goal

The nook's north wall carries a relief plan of the building: three of them in
a row, the cellar, the ground floor and the floor the glass room stands on.

## Why there

The wall is ten metres of grained panel above a dado, with two hanging shades
throwing two pools of light onto nothing. That is a gallery hang with the
pictures missing, and Jake said so looking at it: the north wall of the nook
begs for something.

**And because a visitor has no idea what this building is.** Eleven rooms,
seven of them sealed, four referring to each other. The nook is passed
through on the way in. A plan is the one thing on a wall that answers where
am I and what else is there without saying a word.

## The thing that decides the whole of it

**The plan is cut from the building, not drawn.** `Room::solid` is every box
a body walks into, which is the building's own account of where its walls
are. A plan is a horizontal cut through walls at about waist height, so each
storey's plan is that list kept to the boxes which straddle that storey's cut
and laid flat.

Drawn by hand instead it would be a second account of the layout, and this
repo has spent spec 0107 deleting the last one of those. A wall that moves
and a plan that does not is the same fault one storey down, and the only
thing that would ever notice is somebody standing in the nook comparing a
carving to a memory of a room.

**What it cannot hold, which was measured rather than reasoned.** The cut
at waist height over the whole building gives 463 boxes on the ground floor,
and 360 of them are one room: the space behind the hall's end wall, which is
the solar system at true scale and runs 156 metres east and 168 north. A
plan cannot hold a corridor and an astronomical unit at one scale. So the
building here is the part of it built at a body's size, which leaves out
`Behind` and the field, and that is a statement about scale rather than
about what counts as a room.

What is left is 39 boxes in the cellar, about a hundred on the ground floor
and 48 above, over roughly 35 metres by 46. At two metres of wall for the 46
that is about 1:23, which puts each plan a metre and a half wide and three of
them inside half the wall.

**What it costs to be true: it will be ugly in places.** A real plan is drawn
by a person who leaves things out. This leaves nothing out, so the cellar's
barrels and the colonnade are in it as a scatter of dots. That is a thing to
look at and judge rather than to argue about, and the first build is where it
gets judged.

## Behavior

**Three plans, left to right, in the order you would walk them**: the cellar,
the ground floor, the floor above. Each is the same scale, so the building's
own proportions read across all three and a room over a room lines up.

**The rooms are raised and the walls are the grooves between them.** The
first build raised the walls instead, which is a plan drawn as its own
strokes, and Jake read it off the wall: it reads abstract, like ancient cave
wall art, hard to tell it has anything to do with the building. What a
person reads a plan by is the shape of a room, so the room is the thing with
a surface on it and a wall is where that surface stops.

**A room is a cell with a roof over it and a floor under it**, both asked of
the colliders. Neither alone is enough, and each was tried: clear of the
colliders alone fills the whole county, since outside the building there is
nothing in the way either, and a roof without a floor filled the earth under
the hall, drawing the cellar 45 metres across where the cellar is 18.

**Read lowest first, left to right**, which on this wall is the wall's own
minus x: facing it you are facing north and facing north your right hand
points west. The bars inside each plan are turned round for the same reason.
Getting one of those and not the other put the three in reverse order for a
build.

**The cut height is a storey's floor plus a body's waist**, which is what a
plan is, and what puts a doorway in it as a gap rather than as a line.

**Sampled on a fifteen centimetre grid and merged.** A row of open cells
becomes one bar, and a bar the same width as the one before it joins to it,
so the hall is a handful of boxes however fine the grid: 178 bars at fifteen
centimetres against 298 at fifty, and 31 milliseconds to cut. Cut once at
startup, because this walks every collider in the building and the first
build did it every frame.

**Nothing is named.** It is cut stone, not a directory. A relief says how big
the building is, how its rooms sit together, and that there is more of it
than you have seen, and it says that without a word on it. What names would
cost is a sign in a building that has decided signs are `aim`'s business, and
that is spec 0113's question rather than this one.

**No state, and nothing to work.** Spec 0036's trail records what each room
has given up and nothing in the building draws any of it. A room picked out
in a different stone is still a plan, so this leaves the door open for that
without opening it: whoever builds the trail's end can colour a room here,
and someone who never finds the trail sees an ornament either way.

## Acceptance criteria

- Every storey's plan holds a bar for every wall of the building that the cut
  passes through, and none for a wall it does not. — `plan::tests::each_storey_cuts_its_own_walls`
- The three plans are one scale, so a room above a room is above it on the
  wall. — `plan::tests::the_three_are_one_scale`
- The whole relief fits between the dado and the cornice and inside the
  wall's width, with nothing of it in the lamps or through the ceiling. —
  `plan::tests::it_fits_the_wall_it_is_on`
- Moving a wall moves the plan, which is the claim this spec is built on. —
  `plan::tests::a_wall_that_moves_moves_the_plan`
- And the drawing fills the board rather than sitting in the middle of it as
  a smudge, which is what holds the bound: without one, `room_at` calls the
  long way out to the village the hall and the whole building draws at
  1:115. Every other test here passed in that state. —
  `plan::tests::the_drawing_fills_the_board`

### Verified by hand

- **Whether it reads as a plan or as gravel.** Two builds were needed for
  this and both were judged off a screenshot: walls alone read as cave art,
  and filled rooms read as rooms. Whether filled rooms read as *this*
  building is the open one.
- **The baths' pool is a hole in the cellar**, being water rather than
  floor, which is honest and may read as a mistake.
- **Whether the two lamps light it.** They are hung for a blank wall. A
  relief wants them raking across it rather than flat on.

## Out of scope

**Names, legends, a you-are-here.** Named above.

**The trail.** Spec 0036's, and this only leaves room for it.

**Anything to press.** The panel of spec 0112 is the thing in this building
you work; this is a thing you look at.
