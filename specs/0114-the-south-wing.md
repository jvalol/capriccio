# 0114 The south wing

**Status:** implemented
**Date:** 2026-11-17, built the same day

## Goal

The glass room moves off the nook's ceiling into a wing of its own, south of
the nook, and the way up becomes one straight flight climbing south out of the
nook's end wall. You walk into the library, see the stair through an opening
in its far wall, walk up it, and arrive facing the sphere down the length of
the room.

Under that wing is new ground floor, which this spec builds the shell of and
leaves empty.

## Why this room moves

**Six walks over one night, and the stair never came right.** Spec 0064 put
the flight inside the nook, and every arrangement of it inside the nook traded
one storey against the other.

- Climbing south at the nook's north end, you met its blank flank walking in
  and went round it and back. *Not something I have to walk around to in order
  to walk up.*
- Turned round to climb north, it stood in open floor with carpet on four
  sides. *It's still in the middle of the room which makes it feel cramped,
  far wall would be better.*
- At the far end it divided nothing, and upstairs it arrived facing the end
  wall with its own stairwell between you and the sphere. *Now it's hard to
  get to the sphere.*
- Turned back on itself it solved that and cost the straight climb. *Not what
  I had in mind but it works.*

**The bind is arithmetic and it is worth writing down.** A straight flight
that lands you facing the sphere has to climb towards it, so its head is on
the far side of the room from the sphere; the sphere has to stand four metres
off an end wall, which is spec 0064's own bar; and the nook is sixteen metres.
Those three together put the flight in the middle of the library whichever way
it faces. Nothing about the stair fixes that. The room has to be somewhere
else.

**And there is somewhere else.** Above 3.2 metres everything south of the
nook's end wall is empty: the entrance corridor and the room at the south end
are single storey, and the only tall thing down there is that wall itself.
South of z −25 there is nothing at all, and the way into the arcade is at the
north end, so growing this way blocks no approach.

**Jake's, 2026-11-17:** *a stair against the southern wall leading south,
opening from the wall. there's room for the glass room on the second story and
room for more building space on the ground floor.*

## Behavior

**The wing's upper storey is the glass room, 9.4 by 16.** Its inside runs
x −12.4 to −3.0, which is the nook's own width on the nook's own lines, and
z −9.5 to −25.5, measured from the south face of the wall it shares with the
nook. It sits at the height the glass room sits at now, y 3.5 to 6.7, so the
colonnade, the bays, the cornice and the lid are what they already are.

**The ground floor under it was an L, because the south room was already
there.** That room ran x −5.2 to 5.2 with its west wall at x −5.5, so the new
ground floor was x −12.4 to −5.5 for its first six metres and the full x
−12.4 to −3.0 south of that, with the upper floor the whole rectangle,
overhanging by 2.2.

**It is a rectangle since 2026-11-27.** Spec 0022's gallery was that wide
because its row of shapes was, and that row went down its side walls: the
room is the hall's own width now, which is x −2.7 to 2.7 with its west wall
at −3.0, and −3.0 is this wing's east face. One wall between the two rooms
rather than two in a plane, one slab under this one rather than two round a
corner, and nothing overhanging anything.

**One straight flight, climbing south.** Fourteen risers of `cellar::RISER`
and 4.2 metres of going, as spec 0064 has always had, on the line x −7.7 that
is the middle of both rooms. Its foot is at the opening in the nook's end
wall, z −9.5, and its head is at z −13.7.

**The opening is the whole wall.** It was a doorway and not a stairwell,
2.1 wide and 2.1 high on the flight's line, with a 1.2 door beside it
because 2.1 less the flight's 1.8 leaves 0.15 either side against a body of
0.9 and the storey below could not otherwise be got into. Standing in that
storey and looking north you saw a brown wall with two slots in it. Jake,
2026-11-26: *i wonder if this wall needs to be here, the room could feel
more open*, and then *removing the entire brownish wall would feel more
spacious*.

