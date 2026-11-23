//! A map in the corner of the screen. Spec 0132.
//!
//! **The same cut as the wall plan, drawn small and following you.** Spec
//! 0115 takes a storey's open floor off the colliders at waist height and
//! lays it flat; this draws that, centred on where you are standing, at a
//! scale where you can see the room you are in and the ones either side of
//! it rather than the whole building.
//!
//! **It turns under you, the way GTA's does.** Jake asked for north up
//! first and for this second, and the second is what a map in a corner is
//! for: the question it answers is what is through the wall in front of
//! you, and an answer that needs the reader to rotate the building in their
//! head is a plan on a wall, which this building already has. What it costs
//! is the building's own north, which stops being the same way twice.
//!
//! It wanted two things the overlay could not do, and they are engine specs
//! 0112 and 0113 now: a quad that turns about its own middle, and a quad cut
//! to a window of the screen. The first draws the rooms; the second stops
//! them running out across the floor, which is what an axis-aligned panel
//! used to do by shrinking each rectangle until it fitted.
//!
//! **A dot and a nose, still.** The nose no longer turns, because the map
//! does. It stands straight up the panel, which is now what facing means.

use blitzkit::collision::Aabb;
use glam::{vec2, Vec2, Vec3};

/// How big the panel is, how far in from the window's corner it sits, and
/// how wide its border is, all in pixels.
///
/// **Bottom right and not bottom left.** GTA's is on the left, and so is
/// this building's frame-cost readout, which is spec 0056 and was there
/// first.
pub const WIDE: f32 = 224.0;
pub const IN: f32 = 22.0;
pub const EDGE: f32 = 3.0;

/// How far the map reaches from its middle, in metres.
///
/// **Twelve, which is a room and its neighbours.** The building is 46 along
/// and the hall alone is 19: at the whole building's scale this is spec
/// 0115 again, badly, and the thing a map in the corner answers is not
/// where everything is but what is through the wall in front of you.
pub const REACH: f32 = 12.0;

/// The dot that says where you are, the nose in front of it, and how far out
/// that nose stands, all in pixels.
pub const MARK: f32 = 9.0;
pub const NOSE: f32 = 5.0;
pub const NOSE_OUT: f32 = 13.0;

/// Where the panel sits: its middle and its size, in pixels, measured from
/// the window's top left with y running down.
pub fn panel(window: (f32, f32)) -> (Vec2, Vec2) {
    (
        vec2(window.0 - IN - WIDE * 0.5, window.1 - IN - WIDE * 0.5),
        Vec2::splat(WIDE),
    )
}

/// The panel less its border, which is what the map is drawn inside and what
/// every piece of it is cut to.
pub fn inside(window: (f32, f32)) -> (Vec2, Vec2) {
    let (middle, size) = panel(window);
    let half = size * 0.5 - Vec2::splat(EDGE);

    (middle - half, middle + half)
}

/// How many pixels a metre is on it.
pub fn across() -> f32 {
    WIDE * 0.5 / REACH
}

/// Which way the whole map is turned, in radians, clockwise on the screen.
///
/// **One angle for the map and not one for each room.** Every rectangle on
/// it turns by this, the panel included, so it is worked out once and read
/// where it is wanted. Two places saying how far the map has turned is the
/// fault this building has written down under nine other names.
///
/// Nought is facing the building's -z, which is yaw nought: the map is then
/// the page the wall plan is drawn on, turned by nothing.
pub fn turn(facing: Vec3) -> f32 {
    let way = heading(facing);

    (-way.x).atan2(-way.y)
}

/// The way you are facing, flattened and made a unit, in the panel's own
/// axes: x east, y north, before any turning.
///
/// Straight up when you are looking at your feet, which is a direction a
/// normalise cannot give and a map still has to draw something for.
fn heading(facing: Vec3) -> Vec2 {
    let way = vec2(facing.x, facing.z);

    if way.length_squared() > 1e-6 {
        way.normalize()
    } else {
        vec2(0.0, 1.0)
    }
}

