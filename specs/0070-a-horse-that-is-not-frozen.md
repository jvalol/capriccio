# 0070 A horse that is not frozen

**Status:** implemented
**Date:** 2026-11-14

## What this is for

Spec 0067 gave the people in this village something to do while they are
standing. The horses at the rail did not get it, and `main.rs` said why:

> They stand. Spec 0067's gait is a two-legged walk and a horse that used it
> would be a person on all fours.

That argument is about walking and it still holds. Standing is not walking.
The horses are the last thing in the village that holds one pose for ever,
and they are stood next to people who breathe and change feet, which reads
worse than it did before 0067 went in.

Jake, 2026-11-14, picking it off a list of three: the horse.

## What it does

Four motions, each of them one of spec 0067's rules on a four-legged body,
and none of them a gait. `gesture::hitched` answers them all off the clock
and a number that says which horse, the way `gesture::standing` does.

**The head is the big one, where a person's is the weight shift.** A person
stands on two feet and manages it for a few seconds at a time, so what says
they are alive is the hip. A horse stands on four and is stable, so its
stance says very little and its head says everything: a metre of neck with a
head on the end, where two centimetres at the shoulder is ten at the muzzle.
It drops and comes back, on its own time, and swings a little off the body's
line under that.

The neck and the mane come with it for nothing, because both are already
derived from where the head is carried rather than written down again. One
number moves three pieces, which is the only reason the head is cheap enough
to be the big one.

**The tail swishes in bouts.** A tail swinging steadily is a pendulum, which
is the metronome 0067 warns about in its own words. A horse swishes at a fly:
two or three quick strokes that die away, then ten or twenty seconds of
nothing. The bouts come off `turn_of` like every other cycle here, so no two
are the same length, and the strokes inside one decay.

It swings about the horse's own forward and from its root rather than about
its middle, because a box turned about its centre takes its top off the rump
with it. At rest the construction gives back exactly the placement it
replaced.

**A hind leg rests.** This is the four-legged weight shift: one hind hoof
tipped back with the load off it, swapped every half a minute or so, with
both down in between. Only a hind leg, because a horse rests a hind leg; a
foreleg cocked the same way is a lame horse. The body drops a centimetre
while one is resting, which is small enough to be felt rather than seen.

**And it breathes**, five seconds to the cycle, six millimetres at the back.
Slower than the people, who are on 3.7, because a horse at rest is.

## What it does not do

- **No gait.** A hitched horse stays where it is. The animal is tied to a
  rail and the reason in `main.rs` for not giving it 0067's walk is
  untouched.
- **No head on the neck.** The head is carried on the neck's own line, so a
  horse that looks at something turns the whole neck, the way a figure here
  turns its whole body rather than its head. Same crudeness, same reason.
- **No ears, no nostrils, no blink.** Nothing here is finer than a box.
- **Nothing is driven by the world.** The tail swishes at a fly that is not
  there, nobody's hand is on the neck, and a horse tended by somebody on
  spec 0067's `Tend` errand does not know it.

## Acceptance criteria

- A hitched horse is never in the same pose twice running. — `gesture::tests::a_hitched_horse_is_never_still`
- Its head moves further than anything else on it. — `gesture::tests::a_horse_says_most_of_it_with_its_head`
- The tail is still most of the time and quick when it goes. — `gesture::tests::a_tail_swishes_in_bouts_rather_than_swinging`
- Only one hind leg is ever resting. — `gesture::tests::a_horse_rests_one_hind_leg_at_a_time`
- Two horses at one rail are not doing the same thing. — `gesture::tests::two_horses_are_not_the_same_horse`
- The horses are drawn. — `tests::the_village_horses_are_drawn`

### Verified by hand

- It reads as a horse standing rather than a horse twitching. — run the
  arcade, walk to the rail in the village and watch one for a minute.
