# 0115 A plan cut from the building

**Status:** implemented
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

## Where it hangs

**On the cellar's south wall**, between the wine racks, over the pool table.
It had the nook's north wall and Jake took that back for pictures, which is
spec 0131: ten metres is a mural's wall and a plan is a diagram, which is
most of why it never sat right there however it was dressed. The stretch down
here is clear because the racks are skipped where the table stands, so
`board` asks `racks()` for the gap rather than measuring it again, and
moving the table moves the plan.

The same panelled mahogany, so nothing about the drawing changes. Two things
do, and they are one fact: facing this wall you are facing south where in the
nook you faced north, so the relief stands out of the face the other way and
a page read with east to the reader's right now puts east on the wall's plus
x. Getting one and not the other is a drawing with its back to you or a
mirrored one, so they are one constant each, `OUT` and `EAST`, and the stride
from storey to storey turns with them.

**And it hangs on the panelling, not on the wall.** The cellar's boards stand
five centimetres off the structural face, which is a great deal more than the
relief does. Cut on the wall, the whole thing was inside the timber: faint
lines showing through and nothing else.

**The two lamps are at either end of the stretch rather than over the middle
of it**, which is the raking light a relief wants and which the nook's pair
never gave it.

## Behavior

**Three plans, left to right, in the order you would walk them**: the cellar,
the ground floor, the floor above. Each is the same scale, so the building's
own proportions read across all three and a room over a room lines up.

**And each on a sheet of its own, the size of the whole building.** Not the
size of that storey: the same rectangle three times, with the storey sitting
where it really does inside it, so the floor above reads as a corner of what
is under it and the cellar as the north end. Cut at one scale and then left
to float wherever its own rooms happened to sit, the three hung at three
heights and three apparent sizes with bare panelling between them, and Jake
read the wall as a set of unrelated shapes: the map doesn't quite land and
it's hard to read.

The sheet is also the drawing's ground. A cut plan is pale stone and the
panelling behind it is lit timber of much the same brightness, so a room only
read where its edge fell on a shadow. A grey sheet darker than everything on
it gives the stone something to be pale against.

**With a lip round it.** Three grey rectangles cut into a timber wall read as
three rectangles; a rim catching the light off the lamps either side is what
says a drawing is hung there. It lies on the sheet's own border rather than
outside it, because the band between the dado and the cornice is what set the
scale and there is nothing spare over the top. A centimetre of it is about
the building's own outer wall at this scale, so what it covers is the wall it
stands for.

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
plus x: facing it you are facing south and facing south your right hand
points east. The bars inside each plan turn with the stride rather than
separately, because they are the same fact. Getting one of those and not the
other put the three in reverse order for a build.

**The cut height is a storey's floor plus a body's waist**, which is what a
plan is, and what puts a doorway in it as a gap rather than as a line.

**Sampled on a fifteen centimetre grid and merged.** A row of open cells
becomes one bar, and a bar the same width as the one before it joins to it,
so the hall is a handful of boxes however fine the grid: 178 bars at fifteen
centimetres against 298 at fifty, and 31 milliseconds to cut. Cut once at
startup, because this walks every collider in the building and the first
build did it every frame.

**The names are cut into the rooms, not plated onto them.** The engine's
`text::drawn` keeps a letter's black ground, which is a plate at no cost on a
dark panel and the wrong thing on a pale room; lit at a broad shine that
ground took so much specular that the plate came out brighter than the
letters on it, and every name read as a smear. `text::stencilled` drops the
ground and the letters sit on the room itself, dark on stone.

**Each room carries its own name**, on the largest single piece of itself:
the hall, the nook, the gallery, the cellar, the baths, the glass room, the
imaginarium. The mark says
where you are and the nose says which way you face, and neither of them says
what anything is.

**The words are the building's own.** Every one is what capriccio's specs
call that room and have called it throughout: the nook 216 times, the cellar
82, the gallery 68, the baths 66, the imaginarium 61, the hall 43. The sign over the aisle says NOOK and these
do not follow it: a sign shouts across a room and a plan is read at arm's
length. Jake's, asked and answered. Nothing here was drafted, which is why
none of it carries a copy marker.

**On the largest piece and not in the middle.** A room's weighted middle is
outside it whenever the room is an L or has a hole, and the baths are both:
the pool is a hole and BATHS was written on bare plaster.

**`room_at` cannot name the two rooms upstairs**, having no variant for
either, so that floor is named by where the glass room stands and whatever
else is up there is the imaginarium.

**One thing that moves: where you are standing, and which way you face.**
A dot on the storey you are on, with a nose out in front of it, in brass
against the stone. Jake, looking at the plan without it: this is unreadable,
how do I know where I am or what I am looking at. A plan of a building you
only ever see from the inside has nothing a reader can match it against, and
the mark is what it is matched against.

Drawn rather than cut, because it moves. The rest is cut once at startup.

**And no other state.** Spec 0036's trail records what each room has given
up and nothing in the building draws any of it. A room picked out in a
different stone is still a plan, so this leaves the door open for that
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
- The mark lands on the room you are standing in. — `plan::tests::the_mark_lands_on_the_room_you_are_in`
- Every room the building can name is named once, and on itself. — `plan::tests::every_room_is_named_once_and_on_itself`
- East is on the right as the wall is read, which is the only thing a
  mirrored plan breaks: the drawing and the mark go through one mapping, so
  turning it round turns both and everything else still agrees. —
  `plan::tests::east_is_on_the_right_as_it_is_read`
- Each storey is drawn on a sheet the size of the whole building, the three
  sheets are one rectangle three times, they sit side by side inside the
  board, every storey's bars land on its own sheet, and the sheet and its lip
  are in a plane with
  nothing. — `plan::tests::each_storey_is_drawn_on_a_sheet_the_size_of_the_building`
- The three fill the wall they are given, and the building fills the sheet it
  is drawn on, which is what holds the bound: without one, `room_at` calls
  the long way out to the village the hall and the whole building draws at
  1:115. Every other test here passed in that state. —
  `plan::tests::the_building_fills_its_sheet_and_the_three_fill_the_wall`
- The board is clear of the wine racks either side of it and of the skirting
  and cornice, and every bar stands on the panelling's face rather than in
  it. — `plan::tests::it_fits_the_wall_it_is_on`

### Verified by hand

- **Whether it reads as a plan or as gravel.** Two builds were needed for
  this and both were judged off a screenshot: walls alone read as cave art,
  and filled rooms read as rooms. Whether filled rooms read as *this*
  building is the open one.
- **The baths' pool is a hole in the cellar**, being water rather than
  floor, which is honest and may read as a mistake.
- **Whether the cellar's two lamps light it.** They stand at either end of
  the stretch, which is the raking light a relief wants, but the cellar is
  deliberately dark at the edges and the middle sheet is the furthest from
  both.
- **Whether two mostly empty sheets read as informative or as unfinished.**
  The cellar fills the top-left of its sheet and the floor above the
  bottom-right, which is true of the building and is the reason for the
  shared rectangle, but it is a lot of grey.

## Out of scope

**Names, legends, a you-are-here.** Named above.

**The trail.** Spec 0036's, and this only leaves room for it.

**Anything to press.** The panel of spec 0112 is the thing in this building
you work; this is a thing you look at, and the mark on it is not worked
either.

**What a room is for.** The names say what each room is called and nothing
says what is in it or why to walk there.
