# 0088 The clip shows the way out

**Status:** implemented
**Date:** 2026-11-15

## Goal

Spec 0086 wore a path out of the gap because the village sat in the field with
nothing leading to it. The one film of this building then stood in that gap
and looked over the top of it: three beats out there and every one above the
horizon. Put the path in the clip.

## Behavior

A beat at the way out, before the bridge and the moon. Those two are turns
upward and the comment already in `clip` is right that a turn ending upward is
the better last thing to see of the field, so this goes first, being the one
that is not.

**Aimed at where the run begins, which is as far along it as the gap
allows.** The first build looked at the far end and filmed a wall. The stand
is `BACK_FROM_THE_WAY` inside the garden, the gap is 3.2 metres wide and the
village is south west, so the wedge the opening leaves is west and nothing
else. Twelve metres along the run is behind the garden's south wall, five is
too, and the arithmetic puts the limit near three. What the start gives is the
path leaving the opening and bending out of frame with the runestone and the
bridge behind it, which is the cue rather than the destination and is all this
beat can carry.

Walking out instead is not available, and `BACK_FROM_THE_WAY`'s own comment
says why: the field is not in `room::grounds`, so the planner lays no lattice
over it and a stop out there plans no way at all.

**Two numbers in `refresh-examples` moved, and neither is a new decision.**
The beat costs about three seconds, so the route went 54.2 to 56.5, measured
by `tests::the_clip_walks_the_whole_route` rather than off a take. Filming 58
left a second and a half of pool to end on where it used to leave nearly four,
so it films 60. And Jake's 28 second still landed back in the dark room it was
moved out of on 2026-11-14, so it comes from 31.5, the middle of the hall
stretch, which now runs 29.5 to 34.0. That is an attempt to keep the frame he
picked rather than to pick a different one.

**And a test that was missing.** Nothing caught the beat aimed at a wall. The
route tests passed, the clip filmed it, and what found it was rendering the
frame. `every_look_in_the_clip_has_something_to_look_at` asks whether anything
solid stands between the eye and what a `Look` names, which is the other half
of spec 0049's `every_look_in_the_clip_ends_with_what_it_names_in_shot`: that
one asks whether the turn points at the thing, this one whether the thing can
be seen. The wall case passes the first and fails the second, which was
checked by putting the fault back.

It stops the ray short of the target, the way `room::can_see_outdoors` does,
because a look is usually at a solid thing: the bookcase, the leaf, the cube.
What is asked is whether something stands between, not whether the thing
itself is solid.

**And one test caught the beat on its own.** `the_clip_looks_at_the_moon`
pinned the number of looks from that stand at two and then checked the moon by
index, so a third beat made it fail twice over. The count is three now and
still pinned, so a beat going missing is somebody's decision rather than a
silence. The moon is taken as the last look instead of a fixed index: the
route's own comment says it goes after the bridge because a turn ending upward
is the better last thing to see of the field, so last is the rule and the
number was a restatement of it.

## Acceptance criteria

- Every look in the clip has line of sight to what it names. — `tour::tests::every_look_in_the_clip_has_something_to_look_at`
- Every look in the clip ends with that thing in shot, which is the other half. — `tests::every_look_in_the_clip_ends_with_what_it_names_in_shot`
- The route still walks end to end, and says what it costs. — `tests::the_clip_walks_the_whole_route`
- The route still never stands inside a planet. — `tour::tests::the_clip_never_stands_inside_a_body`
- Three looks from the way out, the last of them the moon. — `tour::tests::the_clip_looks_at_the_moon`

### Verified by hand

- The beat shows the path leaving the gap: `./target/release/xanadu --film=clip --done --picture-for 12.7 --picture-to /tmp/a.png`.
- The still is still the hall: the same with `--picture-for 31.5`.

## Out of scope

Showing the village itself. It is south west of a west-facing opening and no
aim from this stand reaches it. That needs the route to leave the garden,
which needs the field in the planner's lattice.

The daylight case, as in spec 0086. Every frame here is the evening the film
opens in.
