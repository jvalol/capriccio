//! A relief plan of the building, cut from the building. Spec 0115.
//!
//! **Nothing here draws a plan. It cuts one.** `Room::solid` is every box a
//! body walks into, which is the building's own account of where its walls
//! are, and a plan is a horizontal cut through walls at about waist height.
//! So each storey is that list kept to what the cut passes through, laid
//! flat and stood on a wall.
//!
//! Drawn by hand it would be a second account of the layout, which is the
//! fault the engine's spec 0107 spent itself deleting one floor up. A wall
//! that moves and a plan that does not is that fault again, and the only
//! thing that would ever notice is a person in the nook comparing a carving
//! against a memory of a room.

use crate::room::{Room, TALL};
use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far above a storey's floor the cut is taken.
///
/// **A plan is cut at a body's waist**, which is what puts a doorway in it
/// as a gap rather than as a line: a door's head is above this and its
/// threshold below, so the cut passes through the opening and finds nothing.
pub const WAIST: f32 = 1.2;

/// How far the relief stands off the wall it is on.
///
/// The panelling's own `PROUD` is 0.02 and the rail twice that. This is less
/// than either, so the plan reads as cut into the wall's face rather than as
/// something hung on it.
pub const RELIEF: f32 = 0.012;

/// The gap between one storey's plan and the next, as a share of a plan's
/// own width.
pub const BETWEEN: f32 = 0.22;

/// The floor each of the three storeys stands on, lowest first.
///
/// **Three and not one, because the building is three.** Each is the floor
/// itself rather than the cut, so `WAIST` is added in one place.
pub fn floors() -> [f32; 3] {
    [-crate::cellar::DOWN, 0.0, crate::glass::UP]
}

/// Whether a room of the building belongs in a plan of it.
///
/// **The space behind the hall's end wall does not, and that is a statement
/// about scale.** It is the solar system at true scale: measured 2026-11-17,
/// the ground floor's cut gives 463 boxes and 360 of them are that one room,
/// running 156 metres east and 168 north against a building 35 by 46. A plan
/// cannot hold a corridor and an astronomical unit at one scale. The field
/// goes with it for the same reason, the ground out there not stopping.
pub fn drawn(which: crate::noise::Room) -> bool {
    !matches!(
        which,
        crate::noise::Room::Behind | crate::noise::Room::Field
    )
}

/// One storey's plan, in the building's own coordinates: every wall the cut
/// at `floor + WAIST` passes through.
pub fn cut(room: &Room, floor: f32) -> Vec<Aabb> {
    let at = floor + WAIST;

    room.solid()
        .into_iter()
        .filter(|box_| box_.min.y <= at && box_.max.y >= at)
        .filter(|box_| drawn(crate::noise::room_at(room.reaches, box_.center())))
        .collect()
}

/// The footprint all three storeys share, which is what makes them one
/// scale and what says where the building stops.
///
/// **The two storeys that bound themselves are what bound the third.** The
/// cellar and the floor above are each a handful of rooms and go no further;
/// the ground floor does not stop, because `room_at` calls the long way out
/// to the village and the bridge the hall, and asked without this the plan
/// came out at 1:115 with every wall of it two and a half millimetres wide.
///
/// **The lids are in it for the east side only**, which is measured rather
/// than argued: taking them out moves this footprint from x 10.4 to x -2.7
/// and moves nothing north or south at all. They are the building's own
/// answer to where it ends, which is the answer `Room::outside` builds the
/// skin from, and here they are the half of it the other two do not cover.
pub fn footprint(room: &Room) -> Aabb {
    let mut least = Vec3::splat(f32::MAX);
    let mut most = Vec3::splat(f32::MIN);
    let mut hold = |box_: Aabb| {
        least = least.min(box_.min);
        most = most.max(box_.max);
    };

    for lid in room.lids() {
        hold(lid);
    }

    for floor in [floors()[0], floors()[2]] {
        for box_ in cut(room, floor) {
            hold(box_);
        }
    }

    Aabb::new(vec3(least.x, 0.0, least.z), vec3(most.x, 0.0, most.z))
}

