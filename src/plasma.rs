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
pub const CORE: f32 = 0.011;
pub const STEM: f32 = 0.010;

/// Where the pedestal stands, derived off the ball's plinth so that moving
/// one moves the other.
pub fn pedestal(reaches: f32) -> Aabb {
    let plinth = crate::glass::plinth(reaches);
    let middle = plinth.center();
    let west = plinth.max.x + APART;

    Aabb::new(
        vec3(west, plinth.min.y, middle.z - PEDESTAL * 0.5),
        vec3(
            west + PEDESTAL,
            plinth.min.y + PEDESTAL_UP,
            middle.z + PEDESTAL * 0.5,
        ),
    )
}

/// The globe: where its middle is and how big it is.
pub fn globe(reaches: f32) -> (Vec3, f32) {
    let stands = pedestal(reaches);
    let at = stands.center();

    (vec3(at.x, stands.max.y + SHELL, at.z), SHELL)
}

/// What a body walks into, which is the pedestal and nothing else: the shell
/// is 0.22 inside a footprint of 0.26, so it never stands out past what is
/// holding it.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    vec![pedestal(reaches)]
}

/// How many filaments there are and how many links each is drawn in.
pub const FILAMENTS: usize = 7;
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
pub const THREAD: f32 = 0.0038;
pub const THINS_TO: f32 = 0.12;

/// And how much of its brightness is left there, for the same reason.
pub const DIMS_TO: f32 = 0.35;

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
pub const WANDER: f32 = 0.055;
pub const WANDERS_BY: f32 = 0.55;
pub const CRINKLE: f32 = 0.018;
pub const CRINKLES_BY: f32 = 2.4;

/// How long an arm holds still between jumps, and how much of that hold it
/// spends moving.
///
/// **Because they were all turning the same way at the same speed.** Jake,
/// watching it: the arms should move erratically, not in sync with each
/// other. Every one was a sine of the time at a rate a few hundredths off
/// its neighbour's, which is seven things doing one thing slightly out of
/// step rather than seven things each doing their own. And a discharge does
/// not sweep. It stands where it is, and then it is somewhere else.
pub const HOLDS: f32 = 0.55;
pub const JUMPS_IN: f32 = 0.12;

/// A number between nought and one from a number.
///
/// How an arm gets somewhere different on every jump without anything being
/// kept: the whole globe is worked out from the time and the eye, per spec
/// 0122, so what stands in for a dice is a function of which arm and which
/// jump. Not a good hash and it does not need to be one. It needs to be
/// unlike its neighbour, which the multiply and the fraction see to.
pub fn scatter(of: f32) -> f32 {
    let x = (of * 12.9898 + 78.233).sin() * 43758.547;

    x - x.floor()
}

