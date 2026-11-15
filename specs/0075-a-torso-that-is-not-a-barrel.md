# 0075 A torso that is not a barrel

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Jake, 2026-11-14, having watched the people go about the village and the
steading all afternoon: the torsos of the people could look a lot better.

Everything on a figure here comes off one lathe, which is what spec 0033
settled and what makes the bodies cheap. A lathe turns a circle, so a torso
built on one is as deep front to back as it is wide across, and three things
followed from that which nobody had looked at since.

## What it does

**A torso is wider than it is deep.** `TORSO_DEEP` is 0.70, which is about
what a chest is: 0.32 across and 0.22 through. The limbs keep the round
section, because an arm and a leg are round. This is the whole of the
difference in profile: the same body that was a barrel from the side is a
person walking.

Done as a scale on the push rather than in the mesh, so one wheel still
serves the torso, the cloak and both bones of both limbs.

**An arm hangs where the body is widest.** `ARM_FROM` was 1.42, which is
`TORSO_FROM + TORSO_TALL` and so the very top of the lathe, where the
profile has already come in to 0.21 to let a neck through. The shoulder
hangs `ARMS_APART` off the middle, so every arm in this building stood
fourteen centimetres clear of the body it belonged to, with daylight
between. It is the torso's widest point now, worked out from the profile
rather than written down, and the arm sits against the body there.

**And the torso shuts at the bottom instead of tapering to one.** The line
that set it said the point it closes to is out of sight between the tops of
the legs. It is not: the legs stand `LEGS_APART` either side and there is a
clear view up between them, so a tenth of the torso's height spent going to
nothing is a cone where a body is widest. It closes in a thirtieth now,
which is a flat bottom nobody can see.

## What it does not do

- **It is still a lathe.** There is no chest that differs from a back, no
  shoulder blade, no collar and no hem. A surface of revolution cannot say
  which way it is facing, and what says that here is the face, the feet and
  the swing of the arms.
- **It does not touch the head, the neck or the limbs.** Those are round and
  a head and a limb are round.
- **The shoulder's shape is unchanged.** How the profile gets from its
  widest to the neck is a question about what a shoulder looks like, which
  is Jake's rather than arithmetic.

## Acceptance criteria

- An arm hangs against the body, at the body's widest. — `people::tests::an_arm_hangs_against_the_body`
- The torso shuts at the bottom rather than tapering to a point. — `people::tests::a_torso_is_not_a_cone_at_the_bottom`
- The figure still draws. — `tests::the_figure_is_drawn_close`

### Verified by hand

- A body reads as a person from the side rather than as a barrel. — run the
  arcade, walk out to the village and stand beside the street.