/// Whether a bar of a storey's cut is inside the building at all.
fn within(whole: &Aabb, box_: &Aabb) -> bool {
    let at = box_.center();

    at.x >= whole.min.x && at.x <= whole.max.x && at.z >= whole.min.z && at.z <= whole.max.z
}

/// How finely the floor is sampled when the rooms are filled in.
///
/// **Fifteen centimetres, which is three millimetres on the wall.** Half a
/// metre was the first go and its stepped edges were the thing you noticed
/// about the drawing. What makes this affordable is not the sampling, which
/// is tens of milliseconds at startup, but the merging under it: a row of
/// cells becomes one bar and a bar the same width as the one before it is
/// joined to it, so a hall is a handful of boxes however fine the grid.
///
/// A plan drawn as
/// the walls alone reads as scratches: Jake, looking at the first build, it
/// reads abstract, like ancient cave wall art, hard to tell it has anything
/// to do with the building. What a plan is read by is the shape of its
/// rooms, so the rooms are filled and the walls are what is left between
/// them.
pub const GRID: f32 = 0.15;

/// The open floor of one storey, as runs of cells that are not inside
/// anything solid.
///
/// **Asked of the colliders, like everything else here.** A cell is floor
/// if a body standing on it would be standing in air, which is the same
/// question `walk` asks and is the building's own account of its rooms.
pub fn filled(room: &Room, floor: f32) -> Vec<Aabb> {
    let whole = footprint(room);
    let at = floor + WAIST;
    let solid: Vec<Aabb> = room
        .solid()
        .into_iter()
        .filter(|box_| box_.min.y <= at && box_.max.y >= at)
        .collect();
    // **And what counts as a room is what has something over it.** The
    // first fill asked only whether a cell was clear of the colliders,
    // which is true of the whole county: outside the building there is
    // nothing in the way either, so all three storeys came out as solid
    // rectangles with a scratch of detail in them. A plan is of the inside.
    //
    // The garden has nothing over it and so is a hole in the fill, which is
    // what a plan does with a courtyard.
    let over: Vec<Aabb> = room
        .solid()
        .into_iter()
        .chain(room.lids())
        .filter(|box_| box_.min.y > at)
        .collect();
    // **And something under it to stand on.** A roof over your head is not
    // enough: below the hall's floor there is no collider at all, so the
    // cellar filled the whole footprint and was drawn 45 metres across
    // where the cellar is 22. What tells a room from earth is the slab it
    // stands on, which is a box whose top is this storey's floor.
    let under: Vec<Aabb> = room
        .solid()
        .into_iter()
        .chain(room.lids())
        .filter(|box_| (box_.max.y - floor).abs() < 0.05)
        .collect();
    let mut out = Vec::new();
    let mut z = whole.min.z;

    while z < whole.max.z {
        let mut run: Option<f32> = None;
        let mut x = whole.min.x;

        while x < whole.max.x {
            let here = vec3(x + GRID * 0.5, at, z + GRID * 0.5);
            let covers = |box_: &Aabb| {
                box_.min.x <= here.x
                    && box_.max.x >= here.x
                    && box_.min.z <= here.z
                    && box_.max.z >= here.z
            };
            let open = !solid.iter().any(covers)
                && over.iter().any(covers)
                && under.iter().any(covers)
                && drawn(crate::noise::room_at(room.reaches, here));

            match (open, run) {
                (true, None) => run = Some(x),
                (false, Some(from)) => {
                    out.push(Aabb::new(vec3(from, 0.0, z), vec3(x, 0.0, z + GRID)));
                    run = None;
                }
                _ => (),
            }

            x += GRID;
        }

        if let Some(from) = run {
            out.push(Aabb::new(
                vec3(from, 0.0, z),
                vec3(whole.max.x, 0.0, z + GRID),
            ));
        }

        z += GRID;
    }

    merged(out)
}

/// Runs joined to the run before them where the two are the same width,
/// which is what keeps a fine grid from costing a box a cell.
///
/// **A room's own shape does the work.** A straight wall gives the same run
/// row after row, so a hall forty rows deep is one box rather than forty,
/// and what is left a box a row is the stepped edge of something round.
fn merged(runs: Vec<Aabb>) -> Vec<Aabb> {
    let mut out: Vec<Aabb> = Vec::new();

    for run in runs {
        let joined = out.iter_mut().find(|held| {
            (held.min.x - run.min.x).abs() < 1e-4
                && (held.max.x - run.max.x).abs() < 1e-4
                && (held.max.z - run.min.z).abs() < 1e-4
        });

        match joined {
            Some(held) => held.max.z = run.max.z,
            None => out.push(run),
        }
    }

    out
}

