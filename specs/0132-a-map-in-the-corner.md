# 0132 A map in the corner

**Status:** implemented
**Date:** 2026-11-23
**Turned:** 2026-11-23, the same day, on the second look. Un-mirrored the same
hour, on the third.

## Goal

A map in the corner of the screen, showing the storey you are on, centred on
where you stand, with a mark that says which way you are looking. On by
default, because a HUD that is not there is not one, and on N when you want
it gone.

## Why, and what it is not

Jake, after spec 0115's plan moved to the cellar: maybe instead of the plan,
or in addition to it, a HUD like GTA6 style.

In addition. The plan is a thing in a room, cut from the building and hung on
a wall; this is always there and follows you. They answer the same question
at different distances, and they are the same cut: `plan::filled` takes a
storey's open floor off the colliders at waist height, which the wall plan
lays flat on a wall and this lays flat in a corner.

**It turns, and the first build of it did not.** That build drew north up and
said here that it could not do otherwise: the overlay drew rectangles square
to the screen and nothing else, no rotation in a quad and none in its uv, so a
map turning under a fixed arrow meant rasterising the whole panel into a
texture every frame and writing it to the GPU.

Jake, having looked at it: rotate the map so the player is always facing up as
the rooms rotate around them.

The way out was not the texture. The overlay works its four corners out on the
CPU and writes them to a vertex buffer a pixel at a time, so a quad that turns
is four points turned on the way in and no change to any pipeline; and the cut
at the panel's edge that a turned rectangle breaks is a scissor, which the
pass had never set. Those are engine specs 0112 and 0113 and they are twenty
lines between them. The texture was the answer to a question nobody had
checked.

What it costs is the building's own north, which stops being the same way
twice. What it buys is that the map does not have to be read: what is in front
of you is above the mark, which is the question a map in a corner is asked.

**And the mark is a dot and a nose, not an arrow.** An arrow that turns is
still not a thing one quad can be, two squares are, and it no longer has to
turn: the map does, so the nose stands straight up the panel for ever. It is
what spec 0115's `here` already does on the wall, which is this building's own
answer to this exact question.

**And the first turning build was mirrored.** Jake, from the first screenshot
of it: the map looks inverted, the space in the map on the right is actually
on the left.

It was. `onto` had the right-hand side of the heading negated, which is a map
that reads, turns, gets every room's shape and every distance right, and sends
you the wrong way down the hall. Which way is right is settled twice in
`main` and was read from neither: the mouse's x is added to the yaw, so more
yaw is a turn to the right, and `facing` is `(sin yaw, -cos yaw)`, so a
quarter more takes `(x, z)` to `(-z, x)`.

What let it through is that the test said what is ahead is up and never said
what is on your right is right. Forward is the one axis a mirror leaves alone,
so the whole of the first suite passed on a mirrored map. The missing half is
`what_is_on_your_right_is_right_of_the_mark`, and the direction it measures
against is written out from `main`'s two lines rather than called out of this
module, so `minimap` agreeing with itself is not what passes.

## Behavior

**The floor is drawn and the walls are the gaps**, which is spec 0115's
reading and the one that works at this size: a wall here is a line a pixel
wide and a room is a shape.

**Twelve metres from the middle to an edge.** The building is 46 along and
the hall alone is 19, so at the whole building's scale this is the wall plan
again and worse. What a map in a corner answers is not where everything is,
it is what is through the wall in front of you.

**Bottom right.** GTA's is bottom left and so is this building's frame-cost
readout, which is spec 0056 and was there first.

**Cut to its own panel, by the scissor of engine spec 0113.** A rectangle that
runs off the panel runs across the room behind it, and at this scale most of
the building is off the panel. What this drops is only what cannot reach the
panel at all, measured round each rectangle's own corner, and the edge does
the rest. Trimming them here instead is what the north-up map did, and is what
a turn makes impossible: a turned rectangle and a square panel overlap in a
polygon of up to eight sides.

**One angle for the whole map.** Every rectangle on it turns by
`minimap::turn`, read once where the quads are built. Two places saying how far
the map has turned is the fault this building has written down under nine other
names.

