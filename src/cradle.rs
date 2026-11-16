//! A Newton's cradle on a bench. Spec 0006.
//!
//! Five balls hung in a line, touching. Lift the one on the end and let it go,
//! and it stops dead on the others while the one at the far end flies out. The
//! middle three barely move.
//!
//! It is the first thing in this project that asks the link solver and the
//! contact solver to work together rather than take turns: the ropes hold the
//! balls on their arcs while a blow travels the row through four contacts. Spec
//! 0041 gave the engine the ropes; this is what they were for.
//!
//! Where everything sits is arithmetic and is checked without a window.

use blitzkit::collision::Aabb;
use blitzkit::link::Link;
use blitzkit::physics::{Body, Solver};
use glam::{vec3, Vec3};

/// What the bench it stands on is called.
pub const NAME: &str = "newton's cradle";

/// How many balls hang.
pub const BALLS: usize = 5;

pub const BALL: f32 = 0.085;
pub const BALL_MASS: f32 = 1.0;

/// How long a rope is, from its hook to the middle of its ball.
pub const ROPE: f32 = 0.52;

/// The frame: how high the bar sits above the bench, and how thick the
/// uprights and the bar are.
pub const BAR_UP: f32 = ROPE + BALL * 2.0 + 0.18;
pub const STOCK: f32 = 0.045;

/// How far out an upright stands, measured from the middle.
///
/// From the swing and not from the row. Sized to the row, the frame was exactly
/// as long as the balls hanging still, and the first thing the end one did was
/// swing through it. The ropes hold the balls and nothing holds them off the
/// frame, so the frame has to be built where they never reach.
pub fn upright_at() -> f32 {
    let end = resting(0).z.abs();
    let swing = ROPE * LIFTED.sin();

    end + swing + BALL + STOCK
}

/// How much of a blow a ball gives back.
///
/// High, because the whole point is that the row passes it along rather than
/// swallowing it.
///
/// **0.92 was the brake, and it should not have been.** Spec 0100. Nothing
/// else here took any speed off a ball, so the only way to stop a row that
/// would otherwise drift for ever was to make every blow lossy, and that is
/// the note this constant used to carry: not one, or the thing never stops.
/// What it cost is the thing itself. Measured from a lift at `LIFTED`, the
/// row was down to four per cent of its energy in six seconds, where a real
/// cradle runs for minutes and a steel ball gives back 0.95 to 0.99 of a
/// blow.
///
/// `KEEPS` is the brake now, which is where the loss belongs: a swing dies
/// to the air over its whole arc, not to the one instant it touches the
/// next ball. At 0.99 with that drag the row still has 45 per cent at six
/// seconds and a swing you can see at eleven, and it is quiet by sixteen.
pub const BOUNCE: f32 = 0.99;

/// How far the end ball is lifted when it is set going, as an angle.
///
/// About a third of a right angle, which is what a hand does to one of these.
/// It was half a right angle, and at that the end ball swung clean through the
/// upright beside it.
pub const LIFTED: f32 = 0.6;

/// How fast the toy is stepped, and how many passes a step gets.
///
/// Faster and harder than the engine's default. A row of touching balls is the
/// case sequential impulses are worst at: each pass moves a blow one contact
/// along, so four contacts want four passes just to carry it end to end, and
/// that is before anything is solved.
pub const STEP: f32 = 1.0 / 240.0;
pub const PASSES: usize = 48;

/// Where a ball hangs when the row is at rest, measured from the middle of the
/// bench top.
///
/// Touching, not spaced: the gap between two balls is what a blow has to cross,
/// and a cradle with gaps is a row of pendulums that clack at random.
pub fn resting(ball: usize) -> Vec3 {
    let across = (ball as f32 - (BALLS - 1) as f32 * 0.5) * BALL * 2.0;

    vec3(0.0, -ROPE, across)
}

/// Where a ball's rope is hooked to the bar.
pub fn hook(ball: usize) -> Vec3 {
    let at = resting(ball);

    vec3(at.x, BAR_UP, at.z)
}