/// Where a spot in the building lands on the panel.
///
/// **What you face is up, and what is on your right is right.** The second
/// half is the one that was wrong, and a mirrored map is a map: it reads,
/// it turns, every room is the right shape and every distance is right, and
/// it sends you the wrong way down the hall. Jake found it in a minute, from
/// the first screenshot of it.
///
/// Your right is `facing` a quarter turn further on, which `main`'s own two
/// lines settle: the mouse's x is added to the yaw, so more yaw is a turn to
/// the right, and `facing` is `(sin yaw, -cos yaw)`, so a quarter more takes
/// `(x, z)` to `(-z, x)`. That is [`right_of`], and this used its negative.
pub fn onto(window: (f32, f32), at: Vec3, facing: Vec3, spot: Vec2) -> Vec2 {
    let (middle, _) = panel(window);
    let way = heading(facing);
    let right = right_of(way);
    let off = vec2(spot.x - at.x, spot.y - at.z);

    middle + vec2(off.dot(right), -off.dot(way)) * across()
}

/// The right-hand side of a heading, in the same flattened x and z.
///
/// **A quarter turn further round, which is what the game does with it.**
/// Written once and called, rather than as a pair of signs in the one place
/// that needed it, because a pair of signs is what got it backwards.
pub fn right_of(way: Vec2) -> Vec2 {
    vec2(-way.y, way.x)
}

/// The open floor of the storey you are on, as rectangles on the panel, each
/// to be drawn turned by [`turn`].
///
/// **Dropped rather than trimmed when it is out of the panel.** A turned
/// rectangle and a square panel overlap in a polygon, so the old trim is
/// gone and engine spec 0113 cuts what is left at the edge. What this drops
/// is only what cannot reach the panel at all, measured round each
/// rectangle's own corner, so nothing that would have shown is thrown away.
pub fn rooms(window: (f32, f32), at: Vec3, facing: Vec3, filled: &[Aabb]) -> Vec<(Vec2, Vec2)> {
    let (middle, _) = panel(window);
    let corner = Vec2::splat(WIDE * 0.5).length();

    filled
        .iter()
        .filter_map(|box_| {
            let one = onto(window, at, facing, vec2(box_.min.x, box_.min.z));
            let two = onto(window, at, facing, vec2(box_.max.x, box_.max.z));
            let spot = (one + two) * 0.5;
            let size = vec2(
                (box_.max.x - box_.min.x) * across(),
                (box_.max.z - box_.min.z) * across(),
            );

            (spot.distance(middle) <= corner + size.length() * 0.5).then_some((spot, size))
        })
        .collect()
}

/// Which station the map marks, and nothing at all until the building has
/// given something up.
///
/// **Spec 0036's rule, written where it can be tested.** Somebody who never
/// finds the trail should not be able to tell it is there, and this map is
/// on by default, so an untouched run must show an ordinary map of an
/// ordinary building. Left as an `if` inside the drawing, that rule is a
/// line nothing can check, which is how the same drawing came to hold a
/// mirrored map earlier the same day.
///
/// **One, never a list.** `bridge::still_held` is spec 0118's own answer,
/// used here rather than answered again, so the map and the stone in the
/// field name the same place.
pub fn marks(trail: &crate::trail::Trail) -> Option<crate::trail::Asked> {
    if trail.is_empty() {
        return None;
    }

    crate::bridge::still_held(trail)
}

/// How big the mark for the trail's next station is, how far in from the
/// panel's edge it is pinned when that station is off the map, and the tick
/// that says it is on another storey.
pub const STATION: f32 = 11.0;
pub const STATION_IN: f32 = 15.0;
pub const STOREY: Vec2 = vec2(13.0, 3.0);
pub const STOREY_OUT: f32 = 10.0;

/// Where the station's mark goes, and whether it had to be pinned to the
/// rim because the station is off the map.
///
/// **Pinned on its own bearing and not clamped per axis.** The panel is
/// square, so clamping x and y one at a time is cheaper and bends the
/// direction at the corners: a station due north-east comes out pointing
/// north, which is the one thing a rim marker is for. Scaling the whole
/// offset until its larger side touches the rim keeps the bearing exact and
/// costs a divide.
///
/// The map reaches twelve metres and the building is 46 long plus a field,
/// so pinned is the usual answer rather than the edge case.
pub fn station(window: (f32, f32), at: Vec3, facing: Vec3, target: Vec3) -> (Vec2, bool) {
    let (middle, _) = panel(window);
    let spot = onto(window, at, facing, vec2(target.x, target.z));
    let half = WIDE * 0.5 - STATION_IN;
    let off = spot - middle;
    let most = off.x.abs().max(off.y.abs());

    if most <= half || most < 1e-4 {
        (spot, false)
    } else {
        (middle + off * (half / most), true)
    }
}

