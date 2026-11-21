# 0121 The deep is parked

**Status:** implemented
**Date:** 2026-11-21

## Goal

Take spec 0018's thirty three kilometres of true-scale solar system out of the
building, and spec 0017's door with it, the way spec 0063 took the wing. Keep
every line of both compiling and tested under `src/parked/`.

## Why

Jake walked it and said the premise does not land, and that the room you enter
it from is far more interesting. He is right, and the reason is worth writing
down because it is a rule about this building rather than a complaint about
one room.

**The orrery is a working model. The deep is a true statement.** Spec 0015's
room is 111 metres with nine bodies and their moons in it, all of it turning
on a twenty second day and a sixty second year, and you can get your face up
to any part of it. It buys that with two lies, a 306 times gap between body
scale and orbit scale, and the lies are exactly what make it a thing rather
than a fact. The deep gives up density, gives up motion, which spec 0018 put
out of scope in as many words, and gives up ever seeing more than one body at
a time. What it buys with all three is accuracy, and accuracy is not an
experience.

Spec 0015 said this would happen before 0018 was written: the honest system
"is not something anybody can look at". Spec 0018 made it lookable by removing
the looking, which is what the ride is.

**Density is the variable.** Every room in this building that works is a
quantity too large to picture made walkable, and walkable here means something
every few paces. The hotel's corridor is two kilometres with a door every
stride. The orrery is 111 metres with seventeen bodies. The deep is 33,000
metres with nine, which is one thing every 3.7 kilometres, and the ride exists
because walking that is unbearable.

**And it repeated the room before it.** You walked out of a solar system into
a solar system, so the second could only be a more pedantic version of the
first. Nothing else in this building does that.

The replacement was discussed and nothing proposed cleared the same test, mine
included: a walk whose markers are the Fibonacci metres fails identically and
worse, its emptiness being the point, and spec 0035 had already built and
parked that room. Rather than leave a room nobody likes standing while
something better is thought of, the door is closed.

## What goes

**`beyond`**, whole, to `src/parked/beyond.rs`: the floor, the bodies at true
scale, the ride and its stops, the posts, the unit numbers, the map, the list,
the sky, the kerbs and the catch. Its own 24 tests go with it and still run.

**Spec 0017's door**, which existed only to be the way through. The hidden
room's north wall is one piece again, with no opening, no frame, no sill and
no portal on either face, which is what it was before spec 0017 was written.

**What the building kept of it**, which is the part worth naming because none
of it was spec 0018's to begin with:

- `orrery::stands_off`, which was `beyond::stands_off`. The trail's sun
  station reads it and the ride was the only other caller.
- `orrery::galaxied`, which has said in its own doc since it was written that
  a galaxy wants the hidden room's floor and its lid, and was never called for
  either: its one caller was the sky out past the door. It is called for them
  now, so the galaxies stay in the building and the doc is true.
- The staged shot. `media/screenshot.png` was Saturn and Titan out past the
  door and three more pictures were the Earth, the Moon and Jupiter. The
  hall's own pose has been in `main.rs` all along, kept for its reasoning,
  with no file on the site to put it in. It has one now.

## What stays

**The trail is untouched.** Its five stations are the case, the pool, the
cube, getting behind the wall and reaching the sun, and the sun has been the
orrery's since 2026-11-21. Nothing in the bridge, the stone, the ledger or
what the building remembers between visits refers to any of this.

**`Room::solid` and `Room::building` are the same list now.** They were kept
apart because spec 0018's floor was in one and not the other. The pair stands
because every caller in the building asks for one by name.

## Acceptance criteria

- Nothing of the building reaches into the parked modules. —
  `room::tests::nothing_of_the_building_is_parked`
- The hidden room's far wall is solid all the way across, the middle
  included. — `behind::tests::the_far_wall_is_solid_all_the_way_across`
- The far corner of the field still has ground under it, which is the one
  claim of spec 0018's that was really about the field. —
  `tests::the_far_corner_of_the_field_is_still_the_field`
- And you still cannot get behind the building. —
  `tests::you_cannot_walk_behind_the_building`
- Every ground in the set is a box the building serves, with the slab beyond
  the door no longer among them. —
  `room::tests::every_ground_is_a_box_the_building_serves`
- The hidden room is drawn, galaxies and all. —
  `tests::what_is_behind_the_wall_is_drawn`

### Verified by hand

- Whether the hidden room reads better with galaxies on its floor and lid than
  with plain stars, which is the one visible change in here and the one thing
  no test can answer.
- Whether the building misses it.

## Out of scope

**What the door should have led to.** Nothing proposed cleared the test above,
and a second secret behind the first is worth having only when there is
something to put there. The wall can be opened again the day there is.

**Deleting any of it.** Parked means compiled and tested, for the reason
`src/parked/mod.rs` gives: a deleted room is recoverable and rotten, because
nothing builds it and the first engine change that touches its corner of the
API breaks it silently.
