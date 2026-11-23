# 0132 A map in the corner

**Status:** implemented
**Date:** 2026-11-23

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

**It does not turn, and GTA's does.** The overlay draws rectangles square to
the screen and nothing else. There is no rotation in a quad and none in its
uv either, so a map that turns under a fixed arrow means rasterising the
whole panel into a texture every frame and writing it to the GPU: a
scanline fill of a few hundred rotated rectangles into about forty thousand
pixels, sixty times a second. That is a real thing to build and it is not
this one. What a north-up map loses is the "forward is up" reading; what it
keeps is that the building's north is always the same way on it, which is
not worse, only different.

**And the mark is a dot and a nose, not an arrow.** Same reason: an arrow
that turns is not a thing a quad can be, two squares are, and the one in
front says the heading. It is what spec 0115's `here` already does on the
wall, which is this building's own answer to this exact question.

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

**Clipped to its own panel.** A quad has no scissor, so a room rectangle
that runs off the panel runs across the room behind it, and at this scale
most of the building is off the panel. Anything wholly outside is dropped
and anything crossing the edge is cut at it.

**Cut once, at startup**, for all three storeys. `plan::filled` walks every
collider in the building against a fifteen centimetre grid, which is five
milliseconds a storey and is not a thing to do sixty times a second. The
drawing then costs one quad a room piece.

**The storey you are on**, by `plan::storey_at`, which is the same answer the
wall plan's mark uses.

## Acceptance criteria

- Nothing of the map is drawn outside its own panel, from spots spread over
  all three storeys. — `minimap::tests::nothing_of_it_is_drawn_outside_the_panel`
- The floor you are standing on is under the
  mark. — `minimap::tests::what_you_are_standing_on_is_under_the_mark`
- The nose stands on the side of the dot you are facing, with north up and
  east right. — `minimap::tests::the_nose_points_the_way_you_face`
- Two storeys do not draw the same
  map. — `minimap::tests::two_storeys_do_not_draw_the_same_map`

### Verified by hand

- **Whether a map that does not turn is readable while you walk.** Run the
  arcade and walk the hall, the nook and the cellar without looking up.
- **Whether it reads as a map or as a grey shape.** The rooms are pale on a
  dark plate and the cabinets show as notches down the hall, which is the
  one place the cut has any detail in it.
- **Whether an always-on HUD suits this building at all.** Everything else
  here is found by walking, and a map in the corner is the first thing in it
  that hands you the layout.

## Out of scope

- **No turning.** Named above, with what it would cost.
- **No marks on it.** No doors, no toys, no trail. Spec 0036's trail records
  what each room has given up and nothing draws any of it, which this leaves
  open rather than opening.
- **No zoom.** One scale, no dial. Spec 0064 refused an index on the arrow
  keys for the same reason.
- **Nothing of `behind`.** `plan::drawn` already leaves it out, for the scale
  reason spec 0115 gives: 360 of the ground floor's 463 pieces are that one
  room. That it also keeps a secret off the map is luck and not design, and
  should not be leaned on.
