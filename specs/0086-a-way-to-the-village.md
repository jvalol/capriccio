# 0086 A way to the village

**Status:** implemented
**Date:** 2026-11-15

## Goal

Jake: the village seems sort of just randomly out there with no obvious path
to it from anywhere until it's already in view. Wear one, and stop the field
growing across the view down it.

## Behavior

Spec 0068 sited this place and wrote the reason down in the module header:
forty-four south is "near enough that you can see it from the gap and decide
to go". Standing in the gap and aiming straight at it says otherwise. A stand
of birch and conifer fills the whole line of sight, with one lit roof showing
through a gap, and under that seventy-two metres of undifferentiated grass.
`grep` for a track, a path, ruts or anything worn across `field.rs` returns
nothing.

The distance was never the problem and moving the village would not have
fixed it. Two things were missing, and they are separate:

**Something to follow.** `path` wears a line from two metres out of the gap
to where the street's own ruts run out, which is seven metres past the
street's east end. It ends there rather than at the street, so there is one
way of getting here rather than two things that nearly touch.

A footpath and not a cart track. The street's ruts are two lines at a wagon's
wheel spacing because a cart goes up and down it; nothing drives out to a gap
in a garden wall, so this is one band at `PATH_WIDE`, wide enough for two to
pass, and `RUT_WIDE` is deliberately not reused. It wanders for the same
reason the ruts do: a worn line perfectly straight for fifty metres is a road
somebody surveyed.

**A view down it.** `along_the_path` is the nearest point on the run rather
than a box round it, since a box round a fifty metre diagonal has most of a
field in it. `field::trees` and `field::cover` both honour it, beside the
keep-outs the bridge and the village already have. `PATH_CLEAR` is four
metres, which is the sightline's reach and not a crown's: the village's own
keep-out is a share wider than `around` so a trunk just outside does not hang
a crown over a roof, and that is a question about what is above you. This one
is a question about what you can see along. Crowns meeting overhead are
welcome.

Four and not two and a half, which was tried rather than reasoned about. Two
and a half keeps the run clear of trunks, which is all a footpath needs, and
left a conifer standing square across the view of the village from the gap.
The run and the sightline are not the same line: the run meets the street's
east end and the village's middle is most of thirty degrees west of it, so a
tree beside the path can still stand in front of the place. Four took that one
out. This clears the view by happening to rather than by aiming, and a second
keep-out along gap to middle would be two bare stripes crossing a field.

**Two reference pictures moved**, `field.png` and the four skies, because the
field they are of now has a path across it and two birches fewer. The field
one was looked at before it was blessed: the diff is the two trunks, the
undergrowth along the run, and the path itself, which shows in that frame as a
pale band leaving the gap. Six per cent of its pixels.

**Two things it got wrong on the way**, both written into the code where they
happened. The pieces are turned, because the gap is north east of the village
and nothing between them is square to anything, and the first build handed
`Quat::from_rotation_y` the angle `people::facing` uses. That one answers
which way a body looks and its forward is `(sin, 0, -cos)`; the rotation
sends a box's own x to `(cos, 0, -sin)`. Every piece came out laid across the
run instead of along it, a metre out of line at each join, and it read as the
wander being far too coarse. The wander was also too coarse, seeded on the
piece's number rather than the distance down the run, which is the kerb the
ruts' own comment already describes.

## Acceptance criteria

- The way there leaves the gap, ends where the ruts run out, and nothing the field sows stands on it. — `village::tests::nothing_grows_across_the_way_to_the_village`
- The village still keeps trees off itself, which this did not disturb. — `village::tests::nothing_is_sown_in_the_village`
- The field and the skies are drawn as the blessed references have them, path and all. — `tests::the_field_is_drawn`, `tests::the_skies_are_drawn`

### Verified by hand

- From the gap there is a path leading off and lit cabins at the end of it, where before there was a wall of trees: `./target/release/xanadu --done --pose "-18.85 0 15.15 1.55 309.4 -3.0 120.0" --picture-to /tmp/a.png`.
- Looking down the run itself, bearing 338.6, it reads as a worn path rather than a line of slabs.

## Out of scope

Anything at the far end of it. The path meets the ruts and stops; what makes
the village worth walking to is spec 0068's business and is unchanged.

The daylight case. Every picture here is the night the pose carries, and the
pose pins the clock, so the daylight view down the run is untested. What is
cleared is geometry rather than light, so it holds either way, but nobody has
looked.

A track anywhere else. The bridge's foot has no worn approach either and this
does not give it one.