/// The mark: where you are, and which way you are looking.
///
/// **It does not turn any more.** The map does, so the nose stands straight
/// up the panel and up is what you are facing.
pub fn mark(window: (f32, f32)) -> [(Vec2, Vec2); 2] {
    let (middle, _) = panel(window);

    [
        (middle, Vec2::splat(MARK)),
        (middle - vec2(0.0, NOSE_OUT), Vec2::splat(NOSE)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    fn window() -> (f32, f32) {
        (1600.0, 1200.0)
    }

    fn building() -> crate::room::Room {
        crate::room::Room::of(crate::games())
    }

    /// Where a body can stand on each storey, as a handful of spots spread
    /// over the building rather than one.
    fn about(room: &crate::room::Room) -> Vec<Vec3> {
        let mut out = Vec::new();

        for floor in crate::plan::floors() {
            let fill = crate::plan::filled(room, floor);

            for n in [0, fill.len() / 3, fill.len() / 2, fill.len() - 1] {
                let box_ = fill[n.min(fill.len() - 1)];

                out.push(vec3(
                    (box_.min.x + box_.max.x) * 0.5,
                    floor + crate::plan::WAIST,
                    (box_.min.z + box_.max.z) * 0.5,
                ));
            }
        }

        out
    }

    /// Spec 0132: what is in front of you is up the panel, whichever way
    /// you are standing.
    ///
    /// **The whole of what turning the map buys.** Written as a spot put
    /// five metres ahead in the building and read back off the panel, so a
    /// mapping that happens to be right for one heading cannot pass.
    #[test]
    fn what_is_in_front_of_you_is_up_the_panel() {
        let window = window();
        let (middle, _) = panel(window);
        let at = vec3(3.0, 0.0, -4.0);

        for degrees in [0.0_f32, 37.0, 90.0, 180.0, 250.0, 359.0] {
            let facing = vec3(degrees.to_radians().sin(), 0.0, degrees.to_radians().cos());
            let ahead = at + facing * 5.0;
            let spot = onto(window, at, facing, vec2(ahead.x, ahead.z));

            assert!(
                (spot.x - middle.x).abs() < 1e-2,
                "facing {} a spot straight ahead sits {:.3} px to the side",
                degrees,
                spot.x - middle.x
            );
            assert!(
                spot.y < middle.y - 1.0,
                "facing {} a spot straight ahead sits at {:.3} and the mark is at {:.3}",
                degrees,
                spot.y,
                middle.y
            );
        }
    }

    /// Spec 0132: what is on your right is right of the mark.
    ///
    /// **The half that was missing, and the half that was wrong.** A map
    /// mirrored about the way you are looking reads, turns, gets every shape
    /// and every distance right, and sends you the wrong way down the hall.
    /// `what_is_in_front_of_you_is_up_the_panel` passes on one, because
    /// forward is the axis a mirror leaves alone.
    ///
    /// **Which way is right comes from `main` and not from this module.**
    /// `self.yaw += delta.x * LOOK`, so more yaw is a turn to the right, and
    /// `facing` is `(sin yaw, sin pitch, -cos yaw)`. Both are written out
    /// here rather than called, so that `minimap` agreeing with itself is
    /// not what passes.
    #[test]
    fn what_is_on_your_right_is_right_of_the_mark() {
        let window = window();
        let (middle, _) = panel(window);
        let at = vec3(3.0, 0.0, -4.0);
        let looking = |yaw: f32| vec3(yaw.sin(), 0.0, -yaw.cos());

        for degrees in [0.0_f32, 37.0, 90.0, 180.0, 250.0, 359.0] {
            let yaw = degrees.to_radians();
            let facing = looking(yaw);
            let beside = at + looking(yaw + std::f32::consts::FRAC_PI_2) * 5.0;
            let spot = onto(window, at, facing, vec2(beside.x, beside.z));

            assert!(
                spot.x > middle.x + 1.0,
                "facing {} a spot at your right hand sits at {:.3} and the mark is at {:.3}",
                degrees,
                spot.x,
                middle.x
            );
            assert!(
                (spot.y - middle.y).abs() < 1e-2,
                "facing {} a spot at your right hand sits {:.3} px up or down the panel",
                degrees,
                spot.y - middle.y
            );
        }
    }

    /// Spec 0132: the map's turn and the mapping agree, because a rectangle
    /// drawn at `onto`'s spot and turned by anything else is a room sliding
    /// off its own floor.
    ///
    /// **Measured as a corner rather than as an angle.** The turn goes to
    /// the engine and the spot comes from here, and the thing that has to
    /// hold is that the two land a corner in the same place.
    #[test]
    fn the_turn_and_the_mapping_are_one_angle() {
        let window = window();
        let at = vec3(0.0, 0.0, 0.0);

        for degrees in [0.0_f32, 37.0, 90.0, 180.0, 250.0] {
            let facing = vec3(degrees.to_radians().sin(), 0.0, degrees.to_radians().cos());
            let box_ = (vec2(2.0, 1.0), vec2(6.0, 4.0));

            let one = onto(window, at, facing, box_.0);
            let two = onto(window, at, facing, box_.1);
            let spot = (one + two) * 0.5;
            let size = vec2(
                (box_.1.x - box_.0.x) * across(),
                (box_.1.y - box_.0.y) * across(),
            );

            // The engine turns a quad's corners about its middle, and with
            // the map's own axes lying along the building's x and z, the
            // corner at the smallest of both is the rectangle's own top
            // left.
            let (sin, cos) = turn(facing).sin_cos();
            let off = -size * 0.5;
            let drawn = spot + vec2(off.x * cos - off.y * sin, off.x * sin + off.y * cos);

            assert!(
                drawn.distance(one) < 1e-2,
                "facing {} the engine draws the corner at {:?} and the mapping puts it at {:?}",
                degrees,
                drawn,
                one
            );
        }
    }

    /// Spec 0132: the floor you are standing on is under the mark.
    ///
    /// **The one thing a map in a corner has to get right.** Everything
    /// else about it is decoration: if what is in the middle is not where
    /// you are, it is a picture and not a map.
    #[test]
    fn what_you_are_standing_on_is_under_the_mark() {
        let room = building();
        let window = window();
        let (middle, _) = panel(window);

        for at in about(&room) {
            let fill =
                crate::plan::filled(&room, crate::plan::floors()[crate::plan::storey_at(at.y)]);
            let facing = vec3(0.3, 0.0, -0.9);
            let drawn = rooms(window, at, facing, &fill);
            let (sin, cos) = turn(facing).sin_cos();

            // back into the rectangle's own axes, which is where its size
            // still says what it covers
            let under = drawn.into_iter().any(|(spot, size)| {
                let off = middle - spot;
                let own = vec2(off.x * cos + off.y * sin, -off.x * sin + off.y * cos);

                own.x.abs() <= size.x * 0.5 + 1e-3 && own.y.abs() <= size.y * 0.5 + 1e-3
            });

            assert!(
                under,
                "standing at {:?} the middle of the map is bare floor",
                at
            );
        }
    }

    /// Spec 0132: nothing is offered that could not reach the panel, and
    /// everything that could is still offered.
    ///
    /// **The cull is allowed to be generous and not allowed to be mean.**
    /// Engine spec 0113 cuts what is drawn at the panel's edge, so a
    /// rectangle kept needlessly costs a few pixels nobody sees; one dropped
    /// wrongly is a hole in the floor.
    #[test]
    fn nothing_that_could_show_is_dropped() {
        let room = building();
        let window = window();
        let (low, high) = inside(window);

        for at in about(&room) {
            let fill =
                crate::plan::filled(&room, crate::plan::floors()[crate::plan::storey_at(at.y)]);
            let facing = vec3(0.0, 0.0, 1.0);
            let kept = rooms(window, at, facing, &fill);

            assert!(!kept.is_empty(), "the map is empty at {:?}", at);

            for box_ in &fill {
                let corners = [
                    vec2(box_.min.x, box_.min.z),
                    vec2(box_.max.x, box_.min.z),
                    vec2(box_.max.x, box_.max.z),
                    vec2(box_.min.x, box_.max.z),
                ];
                let touches = corners.iter().any(|corner| {
                    let spot = onto(window, at, facing, *corner);

                    spot.x >= low.x && spot.x <= high.x && spot.y >= low.y && spot.y <= high.y
                });

                if !touches {
                    continue;
                }

                let one = onto(window, at, facing, corners[0]);
                let two = onto(window, at, facing, corners[2]);
                let spot = (one + two) * 0.5;

                assert!(
                    kept.iter().any(|(kept, _)| kept.distance(spot) < 1e-2),
                    "a room with a corner on the panel was dropped at {:?}",
                    spot
                );
            }
        }
    }

    /// Spec 0132: it is of the storey you are on, which is the thing a
    /// three storey building makes it easy to get wrong.
    #[test]
    fn two_storeys_do_not_draw_the_same_map() {
        let room = building();
        let window = window();
        let floors = crate::plan::floors();
        let facing = vec3(0.0, 0.0, 1.0);
        let under: Vec<usize> = floors
            .iter()
            .map(|floor| crate::plan::filled(&room, *floor).len())
            .collect();

        assert!(
            under[0] != under[1] || under[1] != under[2],
            "the three storeys cut to the same number of pieces: {:?}",
            under
        );

        let at = vec3(0.0, floors[1] + crate::plan::WAIST, 0.0);
        let ground = rooms(window, at, facing, &crate::plan::filled(&room, floors[1]));
        let upper = rooms(
            window,
            vec3(at.x, floors[2] + crate::plan::WAIST, at.z),
            facing,
            &crate::plan::filled(&room, floors[2]),
        );

        assert!(
            ground != upper,
            "the hall and the floor over it draw the same map"
        );
    }

    /// Spec 0132: a station inside the map's reach is drawn where it
    /// stands, and one outside it is pinned to the rim without the bearing
    /// moving.
    ///
    /// **The bearing is the whole of what a pinned marker says.** It cannot
    /// say how far, it cannot say what room; all it has is which way to set
    /// off, so a pin that bends the direction is a pin that lies.
    #[test]
    fn a_station_off_the_map_keeps_its_bearing_on_the_rim() {
        let window = window();
        let (middle, _) = panel(window);
        let at = vec3(0.0, 0.0, 0.0);
        let facing = vec3(0.0, 0.0, 1.0);
        let half = WIDE * 0.5 - STATION_IN;

        // near enough to draw where it stands
        let near = vec3(3.0, 0.0, 2.0);
        let (spot, pinned) = station(window, at, facing, near);

        assert!(!pinned, "a station three metres off was pinned to the rim");
        assert!(
            spot.distance(onto(window, at, facing, vec2(near.x, near.z))) < 1e-3,
            "a station on the map is not drawn where it stands"
        );

        // and far enough to pin, from several ways round
        for degrees in [0.0_f32, 31.0, 90.0, 147.0, 215.0, 300.0] {
            let turn = degrees.to_radians();
            let far = vec3(turn.sin() * 80.0, 0.0, turn.cos() * 80.0);
            let want = onto(window, at, facing, vec2(far.x, far.z)) - middle;
            let (spot, pinned) = station(window, at, facing, far);
            let got = spot - middle;

            assert!(pinned, "a station eighty metres off was drawn on the map");
            assert!(
                got.x.abs() <= half + 1e-3 && got.y.abs() <= half + 1e-3,
                "a pinned station sits at {:?} and the rim is {:.1}",
                got,
                half
            );
            assert!(
                (got.normalize().dot(want.normalize()) - 1.0).abs() < 1e-4,
                "facing {} the bearing moved from {:?} to {:?}",
                degrees,
                want.normalize(),
                got.normalize()
            );
        }
    }

    /// Spec 0036: the map says nothing about the trail until the trail has
    /// started, and spec 0118: after that it says one thing.
    ///
    /// **The whole of what was argued over before this was built.** Jake
    /// asked for status indicators for the places that unlock valhalla;
    /// 0036 forbids telling somebody the trail exists before they find it.
    /// This is where the two were settled, so a later hand moving the test
    /// has to move the argument with it.
    #[test]
    fn the_map_says_nothing_until_the_trail_has_started() {
        use crate::trail::{Asked, Trail};

        assert_eq!(
            marks(&Trail::new()),
            None,
            "an untouched run is told there is a trail"
        );

        let mut started = Trail::new();
        started.gave(Asked::every()[0]);

        assert_eq!(
            marks(&started),
            Some(Asked::every()[1]),
            "a started trail does not name the next station"
        );

        let mut done = Trail::new();
        for what in Asked::every() {
            done.gave(*what);
        }

        assert_eq!(
            marks(&done),
            None,
            "a finished trail still has the map pointing somewhere"
        );
    }
}