/// The bodies and the ropes, in a frame whose origin is the middle of the
/// bench top. The hooks come first, then the balls.
pub fn strung() -> (Vec<Body>, Vec<Link>) {
    let mut bodies = Vec::new();
    let mut links = Vec::new();

    for ball in 0..BALLS {
        bodies.push(Body::immovable(hook(ball), STOCK * 0.5));
    }
    for ball in 0..BALLS {
        bodies.push(
            Body::new(hook(ball) + Vec3::NEG_Y * ROPE, BALL, BALL_MASS)
                .with_restitution(BOUNCE)
                .with_friction(0.02),
        );
    }
    for ball in 0..BALLS {
        links.push(Link::rope(ball, BALLS + ball, ROPE));
    }

    (bodies, links)
}

/// Which body in `strung` is a given ball.
pub fn ball_at(ball: usize) -> usize {
    BALLS + ball
}

/// Puts every ball back where it hangs, still. Spec 0101.
///
/// **Placed and stilled rather than left to the drag.** A row set going
/// takes the better part of half a minute to be quiet, which is right when
/// you are watching it and wrong when you want to start again.
pub fn settle(bodies: &mut [Body]) {
    for ball in 0..BALLS {
        let body = &mut bodies[ball_at(ball)];

        body.position = hook(ball) + Vec3::NEG_Y * ROPE;
        body.velocity = Vec3::ZERO;
        body.spin = Vec3::ZERO;
        body.wake();
    }
}

/// Which way a ball swings out of the row: toward the end it is nearer.
///
/// **Its own function because two verbs want it.** Lifting a ball and
/// shoving one both need to know which side of the row it belongs to, and
/// `set_going` used to work it out inline for the one ball it could reach.
/// Spec 0098.
pub fn away(ball: usize) -> f32 {
    if ball * 2 < BALLS {
        -1.0
    } else {
        1.0
    }
}

/// Shoves a ball along the row rather than lifting it out of one.
///
/// **The chaotic one, and chaotic on purpose.** `set_going` places a ball
/// on its arc and lets gravity do the rest, which is the demonstration
/// everybody knows. This gives it a velocity where it hangs, and
/// `set_going`'s own note says what that does in a row already touching:
/// the ball spends it on the contact in front of it. What comes out is a
/// row that rings rather than a row that clicks, which is what Jake asked
/// for when he said making it chaotic is not bad either. Spec 0098.
pub fn shove(bodies: &mut [Body], ball: usize, along: f32) {
    let ball = ball_at(ball.min(BALLS - 1));

    bodies[ball].velocity += vec3(0.0, 0.0, along * SHOVE);
    bodies[ball].wake();
}

/// How hard a shove is, in metres a second.
///
/// A ball falling from `LIFTED` arrives at about 0.9, so this is a little
/// under what the cradle does to itself: a shove is a hand on a ball and
/// not a bat.
pub const SHOVE: f32 = 0.7;

/// Swings a ball up and lets it go.
///
/// Placed rather than shoved: a ball given a velocity in a row that is already
/// touching spends it on the contact in front of it before it has anywhere to
/// swing from.
pub fn set_going(bodies: &mut [Body], from_the_end: usize) {
    let ball = ball_at(from_the_end.min(BALLS - 1));
    let hook = hook(from_the_end.min(BALLS - 1));
    let away = away(from_the_end.min(BALLS - 1));

    bodies[ball].position = hook + vec3(0.0, -ROPE * LIFTED.cos(), away * ROPE * LIFTED.sin());
    bodies[ball].velocity = Vec3::ZERO;
    bodies[ball].spin = Vec3::ZERO;
    bodies[ball].wake();
}

/// How much of its speed a ball keeps each second, to air and to the rope.
///
/// **Because nothing here was taking any.** Spec 0100. Measured over twenty
/// seconds from a lift at `LIFTED`, the row kept 98 per cent of the energy
/// it started with and never gave the rest up: the swing stopped looking
/// like a swing within three seconds, not because it was dying but because
/// it had spread across all five balls as a buzz of a few centimetres each,
/// and there it stayed. Jake, playing: gravity doesn't slow things like the
/// cradle down realistically.
///
/// A real one is quiet in under a minute. This is a drag on velocity rather
/// than a floor on it, so it takes the buzz out faster than the swing: a
/// jitter at a tenth of the amplitude is a tenth of the speed and loses the
/// same share of it every step, which is what makes a row of five settle
/// into a row of five hanging still.
pub const KEEPS: f32 = 0.955;

