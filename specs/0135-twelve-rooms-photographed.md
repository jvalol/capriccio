# 0135 Twelve rooms, photographed

**Status:** implemented
**Date:** 2026-11-25

## Goal

Twelve of this building's rooms as published pictures, and one grid of them
beside the engine's other two, so somebody who has not walked it can see that
it is more than a hall of cabinets.

## Why there is nothing to show

Jake, looking at the two grids after a refresh: those all look good but i
don't see the rooms in the grid which are equally impressive.

This building stages two shots. `shots()` names the hall, which goes to
`media/screenshot.png`, and the village, which goes to `media/village.png`,
and `refresh-screenshots` can take exactly what that function names. Every
other room exists only as a 320 by 240 reference under `tests/rooms`, which
is a fixture rather than a picture: it is that size because a test wants to
fail cheaply, and nothing publishes it.

So the arcade's one appearance in the engine's grids is a frame of its clip,
which is the hall, and the hall is the least surprising room in it.

## Which twelve

The hall, the nook, the cellar, the baths, the garden, the pond in it, the
gallery, the back room, the hall of mirrors, the glass room, Valhalla's hall,
and the tesseract room.

**Places you stand in**, which is the whole of the rule. What it leaves out
is everything framed to make a test fail rather than to be looked at: what is
behind the wall is nearly black on purpose, and the clock, the readout, the
skies, the sun and the moon are one object against a backdrop.

**And not the village**, which is out of doors and already has a published
picture of its own. The field, the horizon and the steading go with it.

**The garden twice, which is deliberate.** One is the room from its way in
and one is the pond, which is the thing people stand and look at. They are
the two pictures of it that are not the same picture.

**Not the imaginarium and not the way up, because they do not exist.** Jake
parked them, the hotel and the register with them, on 2026-11-11, under
`src/parked/`: every room here that works puts its idea in the world and
those three put the idea in your head first. This spec named both of them in
its first draft, off a contact sheet of `tests/rooms`, where
`imaginarium.png`, `wayup.png` and `register.png` are still sitting with no
test that reads them. They are leftovers of the parking, and a picture file
is not evidence that a room is in the building.

**The tesseract gets the building's first picture test of it.** It is a room
you stand in, it is live, and `tesseract::posed` is its pose already, so what
is missing is the test, which this spec's own design says is where a pose
belongs.

## One pose, read twice

This is the whole of the design and the rest is plumbing.

Each room's camera is named once, in `rooms::shot`, and both the picture test
and the staged shot read it. Eleven of the twelve poses exist already and
each was argued over: the nook is shot along its length because across it is a grey slab once
the parapet is up, the hall of mirrors stands with its feet on the floor
because a pose passing `EYE` as `at.y` puts the camera at twice eye height
and photographs the ceiling, and the back room is at the rig rather than
inside the door because spec 0025 made the rig the point.

A published picture taken from a second pose would be a picture of a room
nobody tested, and the two would drift the way every pair of lists in this
building has drifted. One of them moves both.

## Behavior

**Twelve more shots.** `--screenshots` names each slug and the file it
belongs in, and `refresh-screenshots` takes them at 1200 wide into
`media/<slug>.png`.

**One grid.** `blitzkit/media/rooms.png`, laid out by `./contact-sheet`, four
across, which is the layout that script holds for the other two.

**Where it is shown.** The site's `/docs/xanadu` page, under the clip. The
engine's own readme is the engine's shop window and this is one game's
rooms, so it does not go there.

**And where the tiles live, which is not where a published picture
lives.** `media/rooms/<slug>.png`, a folder down. `./check-listings` asks
that every file in a repo's `media` be shown somewhere on the site or it
will rot unseen, and eleven pictures that are only ever seen inside a grid
would each have failed that. They are not an exception to the rule: a tile
is shown, in the grid, and the grid is checked like any other picture. The
folder says which kind of thing they are and the check walks past a
directory. The hall is the one that is both, its tile being the building's
own `media/screenshot.png`.

**An hour and a sky, for a room that has one.** A day here is twenty
seconds and a run that names no hour opens at nought, which is dawn on day
one, and day one is a storm: the garden and the pond came back as two dark
pictures of a wet morning. Spec 0071 pinned an hour, a sky and a long day
for the village's shot and this is that again, reading the same numbers
rather than a second pair that happens to match.

## Acceptance criteria

- Every shot names a slug and a file of its own, and no two of them
  collide. — `rooms::tests::no_two_shots_want_the_same_name_or_file`
- The camera for a room stands inside the room it is
  photographing. — `rooms::tests::every_shot_stands_in_the_room_it_takes`
- The staged shot and the picture test read one pose, so a room cannot be
  published from a place nobody tested. — `rooms::tests::the_tests_and_the_photographer_share_a_pose`
- The tesseract room draws what it drew, which is the one picture test the
  twelve were missing. — `tests::the_tesseract_is_drawn`

### Verified by hand

- **The twelve pictures.** Framing is Jake's and nothing here can judge it.
  The tested poses are the starting point because they are the ones with
  reasoning attached, not because they are known to look good at this size.

## Out of scope

**Re-framing.** A pose written to make a 320 by 240 test fail cheaply may
read badly at 1200, and the answer to one that does is to move it in the one
place both readers share, which is a change to this spec's own design rather
than a thing it has to get right first time.

**The other rooms.** What is behind the wall and the shapes each have a pose
already and are left out by the rule above, not by an oversight. A thirteenth
tile is a line in `rooms::shot`.

**The three orphan references.** `imaginarium.png`, `wayup.png` and
`register.png` are read by nothing. Whether a parked room keeps its last
picture is a question about parking rather than about this, and deleting
them is a line of its own.

**The village and Valhalla's bridge.** Out of doors, and the one grid this
adds is of rooms.