**Cut once, at startup**, for all three storeys. `plan::filled` walks every
collider in the building against a fifteen centimetre grid, which is five
milliseconds a storey and is not a thing to do sixty times a second. The
drawing then costs one quad a room piece.

**The storey you are on**, by `plan::storey_at`, which is the same answer the
wall plan's mark uses.

**No panel where there is nothing to map** (2026-11-24). The plate used to
be drawn before anything was known about what would go on it, so standing
in the field or the village gave a grey rectangle with the mark alone in
it. That is not a map of open country; it is what a map looks like when it
has broken, and the village's own published photograph had carried one in
the corner since the day that shot was first taken. The rooms are gathered
first now and an empty list draws nothing at all, which is also the answer
spec 0134 gives for a layer with no map of its own.

## Acceptance criteria

- What is in front of you is up the panel, whichever way you are
  standing. — `minimap::tests::what_is_in_front_of_you_is_up_the_panel`
- What is on your right is right of the mark, which is the half a mirror
  leaves intact. — `minimap::tests::what_is_on_your_right_is_right_of_the_mark`
- The angle the map is turned by and the mapping that places it agree on where
  a corner lands. — `minimap::tests::the_turn_and_the_mapping_are_one_angle`
- The floor you are standing on is under the
  mark. — `minimap::tests::what_you_are_standing_on_is_under_the_mark`
- Nothing that could show on the panel is culled before the edge gets
  it. — `minimap::tests::nothing_that_could_show_is_dropped`
- Two storeys do not draw the same
  map. — `minimap::tests::two_storeys_do_not_draw_the_same_map`
- The map says nothing about the trail until the trail has started, and one
  thing after. — `minimap::tests::the_map_says_nothing_until_the_trail_has_started`
- A station off the map is pinned to the rim without its bearing
  moving. — `minimap::tests::a_station_off_the_map_keeps_its_bearing_on_the_rim`
- There is no panel where there is nothing to map, so the field and the
  village show none. — `tests::no_map_is_drawn_where_there_is_nothing_to_map`
- Every station has somewhere to be pointed
  at. — `trail::tests::every_station_has_a_place`
- No two stations stand in one
  place. — `trail::tests::no_two_stations_stand_in_one_place`
- A station stands where its own room would record
  it. — `trail::tests::a_station_stands_where_its_own_room_would_record_it`

### Verified by hand

- **Whether a turning map is readable while you walk.** Run the arcade and
  walk the hall, the nook and the cellar without looking up. The thing to watch
  for is the opposite of the north-up fault: a map that swings under you every
  time you turn your head.
- **Whether it stops at its own border.** The scissor is the only thing holding
  the rooms inside the panel, and a rectangle escaping it draws across the
  floor.
- **Whether it reads as a map or as a grey shape.** The rooms are pale on a
  dark plate and the cabinets show as notches down the hall, which is the one
  place the cut has any detail in it.
- **Whether an always-on HUD suits this building at all.** Everything else here
  is found by walking, and a map in the corner is the first thing in it that
  hands you the layout.
- **Whether a marker pinned to the rim reads as a direction.** Start the trail,
  then walk the building with the station off the panel and see whether the
  mark is something you follow or something you ignore.

## Out of scope

- **No circle.** A driving game's map is round and this one is square, because
  a scissor is a rectangle. A round one wants a stencil or a mask and nobody
  has asked for one.
- **No marks on it beyond the trail's next station.** No doors, no toys, no
  trail behind you. Spec 0036 records what each room has given up and nothing
  draws any of it, which this still leaves alone: what is marked is the one
  place you have not been, never the four you have.
- **No zoom.** One scale, no dial. Spec 0064 refused an index on the arrow
  keys for the same reason.
- **Nothing of `behind`.** `plan::drawn` already leaves it out, for the scale
  reason spec 0115 gives: 360 of the ground floor's 463 pieces are that one
  room. That it also keeps a secret off the map is luck and not design, and
  should not be leaned on. The station marker is drawn over the panel rather
  than cut from the floor, so pointing at the sun in there gives away no more
  than the stone in the field already does.