/// One step of a cradle: the solver, and then what the air takes.
///
/// **Both halves or neither.** Spec 0100. The drag went into the game's
/// loop first and the toy's own tests stepped it without, so the row they
/// were watching was not the row the building draws: one of them put the
/// bounce back up and failed, which is how this was found. A thing that
/// knows how it is strung should know what it loses.
pub fn step(solver: &mut Solver, bodies: &mut [Body], links: &[Link], gravity: Vec3) {
    solver.step_linked(bodies, links, &NO_WORLD, gravity, STEP);
    drags(bodies, STEP);
}

/// Takes that much off every ball, once per step.
fn drags(bodies: &mut [Body], dt: f32) {
    let keeps = KEEPS.powf(dt);

    for ball in 0..BALLS {
        let body = &mut bodies[ball_at(ball)];

        body.velocity *= keeps;
        body.spin *= keeps;
    }
}

/// A solver set up the way a cradle wants one.
pub fn solver() -> Solver {
    Solver::new().with_passes(PASSES)
}

/// How much of the row is moving, which is what says whether it has settled.
pub fn stirring(bodies: &[Body]) -> f32 {
    (0..BALLS)
        .map(|ball| bodies[ball_at(ball)].velocity.length())
        .fold(0.0f32, f32::max)
}

/// Nothing to collide with: the balls meet each other and the ropes hold them.
/// The bench is below them and they never reach it.
pub const NO_WORLD: [Aabb; 0] = [];

#[cfg(test)]
mod tests {
    use super::*;

    const GRAVITY: Vec3 = Vec3::new(0.0, -9.81, 0.0);

    fn run(bodies: &mut [Body], links: &[Link], seconds: f32) {
        let mut solver = solver();
        for _ in 0..(seconds / STEP) as u32 {
            step(&mut solver, bodies, links, GRAVITY);
        }
    }

    #[test]
    fn the_balls_hang_touching() {
        let (bodies, _) = strung();

        for ball in 1..BALLS {
            let apart = bodies[ball_at(ball)]
                .position
                .distance(bodies[ball_at(ball - 1)].position);

            assert!(
                (apart - BALL * 2.0).abs() < 1e-5,
                "balls {} and {} hang {} apart",
                ball - 1,
                ball,
                apart
            );
        }
    }

    #[test]
    fn every_ball_hangs_from_its_own_hook() {
        let (bodies, links) = strung();

        assert_eq!(links.len(), BALLS);
        for (ball, link) in links.iter().enumerate() {
            assert!((link.span(&bodies) - ROPE).abs() < 1e-5);
            assert_eq!(link.one, ball);
            assert_eq!(link.other, ball_at(ball));
            assert_eq!(bodies[link.one].inverse_mass, 0.0, "the hook moved");
        }
    }

    #[test]
    fn a_row_left_alone_stays_put() {
        let (mut bodies, links) = strung();
        run(&mut bodies, &links, 3.0);

        for ball in 0..BALLS {
            let moved = bodies[ball_at(ball)]
                .position
                .distance(hook(ball) + Vec3::NEG_Y * ROPE);
            assert!(moved < 0.02, "ball {} drifted {}", ball, moved);
        }
    }

    #[test]
    fn lifting_one_end_swings_the_other() {
        let (mut bodies, links) = strung();
        set_going(&mut bodies, 0);

        // the blow has to cross four contacts, so give it the time to
        let mut furthest: f32 = 0.0;
        let mut solver = solver();
        for _ in 0..(1.2 / STEP) as u32 {
            solver.step_linked(&mut bodies, &links, &NO_WORLD, GRAVITY, STEP);

            let far =
                bodies[ball_at(BALLS - 1)].position.z - (hook(BALLS - 1) + Vec3::NEG_Y * ROPE).z;
            furthest = furthest.max(far);
        }

        assert!(
            furthest > BALL * 2.0,
            "the far ball only moved {}",
            furthest
        );
    }

