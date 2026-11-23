//! The plasma globe on the pedestal beside the mirror ball. Spec 0122.
//!
//! **It stands beside spec 0064's ball rather than being it.** That ball is
//! `Scene::lens` with the index at `glass::MIRROR`, and that path returns the
//! room it traces times `glass::SILVER`: there is no transmission term, so
//! there is no inside for anything to glow from. Raising the tint would glow,
//! and what would glow is the whole reflection.
//!
//! **And nothing in here needs the tracer.** Glow is a colour channel past
//! one and see-through is an alpha under one, both of which this building
//! already draws in the hall's neon and the garden's pond. The ball is the
//! only thing that needs the tracer, so standing them together pays twice:
//! the mirror shows the glowing thing next to it, which is what a mirror is
//! for.

use blitzkit::collision::Aabb;
use blitzkit::mesh::Transform;
use blitzkit::renderer::scene::{MeshId, Scene};
use glam::{vec3, Quat, Vec3};

use crate::aim::Plain;

/// How far east of the ball's plinth the pedestal stands.
///
/// **Under half a metre on purpose.** This building's rule is 1.05 clear
/// wherever anybody walks, and the room skill's other half is that 0.6 is a
/// gap you can see through and cannot get to. So the two either stand far
/// enough apart to walk between or close enough to read as one group, and
/// there is no room in the walk for the first.
pub const APART: f32 = 0.30;

/// Which side of the ball's plinth each one stands on.
///
/// **Two, since Jake asked for the room to be symmetrical.** One globe
/// beside a ball is a ball with a thing next to it; one either side is a
/// row, and the ball is the middle of it rather than the odd one over.
pub const SIDES: [f32; 2] = [-1.0, 1.0];

/// The pedestal: how wide it is and how high it stands.
///
/// **Lower than the ball's plinth, which is the whole siting.** The ball's
/// middle is at eye height because walking at it is walking into it. This
/// one's is 1.17 over the floor, which is where a hand falls, because
/// reaching is how it is met.
pub const PEDESTAL: f32 = 0.52;
pub const PEDESTAL_UP: f32 = 0.95;

/// The shell, the electrode inside it, and the stem that carries it.
pub const SHELL: f32 = 0.22;
pub const CORE: f32 = 0.040;
pub const STEM: f32 = 0.010;

/// Where the pedestal stands, derived off the ball's plinth so that moving
/// one moves the other.
pub fn pedestal(reaches: f32, side: usize) -> Aabb {
    let plinth = crate::glass::plinth(reaches);
    let middle = plinth.center();
    let from = if SIDES[side] > 0.0 {
        plinth.max.x + APART
    } else {
        plinth.min.x - APART - PEDESTAL
    };

    Aabb::new(
        vec3(from, plinth.min.y, middle.z - PEDESTAL * 0.5),
        vec3(
            from + PEDESTAL,
            plinth.min.y + PEDESTAL_UP,
            middle.z + PEDESTAL * 0.5,
        ),
    )
}

/// The globe: where its middle is and how big it is.
pub fn globe(reaches: f32, side: usize) -> (Vec3, f32) {
    let stands = pedestal(reaches, side);
    let at = stands.center();

    (vec3(at.x, stands.max.y + SHELL, at.z), SHELL)
}

/// What a body walks into, which is the pedestal and nothing else: the shell
/// is 0.22 inside a footprint of 0.26, so it never stands out past what is
/// holding it.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    (0..SIDES.len())
        .map(|side| pedestal(reaches, side))
        .collect()
}

/// How many filaments there are and how many links each is drawn in.
pub const FILAMENTS: usize = 16;
pub const LINKS: usize = 18;

/// How thick one is at the electrode, and how much of that is left at the
/// glass.
///
/// **Thin, tapered, and in more links than seems necessary.** A link is a
/// box, so a link as thick as it is long is a cube and a bolt of cubes is a
/// white starfish, which is what the first build drew. A thread of one
/// thickness end to end is the next thing along and is what Jake saw: a
/// glowing tentacled monster rather than an electric orb. A discharge is
/// widest where it leaves the electrode and comes to nothing at the glass,
/// and that taper is most of what tells the two apart.
pub const THREAD: f32 = 0.0050;
pub const THINS_TO: f32 = 0.12;

/// And how much of its brightness is left there, for the same reason.
pub const DIMS_TO: f32 = 0.35;

/// And how quickly it goes from white to violet, as a multiple of its run.
///
/// Over two, so it is the colour of the glass end by the time it is less
/// than half way there. Graded evenly it was a white tube with a violet tip,
/// where the photograph is a white root on a violet thread.
pub const COOLS_BY: f32 = 2.4;

/// How wide the fan of them opens off the line to the eye, as a half angle.
///
/// **A cone and not a pull.** The first way to write this was to lean each
/// filament from where it would wander toward where the eye is, which has
/// two settings and no good one: weighted to the eye, all five point the
/// same way and the globe reads as a torch, and weighted away, a filament
/// can come out facing the far side and the thing it is meant to be doing
/// stops being true. Built as a cone off the eye's own line, every one of
/// them leans toward whoever is there by construction, and how far it leans
/// is what wanders. Eighty degrees, which fans them over most of the near
/// face.
pub const FAN: f32 = 1.5;

/// How much of that fan a keen arm gives up to point nearly at the eye.
///
/// **Because the reach was true and could not be seen.** Every arm leans
/// toward whoever is there, by construction and under test, and Jake walked
/// round it and asked for the arms to reach toward where he was standing.
/// The fan is eighty-six degrees, which is most of a hemisphere, so the
/// bunch does swing as you move and what you see is a starfish either way.
///
/// So the arms are not all alike in this. Each keeps a keenness of its own,
/// squared off its own dice so that most of them are dull and a few are not,
/// and a keen one gives up four fifths of the fan and comes nearly to you
/// while the rest spread as before. Enough of sixteen to read as a handful
/// coming to your hand, few enough to leave the globe a globe.
pub const KEENEST: f32 = 0.82;

/// How quick one arm's own writhing is, between nought and one.
///
/// **Off a scatter, and never off the arm's seed.** That seed carries which
/// globe the arm stands on, which is a number in the hundreds, and five
/// places were multiplying time by it: the east globe's threads writhed six
/// times faster than the west's and its sparks flickered at forty a second.
/// Jake, watching the pair: the right one is a lot more jittery than the
/// left one. A seed is for seeding. Anything that multiplies time has to be
/// bounded, and this is.
pub fn quickness(which: usize, side: usize) -> f32 {
    scatter(of_the_arm(which, side) + 41.9)
}

/// How keenly one arm favours the eye, which is a thing about the arm and
/// not about the moment: a keen one is keen for as long as it is lit.
pub fn keenness(which: usize, side: usize) -> f32 {
    let of = scatter(of_the_arm(which, side) + 29.3);

    of * of
}

/// How far a filament wanders off its own line, as a share of the shell, and
/// how far round that wander turns from one link to the next.
///
/// **Two of each, because one is either a curve or a sawtooth.** At 0.22 of
/// the shell and 1.9 radians a link the wander swung a hundred and nine
/// degrees between one link and the next and a filament was a run of hard
/// right angles: a lightning bolt in a comic. At 0.09 and 0.6 it is a smooth
/// helix, which Jake read at once as a glowing tentacled monster, because a
/// smooth glowing curve of one thickness is a tentacle. One turn rate gives
/// one or the other and nothing in between.
///
/// A spark is a line that wanders slowly and crinkles quickly at the same
/// time, so there are two terms: a wide slow one that bends the whole thread
/// and a small fast one laid on it. Both are nought at the electrode and
/// nought at the glass, so a filament starts and ends exactly where it is
/// meant to and only its run is crooked.
pub const WANDER: f32 = 0.022;
pub const WANDERS_BY: f32 = 0.55;
pub const CRINKLE: f32 = 0.007;
pub const CRINKLES_BY: f32 = 2.4;

/// How long an arm's life beat is: how often it is thrown for whether it is
/// lit at all.
///
/// **Not the same clock as its movement, which is the point.** Jake, on the
/// version where it was: the arms should move erratically, not in sync with
/// each other. An arm whose coming and going and whose crawling share one
/// clock does both on the same stride, and seven of those are one thing
/// happening seven times. Two clocks per arm, each with its own rate and its
/// own offset, is where the variety comes from.
pub const HOLDS: f32 = 0.40;

