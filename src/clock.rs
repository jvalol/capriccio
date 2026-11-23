//! The clock in the back room, and the train that runs it. Spec 0130.
//!
//! **In the load room, because it is about the building rather than in it.**
//! Spec 0024's back room is where the arcade keeps its instruments: a bench,
//! a block of cubes, and gauges saying what a frame costs. Jake, on where
//! this belongs: the load room, since it also has to do with representing
//! the overall mechanics of the building. A clock whose wheels are in plain
//! sight is the same kind of thing as a gauge that says what a frame costs,
//! and it was a utility room of its own for an hour before that was said.

use glam::{vec2, vec3, Quat, Vec2, Vec3};

/// How big a tooth is: the pitch diameter of a wheel is this times its
/// count.
///
/// **One module across the whole train**, so the distance between two
/// arbors is half of this times the sum of their tooth counts and nothing
/// is placed by hand. Nine millimetres, which makes the centre wheel of
/// sixty four a little over half a metre: big enough to count the teeth on
/// from across the room.
pub const MODULE: f32 = 0.009;

/// How far a tooth stands over the pitch circle and how far the root falls
/// under it, and how thick a wheel is.
pub const TOOTH_OUT: f32 = MODULE * 0.9;
pub const TOOTH_IN: f32 = MODULE * 1.1;
pub const WHEEL_THICK: f32 = 0.016;

/// The chapter ring: how far across it is and how wide its band.
/// **As large as the room will take.** The ceiling is the baths' own at
/// 2.4 and the pendulum wants a metre under the movement, so a ring any
/// bigger than this is a ring with its bob in the floor. A deeper room
/// would carry a grander one, and that is a step down through the doorway
/// and its own spec.
pub const RING: f32 = 1.84;
pub const RING_BAND: f32 = 0.125;

/// How high the middle of the dial stands off the floor.
/// **Higher and larger than the first one**, because the back room is 2.8
/// where the utility room was the baths' own 2.4. The pendulum wants a metre
/// under the escape arbor and the ring wants to clear the floor by more than
/// the bob swings.
pub const DIAL_UP: f32 = 1.76;

/// One wheel of the train: what it has on its arbor, where that arbor
/// stands in the face of the clock, and how many turns an hour it makes.
pub struct Wheel {
    /// The wheel's own teeth, which are what drives the next pinion.
    pub teeth: u32,
    /// The pinion on the same arbor, driven by the wheel before it. Nought
    /// for the first one, which is driven by nothing.
    pub pinion: u32,
    /// Where the arbor stands, from the middle of the dial.
    pub at: Vec2,
    /// Turns an hour, signed: a wheel drives its pinion the other way
    /// round.
    pub turns: f32,
}

/// How many turns an hour each hand makes, which is what the arbor it is on
/// makes.
pub const MINUTE_HAND: f32 = 1.0;
pub const HOUR_HAND: f32 = 1.0 / 12.0;
pub const SECOND_HAND: f32 = 60.0;

/// Where the going train's arbors stand, as the angle from the arbor before
/// them.
///
/// Down and to one side rather than straight down, so the escape wheel and
/// its pendulum stay inside the ring: straight down the train is longer
/// than the dial is wide.
pub const THIRD_WAY: f32 = 4.0;
pub const ESCAPE_WAY: f32 = 5.3;

/// The going train, from the arbor that carries the minute hand down to the
/// escape wheel that carries the seconds.
///
/// **The rates are worked out and not written down.** A wheel of `w` teeth
/// driving a pinion of `p` turns it `w / p` times as fast and the other way
/// round, so the only numbers here are tooth counts and the places they put
/// the arbors.
pub fn going() -> Vec<Wheel> {
    let mut out = vec![Wheel {
        teeth: 64,
        pinion: 0,
        at: Vec2::ZERO,
        turns: MINUTE_HAND,
    }];

    for (teeth, pinion, way) in [(60u32, 8u32, THIRD_WAY), (30, 8, ESCAPE_WAY)] {
        let before = out.last().expect("a wheel to be driven by");
        let apart = MODULE * (before.teeth + pinion) as f32 * 0.5;
        let (sin, cos) = way.sin_cos();

        out.push(Wheel {
            teeth,
            pinion,
            at: before.at + vec2(cos, sin) * apart,
            turns: -before.turns * before.teeth as f32 / pinion as f32,
        });
    }

    out
}