/// Where one arm stands on one of its jumps, as how far it leans off the line
/// to the eye and where round that line it sits.
///
/// Never past the fan, whatever the dice say, which is what keeps every arm
/// on the side the eye is on.
pub fn aim(which: usize, step: f32) -> (f32, f32) {
    let seed = which as f32 * 31.7 + step * 7.13;

    (
        FAN * (0.22 + 0.78 * scatter(seed)),
        scatter(seed + 3.7) * std::f32::consts::TAU,
    )
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
pub fn filament(reaches: f32, which: usize, eye: Vec3, since: f32) -> [Vec3; LINKS + 1] {
    let (middle, shell) = globe(reaches);
    let toward = {
        let out = (eye - middle).normalize_or_zero();

        if out == Vec3::ZERO {
            Vec3::Y
        } else {
            out
        }
    };
    // a frame about that line, picked off whichever axis it is least like,
    // because the cross product of a line with itself is nothing
    let side = if toward.y.abs() < 0.9 {
        toward.cross(Vec3::Y).normalize()
    } else {
        toward.cross(Vec3::X).normalize()
    };
    let other = toward.cross(side);

    let n = which as f32;
    // where it stands: held still, then struck somewhere else. Its own beat
    // and its own offset into that beat, so no two arms ever move together.
    let beat = HOLDS * (0.55 + 0.9 * scatter(n + 0.5));
    let tick = since / beat + scatter(n + 11.3) * 7.0;
    let step = tick.floor();
    // and how far through the jump it is, leaving fast and settling
    let frac = ((tick - step) / JUMPS_IN).min(1.0);
    let eased = 1.0 - (1.0 - frac) * (1.0 - frac);

    let (was_lean, was_roll) = aim(which, step - 1.0);
    let (now_lean, now_roll) = aim(which, step);
    // the short way round, or an arm at six radians crossing to one at a
    // tenth sweeps the whole circle to get there
    let round = std::f32::consts::TAU;
    let turn = (now_roll - was_roll + round * 0.5).rem_euclid(round) - round * 0.5;
    let lean = was_lean + (now_lean - was_lean) * eased;
    let roll = was_roll + turn * eased;
    let along = toward * lean.cos() + (side * roll.cos() + other * roll.sin()) * lean.sin();

    let mut links = [Vec3::ZERO; LINKS + 1];

    for (k, link) in links.iter_mut().enumerate() {
        let t = k as f32 / LINKS as f32;
        let held = (t * std::f32::consts::PI).sin() * shell;
        // the thread's own shape is re-struck with the arm, so a jump is a
        // new discharge rather than the same one swinging
        let struck = scatter(n * 5.1 + step) * std::f32::consts::TAU;
        let slow = since * (1.7 + n * 0.3) + k as f32 * WANDERS_BY + struck;
        let fast = since * (4.3 + n * 0.5) + k as f32 * CRINKLES_BY + struck * 2.0;
        let off = (side * slow.cos() + other * slow.sin()) * (held * WANDER)
            + (side * fast.cos() + other * fast.sin()) * (held * CRINKLE);

        *link = middle + along * (t * shell) + off;
    }

    links
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
pub fn lamp(reaches: f32) -> Vec3 {
    globe(reaches).0
}

/// What the ball sees of it, since the tracer reads a box's shape and would
/// call a pedestal a frame on a wall.
///
/// The box is `pedestal` itself, asked by name, so there is nothing here to
/// drift from it. The globe is a lathe and the filaments are a hand's width,
/// and neither is in `solid`, so neither is traced.
pub fn named(reaches: f32) -> Vec<(Aabb, glam::Vec4)> {
    vec![(pedestal(reaches), crate::aim::GLASS_TRIM)]
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
pub fn pieces(reaches: f32) -> Vec<(Aabb, glam::Vec4)> {
    crate::glass::plinth_courses(pedestal(reaches))
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
    let stands = pedestal(reaches);

    for (box_, colour) in pieces(reaches) {
        scene.push_plain(
            cube,
            &Transform::at(box_.center()).with_scale(box_.size()),
            colour,
        );
    }

    let (middle, shell) = globe(reaches);

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
    // it set out.
    for which in 0..FILAMENTS {
        let links = filament(reaches, which, eye, since);

        for (k, pair) in links.windows(2).enumerate() {
            let (from, to) = (pair[0], pair[1]);
            let span = to - from;
            let long = span.length();

            if long <= f32::EPSILON {
                continue;
            }

            let t = (k as f32 + 0.5) / (LINKS - 1) as f32;
            let thick = THREAD * (1.0 - (1.0 - THINS_TO) * t);
            let lit = 1.0 - (1.0 - DIMS_TO) * t;

            scene.push_fitting(
                cube,
                &Transform::at((from + to) * 0.5)
                    .with_rotation(Quat::from_rotation_arc(Vec3::Y, span / long))
                    .with_scale(vec3(thick, long, thick)),
                (crate::aim::PLASMA.truncate() * lit).extend(1.0),
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
    // hung in the air and the thing read as a sparkler rather than
    // something under a dome. What a hot spot off the bay lamps gives it is
    // a rim and a curve.
    // **With a highlight on it, which is the whole of what says glass.**
    // Drawn plain and nearly clear it was not there at all: the filaments
    // hung in the air and the thing read as a sparkler rather than
    // something under a dome. What a hot spot off the bay lamps gives it is
    // a rim and a curve.
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
    fn all_round(reaches: f32) -> Vec<Vec3> {
        let (middle, _) = globe(reaches);

        (0..16)
            .map(|n| {
                let turn = n as f32 / 16.0 * std::f32::consts::TAU;

                middle + vec3(turn.cos() * 1.4, 0.38, turn.sin() * 1.4)
            })
            .collect()
    }

    /// Spec 0122: every filament runs from the electrode to the shell and
    /// stays inside it.
    ///
    /// **Both ends exactly, which is what the bow is shaped for.** A
    /// crooked bolt is drawn by pushing its middle off its own line, and
    /// the obvious way to do that leaves the ends pushed off too: one
    /// hanging in the air beside the electrode and one through the glass.
    /// `BOW` is a sine over the run, so it is nought at nought and nought
    /// at one.
    #[test]
    fn every_filament_runs_from_the_core_to_the_glass() {
        let reaches = reaches();
        let (middle, shell) = globe(reaches);

        for eye in all_round(reaches) {
            for tenths in 0..40 {
                let since = tenths as f32 * 0.37;

                for which in 0..FILAMENTS {
                    let links = filament(reaches, which, eye, since);

                    assert!(
                        links[0].distance(middle) < 1e-5,
                        "filament {} starts {} off the electrode",
                        which,
                        links[0].distance(middle)
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
        let (middle, shell) = globe(reaches);
        let least = FAN.cos();

        assert!(least > 0.0, "a fan of {} is not a lean at all", FAN);

        for eye in all_round(reaches) {
            let toward = (eye - middle).normalize();

            for tenths in 0..40 {
                let since = tenths as f32 * 0.37;

                for which in 0..FILAMENTS {
                    let tip = filament(reaches, which, eye, since)[LINKS];
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

    /// Spec 0122: each arm holds still, jumps, and does not jump with its
    /// neighbours.
    ///
    /// **Jake, watching the first one that moved: the arms should move
    /// erratically, not in sync with each other.** Every arm was a sine of
    /// the time at a rate a few hundredths off its neighbour's, so all seven
    /// turned the same way at nearly the same speed, which is one movement
    /// shared out rather than seven of them.
    ///
    /// Three claims, because "erratic" on its own cannot fail. Each arm is
    /// still most of the time, which is what a discharge does. Each arm does
    /// move. And no two of them are moving at once much more often than two
    /// unrelated things would be, which is the sync itself: seven arms each
    /// moving a eighth of the time meet by chance in about a sixtieth of
    /// frames, and in step they would meet in every frame one of them moved.
    #[test]
    fn the_arms_jump_rather_than_sweep_and_never_together() {
        let reaches = reaches();
        let (middle, shell) = globe(reaches);
        let eye = middle + vec3(1.2, 0.3, 0.6);
        let frames = 2400;
        let step = 1.0 / 60.0;
        let moving = 0.004;

        let mut budged = vec![vec![false; frames]; FILAMENTS];

        for frame in 0..frames {
            let since = frame as f32 * step;

            for (which, mine) in budged.iter_mut().enumerate() {
                let was = (filament(reaches, which, eye, since)[LINKS] - middle) / shell;
                let now = (filament(reaches, which, eye, since + step)[LINKS] - middle) / shell;

                mine[frame] = was.distance(now) > moving;
            }
        }

        for (which, mine) in budged.iter().enumerate() {
            let share = mine.iter().filter(|it| **it).count() as f32 / frames as f32;

            assert!(
                share > 0.02,
                "arm {} moved in {:.3} of frames, so it is standing still",
                which,
                share
            );
            assert!(
                share < 0.30,
                "arm {} moved in {:.3} of frames, so it is sweeping rather than jumping",
                which,
                share
            );
        }

        for (which, mine) in budged.iter().enumerate() {
            for (other, theirs) in budged.iter().enumerate().skip(which + 1) {
                let together = mine
                    .iter()
                    .zip(theirs.iter())
                    .filter(|(one, two)| **one && **two)
                    .count() as f32
                    / frames as f32;

                assert!(
                    together < 0.06,
                    "arms {} and {} are both moving in {:.3} of frames",
                    which,
                    other,
                    together
                );
            }
        }
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
            ("the filaments", crate::aim::PLASMA),
            ("the electrode", crate::aim::PLASMA_CORE),
        ] {
            assert!(
                colour.max_element() > 1.0,
                "{} do not glow at {:?}",
                what,
                colour
            );
            // **And one channel under one, which is what keeps the hue.**
            // The screen clips at one, so a colour past it in all three is
            // white however it was mixed, and the first build of this drew
            // five white bolts. The violet only survived in the ball's
            // reflection, where it is dimmer.
            assert!(
                colour.truncate().min_element() < 1.0,
                "{} are past one in every channel, so they are white: {:?}",
                what,
                colour
            );
            assert_eq!(colour.w, 1.0, "{} are see-through", what);
        }

        assert!(
            crate::aim::PLASMA_CORE.max_element() < crate::aim::PLASMA.max_element(),
            "the electrode is brighter than what comes off it"
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
        let (middle, shell) = globe(reaches);
        let stands = pedestal(reaches);

        assert!(
            lamp(reaches).distance(middle) < shell,
            "the lamp is outside the glass"
        );
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
        drawn.extend(pieces(reaches).into_iter().map(|(b, _)| b));

        let corner = Aabb::from_center_size(
            pedestal(reaches).center(),
            Vec3::splat(PEDESTAL * 2.0 + 1.0),
        );

        for fight in Fights::of(drawn, 0, 0, &looking).fights.iter() {
            assert!(
                !corner.contains_point(fight.patch.center()),
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
        let (middle, shell) = globe(reaches);
        let ball = Aabb::from_center_size(middle, Vec3::splat(shell * 2.0));
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

        for one in mine.iter().chain(std::iter::once(&ball)) {
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
}