/// How fast an arm crawls round the inside of the glass, in radians a
/// second, before its own share of it.
///
/// **Crawling and not only jumping.** Jake again: they should crawl more
/// than jump across the inner surface, though some jumping is good too, and
/// in general more variety in how they move. What was here held an arm
/// still for a beat and snapped it somewhere else, which is all jump and no
/// crawl, and every arm did it at the same size of stride.
///
/// Each arm takes its own share of this and its own direction, so some creep
/// and some travel, and two next to each other are not going the same way.
pub const CRAWLS_AT: f32 = 0.70;

/// How often an arm jumps anyway, in seconds, and how long a jump takes.
///
/// A jump is rare against the crawl: a few seconds apart, each arm on its
/// own era, so what you mostly see is creeping and what you sometimes see is
/// a strike landing somewhere else. The jump itself is a tenth of a second,
/// because anything slower is a sweep and the whole argument against the
/// first version was that it swept.
pub const ERA: f32 = 1.5;
pub const JUMPS_IN: f32 = 0.09;

/// How far an arm's lean sways on its own, as a share of the fan, and how
/// fast. Its own share of each, so one arm breathes in and out of the middle
/// while another sits near the rim.
pub const SWAYS: f32 = 0.26;
pub const SWAYS_AT: f32 = 0.34;

/// How often an arm goes out for a beat instead of striking again, and how
/// much of a beat the striking and the dying take.
///
/// **Because jumping about is not the same as coming and going.** Jake:
/// there should be arms that pop in and out of existence in addition to
/// jumping around the sphere. Seven arms that are always there and only
/// ever move are seven arms, and what the thing does is strike, hold, and
/// stop, with the next one somewhere else. The count is what should waver,
/// not only the places.
///
/// Under a fifth, which leaves eight of the ten lit on average and still
/// gives runs of two and three dark often enough to notice. It was nearly a
/// third of seven, which Jake read as too few arms. Both ramps are short,
/// because a filament that fades in over half a second is a lamp on a
/// dimmer: what reads as a strike is a frame or two.
pub const GOES_OUT: f32 = 0.18;
pub const STRIKES_IN: f32 = 0.05;
pub const DIES_IN: f32 = 0.09;

/// A number between nought and one from a number.
///
/// How an arm gets somewhere different on every jump without anything being
/// kept: the whole globe is worked out from the time and the eye, per spec
/// 0122, so what stands in for a dice is a function of which arm and which
/// jump. Not a good hash and it does not need to be one. It needs to be
/// unlike its neighbour, which the multiply and the fraction see to.
/// The number an arm's own dice are drawn off: which arm it is, and which
/// globe it is on.
///
/// **Both of them, since there are two globes.** Seeded off the arm number
/// alone, the pair would be one globe drawn twice, and they stand a metre
/// and a half apart in plain sight of each other.
fn of_the_arm(which: usize, side: usize) -> f32 {
    which as f32 + side as f32 * 101.7
}

pub fn scatter(of: f32) -> f32 {
    let x = (of * 12.9898 + 78.233).sin() * 43758.547;

    x - x.floor()
}

/// How far back a lit run is followed to find where it began.
///
/// Bounded, because an arm lit for a thousand beats is still only asking
/// where this run started and a walk with no end is a frame that never
/// finishes. Past the bound every step answers the same thing rather than
/// a different one each time, or a very long run would re-seed itself every
/// beat, which is the jumping this is here to avoid.
pub const RUNS_BACK: u32 = 64;

/// Which beat an arm's current lit run began on.
///
/// What it is for is that an arm which has just struck should come back
/// somewhere new rather than where the last one went out. Inside a run it
/// does not move, so the arm crawls on from where it is.
pub fn struck_at(which: usize, side: usize, step: f32) -> f32 {
    let mut at = step;

    for _ in 0..RUNS_BACK {
        if !alight(which, side, at - 1.0) {
            return at;
        }

        at -= 1.0;
    }

    0.0
}

/// Where one arm stands now, as how far it leans off the line to the eye and
/// where round that line it sits.
///
/// Three things in one: where this lit run started, a crawl from there at
/// this arm's own rate and in its own direction, and the kick of whichever
/// jump era it is in. Never past the fan, whatever any of them say, which is
/// what keeps every arm on the side the eye is on.
pub fn aim(which: usize, side: usize, since: f32) -> (f32, f32) {
    let n = of_the_arm(which, side);
    let round = std::f32::consts::TAU;
    let run = struck_at(which, side, clock(which, side, since).0);
    let seed = n * 31.7 + run * 7.13;

    // where the run began
    let from_roll = scatter(seed) * round;
    let from_lean = 0.18 + 0.78 * scatter(seed + 3.7);

    // and how it creeps from there: its own rate, its own way round
    let rate = CRAWLS_AT
        * (0.11 + 0.89 * scatter(n + 2.3))
        * if scatter(n + 5.9) > 0.5 { 1.0 } else { -1.0 };

    // the jumps it makes while it is lit, each arm on its own era
    let era = ERA * (0.6 + 1.2 * scatter(n + 7.1));
    let tick = since / era + scatter(n + 13.7) * 5.0;
    let mark = tick.floor();
    let into = (tick - mark) * era;
    let frac = (into / JUMPS_IN).min(1.0);
    let eased = 1.0 - (1.0 - frac) * (1.0 - frac);

    let (was_lean, was_roll) = kick(which, side, mark - 1.0);
    let (now_lean, now_roll) = kick(which, side, mark);
    // the short way round, or a kick at six radians crossing to one at a
    // tenth sweeps the whole circle to get there
    let turn = (now_roll - was_roll + round * 0.5).rem_euclid(round) - round * 0.5;

    // and the sway, which is this arm's own breathing in and out of the fan
    let sway = (since * SWAYS_AT * (0.4 + 1.2 * scatter(n + 17.1)) + n * 1.7).sin()
        * SWAYS
        * (0.25 + 0.75 * scatter(n + 19.3));

    // and how near the line to the eye this one keeps, which is the whole
    // of what makes the reaching legible
    let tight = 1.0 - KEENEST * keenness(which, side);
    let lean =
        ((from_lean + was_lean + (now_lean - was_lean) * eased + sway) * tight).clamp(0.06, 1.0);
    let roll = from_roll + rate * since + was_roll + turn * eased;

    (FAN * lean, roll)
}

/// What one jump moves an arm by: a shove round the line and a nudge in or
/// out of the middle.
fn kick(which: usize, side: usize, mark: f32) -> (f32, f32) {
    let seed = of_the_arm(which, side) * 23.9 + mark * 11.37;

    (
        (scatter(seed) - 0.5) * 0.5,
        scatter(seed + 9.1) * std::f32::consts::TAU,
    )
}

/// An arm's own clock: which beat it is in, and how far through that beat.
///
/// Its own function because three things ask it, and an arm whose shape,
/// whose jump and whose life disagreed about which beat it was in would
/// strike in one place and light up in another.
pub fn clock(which: usize, side: usize, since: f32) -> (f32, f32) {
    let n = of_the_arm(which, side);
    let beat = HOLDS * (0.55 + 0.9 * scatter(n + 0.5));
    let tick = since / beat + scatter(n + 11.3) * 7.0;
    let step = tick.floor();

    (step, tick - step)
}

/// Whether an arm is lit at all on one of its beats, and by how much it
/// cleared the threshold, which is what settles ties when none of them did.
pub fn alight(which: usize, side: usize, step: f32) -> bool {
    stayed(which, side, step) > GOES_OUT
}

fn stayed(which: usize, side: usize, step: f32) -> f32 {
    scatter(of_the_arm(which, side) * 31.7 + step * 7.13 + 19.4)
}