/// The motion work, which is the twelve to one that the hour hand wants.
///
/// **Two meshes of the same span.** The cannon pinion rides the minute
/// hand's own arbor and the hour wheel is concentric with it, so the minute
/// wheel between them is the same distance from the centre in both meshes:
/// the two pairs have to sum to the same number of teeth. Ten into thirty
/// and eight into thirty two is three times four, and both sums are forty.
pub fn motion() -> Vec<Wheel> {
    let apart = MODULE * (CANNON + MINUTE_WHEEL) as f32 * 0.5;

    vec![
        Wheel {
            teeth: CANNON,
            pinion: 0,
            at: Vec2::ZERO,
            turns: MINUTE_HAND,
        },
        Wheel {
            teeth: MINUTE_PINION,
            pinion: MINUTE_WHEEL,
            at: vec2(0.0, -apart),
            turns: -MINUTE_HAND * CANNON as f32 / MINUTE_WHEEL as f32,
        },
        Wheel {
            teeth: 0,
            pinion: HOUR_WHEEL,
            at: Vec2::ZERO,
            turns: MINUTE_HAND * CANNON as f32 / MINUTE_WHEEL as f32 * MINUTE_PINION as f32
                / HOUR_WHEEL as f32,
        },
    ]
}

pub const CANNON: u32 = 10;
pub const MINUTE_WHEEL: u32 = 30;
pub const MINUTE_PINION: u32 = 8;
pub const HOUR_WHEEL: u32 = 32;

/// How long a pendulum that beats seconds is.
///
/// **A fact about this planet and not a number that fits the room.** The
/// half period of a pendulum is pi root l over g, so a beat of one second
/// wants g over pi squared, which is very nearly a metre. Everything else
/// about this clock follows from the beat: one tooth of the escape wheel
/// every two of them, thirty teeth, a minute.
pub const GRAVITY: f32 = 9.80665;

pub fn pendulum_long() -> f32 {
    GRAVITY / (std::f32::consts::PI * std::f32::consts::PI)
}

/// How far the pendulum swings each way, in radians.
pub const SWING: f32 = 0.08;

/// Where the pendulum hangs from, which is the escape wheel's own arbor.
pub fn pendulum_at() -> Vec2 {
    going().last().map(|wheel| wheel.at).unwrap_or(Vec2::ZERO)
}

/// What time it is on the machine this is running on, as hours past
/// midnight.
///
/// **The player's own clock and not the building's.** The arcade's sky runs
/// a day in `garden::DAY`, which is twenty seconds, and this room ignores
/// it: the one thing a clock on a wall is for is agreeing with the watch on
/// your wrist. There is no readable alternative at that length anyway. A
/// minute hand on the building's own time would go round three times a
/// second and the escapement, whose whole point is one tick a second, would
/// beat a hundred and eighty times in one.
///
/// Read every frame rather than once, so a machine whose clock is set while
/// you are standing there follows it, and so does one that crosses a time
/// zone.
pub fn now() -> f32 {
    use chrono::Timelike;

    let now = chrono::Local::now();

    now.hour() as f32
        + now.minute() as f32 / 60.0
        + (now.second() as f32 + now.nanosecond() as f32 * 1e-9) / 3600.0
}

/// The hour a picture of this room is taken at.
///
/// **Ten past ten, which is where the hands sit in every photograph of a
/// clock ever taken**, and where the nook's own mantel clock is stuck. A
/// picture of a room whose whole point is the time has to be taken at one,
/// or the reference is a different picture every minute.
#[cfg(test)]
pub const SHOWN: f32 = 10.0 + 10.0 / 60.0;

/// How far round a wheel has turned at `hours` past midnight.
///
/// Clockwise seen from the room, which is the way a clock turns and the way
/// the hands on these arbors have to go.
pub fn turned(turns_an_hour: f32, hours: f32) -> f32 {
    -turns_an_hour * hours * std::f32::consts::TAU
}

