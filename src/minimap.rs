//! A map in the corner of the screen. Spec 0132.
//!
//! **The same cut as the wall plan, drawn small and following you.** Spec
//! 0115 takes a storey's open floor off the colliders at waist height and
//! lays it flat; this draws that, centred on where you are standing, at a
//! scale where you can see the room you are in and the ones either side of
//! it rather than the whole building.
//!
//! **North up, not turning with you.** Jake asked for a HUD like GTA's,
//! whose map turns under a fixed arrow. The overlay draws rectangles square
//! to the screen and nothing else: no rotation, and no rotation in a quad's
//! uv either. A turning map therefore means rasterising the whole panel into
//! a texture every frame, which is a different piece of work and is written
//! down in the spec rather than guessed at. What a north-up map loses is the
//! "forward is up" reading; what it keeps is that the building's own north
//! is always the same way, which is the other way round and is not worse.
//!
//! **A dot and a nose, for the same reason.** An arrow that turns is not a
//! thing a quad can be, and two squares are: the one in front says which way
//! you face. It is what spec 0115's `here` already does on the wall, which
//! is the building's own answer to this exact question.

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

/// The panel less its border, which is what the map is drawn inside.
pub fn inside(window: (f32, f32)) -> (Vec2, Vec2) {
    let (middle, size) = panel(window);
    let half = size * 0.5 - Vec2::splat(EDGE);

    (middle - half, middle + half)
}

/// How many pixels a metre is on it.
pub fn across() -> f32 {
    WIDE * 0.5 / REACH
}

/// Where a spot in the building lands on the panel.
///
/// **North up and east right, which is a page**, and the same reading the
/// wall plan is drawn to. Screen y runs down, so north is minus y.
pub fn onto(window: (f32, f32), at: Vec3, spot: Vec2) -> Vec2 {
    let (middle, _) = panel(window);

    middle + vec2(spot.x - at.x, at.z - spot.y) * across()
}

/// The open floor of the storey you are on, as rectangles on the panel.
///
/// Clipped to the panel rather than drawn over the whole window, because a
/// quad has no scissor of its own and the building is a great deal wider
/// than twenty-four metres.
pub fn rooms(window: (f32, f32), at: Vec3, filled: &[Aabb]) -> Vec<(Vec2, Vec2)> {
    let (low, high) = inside(window);

    filled
        .iter()
        .filter_map(|box_| {
            let one = onto(window, at, vec2(box_.min.x, box_.min.z));
            let two = onto(window, at, vec2(box_.max.x, box_.max.z));
            let least = one.min(two).max(low);
            let most = one.max(two).min(high);

            (most.x > least.x && most.y > least.y).then(|| ((least + most) * 0.5, most - least))
        })
        .collect()
}

/// The mark: where you are, and which way you are looking.
pub fn mark(window: (f32, f32), facing: Vec3) -> [(Vec2, Vec2); 2] {
    let (middle, _) = panel(window);
    let way = vec2(facing.x, facing.z);
    let way = if way.length_squared() > 1e-6 {
        way.normalize()
    } else {
        vec2(0.0, -1.0)
    };

    [
        (middle, Vec2::splat(MARK)),
        // north is minus y on the panel, so the nose turns with the map
        (middle + vec2(way.x, -way.y) * NOSE_OUT, Vec2::splat(NOSE)),
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

    /// Spec 0132: nothing of the map is drawn outside its own panel.
    ///
    /// **Which a quad cannot be told.** The overlay has no scissor, so a
    /// rectangle that runs off the panel runs across the room, and the
    /// building is four times wider than the map reaches.
    #[test]
    fn nothing_of_it_is_drawn_outside_the_panel() {
        let room = building();
        let window = window();
        let (low, high) = inside(window);

        for at in about(&room) {
            let fill =
                crate::plan::filled(&room, crate::plan::floors()[crate::plan::storey_at(at.y)]);
            let drawn = rooms(window, at, &fill);

            assert!(!drawn.is_empty(), "the map is empty at {:?}", at);

            for (middle, size) in drawn.iter().chain(mark(window, Vec3::Z).iter()) {
                let (least, most) = (middle - size * 0.5, middle + size * 0.5);

                assert!(
                    least.x >= low.x - 1e-3
                        && least.y >= low.y - 1e-3
                        && most.x <= high.x + 1e-3
                        && most.y <= high.y + 1e-3,
                    "a piece of the map runs {:?}..{:?} and the panel holds {:?}..{:?}",
                    least,
                    most,
                    low,
                    high
                );
            }
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
            let under = rooms(window, at, &fill).into_iter().any(|(spot, size)| {
                (spot.x - middle.x).abs() <= size.x * 0.5 + 1e-3
                    && (spot.y - middle.y).abs() <= size.y * 0.5 + 1e-3
            });

            assert!(
                under,
                "standing at {:?} the middle of the map is bare floor",
                at
            );
        }
    }

    /// Spec 0132: the nose stands on the side of the dot you are facing,
    /// with north up and east right.
    #[test]
    fn the_nose_points_the_way_you_face() {
        let window = window();
        let (middle, _) = panel(window);

        for (facing, toward) in [
            (vec3(0.0, 0.0, 1.0), vec2(0.0, -1.0)),
            (vec3(0.0, 0.0, -1.0), vec2(0.0, 1.0)),
            (vec3(1.0, 0.0, 0.0), vec2(1.0, 0.0)),
            (vec3(-1.0, 0.0, 0.0), vec2(-1.0, 0.0)),
        ] {
            let [(dot, _), (nose, _)] = mark(window, facing);

            assert_eq!(dot, middle);
            assert!(
                (nose - middle).normalize().distance(toward) < 1e-3,
                "facing {:?} the nose stands at {:?} off the dot and should be {:?}",
                facing,
                (nose - middle).normalize(),
                toward
            );
        }
    }

    /// Spec 0132: it is of the storey you are on, which is the thing a
    /// three storey building makes it easy to get wrong.
    #[test]
    fn two_storeys_do_not_draw_the_same_map() {
        let room = building();
        let window = window();
        let floors = crate::plan::floors();
        let (middle, _) = panel(window);
        // one spot in plan, three storeys: the cellar, the hall and the
        // floor over them are different rooms at the same x and z
        let spot = vec2(middle.x, middle.y);
        let _ = spot;
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
        let ground = rooms(window, at, &crate::plan::filled(&room, floors[1]));
        let upper = rooms(
            window,
            vec3(at.x, floors[2] + crate::plan::WAIST, at.z),
            &crate::plan::filled(&room, floors[2]),
        );

        assert!(
            ground != upper,
            "the hall and the floor over it draw the same map"
        );
    }
}