So there is no wall west of the gallery. The nook and the storey under the
wing are one room, open the 7 metres between them, and the flight climbs
out of the middle of that opening. What is left of the hall's far end is
the stretch with the gallery behind it, which divides two rooms and stays.
The tight side is now east of the flight, where 1.3 is clear to the
gallery's west wall.

**And the floor follows the room.** The storey below was the aisle's own
confetti carpet, carried in because a room with no floor drawn stands on
black and it had no purpose of its own yet. It has one now, which is that
it is the nook: Jake, *the flooring shouldn't be arcade style*. Boards, at
the nook's own plank width.

**What the stair leaves upstairs.** The well is the flight and a balustrade
each side of it, z −9.5 to −13.7, at the north end of the room. The floor runs
whole from z −13.7 to the south wall, 11.8 metres of it, with nothing in it
but the plinth.

**And where the sphere stands.** The middle of that walk, z −19.6, which is
5.9 metres off the south wall and 10.1 off the north: over spec 0064's bar of
four at both ends, and with the room's own length behind it from the head.

**What closes.** The nook has no stairwell any more. Its lid runs whole, its
pendant grid goes back to the two rows it had before anything was cut out of
it, and the two shades this session hung beside the stairwell come out with
the stairwell.

**What the wing needs of its own.** A floor, three ground-storey walls, a lid
over the ground storey with the well cut out of it, and the sun's own box
round the building grown to hold it.

## Acceptance criteria

- You walk in at the nook's way in, through the opening and up the flight,
  and arrive on the upper floor. — `glass::tests::you_walk_in_and_walk_up`
- From the head the sphere is straight ahead with floor the whole way, and
  walking at it arrives at the plinth; and the floor's edge along the well
  stops a body. — `glass::tests::the_well_is_fenced_and_you_can_get_round_it`
- Nothing of the stair is north of the wall the two rooms share.
  — `glass::tests::the_stair_is_out_of_the_library`
- The flight stands inside the wing end to end, and its top tread is level
  with the floor it arrives at. — `glass::tests::the_stair_fits_the_wing`
- The sphere has the room's length behind it from either end.
  — `glass::tests::the_sphere_has_length_behind_it_either_way`
- The room stands over the wing and over nothing else.
  — `glass::tests::the_room_stands_over_the_wing`
- The storey under it is open to the nook along its whole width, on both
  sides of the flight, with the gallery's own wall still standing east of
  it. — `glass::tests::the_storey_below_is_open_to_the_nook`
- And that room is drawn: both halves of it, the stair in the middle of
  the opening, and boards running from one into the
  other. — `tests::the_storey_under_the_wing_is_drawn`
- A body pushed at either side of the flight is stopped by that side's
  balustrade. — `glass::tests::you_cannot_walk_off_the_side_of_the_flight`
- Every part of the nook's open floor has a lamp on it with the stairwell
  and the two shades beside it gone.
  — `room::tests::every_part_of_the_open_floor_has_a_lamp_on_it`
- The lid is one grid laid to the world, with the well cut out of the
  wing's ceiling and the nook's whole again.
  — `room::tests::the_lid_is_one_grid_laid_to_the_world`

### Verified by hand

- Walking in and seeing the stair through the opening, which is the picture
  this spec exists for. Run the arcade, come into the nook, look south.
- The ground floor under the wing, which is an empty shell and is meant to
  look like one rather than like a room somebody forgot to furnish.

## Out of scope

**What goes in the new ground floor.** It is left empty here on purpose and
what it becomes is its own spec and Jake's to name. What it is not left
without is a shell: it has a floor laid with the building's own carpet, a
door of its own beside the stair, pendants on the nook's grid, and the
lid's ceiling over it. A room with none of those is a room standing on
black with no way in, which is the fault the gallery's first build and this
one both had.

**The turning stair.** Spec 0064's half turn was built on 2026-11-17 and works;
it is replaced here because the wing makes a straight flight possible, not
because it was wrong. Nothing of it is kept.

**The nook's own furniture.** The library keeps its benches, its bookcases and
its carpet exactly where they are. The only thing it loses is the stair and
the hole in its ceiling.