/// Where the pendulum stands at `hours` past midnight, in radians off
/// upright.
pub fn swung(hours: f32) -> f32 {
    let seconds = hours * 3600.0;

    SWING * (seconds * std::f32::consts::PI).sin()
}

/// Which way a thing at `turn` lies in the face of the clock: across the
/// dial and up it, as a unit vector.
///
/// **One direction, read twice.** A hand is a bar as long as its own reach,
/// slid half that along the way it points, so where it sits and which way
/// it lies are the same number. Written out twice they came apart: the
/// offset ran one way round the dial and the rotation the other, which left
/// every hand a chord with neither end on the arbor it is meant to turn on.
/// Everything that turns on the face asks this, and `turned_to` is the same
/// answer as a rotation.
pub fn along(turn: f32) -> Vec2 {
    let (sin, cos) = turn.sin_cos();

    vec2(sin, cos)
}

/// The same turn as a rotation, to be applied after `facing`.
pub fn turned_to(turn: f32) -> Quat {
    Quat::from_rotation_x(turn)
}

/// The quarter turn that takes the clock's own face to the wall it hangs
/// on, which is the only thing in it that knows which way the room points.
pub fn facing() -> Quat {
    Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
}

/// Where a thing in the face of the clock stands in the room: across the
/// dial, up it, and out of the wall toward the floor you stand on.
pub fn stood(face: Vec3, at: Vec2, out: f32) -> Vec3 {
    face + facing() * vec3(out, at.y, at.x)
}

/// How many spokes a wheel is crossed out with, how far the hub runs and
/// where the rim starts, as shares of the wheel's own radius.
///
/// **Because a wheel that is not crossed out is a disc.** A clock wheel has
/// five or six crossings cut in it, and what you see through them is the
/// wheel behind: solid, the train came out as a stack of brass plates with
/// a fringe, which is not a movement.
pub const CROSSINGS: u32 = 5;
pub const HUB: f32 = 0.17;
pub const RIM_IN: f32 = 0.78;
pub const SPOKE: f32 = 0.09;

/// How far in the face of a wheel of `teeth` reaches at a share of the way
/// round it: the hub and the spokes, or the inside of the rim between them.
pub fn face_in(teeth: u32, on: f32) -> f32 {
    let out = MODULE * teeth as f32 * 0.5;
    let along = (on * CROSSINGS as f32).fract();
    let spoke = along < SPOKE * 0.5 || along > 1.0 - SPOKE * 0.5;

    if spoke || teeth <= 12 {
        out * HUB
    } else {
        out * RIM_IN
    }
}

/// How far out a wheel of `teeth` stands at a share of the way round it.
///
/// A trapezoid: flat across the tip, flat across the root, and a flank
/// between. Nothing here is an involute, which is a curve that matters to a
/// gear that carries a load and not to one you are looking at.
pub fn tooth_out(teeth: u32, on: f32) -> f32 {
    let pitch = MODULE * teeth as f32 * 0.5;
    let along = (on * teeth as f32).fract();
    // a tooth and a gap are half the pitch each, with a flank of a fifth
    // either side of the tooth
    let out = if along < 0.1 {
        along / 0.1
    } else if along < 0.4 {
        1.0
    } else if along < 0.5 {
        (0.5 - along) / 0.1
    } else {
        0.0
    };

    pitch - TOOTH_IN + (TOOTH_IN + TOOTH_OUT) * out
}

/// A wheel, as a mesh: a disc with its teeth cut, lying with its arbor
/// along x so that turning it is a turn about x.
pub fn wheel_mesh(teeth: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let half = WHEEL_THICK * 0.5;
    let mut mesh = blitzkit::mesh::MeshData::surface(teeth * 8, 8, |round, across| {
        let turn = -round * TAU;
        let out = tooth_out(teeth, round);
        let inner = face_in(teeth, round);
        // the face out from the hub or the inside of the rim, then the rim
        // itself, then the face again
        let (along, far) = if across < 0.25 {
            (half, inner + (out - inner) * (across / 0.25))
        } else if across < 0.75 {
            (half - WHEEL_THICK * (across - 0.25) / 0.5, out)
        } else {
            (-half, inner + (out - inner) * ((1.0 - across) / 0.25))
        };

        vec3(along, turn.cos() * far, turn.sin() * far)
    });
    mesh.compute_normals();

    mesh
}