    #[test]
    fn the_middle_is_left_alone() {
        let (mut bodies, links) = strung();
        let settled = hook(2) + Vec3::NEG_Y * ROPE;
        set_going(&mut bodies, 0);

        let mut worst: f32 = 0.0;
        let mut solver = solver();
        for _ in 0..(1.2 / STEP) as u32 {
            solver.step_linked(&mut bodies, &links, &NO_WORLD, GRAVITY, STEP);
            worst = worst.max(bodies[ball_at(2)].position.distance(settled));
        }

        // it is shoved about a little, and it does not swing out
        assert!(worst < BALL * 2.0, "the middle swung {}", worst);
    }

    /// Spec 0100: it is still swinging to watch after six seconds, and the
    /// swing keeps falling rather than settling on a buzz.
    ///
    /// **The second half is the fault and the first half is the cost of
    /// the old fix for it.** With nothing taking speed off a ball, the only
    /// brake was a lossy blow, so `BOUNCE` was 0.92 and the row went from a
    /// 0.294 metre swing to 0.061 in six seconds. It then sat at 0.061 for
    /// ever: at ten seconds, at sixteen, at twenty four, the same number.
    /// Not a cradle running down, a cradle buzzing.
    ///
    /// Measured as the widest any ball gets over a second and a half, which
    /// is what the eye reads as "still going". `stirring` is one instant and
    /// says 0.048 at three seconds and 0.405 at ten, because it depends on
    /// where in the swing it was asked.
    #[test]
    fn it_still_swings_and_keeps_running_down() {
        let widest_after = |seconds: f32| {
            let (mut bodies, links) = strung();
            set_going(&mut bodies, 0);
            let mut solver = solver();

            for _ in 0..(seconds / STEP) as u32 {
                step(&mut solver, &mut bodies, &links, GRAVITY);
            }

            let mut widest = 0.0f32;
            for _ in 0..(1.5 / STEP) as u32 {
                step(&mut solver, &mut bodies, &links, GRAVITY);
                for ball in 0..BALLS {
                    let off = bodies[ball_at(ball)].position.z - hook(ball).z;
                    widest = widest.max(off.abs());
                }
            }

            widest
        };

        let (six, sixteen) = (widest_after(6.0), widest_after(16.0));

        assert!(
            six > 0.15,
            "six seconds in, the widest swing is {:.3} m of a 0.294 start",
            six
        );
        assert!(
            sixteen < six * 0.5,
            "it swings {:.3} at six seconds and {:.3} at sixteen, which is a buzz and not a run down",
            six,
            sixteen
        );
    }

    #[test]
    fn it_runs_down_rather_than_on_forever() {
        let (mut bodies, links) = strung();
        set_going(&mut bodies, 0);
        run(&mut bodies, &links, 30.0);

        assert!(
            stirring(&bodies) < 0.4,
            "still going at {} after half a minute",
            stirring(&bodies)
        );
    }

    #[test]
    fn a_swinging_ball_never_reaches_an_upright() {
        let upright = upright_at();

        for end in [0, BALLS - 1] {
            let (mut bodies, links) = strung();
            set_going(&mut bodies, end);
            let mut solver = solver();

            for _ in 0..(4.0 / STEP) as u32 {
                solver.step_linked(&mut bodies, &links, &NO_WORLD, GRAVITY, STEP);

                for ball in 0..BALLS {
                    let reach = bodies[ball_at(ball)].position.z.abs() + BALL;
                    assert!(
                        reach < upright - STOCK * 0.5,
                        "ball {} reached {} against an upright at {}",
                        ball,
                        reach,
                        upright
                    );
                }
            }
        }
    }

    #[test]
    fn the_frame_is_wider_than_the_row() {
        assert!(upright_at() > resting(0).z.abs() + BALL);
    }
}