/// And how brightly, now: nought when it is out, and the ramps at either end
/// of a beat it is lit for.
///
/// **The dying ramp asks the next beat, not this one.** An arm that is lit
/// again next beat does not go out in between, it jumps, and fading it down
/// and back up at every jump would be seven things blinking in time with
/// their own movement. It only dies when there is nothing to jump to.
pub fn burning(which: usize, side: usize, since: f32) -> f32 {
    let mine = on_its_own(which, side, since);

    if mine > 0.0 {
        return mine;
    }

    // **An arm cannot go out if it is the last one.** Each arm's life is its
    // own throw and nothing in seven independent throws stops all seven
    // coming up dark: at `GOES_OUT` it happened inside ten minutes, and a
    // plasma globe that blinks off is a broken plasma globe. So when nothing
    // at all is lit, the arm that came nearest to staying strikes instead.
    // It arrives at once rather than ramping, which is what a strike is.
    if (0..FILAMENTS).any(|other| other != which && on_its_own(other, side, since) > 0.0) {
        return 0.0;
    }

    let nearest = (0..FILAMENTS)
        .max_by(|one, two| {
            let of = |it: &usize| stayed(*it, side, clock(*it, side, since).0);

            of(one).total_cmp(&of(two))
        })
        .unwrap_or(0);

    if nearest == which {
        1.0
    } else {
        0.0
    }
}

/// What an arm would be doing with no regard for the others.
fn on_its_own(which: usize, side: usize, since: f32) -> f32 {
    let (step, through) = clock(which, side, since);

    if !alight(which, side, step) {
        return 0.0;
    }

    let up = (through / STRIKES_IN).min(1.0);
    let down = if alight(which, side, step + 1.0) {
        1.0
    } else {
        ((1.0 - through) / DIES_IN).min(1.0)
    };

    up.min(down)
}

/// One filament's links, from the electrode out to the shell.
///
/// **Worked out from the time and the eye, so there is nothing to keep.**
/// Spec 0120 keeps the trail and nothing else, on the argument that a toy
/// you come back to is a toy as it was built, and this is a toy.
///
/// The bow is nought at both ends and widest in the middle, so a filament
/// starts at the electrode and lands on the glass exactly while only its
/// run is crooked. That keeps it inside the shell as well: the furthest any
/// link gets from the middle is the shell times the square root of one plus
/// `BOW` squared at the widest, which is under the shell.
pub fn filament(
    reaches: f32,
    which: usize,
    side: usize,
    eye: Vec3,
    since: f32,
) -> [Vec3; LINKS + 1] {
    let (middle, shell) = globe(reaches, side);
    let toward = {
        let out = (eye - middle).normalize_or_zero();

        if out == Vec3::ZERO {
            Vec3::Y
        } else {
            out
        }
    };
    let (across, other) = frame(toward);

    let n = of_the_arm(which, side);
    // where it stands: crawling on from where this run struck, with the odd
    // jump laid over it. Its own rate, its own way round and its own era, so
    // no two arms are doing the same thing at the same time.
    let (lean, roll) = aim(which, side, since);
    let along = toward * lean.cos() + (across * roll.cos() + other * roll.sin()) * lean.sin();

    let mut links = [Vec3::ZERO; LINKS + 1];

    for (k, link) in links.iter_mut().enumerate() {
        let t = k as f32 / LINKS as f32;
        let held = (t * std::f32::consts::PI).sin() * shell;
        // the thread's own shape is re-struck when the arm strikes, so a
        // fresh arm is a new discharge rather than the old one moved
        let struck = scatter(n * 5.1 + struck_at(which, side, clock(which, side, since).0))
            * std::f32::consts::TAU;
        let quick = quickness(which, side);
        let slow = since * (3.2 + 7.5 * quick) + k as f32 * WANDERS_BY + struck;
        let fast = since * (7.9 + 13.5 * quick) + k as f32 * CRINKLES_BY + struck * 2.0;
        let off = (across * slow.cos() + other * slow.sin()) * (held * WANDER)
            + (across * fast.cos() + other * fast.sin()) * (held * CRINKLE);

        // out of the electrode's own face rather than its middle, since
        // the electrode is a ball you can see and not a point
        *link = middle + along * (CORE + t * (shell - CORE)) + off;
    }

    links
}

/// The spark where an arm meets the glass: how big its core is and how big
/// the bloom round it, both as a share of the shell, and how much of the
/// bloom you see through.
///
/// **Jake: there should be a gentle spark against the glass at the end of
/// each arm.** A filament that thins away and simply stops is a line that
/// ran out. Where a discharge meets glass it spreads, and that is the one
/// place along an arm that is wider than the thread rather than narrower.
///
/// Two spheres and not one, because a hard dot is not gentle and a soft edge
/// has to be built here rather than asked for: a small bright core with a
/// see-through bloom round it, which is the same trick the garden's lanterns
/// are under one room along. Both sit far enough in that the bloom is
/// tangent to the glass rather than through it.
pub const SPARK: f32 = 0.007;
pub const BLOOM: f32 = 0.030;
pub const BLOOM_THROUGH: f32 = 0.07;

const _: () = assert!(SPARK < BLOOM, "the spark's core is wider than its bloom");

/// The same for any point on the glass: pulled in along its own radius by the
/// bloom, so nothing of the spark stands outside.
pub fn pulled_in(reaches: f32, side: usize, tip: Vec3) -> Vec3 {
    let (middle, shell) = globe(reaches, side);
    let out = (tip - middle).normalize_or_zero();

    middle + out * (shell * (1.0 - BLOOM))
}

/// A frame about a line: two directions square to it and to each other.
///
/// Its own function because three places want one, and the cross product of
/// a line with itself is nothing, so each of them has to pick the axis the
/// line is least like before it can take one.
fn frame(along: Vec3) -> (Vec3, Vec3) {
    let line = along.normalize_or_zero();
    let line = if line == Vec3::ZERO { Vec3::Y } else { line };
    let across = if line.y.abs() < 0.9 {
        line.cross(Vec3::Y).normalize()
    } else {
        line.cross(Vec3::X).normalize()
    };

    (across, line.cross(across))
}

/// How far a line leaving a point inside the globe runs before it meets the
/// glass.
///
/// **Asked of the ray and not of the middle.** A fork off the outer end of
/// a branch is nowhere near radial, so landing it at the shell's radius in
/// its own direction from the middle can put it behind where it started.
fn to_the_glass(middle: Vec3, shell: f32, from: Vec3, out: Vec3) -> f32 {
    let p = from - middle;
    let b = p.dot(out);

    -b + (b * b + shell * shell - p.length_squared()).max(0.0).sqrt()
}

/// One limb of an arm: the links it is drawn in, how deep in the tree it
/// stands, which limb it left, and how thick and how bright it is where it
/// left it.
#[derive(Clone)]
pub struct Limb {
    pub links: Vec<Vec3>,
    pub deep: usize,
    pub thick: f32,
    pub lit: f32,
}

/// How many times a fork may fork again, and how many links a limb of each
/// generation is drawn in.
///
/// **Every arm in the real thing forks, and the forks fork.** Jake's video
/// of one: not a single filament in it runs from the electrode to the glass
/// without splitting, and several split twice. One branch per arm is a tree
/// of depth one, which is sixteen copies of the same shape, and Jake said so
/// the moment he saw it: the arms fork, but they move with the same fork.
///
/// Three generations and no more, because the fourth is shorter than a link
/// and costs a hundred boxes a globe to draw a hair.
pub const DEEPEST: usize = 2;
pub const LIMB_LINKS: [usize; DEEPEST + 1] = [LINKS, 8, 5];

/// Where along its parent a fork leaves, and how wide the window it leaves
/// in.
///
/// It leaves in the outer half, because a fork at the root is two arms and
/// not one.
pub const FORKS_AT: f32 = 0.42;
pub const FORK_WINDOW: f32 = 0.40;

/// How far off its parent's run a fork goes, and how often a limb throws a
/// second fork rather than one.
pub const FORK_SPREAD: f32 = 0.55;
pub const FORKS_TWICE: f32 = 0.55;

/// How fast a fork swings round its parent, in radians a second, before its
/// own share of it.
///
/// **Its own share, which is the half Jake was looking at.** Every fork
/// swinging at one rate is a rigid tree: the shape was right and the motion
/// was one piece turning, which is the other way of saying they move with
/// the same fork. Its own rate and its own way round, and the angle between
/// two forks off one parent opens and closes instead of holding.
pub const FORK_SWINGS: f32 = 0.5;

/// How much of its parent's thickness a fork keeps, since a discharge that
/// splits divides what it was carrying.
pub const FORK_KEEPS: f32 = 0.72;