/// Every tooth count the clock has a wheel of, each once.
///
/// **One mesh a count and not one a wheel.** Two wheels of sixty four teeth
/// are the same wheel, and a pinion of eight is a pinion of eight wherever
/// it rides.
pub fn counts() -> Vec<u32> {
    let mut out: Vec<u32> = going()
        .iter()
        .chain(motion().iter())
        .flat_map(|wheel| [wheel.teeth, wheel.pinion])
        .filter(|teeth| *teeth > 0)
        .collect();

    out.sort_unstable();
    out.dedup();

    out
}

/// A flat ring facing east, which is what the chapter ring is and what a
/// bob's rim wants.
pub fn ring_mesh(inner: f32, outer: f32, thick: f32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let mut mesh = blitzkit::mesh::MeshData::surface(96, 8, |round, across| {
        let turn = -round * TAU;
        let half = thick * 0.5;
        // the face, the outer rim, the back, and the inner rim
        let (along, far) = if across < 0.25 {
            (half, inner + (outer - inner) * (across / 0.25))
        } else if across < 0.5 {
            (half - thick * ((across - 0.25) / 0.25), outer)
        } else if across < 0.75 {
            (-half, outer - (outer - inner) * ((across - 0.5) / 0.25))
        } else {
            (-half + thick * ((across - 0.75) / 0.25), inner)
        };

        vec3(along, turn.cos() * far, turn.sin() * far)
    });
    mesh.compute_normals();

    mesh
}

/// Where the twelve hour marks stand on the ring, as the angle of each from
/// upright.
pub fn hours() -> Vec<f32> {
    (0..12)
        .map(|n| n as f32 / 12.0 * std::f32::consts::TAU)
        .collect()
}

/// How long each hand is, as a share of the ring's own radius, and how thick.
pub const HOUR_LONG: f32 = 0.56;
pub const MINUTE_LONG: f32 = 0.84;
pub const SECOND_LONG: f32 = 0.3;
pub const HAND_THICK: f32 = 0.022;

/// How long the pendulum's rod is from the arbor it hangs on, and how big
/// its bob.
pub fn rod_long() -> f32 {
    pendulum_long()
}

pub const BOB: f32 = 0.17;
pub const BOB_THICK: f32 = 0.035;

/// How far the movement stands off the wall, and how far apart the bars
/// that hold it are.
pub const OFF_THE_WALL: f32 = 0.17;
pub const PLATES: f32 = 0.1;