/// The band of the nook's north wall the relief is allowed, as a box with no
/// thickness: the wall's inner face, between the dado rail and the cornice.
pub fn board(reaches: f32) -> Aabb {
    let (at, size) = crate::room::nook_end(reaches);
    let face = at.z - size.z * 0.5;
    // **Off the rail and off the cornice by a finger, and no more.** The
    // band between them is what sets the scale for all three, since the
    // building is longer north to south than the wall is tall, so every
    // centimetre given away here is given away three times over.
    let clear = 0.06;
    let low = crate::study::DADO + crate::study::RAIL * 0.5 + clear;
    let high = TALL - crate::study::CORNICE - clear;
    let half = size.x * 0.5 - 0.3;

    Aabb::new(vec3(at.x - half, low, face), vec3(at.x + half, high, face))
}

/// How big one storey's plan is on that board, and where the first one
/// starts.
///
/// **Scaled to the height and then checked against the width**, because the
/// band between a dado and a cornice is the tighter of the two: the
/// building is 46 metres north to south and the band is about one and a
/// half, where the wall is ten across.
fn laid_out(room: &Room, reaches: f32) -> (f32, f32, Vec3) {
    let board = board(reaches);
    let whole = footprint(room);
    let (across, along) = (whole.max.x - whole.min.x, whole.max.z - whole.min.z);
    let band = board.max.y - board.min.y;
    let wide = board.max.x - board.min.x;
    // three plans and two gaps between them
    let room_for_one = wide / (3.0 + BETWEEN * 2.0);
    let scale = (band / along).min(room_for_one / across);
    // **Lowest storey first as it is read, which is the wall's own minus
    // x.** Facing this wall you are facing north and your right hand points
    // west, so the three were laid out east to west and read upper, ground,
    // cellar. The bars inside each plan were turned round for the same
    // reason and the slots were not.
    let step = -across * scale * (1.0 + BETWEEN);
    let first = board.center().x - step;

    (scale, step, vec3(first, board.center().y, board.min.z))
}