/// One arm, as a tree: the trunk, what it forks into, and what those fork
/// into in their turn.
///
/// Seeded off the same lit run as the trunk, so the whole tree is re-struck
/// when the arm strikes rather than wandering on through a new discharge.
pub fn arm(reaches: f32, which: usize, side: usize, eye: Vec3, since: f32) -> Vec<Limb> {
    let (middle, shell) = globe(reaches, side);
    let run = struck_at(which, side, clock(which, side, since).0);
    let root = of_the_arm(which, side) * 41.3 + run * 13.9;
    let quick = quickness(which, side);

    let mut limbs = vec![Limb {
        links: filament(reaches, which, side, eye, since).to_vec(),
        deep: 0,
        thick: THREAD,
        lit: 1.0,
    }];

    // breadth first, so a limb is grown before anything it grows, and its
    // own place in the tree seeds it: no two forks of one arm are thrown
    // the same dice
    let mut n = 0;

    while n < limbs.len() {
        if limbs[n].deep < DEEPEST {
            let young = forks(
                middle,
                shell,
                &limbs[n],
                root + n as f32 * 7.73,
                quick,
                since,
            );

            limbs.extend(young);
        }

        n += 1;
    }

    limbs
}

/// The forks off one limb: one or two, each leaving somewhere in its
/// parent's outer run and each landing on the glass like its parent.
fn forks(middle: Vec3, shell: f32, parent: &Limb, seed: f32, quick: f32, since: f32) -> Vec<Limb> {
    let how_many = if scatter(seed + 3.1) < FORKS_TWICE {
        2
    } else {
        1
    };

    (0..how_many)
        .map(|c| {
            let own = seed + 17.33 * (c as f32 + 1.0);
            let last = parent.links.len() - 1;
            let at =
                (((FORKS_AT + FORK_WINDOW * scatter(own)) * last as f32) as usize).min(last - 1);
            let from = parent.links[at];
            let t = at as f32 / last as f32;

            // off what is left of its parent's run, so a fork forks the limb
            // it is on rather than the arm it came from
            let (across, other) = frame(parent.links[last] - from);
            let along = (parent.links[last] - from).normalize_or_zero();
            let swings = FORK_SWINGS
                * (0.35 + 0.65 * scatter(own + 11.1))
                * if scatter(own + 23.7) > 0.5 { 1.0 } else { -1.0 };
            let roll = scatter(own + 5.3) * std::f32::consts::TAU + since * swings;
            let spread = FORK_SPREAD * (0.5 + 0.5 * scatter(own + 8.7));
            let out = (along * spread.cos()
                + (across * roll.cos() + other * roll.sin()) * spread.sin())
            .normalize_or_zero();
            let lands = from + out * to_the_glass(middle, shell, from, out);

            let links = LIMB_LINKS[parent.deep + 1];
            let mut path = vec![Vec3::ZERO; links + 1];

            for (k, link) in path.iter_mut().enumerate() {
                let s = k as f32 / links as f32;
                let held = (s * std::f32::consts::PI).sin() * shell;
                let slow = since * (2.9 + 6.0 * quick) + k as f32 * WANDERS_BY + own;
                let fast = since * (6.1 + 9.0 * quick) + k as f32 * CRINKLES_BY + own * 2.0;

                *link = from.lerp(lands, s)
                    + (across * slow.cos() + other * slow.sin()) * (held * WANDER)
                    + (across * fast.cos() + other * fast.sin()) * (held * CRINKLE);
            }

            Limb {
                links: path,
                deep: parent.deep + 1,
                thick: parent.thick * (1.0 - (1.0 - THINS_TO) * t) * FORK_KEEPS,
                lit: parent.lit * (1.0 - (1.0 - DIMS_TO) * t),
            }
        })
        .collect()
}

/// The light it gives off, and how far.
///
/// **In a fitting, by this building's rule, and the fitting is the globe.**
/// Every lamp in here sits in something that is drawn as the thing the light
/// comes from, and a glowing filament is that by definition. Short ranged,
/// because what it is lighting is the pedestal under it and the ball's
/// plinth beside it, and the room already has fourteen bay lamps.
pub const LAMP_COLOUR: Vec3 = vec3(0.68, 0.30, 0.86);
pub const LAMP_LIT: f32 = 0.55;
pub const LAMP_RANGE: f32 = 1.6;

/// Where that lamp hangs, which is the globe's own middle.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    (0..SIDES.len())
        .map(|side| globe(reaches, side).0)
        .collect()
}

/// What the ball sees of it, since the tracer reads a box's shape and would
/// call a pedestal a frame on a wall.
///
/// The box is `pedestal` itself, asked by name, so there is nothing here to
/// drift from it. The globe is a lathe and the filaments are a hand's width,
/// and neither is in `solid`, so neither is traced.
pub fn named(reaches: f32) -> Vec<(Aabb, glam::Vec4)> {
    solid(reaches)
        .into_iter()
        .map(|box_| (box_, crate::aim::GLASS_TRIM))
        .collect()
}

/// The pedestal as it is drawn, in the courses the ball's plinth is cut in.
///
/// One function for both, because how a plinth in this room is cut is one
/// question and not two.
///
/// **And its own box is not drawn, which is the whole of spec 0122's one
/// visible fault.** `glass::draw` draws every box in that room's `solid`
/// list, less the handful it names, and this one went into that list the day
/// the pedestal did. So the box was drawn as a plain cube and these courses
/// were drawn over it, and the die's side faces landed exactly on the cube's:
/// two exposed faces in one plane, which is the fault this building has now
/// found eight times. What it looked like was a checkerboard of lit and
/// shadowed squares with a stair-stepped diagonal through it, which reads
/// like a shadow map and is not one. The plinth beside it is filtered out of
/// that loop by name and has been since it was built.
pub fn pieces(reaches: f32, side: usize) -> Vec<(Aabb, glam::Vec4)> {
    crate::glass::plinth_courses(pedestal(reaches, side))
}

/// Draws the pedestal, the globe and whatever the filaments are doing.
pub fn draw(
    scene: &mut Scene,
    reaches: f32,
    since: f32,
    eye: Vec3,
    cube: MeshId,
    sphere: Option<MeshId>,
) {
    for side in 0..SIDES.len() {
        draw_one(scene, reaches, side, since, eye, cube, sphere);
    }
}