/// Where the clock's middle stands: on the back room's north wall, which is
/// the one clear wall in there.
///
/// **The wall you turn to, not the one you face.** The door is in the west
/// side, the bench is down the east and the bay with the rig in it takes the
/// whole south end: what is left is this, and it is five metres of nothing
/// at the one end of the room you can stand well back from.
pub fn dial_at(reaches: f32) -> Vec3 {
    let (_, east) = crate::back::span();
    let (_, north) = crate::back::deep(reaches);
    let floor = crate::back::floor(reaches).max.y;

    vec3(
        east - crate::back::WIDE * 0.5,
        floor + DIAL_UP,
        north - crate::room::THICK - OFF_THE_WALL,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0130: every meshing pair's arbors stand the distance their own
    /// tooth counts put them at.
    ///
    /// **One module across the train**, so the distance is half the module
    /// times the sum of the counts and nothing is placed by hand. A wheel
    /// that does not reach its pinion is a clock with a gap in it, which is
    /// the one fault a drawing of a movement can have that a drawing of a
    /// room cannot.
    #[test]
    fn the_wheels_mesh_because_of_their_teeth() {
        let going = going();
        // the tip and the root of a wheel as it is actually cut, taken off
        // the same profile the mesh is built from rather than off the
        // module: asked of the module, this test and the placement are the
        // same arithmetic twice and a wheel could be cut any size at all
        let cut = |teeth: u32| {
            let mut tip: f32 = 0.0;
            let mut root = f32::MAX;

            for step in 0..teeth * 32 {
                let out = tooth_out(teeth, step as f32 / (teeth * 32) as f32);

                tip = tip.max(out);
                root = root.min(out);
            }

            (tip, root)
        };

        for pair in going.windows(2) {
            let (driver, driven) = (&pair[0], &pair[1]);
            let apart = driver.at.distance(driven.at);
            let (wheel_tip, wheel_root) = cut(driver.teeth);
            let (pinion_tip, pinion_root) = cut(driven.pinion);

            assert!(
                wheel_tip + pinion_tip > apart,
                "a wheel of {} and a pinion of {} stand {} apart and their teeth reach {}, \
                 so they do not touch",
                driver.teeth,
                driven.pinion,
                apart,
                wheel_tip + pinion_tip
            );
            assert!(
                wheel_root + pinion_root < apart,
                "a wheel of {} and a pinion of {} stand {} apart and their roots reach {}, \
                 so they are cut into each other",
                driver.teeth,
                driven.pinion,
                apart,
                wheel_root + pinion_root
            );
        }

        // and the motion work, whose two meshes have to span the same
        // distance because the hour wheel is concentric with the cannon
        // pinion that drives the wheel between them
        assert_eq!(
            CANNON + MINUTE_WHEEL,
            MINUTE_PINION + HOUR_WHEEL,
            "the motion work's two meshes do not span the same distance"
        );

        let motion = motion();
        let want = MODULE * (CANNON + MINUTE_WHEEL) as f32 * 0.5;

        assert!(
            (motion[0].at.distance(motion[1].at) - want).abs() < 1e-4,
            "the cannon pinion does not reach the minute wheel"
        );
        assert!(
            (motion[2].at.distance(motion[1].at) - want).abs() < 1e-4,
            "the minute wheel's pinion does not reach the hour wheel"
        );
    }

    /// Spec 0130: the train divides an hour into a minute.
    #[test]
    fn the_train_divides_the_hour() {
        let going = going();
        let escape = going.last().expect("an escape wheel");

        assert_eq!(going.len(), 3, "the going train has {} wheels", going.len());
        assert!(
            (escape.turns.abs() - 60.0).abs() < 1e-3,
            "the escape wheel turns {} times an hour",
            escape.turns
        );
        assert_eq!(
            escape.teeth, 30,
            "the escape wheel has {} teeth",
            escape.teeth
        );

        // and the motion work is the twelve to one
        let hour = motion().last().expect("an hour wheel").turns;

        assert!(
            (hour.abs() - 1.0 / 12.0).abs() < 1e-4,
            "the hour wheel turns {} times an hour",
            hour
        );

        // which is what the hands are put on
        assert!((hour.abs() - HOUR_HAND).abs() < 1e-4);
        assert!((escape.turns.abs() - SECOND_HAND).abs() < 1e-3);
    }

    /// Spec 0130: the hands say the time they are given.
    ///
    /// **And the hour hand creeps**, which is the difference between a
    /// clock and a readout: at half past six it stands between six and
    /// seven, not on six.
    #[test]
    fn the_hands_say_the_time() {
        let round = std::f32::consts::TAU;
        // where a hand points, as a share of a turn clockwise from twelve
        let pointing = |turns: f32, hours: f32| {
            let at = -turned(turns, hours) / round;

            at.rem_euclid(1.0)
        };

        for (hours, minute, hour) in [
            (0.0f32, 0.0f32, 0.0f32),
            (3.0, 0.0, 0.25),
            (6.5, 0.5, 6.5 / 12.0),
            (9.25, 0.25, 9.25 / 12.0),
            (23.75, 0.75, 11.75 / 12.0),
        ] {
            assert!(
                (pointing(MINUTE_HAND, hours) - minute).abs() < 1e-4,
                "at {} the minute hand points at {} and wants {}",
                hours,
                pointing(MINUTE_HAND, hours),
                minute
            );
            assert!(
                (pointing(HOUR_HAND, hours) - hour).abs() < 1e-4,
                "at {} the hour hand points at {} and wants {}",
                hours,
                pointing(HOUR_HAND, hours),
                hour
            );
        }

        // the hour hand is between the hours and not on one
        let at_half_six = pointing(HOUR_HAND, 6.5);

        assert!(
            at_half_six > 6.0 / 12.0 && at_half_six < 7.0 / 12.0,
            "at half past six the hour hand points at {}",
            at_half_six
        );
    }

    /// Spec 0130: the pendulum beats seconds, and the escape wheel lets one
    /// tooth by every two beats.
    #[test]
    fn the_pendulum_beats_seconds() {
        // a beat is the swing from one side to the other, so it is back
        // where it started after two of them
        let at = |seconds: f32| swung(seconds / 3600.0);

        assert!(at(0.0).abs() < 1e-5, "it does not start upright");
        assert!(at(0.5) > SWING * 0.98, "it is not over at half a beat");
        assert!(
            at(1.5) < -SWING * 0.98,
            "it is not back at a beat and a half"
        );
        assert!(at(2.0).abs() < 1e-3, "it is not upright after two beats");

        // thirty teeth, one every two beats, is a minute
        let going = going();
        let escape = going.last().expect("an escape wheel");
        let a_tooth = 3600.0 / (escape.turns.abs() * escape.teeth as f32);

        assert!(
            (a_tooth - 2.0).abs() < 1e-4,
            "a tooth goes by every {} seconds, which is not two beats of one",
            a_tooth
        );

        // and the rod is the length a seconds pendulum has
        assert!(
            (rod_long() - 0.994).abs() < 0.01,
            "the pendulum is {} long, against the 0.994 a seconds one wants",
            rod_long()
        );
    }

    /// Spec 0130: a thing laid at a turn lies the way the face says it
    /// lies.
    ///
    /// **This is the one the drawn clock got wrong.** Where a hand sat came
    /// off `along` and which way it lay came off a rotation written
    /// separately, with the sign the other way about. Every hand was then a
    /// chord of the dial running from one arbitrary point to another, and
    /// the picture test passed because a reference only says what it looked
    /// like when somebody was happy with it.
    #[test]
    fn a_turn_lays_a_thing_the_way_the_face_says() {
        for step in 0..24 {
            let turn = turned(MINUTE_HAND, step as f32 / 24.0);
            // the way a bar along the clock's own upright ends up pointing
            let laid = facing() * turned_to(turn) * Vec3::Y;
            let says = along(turn);

            assert!(
                (laid - stood(Vec3::ZERO, says, 0.0)).length() < 1e-5,
                "at {} of a turn the face says {} and a thing laid on it points {}",
                turn,
                says,
                laid
            );
        }
    }

    /// Spec 0130: and the face runs clockwise for somebody standing in the
    /// room.
    ///
    /// **Which is not the same claim.** The two above it agree with each
    /// other whichever way round the dial goes; this is the one that knows
    /// the clock is on the north wall and that a player at it is looking
    /// north, so east is on their left and a hand at quarter past has to
    /// run west.
    #[test]
    fn the_face_runs_the_way_the_room_sees_a_clock_run() {
        const REACHES: f32 = 9.5;

        let face = dial_at(REACHES);
        let points = |hours: f32| stood(face, along(turned(MINUTE_HAND, hours)), 0.0) - face;

        // the room is south of the wall, so out of the face is toward it
        assert!(
            stood(face, Vec2::ZERO, 1.0).z < face.z,
            "the face of the clock looks into the wall"
        );

        let quarter = points(0.25);

        assert!(
            quarter.x < -0.99 && quarter.y.abs() < 0.01,
            "at quarter past the minute hand runs {}, which is not to the west",
            quarter
        );

        let half = points(0.5);

        assert!(
            half.y < -0.99 && half.x.abs() < 0.01,
            "at half past the minute hand runs {}, which is not downward",
            half
        );
    }
}