/// The whole relief, as boxes in the world, with the storey each came from.
///
/// **The rooms are raised and the walls are the grooves between them.** The
/// first build raised the walls instead, which is a plan drawn as its own
/// strokes, and Jake read it off the wall as ancient cave art with nothing
/// of the building in it. What a person reads a plan by is the shape of a
/// room, so the room is the thing with a surface and a wall is where the
/// surface stops.
///
/// **Built once and kept**, because this walks every collider in the
/// building against a grid: 463 runs in five milliseconds, which is nothing
/// at startup and sixty times a second of nothing worth having.
///
/// **North is up and east is to the right**, which is how a drawing is read
/// rather than how the room lies behind it: stood in the nook facing this
/// wall, east is on your left. A plan is a page, not a window.
pub fn relief(room: &Room, reaches: f32) -> Vec<(Aabb, usize)> {
    let (scale, step, first) = laid_out(room, reaches);
    let whole = footprint(room);
    let middle = whole.center();
    let mut out = Vec::new();

    for (n, floor) in floors().iter().copied().enumerate() {
        let home = vec3(first.x + step * n as f32, first.y, first.z);

        for box_ in filled(room, floor)
            .into_iter()
            .filter(|box_| within(&whole, box_))
        {
            // **Cut off at the edge of the sheet.** `within` asks about a
            // box's middle, so a wall that starts inside the building and
            // runs out of it is kept and its far end was drawn past the
            // board: the first build put a bar a centimetre over the
            // cornice. A drawing stops where the paper does.
            let box_ = Aabb::new(
                vec3(
                    box_.min.x.max(whole.min.x),
                    box_.min.y,
                    box_.min.z.max(whole.min.z),
                ),
                vec3(
                    box_.max.x.min(whole.max.x),
                    box_.max.y,
                    box_.max.z.min(whole.max.z),
                ),
            );

            // **East to the viewer's right, which is the wall's own minus
            // x.** Stood in the nook facing this wall you are facing north,
            // and facing north your right hand points west. A plan is read
            // as a page, north up and east right, so the building's x runs
            // backwards along the wall and the two together are the plan
            // the right way round in the frame of whoever is reading it.
            //
            // **And it stands proud of the face, not into it.** The wall's
            // inner face is the board's z and the room is on the near side
            // of that, which is the smaller z: the first build put the
            // relief inside the plaster, where it was drawn and invisible.
            let low = vec3(
                home.x - (box_.max.x - middle.x) * scale,
                home.y + (box_.min.z - middle.z) * scale,
                home.z - RELIEF,
            );
            let high = vec3(
                home.x - (box_.min.x - middle.x) * scale,
                home.y + (box_.max.z - middle.z) * scale,
                home.z,
            );

            out.push((Aabb::new(low, high), n));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn building() -> Room {
        Room::of(crate::games())
    }

    /// Spec 0115: each storey's plan holds a bar for every wall the cut
    /// passes through, and none for a wall it does not.
    #[test]
    fn each_storey_cuts_its_own_walls() {
        let room = building();
        let whole = footprint(&room);

        for floor in floors() {
            let bars = cut(&room, floor);
            let at = floor + WAIST;

            assert!(
                !bars.is_empty(),
                "the storey at {:.1} cuts nothing at all",
                floor
            );

            for bar in bars.iter() {
                assert!(
                    bar.min.y <= at && bar.max.y >= at,
                    "a bar from {:.2} to {:.2} is not on the cut at {:.2}",
                    bar.min.y,
                    bar.max.y,
                    at
                );
            }

            // **And no floor is in it**, which is the thing a cut at waist
            // height is for: the slab a storey stands on lies under the cut
            // and a plan that held them would be a plan of three solid
            // rectangles. Asked of the footprint rather than of a bar's
            // height, because a bar thirty centimetres tall is a rail or a
            // shelf and the spec says in as many words that those are in
            // it.
            for bar in bars.iter() {
                let covers = (bar.max.x - bar.min.x) > (whole.max.x - whole.min.x) * 0.9
                    && (bar.max.z - bar.min.z) > (whole.max.z - whole.min.z) * 0.9;

                assert!(!covers, "a slab the size of the building is in the cut");
            }
        }
    }

    /// Spec 0115: the three are one scale, so a room above a room is above
    /// it on the wall.
    ///
    /// **Measured off the building rather than off the drawing.** Two walls
    /// a known distance apart in the world have to be that distance times
    /// one number apart on the wall, whichever storey each is on.
    #[test]
    fn the_three_are_one_scale() {
        let room = building();
        let bars = relief(&room, room.reaches);
        let mut widest = [0.0f32; 3];

        for (bar, storey) in bars.iter() {
            widest[*storey] = widest[*storey].max(bar.max.x - bar.min.x);
        }

        // the same wall drawn on two storeys would be the same width, and
        // what this can hold without naming a wall is that no storey is
        // drawn at a scale of its own: the whole building is 45.9 across and
        // a storey drawn to its own width would put the cellar, 22.6 across,
        // at twice the scale of the ground floor
        // **Measured off what is drawn, which is the fill.** This asked the
        // cut for the denominator while the relief was the walls, and went
        // on asking it when the rooms were filled instead: the two then
        // disagreed about a storey's width and the test failed over a thing
        // that had not changed.
        let one = filled(&room, floors()[0]);
        let two = filled(&room, floors()[1]);
        let across = |of: &[Aabb]| {
            let (mut least, mut most) = (f32::MAX, f32::MIN);

            for box_ in of {
                least = least.min(box_.min.x);
                most = most.max(box_.max.x);
            }

            most - least
        };
        let drawn_across = |storey: usize| {
            let (mut least, mut most) = (f32::MAX, f32::MIN);

            for (bar, which) in bars.iter() {
                if *which == storey {
                    least = least.min(bar.min.x);
                    most = most.max(bar.max.x);
                }
            }

            most - least
        };
        let first = drawn_across(0) / across(&one);
        let second = drawn_across(1) / across(&two);

        assert!(
            (first - second).abs() < 1e-3,
            "the cellar is drawn at {:.4} to the metre and the ground floor at {:.4}",
            first,
            second
        );
    }

    /// Spec 0115: the whole relief fits between the rail and the cornice and
    /// inside the wall's width, with nothing of it through the wall.
    #[test]
    fn it_fits_the_wall_it_is_on() {
        let room = building();
        let board = board(room.reaches);
        let (at, size) = crate::room::nook_end(room.reaches);
        let face = at.z - size.z * 0.5;

        assert!(
            board.min.y > crate::study::DADO + crate::study::RAIL * 0.5
                && board.max.y < TALL - crate::study::CORNICE,
            "the board runs {:.2} to {:.2}, which is into the rail or the cornice",
            board.min.y,
            board.max.y
        );

        for (bar, _) in relief(&room, room.reaches) {
            assert!(
                bar.min.x >= board.min.x - 1e-3 && bar.max.x <= board.max.x + 1e-3,
                "a bar runs {:.2}..{:.2} and the board is {:.2}..{:.2}",
                bar.min.x,
                bar.max.x,
                board.min.x,
                board.max.x
            );
            assert!(
                bar.min.y >= board.min.y - 1e-3 && bar.max.y <= board.max.y + 1e-3,
                "a bar runs {:.2}..{:.2} up and the board is {:.2}..{:.2}",
                bar.min.y,
                bar.max.y,
                board.min.y,
                board.max.y
            );
            // **On the room's side of the plaster.** The first build put the
            // relief from the face northwards, which is inside the wall, and
            // it was drawn every frame where nobody could see it.
            assert!(
                bar.max.z <= face + 1e-4 && bar.min.z >= face - RELIEF - 1e-4,
                "a bar at z {:.3}..{:.3} is not standing on the face at {:.3}",
                bar.min.z,
                bar.max.z,
                face
            );
        }
    }

    /// Spec 0115: the drawing fills the board it is given.
    ///
    /// **This is the one that holds the bound.** Without something saying
    /// where the building stops, `room_at` calls the long way out to the
    /// village and the bridge the hall, the footprint comes out 194 metres
    /// along, and the whole building is drawn at 1:115 as a smudge in the
    /// middle of the wall with every wall of it two and a half millimetres
    /// wide. Every other test here passed in that state, which is what this
    /// one is for.
    #[test]
    fn the_drawing_fills_the_board() {
        let room = building();
        let board = board(room.reaches);
        let bars = relief(&room, room.reaches);
        let (mut low, mut high) = (f32::MAX, f32::MIN);
        let (mut west, mut east) = (f32::MAX, f32::MIN);

        for (bar, _) in bars.iter() {
            low = low.min(bar.min.y);
            high = high.max(bar.max.y);
            west = west.min(bar.min.x);
            east = east.max(bar.max.x);
        }

        let band = board.max.y - board.min.y;
        let wide = board.max.x - board.min.x;

        assert!(
            high - low > band * 0.8,
            "the three fill {:.2} of a band {:.2} tall, so the building is a smudge",
            high - low,
            band
        );
        assert!(
            east - west > wide * 0.5,
            "the three fill {:.2} of a board {:.2} wide",
            east - west,
            wide
        );
    }

    /// Spec 0115: a wall that moves moves the plan, which is the whole claim.
    ///
    /// **The building cannot be rebuilt in a test, so the room is.** A
    /// `Room` built for a different number of games has a different
    /// `reaches` and so different walls, and a plan cut from it is a
    /// different plan. A plan drawn by hand would not notice.
    #[test]
    fn a_wall_that_moves_moves_the_plan() {
        let room = building();
        let mut shorter = crate::games();

        shorter.truncate(shorter.len() - 6);

        let fewer = Room::of(shorter);

        assert!(
            (fewer.reaches - room.reaches).abs() > 0.1,
            "the two rooms are the same size, so this tests nothing"
        );

        let one = cut(&room, 0.0);
        let two = cut(&fewer, 0.0);

        assert_ne!(
            one.len(),
            two.len(),
            "a room {:.1} shorter cut the same number of walls",
            room.reaches - fewer.reaches
        );
    }
}