/// One of them: its stand, its stem, its arms and their sparks, its electrode
/// and the glass over the lot.
fn draw_one(
    scene: &mut Scene,
    reaches: f32,
    side: usize,
    since: f32,
    eye: Vec3,
    cube: MeshId,
    sphere: Option<MeshId>,
) {
    let stands = pedestal(reaches, side);

    for (box_, colour) in pieces(reaches, side) {
        scene.push_plain(
            cube,
            &Transform::at(box_.center()).with_scale(box_.size()),
            colour,
        );
    }

    let (middle, shell) = globe(reaches, side);

    // the stem, from the stand's top up to the electrode
    let foot = stands.max.y;
    let stem = (middle.y - CORE) - foot;

    scene.push_plain(
        cube,
        &Transform::at(vec3(middle.x, foot + stem * 0.5, middle.z))
            .with_scale(vec3(STEM, stem, STEM)),
        crate::aim::PLASMA_STEM,
    );

    // the filaments, which are what the thing is. Each link is thinner and
    // dimmer than the one before it, so a filament leaves the electrode and
    // comes to nothing at the glass rather than arriving there as thick as
    // it set out, and an arm that is out this beat is not drawn at all.
    for which in 0..FILAMENTS {
        let burning = burning(which, side, since);

        if burning <= 0.0 {
            continue;
        }

        // a tree and not a thread: the trunk forks, what it forks into forks
        // again, and every one of them is thrown its own dice
        let limbs = arm(reaches, which, side, eye, since);
        let n = of_the_arm(which, side);
        let breath = 0.85 + 0.15 * (since * (2.1 + 5.5 * quickness(which, side)) + n * 2.9).sin();

        for limb in &limbs {
            let last = limb.links.len() - 1;

            for (k, pair) in limb.links.windows(2).enumerate() {
                let (from, to) = (pair[0], pair[1]);
                let span = to - from;
                let long = span.length();

                if long <= f32::EPSILON {
                    continue;
                }

                let t = (k as f32 + 0.5) / last as f32;
                let thick = limb.thick * (1.0 - (1.0 - THINS_TO) * t);
                let lit = limb.lit * (1.0 - (1.0 - DIMS_TO) * t) * burning;
                // white where it leaves the electrode and violet well before
                // the glass, because what is hottest is whitest and what is
                // white in the photograph is the first fifth. Read off how
                // far out the link is and not off how far along its own limb
                // it is, or a fork would start white half way to the glass.
                let out = ((from + to) * 0.5 - middle).length() / shell;
                let hue = crate::aim::PLASMA_HOT
                    .truncate()
                    .lerp(crate::aim::PLASMA.truncate(), (out * COOLS_BY).min(1.0));

                scene.push_fitting(
                    cube,
                    &Transform::at((from + to) * 0.5)
                        .with_rotation(Quat::from_rotation_arc(Vec3::Y, span / long))
                        .with_scale(vec3(thick, long, thick)),
                    (hue * lit).extend(1.0),
                );
            }

            // and the spark where it lands, which breathes a little rather
            // than sitting there: the one place on a limb that is a point
            // and not a line, so a steady one reads as a bead stuck to the
            // glass. Sized off the root of its thickness, since a twig a
            // sixth as thick as the trunk wants a spark you can still see.
            let Some(sphere) = sphere else {
                continue;
            };
            let at = pulled_in(reaches, side, limb.links[last]);
            let size = (limb.thick / THREAD).sqrt();

            scene.push_fitting(
                sphere,
                &Transform::at(at).with_scale(Vec3::splat(shell * SPARK * 2.0 * breath * size)),
                (crate::aim::PLASMA.truncate() * burning).extend(1.0),
            );
            scene.push_fitting(
                sphere,
                &Transform::at(at).with_scale(Vec3::splat(shell * BLOOM * 2.0 * breath * size)),
                (crate::aim::PLASMA.truncate() * burning * 0.55).extend(BLOOM_THROUGH),
            );
        }
    }

    let Some(sphere) = sphere else {
        return;
    };

    // the electrode, and then the shell over it, which is see-through and so
    // is drawn after everything inside it
    scene.push_fitting(
        sphere,
        &Transform::at(middle).with_scale(Vec3::splat(CORE * 2.0)),
        crate::aim::PLASMA_CORE,
    );
    // **With a highlight on it, which is the whole of what says glass.**
    // Drawn plain and nearly clear it was not there at all: the filaments
    // hung in the air and the thing read as a sparkler rather than something
    // under a dome. What a hot spot off the bay lamps gives it is a rim and
    // a curve.
    scene.push_material(
        sphere,
        &Transform::at(middle).with_scale(Vec3::splat(shell * 2.0)),
        crate::aim::PLASMA_GLASS,
        crate::aim::MATTE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// A ring of places to stand, so every test that asks about the eye asks
    /// from all round rather than from the one spot that happens to work.
    fn all_round(reaches: f32, side: usize) -> Vec<Vec3> {
        let (middle, _) = globe(reaches, side);

        (0..16)
            .map(|n| {
                let turn = n as f32 / 16.0 * std::f32::consts::TAU;

                middle + vec3(turn.cos() * 1.4, 0.38, turn.sin() * 1.4)
            })
            .collect()
    }

    /// Spec 0122: every filament runs from the electrode's face to the shell
    /// and stays inside it.
    ///
    /// **Both ends exactly, which is what the wander is shaped for.** A
    /// crooked bolt is drawn by pushing its middle off its own line, and
    /// the obvious way to do that leaves the ends pushed off too: one
    /// hanging in the air beside the electrode and one through the glass.
    /// Both terms are a sine over the run, so each is nought at nought and
    /// nought at one.
    ///
    /// Its face and not its middle, since Jake's photograph of the real
    /// thing: the electrode is a ball a quarter of the globe across, and a
    /// filament that starts at the middle of one spends its widest, hottest
    /// quarter buried inside it.
    #[test]
    fn every_filament_runs_from_the_core_to_the_glass() {
        let reaches = reaches();

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);

            for eye in all_round(reaches, side) {
                for tenths in 0..40 {
                    let since = tenths as f32 * 0.37;

                    for which in 0..FILAMENTS {
                        let links = filament(reaches, which, side, eye, since);

                        assert!(
                            (links[0].distance(middle) - CORE).abs() < 1e-5,
                            "filament {} starts {} out and the electrode is {}",
                            which,
                            links[0].distance(middle),
                            CORE
                        );
                        assert!(
                            (links[LINKS].distance(middle) - shell).abs() < 1e-4,
                            "filament {} ends {} from the middle and the shell is {}",
                            which,
                            links[LINKS].distance(middle),
                            shell
                        );

                        for (k, link) in links.iter().enumerate() {
                            assert!(
                                link.distance(middle) <= shell + 1e-4,
                                "link {} of filament {} is {} out and the shell is {}",
                                k,
                                which,
                                link.distance(middle),
                                shell
                            );
                        }
                    }
                }
            }
        }
    }

    /// Spec 0122: the filaments lean toward whoever is standing there, all
    /// the way round.
    ///
    /// **Every one of them and not the average of them.** The first way to
    /// write this was a weighted lean from where a filament wanders toward
    /// the eye, which has no good setting: weighted to the eye all five
    /// point one way and the globe is a torch, and weighted away a filament
    /// can come out facing the far side, so the only claim left to make is
    /// about a mean, and a mean of five wandering things is a test that
    /// passes on a quiet frame. A cone off the eye's own line makes the
    /// claim per filament, which is the claim worth making.
    #[test]
    fn the_filaments_reach_for_whoever_is_there() {
        let reaches = reaches();
        let least = FAN.cos();

        assert!(least > 0.0, "a fan of {} is not a lean at all", FAN);

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);

            for eye in all_round(reaches, side) {
                let toward = (eye - middle).normalize();

                for tenths in 0..40 {
                    let since = tenths as f32 * 0.37;

                    for which in 0..FILAMENTS {
                        let tip = filament(reaches, which, side, eye, since)[LINKS];
                        let leans = ((tip - middle) / shell).dot(toward);

                        assert!(
                            leans >= least - 1e-3,
                            "filament {} leans {} toward an eye at {:?} and the fan allows {}",
                            which,
                            leans,
                            eye,
                            least
                        );
                    }
                }
            }
        }
    }

    /// Spec 0122: each arm crawls at its own rate, jumps now and then, and
    /// does not jump with its neighbours.
    ///
    /// **Three notes from Jake, each of which broke the one before.** First,
    /// the arms should move erratically, not in sync with each other, when
    /// every arm was a sine of the time at a rate a few hundredths off its
    /// neighbour's. Then, they should crawl more than jump across the inner
    /// surface, though some jumping is good too, and in general more variety
    /// in how they move, when the answer to the first note had been to hold
    /// each arm still for a beat and snap it somewhere else, which is all
    /// jump and no crawl.
    ///
    /// So the claims are about the shape of the movement rather than its
    /// amount. Each arm creeps most of the time, which is its median frame.
    /// Each arm sometimes goes much further in one frame than that, which is
    /// a jump. Jumping is the exception and not the rule. No two arms jump
    /// in the same frame much more often than two unrelated things would.
    /// And the arms differ: the quickest creeps several times faster than the
    /// slowest, which is the variety itself and is the one of these that a
    /// single shared rate would fail.
    ///
    /// Measured only while an arm is lit, because movement nobody can see is
    /// not movement, and an arm that strikes after a dark stretch starts
    /// somewhere new on purpose.
    #[test]
    fn the_arms_crawl_at_their_own_rates_and_jump_apart() {
        let reaches = reaches();
        let frames = 7200;
        let step = 1.0 / 60.0;
        let leapt = 8.0;

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);
            let eye = middle + vec3(1.2, 0.3, 0.6);
            let mut creeps = [0.0f32; FILAMENTS];
            let mut leaping = vec![vec![false; frames]; FILAMENTS];

            for (which, mine) in leaping.iter_mut().enumerate() {
                let mut moves: Vec<(usize, f32)> = Vec::new();

                for (frame, _) in (0..frames).map(|f| (f, ())) {
                    let since = frame as f32 * step;

                    if burning(which, side, since) <= 0.0
                        || burning(which, side, since + step) <= 0.0
                    {
                        continue;
                    }

                    let was = (filament(reaches, which, side, eye, since)[LINKS] - middle) / shell;
                    let now =
                        (filament(reaches, which, side, eye, since + step)[LINKS] - middle) / shell;

                    moves.push((frame, was.distance(now)));
                }

                let mut sorted: Vec<f32> = moves.iter().map(|(_, d)| *d).collect();
                sorted.sort_by(|one, two| one.total_cmp(two));

                let creep = sorted[sorted.len() / 2];
                creeps[which] = creep;

                // **A keen arm creeps less and that is the reaching, not a
                // fault.** An arm pointing nearly at you sits near the pole
                // of its own cone, where rolling at the same rate moves its
                // tip hardly at all, so this floor is what a keen arm
                // manages rather than what a dull one does.
                assert!(
                    creep > 0.00015,
                    "arm {} creeps {:.5} a frame, so it is standing still",
                    which,
                    creep
                );
                assert!(
                    creep < 0.02,
                    "arm {} creeps {:.5} a frame, so it is sweeping",
                    which,
                    creep
                );

                for (frame, far) in moves {
                    mine[frame] = far > creep * leapt;
                }

                let leaps = mine.iter().filter(|it| **it).count();

                assert!(leaps > 10, "arm {} never jumped at all", which);
                // A jump takes `JUMPS_IN` out of an era of `ERA`, so the share
                // is a few hundredths by construction. What this asks is the
                // intent rather than the arithmetic: that crawling is what an
                // arm is mostly doing.
                assert!(
                    (leaps as f32 / frames as f32) < 0.15,
                    "arm {} jumped in {:.4} of frames, so it is not crawling",
                    which,
                    leaps as f32 / frames as f32
                );
            }

            let quickest = creeps.iter().copied().fold(f32::MIN, f32::max);
            let slowest = creeps.iter().copied().fold(f32::MAX, f32::min);

            assert!(
                quickest / slowest > 3.0,
                "the quickest arm creeps {:.2} times the slowest, which is one rate shared out",
                quickest / slowest
            );

            for (which, mine) in leaping.iter().enumerate() {
                for (other, theirs) in leaping.iter().enumerate().skip(which + 1) {
                    let together = mine
                        .iter()
                        .zip(theirs.iter())
                        .filter(|(one, two)| **one && **two)
                        .count() as f32
                        / frames as f32;

                    // Two arms each jumping a twentieth of the time meet in a
                    // four hundredth of frames if nothing links them. This
                    // allows four times that and still fails anything in step.
                    assert!(
                        together < 0.02,
                        "arms {} and {} jump together in {:.4} of frames",
                        which,
                        other,
                        together
                    );
                }
            }
        }
    }

    /// Spec 0122: arms come and go, and the globe is never wholly dark.
    ///
    /// **Jake: there should be arms that pop in and out of existence in
    /// addition to jumping around the sphere.** Seven arms that are always
    /// there and only ever move are seven arms, however erratically they
    /// move. What should waver is the count.
    ///
    /// The second half is the one that found something. Each arm's life is
    /// its own throw, so nothing in seven independent throws stops all seven
    /// coming up dark, and this caught it inside ten minutes of running. A
    /// plasma globe that blinks off is a broken plasma globe, so an arm
    /// cannot go out if it is the last one: when nothing at all is lit, the
    /// one that came nearest to staying strikes. That is a guarantee rather
    /// than a probability now, and this holds it against anyone raising
    /// `GOES_OUT`.
    #[test]
    fn the_arms_come_and_go_and_the_globe_is_never_dark() {
        let frames = 36_000;
        let step = 1.0 / 60.0;

        for side in 0..SIDES.len() {
            let mut counted = [0usize; FILAMENTS + 1];
            let mut alight = [0usize; FILAMENTS];

            for frame in 0..frames {
                let since = frame as f32 * step;
                let lit = (0..FILAMENTS).filter(|one| burning(*one, side, since) > 0.0);
                let mut now = 0;

                for which in lit {
                    alight[which] += 1;
                    now += 1;
                }

                counted[now] += 1;
            }

            assert_eq!(counted[0], 0, "the globe went dark");

            for (which, lit) in alight.iter().enumerate() {
                let share = *lit as f32 / frames as f32;

                assert!(
                    (0.45..0.92).contains(&share),
                    "arm {} is lit in {:.3} of frames",
                    which,
                    share
                );
            }

            // and the count wavers rather than sitting at one number
            let seen = counted.iter().filter(|it| **it > frames / 200).count();

            assert!(
                seen >= 4,
                "the lit count only ever took {} values, so nothing comes and goes",
                seen
            );
        }
    }

    /// Spec 0122: the spark at an arm's end is on that arm, against the
    /// glass, and wholly inside it.
    ///
    /// **Jake: there should be a gentle spark against the glass at the end
    /// of each arm.** The three ways that can go wrong are all arithmetic.
    /// It can sit off the arm's own line, which reads as a bead floating
    /// beside the thread. It can sit short of the glass, which is a bead on
    /// a string. And its bloom can reach through the glass, which is a thing
    /// outside a sealed globe. The first build had it four times too big and
    /// what it drew was balloons, which is the one of these no test can say.
    #[test]
    fn the_spark_sits_against_the_glass_and_inside_it() {
        let reaches = reaches();

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);

            for eye in all_round(reaches, side) {
                for tenths in 0..40 {
                    let since = tenths as f32 * 0.37;

                    for which in 0..FILAMENTS {
                        for (n, limb) in arm(reaches, which, side, eye, since).iter().enumerate() {
                            let end = limb.links[limb.links.len() - 1];
                            let tip = end - middle;
                            let at = pulled_in(reaches, side, end) - middle;

                            assert!(
                                at.normalize().distance(tip.normalize()) < 1e-3,
                                "arm {}'s limb {} sparks off its own line",
                                which,
                                n
                            );
                            assert!(
                                at.length() + shell * BLOOM <= shell + 1e-4,
                                "arm {}'s limb {} blooms to {} and the glass is at {}",
                                which,
                                n,
                                at.length() + shell * BLOOM,
                                shell
                            );
                            assert!(
                                at.length() > shell * 0.8,
                                "arm {}'s limb {} sparks {} out of {}, so it is adrift inside",
                                which,
                                n,
                                at.length(),
                                shell
                            );
                        }
                    }
                }
            }
        }
    }

    /// Spec 0122: the two of them stand symmetrically about the ball, and
    /// are not each other.
    ///
    /// **Jake: make it symmetrical, add another sparkle ball to the left.**
    /// One globe beside a ball is a ball with a thing next to it; one either
    /// side is a row with the ball in the middle of it.
    ///
    /// The second claim is the one that needed writing down. Two globes a
    /// metre and a half apart in plain sight of each other, whose arms are
    /// seeded off the arm number alone, are one globe drawn twice, and the
    /// eye catches that at once. Their dice are drawn off which arm *and*
    /// which globe, and this is what holds that.
    #[test]
    fn the_two_stand_either_side_and_are_not_each_other() {
        let reaches = reaches();
        let line = crate::glass::plinth(reaches).center().x;
        let west = pedestal(reaches, 0);
        let east = pedestal(reaches, 1);

        assert!(
            ((line - west.center().x) - (east.center().x - line)).abs() < 1e-4,
            "they stand {:.3} and {:.3} off the ball's own line",
            line - west.center().x,
            east.center().x - line
        );
        assert!(
            west.center().x < line && east.center().x > line,
            "both of them are on one side"
        );
        assert!(
            (west.size() - east.size()).abs().max_element() < 1e-4
                && (west.center().z - east.center().z).abs() < 1e-4
                && (west.center().y - east.center().y).abs() < 1e-4,
            "one of them is a different size or stands somewhere else down the room"
        );

        // and the two are not the same globe twice: an arm of one is
        // somewhere else, and lit at different times, from the arm of the
        // other that shares its number
        let eye = globe(reaches, 0).0 + vec3(1.2, 0.3, 0.6);
        let mut apart = 0;
        let mut differ = 0;

        for tenths in 0..200 {
            let since = tenths as f32 * 0.17;

            for which in 0..FILAMENTS {
                let one = (filament(reaches, which, 0, eye, since)[LINKS] - globe(reaches, 0).0)
                    .normalize();
                let two = (filament(reaches, which, 1, eye, since)[LINKS] - globe(reaches, 1).0)
                    .normalize();

                if one.distance(two) > 0.2 {
                    apart += 1;
                }

                if (burning(which, 0, since) > 0.0) != (burning(which, 1, since) > 0.0) {
                    differ += 1;
                }
            }
        }

        let of = (200 * FILAMENTS) as f32;

        assert!(
            apart as f32 / of > 0.8,
            "the arms of the two globes point the same way in {:.2} of samples",
            1.0 - apart as f32 / of
        );
        assert!(
            differ as f32 / of > 0.15,
            "the two globes are lit and dark together in {:.2} of samples",
            1.0 - differ as f32 / of
        );
    }

    /// Spec 0122: some arms come far nearer the line to you than others, and
    /// which ones is a thing about the arm.
    ///
    /// **Jake, having walked round it: the arms should also reach toward
    /// wherever I am standing.** They already did, by construction and under
    /// `the_filaments_reach_for_whoever_is_there`, and that is the point: it
    /// was true and could not be seen. The fan is eighty-six degrees, which
    /// is most of a hemisphere, so the whole bunch does swing as you move and
    /// what you see is a starfish either way.
    ///
    /// What makes it legible is that the arms are not alike in it. A keen one
    /// gives up four fifths of the fan and comes nearly to you; a dull one
    /// spreads as before. So the claim is a spread of keenness rather than
    /// any one arm's angle, and that the keenest arm is the same arm from
    /// one moment to the next, since a globe whose keen arms changed every
    /// second would read as flicker and not as reaching.
    #[test]
    fn some_arms_come_far_nearer_you_than_others() {
        let reaches = reaches();
        let (middle, _) = globe(reaches, 1);
        let eye = middle + vec3(1.3, 0.2, 0.4);
        let toward = (eye - middle).normalize();
        let mut leans = [0.0f32; FILAMENTS];

        for (which, mine) in leans.iter_mut().enumerate() {
            let mut off: Vec<f32> = (0..300)
                .map(|tenths| {
                    let since = tenths as f32 * 0.11;
                    let tip = (filament(reaches, which, 1, eye, since)[LINKS] - middle).normalize();

                    tip.dot(toward).clamp(-1.0, 1.0).acos()
                })
                .collect();

            off.sort_by(|one, two| one.total_cmp(two));
            *mine = off[off.len() / 2];
        }

        let keenest = leans.iter().copied().fold(f32::MAX, f32::min);
        let dullest = leans.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            keenest < dullest * 0.4,
            "the keenest arm sits {:.2} off the line to you and the dullest {:.2}, \
             which is one spread shared out",
            keenest,
            dullest
        );
        assert!(
            keenest < 0.45,
            "the keenest arm is {:.2} off the line to you, which is not reaching",
            keenest
        );

        // and it is the same arm that is keen, rather than a different one
        // each moment
        let mine = leans
            .iter()
            .enumerate()
            .min_by(|one, two| one.1.total_cmp(two.1))
            .map(|(which, _)| which)
            .unwrap_or(0);

        assert!(
            keenness(mine, 1) > 0.5,
            "the arm that sits nearest you has a keenness of {:.2}, so it got there by chance",
            keenness(mine, 1)
        );
    }

    /// Spec 0122: neither globe writhes faster than the other.
    ///
    /// **Jake, watching the pair: the right one is a lot more jittery than
    /// the left one.** It was, by a factor of six, and the cause is one
    /// line's worth of carelessness repeated five times. An arm's seed
    /// carries which globe it stands on, as a number in the hundreds, and
    /// five places were multiplying time by that seed: the east globe's
    /// threads ran at fifty-four radians a second against the west's three,
    /// and its sparks breathed at forty a second.
    ///
    /// A seed is for seeding. Anything that multiplies time has to be
    /// bounded, and this holds that from the outside: the middle of a thread
    /// moves at the same sort of rate whichever globe it is on.
    #[test]
    fn neither_globe_writhes_faster_than_the_other() {
        let reaches = reaches();
        let frames = 1200;
        let step = 1.0 / 60.0;
        let mut each = Vec::new();

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);
            let eye = middle + vec3(1.3, 0.2, 0.4);
            let mut moves: Vec<f32> = Vec::new();

            for frame in 0..frames {
                let since = frame as f32 * step;

                for which in 0..FILAMENTS {
                    let was = filament(reaches, which, side, eye, since)[LINKS / 2];
                    let now = filament(reaches, which, side, eye, since + step)[LINKS / 2];

                    moves.push(was.distance(now) / shell);
                }
            }

            moves.sort_by(|one, two| one.total_cmp(two));
            each.push(moves[moves.len() / 2]);
        }

        let quick = each.iter().copied().fold(f32::MIN, f32::max);
        let slow = each.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            quick / slow < 1.4,
            "one globe's threads move {:.2} times the other's, at {:.5} against {:.5}",
            quick / slow,
            quick,
            slow
        );
    }

    /// Spec 0122: the shell is see-through and the plasma inside it is not.
    ///
    /// Alpha under one is what spec 0018 of the engine draws after
    /// everything solid, and a channel past one is what this engine means
    /// by a thing that gives off light rather than taking it. The shell
    /// does neither of the second: a glass shell that glows is a paper
    /// lantern.
    #[test]
    fn the_glass_is_see_through_and_the_plasma_is_not() {
        let glass = crate::aim::PLASMA_GLASS;

        assert!(glass.w < 1.0, "the shell is not see-through at {}", glass.w);
        assert!(
            glass.x <= 1.0 && glass.y <= 1.0 && glass.z <= 1.0,
            "the shell glows at {:?}",
            glass
        );

        for (what, colour) in [
            ("the filaments at the glass", crate::aim::PLASMA),
            ("the filaments at the electrode", crate::aim::PLASMA_HOT),
            ("the electrode", crate::aim::PLASMA_CORE),
        ] {
            assert!(
                colour.max_element() > 1.0,
                "{} do not glow at {:?}",
                what,
                colour
            );
            assert_eq!(colour.w, 1.0, "{} are see-through", what);
        }

        // **One channel under one at the glass, which is what keeps the
        // hue.** The screen clips at one, so a colour past it in all three
        // is white however it was mixed, and the first build of this drew
        // five white bolts. The violet only survived in the ball's
        // reflection, where it is dimmer.
        //
        // Only at the glass. The hot end and the electrode are white on
        // purpose, off Jake's photograph of the real thing: what is hottest
        // is whitest, and a filament that is one hue end to end is paint.
        assert!(
            crate::aim::PLASMA.truncate().min_element() < 1.0,
            "the filaments are past one in every channel at the glass, so they are white: {:?}",
            crate::aim::PLASMA
        );
        assert!(
            crate::aim::PLASMA_HOT.truncate().min_element()
                > crate::aim::PLASMA.truncate().min_element(),
            "the hot end is no whiter than the end at the glass"
        );
    }

    /// Spec 0122: the lamp is in the globe, and the globe is on the pedestal.
    ///
    /// **Which is this building's rule about fittings, read the one way it
    /// can fail here.** The lamp is the globe's own middle by construction,
    /// so where it is cannot drift; what can is the globe growing past the
    /// thing holding it, and a lamp inside a ball hanging over the edge of
    /// its pedestal is a lamp in nothing.
    #[test]
    fn the_lamp_is_in_the_globe() {
        let reaches = reaches();

        for (side, at) in lamps(reaches).into_iter().enumerate() {
            let (middle, shell) = globe(reaches, side);
            let stands = pedestal(reaches, side);

            assert!(at.distance(middle) < shell, "the lamp is outside the glass");
            assert!(
                shell <= PEDESTAL * 0.5,
                "a shell of {} hangs over a pedestal of {}",
                shell,
                PEDESTAL
            );
            assert!(
                middle.y - shell >= stands.max.y - 1e-4,
                "the globe sinks {} into the pedestal",
                stands.max.y - (middle.y - shell)
            );
        }
    }

    /// Spec 0122: nothing drawn round the globe shares a plane with anything
    /// else drawn there.
    ///
    /// **This is the fault the first build shipped and nothing caught.** The
    /// pedestal's box went into `glass::solid` so a body would walk into it,
    /// and that room draws every box of `solid` it has not been told to leave
    /// alone. Nobody told it. So the box was drawn as a plain cube and these
    /// courses were drawn over it, their sides landing exactly on its sides,
    /// and what came back was a checkerboard of lit and shadowed squares with
    /// a stair-stepped diagonal through it. It reads like a coarse shadow map
    /// and it is not one, which is why it took eight eliminations to place:
    /// the lamp, the shell, the stem, the filaments, the ball and the plinth
    /// beside it, the courses' oversail, the sun's own bounds, and the
    /// pedestal's place in the room.
    ///
    /// Asked the engine's own coplanar check, the way the hall asks it, and
    /// narrowed to the corner this is about so the rest of the room's own
    /// answer stays the rest of the room's business.
    #[test]
    fn nothing_round_the_globe_is_in_one_plane_with_anything() {
        use blitzkit::renderer::coplanar::Fights;

        let reaches = reaches();
        let looking = blitzkit::renderer::depth::Depth::of(&blitzkit::camera::Camera::default());
        let otherwise = crate::glass::drawn_otherwise(reaches);
        let mut drawn: Vec<Aabb> = crate::glass::solid(reaches)
            .into_iter()
            .filter(|piece| !otherwise.contains(piece))
            .collect();

        drawn.extend(
            crate::glass::plinth_pieces(reaches)
                .into_iter()
                .map(|(b, _)| b),
        );
        for side in 0..SIDES.len() {
            drawn.extend(pieces(reaches, side).into_iter().map(|(b, _)| b));
        }

        let corners: Vec<Aabb> = (0..SIDES.len())
            .map(|side| {
                Aabb::from_center_size(
                    pedestal(reaches, side).center(),
                    Vec3::splat(PEDESTAL * 2.0 + 1.0),
                )
            })
            .collect();

        for fight in Fights::of(drawn, 0, 0, &looking).fights.iter() {
            assert!(
                !corners
                    .iter()
                    .any(|corner| corner.contains_point(fight.patch.center())),
                "two drawn faces share a plane at {:?}, over {:.2} by {:.2}",
                fight.patch.center(),
                fight.patch.size().x.max(fight.patch.size().y),
                fight.patch.size().z.max(fight.patch.size().y)
            );
        }
    }

    /// Spec 0122: none of it is inside the room, the colonnade or the ball's
    /// own plinth.
    ///
    /// Asked of the room's own solid list with the pedestal taken back out
    /// of it, so a piece of furniture added in here is in this test the day
    /// it is added. Shared volume rather than contact, because the pedestal
    /// stands on the floor and shares its face, which is the same question
    /// the back room's clutter answers.
    #[test]
    fn the_globe_stands_inside_the_room() {
        let reaches = reaches();
        let room = crate::glass::inside(reaches);
        let balls: Vec<Aabb> = (0..SIDES.len())
            .map(|side| {
                let (middle, shell) = globe(reaches, side);

                Aabb::from_center_size(middle, Vec3::splat(shell * 2.0))
            })
            .collect();
        let mine = solid(reaches);
        let theirs: Vec<Aabb> = crate::glass::solid(reaches)
            .into_iter()
            .filter(|box_| !mine.contains(box_))
            .collect();

        assert_eq!(
            theirs.len() + mine.len(),
            crate::glass::solid(reaches).len(),
            "the room's own solid list no longer holds the pedestal"
        );

        for one in mine.iter().chain(balls.iter()) {
            assert!(
                one.min.x > room.min.x
                    && one.max.x < room.max.x
                    && one.min.z > room.min.z
                    && one.max.z < room.max.z
                    && one.max.y < room.max.y,
                "{:?} to {:?} is outside a room of {:?} to {:?}",
                one.min,
                one.max,
                room.min,
                room.max
            );

            for other in &theirs {
                let shared = (one.max.min(other.max) - one.min.max(other.min)).min_element();

                assert!(
                    shared <= 1e-3,
                    "the globe is {:.3} inside {:?} to {:?}",
                    shared,
                    other.min,
                    other.max
                );
            }
        }
    }

    /// A fork that does not fork is one shape drawn sixteen times, which is
    /// what Jake saw. Every limb shallower than the deepest throws at least
    /// one, every fork starts on a link of the limb it left, and every one
    /// of them lands on the glass like the trunk does.
    #[test]
    fn every_fork_forks_again_and_leaves_the_limb_it_came_from() {
        let reaches = reaches();

        for side in 0..SIDES.len() {
            let (middle, shell) = globe(reaches, side);

            for eye in all_round(reaches, side) {
                for tenths in 0..20 {
                    let since = tenths as f32 * 0.37;

                    for which in 0..FILAMENTS {
                        let limbs = arm(reaches, which, side, eye, since);

                        // written out rather than read off `DEEPEST`, or
                        // the test follows the constant it is meant to hold
                        // and passes with the forking turned off
                        for deep in [0, 1, 2] {
                            assert!(
                                limbs.iter().any(|limb| limb.deep == deep),
                                "arm {} has nothing {} forks down",
                                which,
                                deep
                            );
                        }

                        for (n, limb) in limbs.iter().enumerate() {
                            assert_eq!(
                                limb.links.len(),
                                LIMB_LINKS[limb.deep] + 1,
                                "arm {}'s limb {} is drawn in the wrong number of links",
                                which,
                                n
                            );

                            let end = limb.links[limb.links.len() - 1];

                            assert!(
                                (end.distance(middle) - shell).abs() < 1e-3,
                                "arm {}'s limb {} ends {} out and the glass is at {}",
                                which,
                                n,
                                end.distance(middle),
                                shell
                            );

                            if limb.deep == 0 {
                                continue;
                            }

                            assert!(
                                limbs.iter().any(|other| other.deep < limb.deep
                                    && other.links.contains(&limb.links[0])),
                                "arm {}'s limb {} starts on nothing",
                                which,
                                n
                            );
                            assert!(
                                limb.thick < THREAD && limb.lit < 1.0,
                                "arm {}'s limb {} is no thinner or dimmer than the trunk",
                                which,
                                n
                            );
                        }
                    }
                }
            }
        }
    }

    /// And the other half of what he said: that they move with the same
    /// fork. Two forks off one limb that swing at one rate turn about it as
    /// a rigid pair, so however much the arm under them moves, the one
    /// never comes round toward the other. Held still, with only the forks'
    /// own clocks running, they have to.
    ///
    /// Of the pair and not of each fork on its own, since two rates drawn
    /// from the same dice can land near enough to each other that one pair
    /// in a hundred does hold its angle for a second.
    #[test]
    fn no_two_forks_off_one_limb_swing_together() {
        let reaches = reaches();
        let (middle, shell) = globe(reaches, 0);
        let (across, other) = frame(Vec3::Y);
        let parent = Limb {
            links: (0..=LINKS)
                .map(|k| middle + Vec3::Y * (shell * k as f32 / LINKS as f32))
                .collect(),
            deep: 0,
            thick: THREAD,
            lit: 1.0,
        };

        // where round the parent each fork's tip has got to
        let apart = |seed: f32, since: f32| {
            let young = forks(middle, shell, &parent, seed, 0.5, since);

            (young.len() == 2).then(|| {
                let round = |limb: &Limb| {
                    let tip = limb.links[LIMB_LINKS[1]] - middle;

                    tip.dot(other).atan2(tip.dot(across))
                };
                let turn = round(&young[1]) - round(&young[0]);

                (turn + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
                    - std::f32::consts::PI
            })
        };

        let mut pairs = 0;
        let mut moved = 0.0;

        for n in 0..400 {
            let seed = n as f32 * 3.37;
            let (Some(was), Some(now)) = (apart(seed, 0.0), apart(seed, 1.0)) else {
                continue;
            };

            pairs += 1;
            moved += (now - was).abs();
        }

        assert!(pairs > 100, "only {} seeds forked twice", pairs);
        assert!(
            moved / pairs as f32 > 0.2,
            "two forks off one limb come round toward each other by {} radians a second, which is a pair turning as one piece",
            moved / pairs as f32
        );
    }
}
