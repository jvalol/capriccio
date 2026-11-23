//! A relief plan of the building, cut from the building. Spec 0115.
//!
//! **On the cellar's south wall**, between the wine racks. It hung in the
//! nook first and Jake took that wall back for pictures: ten metres is a
//! mural's wall and a plan is a diagram, which is most of why it never sat
//! right there. Down here it is the same panelled mahogany, the stretch
//! between the racks is clear because the pool table stands against it, and
//! the two wall lamps are at either end of that stretch rather than over
//! the middle of it, which is the raking light a relief wants.
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

use crate::room::Room;
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

/// Which of the cellar's long walls it is cut into, which way that wall
/// faces, and which way the drawing's east runs along it.
///
/// **Three signs that are one fact, so they are written as one.** Stood in
/// the cellar facing this wall you are facing south, where in the nook you
/// faced north, and both of those decide the same two things: the relief
/// stands out of the face toward the room rather than into the plaster, and
/// a page is read with east to the reader's right. Facing south your right
/// hand points east, so east is the wall's own plus x here where it was the
/// minus x there. Moving the plan flipped both at once, and getting one and
/// not the other is a drawing with its back to you or a mirrored one.
pub const SIDE: f32 = -1.0;
pub const OUT: f32 = 1.0;
pub const EAST: f32 = 1.0;

/// How high the middle of the drawing hangs over the cellar's floor, how
/// much of the wall's own height is left clear at each end of the band, and
/// how much of the clear run is left as margin at either end of it.
///
/// **Hung rather than fitted.** In the nook the band between a dado and a
/// cornice was the whole of what the wall offered and the scale came off
/// the height. Here the wall is clear from the skirting to the cornice,
/// which is two metres, and what binds is the width between the racks. So
/// the band is still everything the wall offers, but centred where a
/// drawing hangs rather than halfway up the plaster: a hand under eye
/// height, which is where a thing you read standing goes.
pub const HANGS: f32 = 1.45;
pub const OFF_THE_TRIM: f32 = 0.06;
pub const CLEAR: f32 = 0.20;

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

/// The clear stretch of that wall: the panelling between the wine racks.
///
/// **Asked of the racks rather than measured.** They are skipped where the
/// pool table stands, which is what leaves the stretch clear in the first
/// place, so the gap is theirs to report. Written down instead, moving the
/// table would move the racks and leave the plan half behind a rack.
pub fn clear_run(reaches: f32) -> (f32, f32) {
    let mut edges: Vec<f32> = crate::cellar::racks(reaches)
        .into_iter()
        .filter(|(_, side)| *side == SIDE)
        .map(|(at, _)| at.x)
        .collect();

    edges.sort_by(|a, b| a.partial_cmp(b).expect("a rack somewhere"));

    let half = crate::cellar::RACK.z * 0.5;
    let mut run = (0.0, 0.0);

    for pair in edges.windows(2) {
        let (west, east) = (pair[0] + half, pair[1] - half);

        if east - west > run.1 - run.0 {
            run = (west, east);
        }
    }

    run
}

/// The face the plan is cut into: the panelling's, not the wall's.
///
/// **The boards stand five centimetres off the structural wall**, which is
/// a lot more than the relief does, so a plan laid on the wall is a plan
/// inside the panelling. Cut once, it was there: faint lines showing
/// through the timber and nothing else. What a thing hung on a panelled
/// wall is hung on is the panelling.
pub fn face(reaches: f32) -> f32 {
    crate::cellar::long_wall(reaches, SIDE) + OUT * crate::cellar::BOARD_OUT
}

/// The band of that wall the relief is allowed, as a box with no thickness.
pub fn board(reaches: f32) -> Aabb {
    let face = face(reaches);
    let (west, east) = clear_run(reaches);
    let floor = -crate::cellar::DOWN;
    let low = floor + crate::cellar::SKIRTING + OFF_THE_TRIM;
    let high = floor + crate::cellar::TALL - crate::cellar::CORNICE - OFF_THE_TRIM;
    // centred on where it hangs, which is nearer the skirting than the
    // cornice, so the half that reaches the cornice first is the one that
    // sets how tall the band can be
    let half = (floor + HANGS - low).min(high - (floor + HANGS));

    Aabb::new(
        vec3(west + CLEAR, floor + HANGS - half, face),
        vec3(east - CLEAR, floor + HANGS + half, face),
    )
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
    // **Lowest storey leftmost as it is read**, which is whichever way round
    // `EAST` says: the three run the way the page does, so the stride and
    // the bars inside each plan turn together and there is one fact to get
    // wrong rather than two.
    let step = EAST * across * scale * (1.0 + BETWEEN);
    let first = board.center().x - step;

    (scale, step, vec3(first, board.center().y, board.min.z))
}

/// What a place in the building is called, in the building's own words.
///
/// **Taken off the specs rather than written.** Every one of these is what
/// capriccio calls that room and has called it for as long as it has had
/// one: the nook 216 times, the cellar 82, the gallery 68, the baths 66,
/// the imaginarium 61, the hall 43, and that is the form they take here,
/// article and all. The sign over the aisle says NOOK and these do not
/// follow it: a sign shouts across a room and a plan is read at arm's
/// length. Jake chose the prose form, asked.
///
/// **`room_at` cannot tell the glass room from the nook**, having no
/// variant for either of the two rooms upstairs, so the floor above is
/// named by where the glass room stands and everything else up there is the
/// imaginarium.
pub fn called(reaches: f32, storey: usize, at: Vec3) -> Option<&'static str> {
    use crate::noise::Room as Which;

    if storey == 2 {
        let glass = crate::glass::inside(reaches);

        return Some(
            if at.x >= glass.min.x
                && at.x <= glass.max.x
                && at.z >= glass.min.z
                && at.z <= glass.max.z
            {
                "the glass room"
            } else {
                "the imaginarium"
            },
        );
    }

    match crate::noise::room_at(reaches, vec3(at.x, floors()[storey] + WAIST, at.z)) {
        Which::Hall => Some("the hall"),
        Which::Nook => Some("the nook"),
        Which::Gallery => Some("the gallery"),
        Which::Cellar => Some("the cellar"),
        Which::Baths => Some("the baths"),
        _ => None,
    }
}

/// How tall a label is on the wall.
///
/// **A drawing's lettering, which is read by leaning in.** The plans are two
/// metres across for a building forty-six wide, so anything a room's own
/// width could carry would be a word taller than the room it names.
pub const LETTERS: f32 = 0.042;

/// Where each room's name goes: the middle of the largest piece of it.
///
/// **The largest piece and not the middle of the whole.** A weighted middle
/// is outside its own room whenever that room is an L or has a hole in it,
/// and the baths have both: the pool is a hole and the middle of the baths
/// landed in the water, which on the wall is bare plaster.
pub fn labels(room: &Room, reaches: f32) -> Vec<(&'static str, Vec3)> {
    let mut held: Vec<(&'static str, usize, Aabb, f32)> = Vec::new();

    for (n, floor) in floors().iter().copied().enumerate() {
        for bar in filled(room, floor) {
            let Some(name) = called(reaches, n, bar.center()) else {
                continue;
            };
            let size = (bar.max.x - bar.min.x) * (bar.max.z - bar.min.z);
            let found = held
                .iter_mut()
                .find(|(held, storey, _, _)| *held == name && *storey == n);

            match found {
                Some((_, _, biggest, most)) => {
                    if size > *most {
                        *biggest = bar;
                        *most = size;
                    }
                }
                None => held.push((name, n, bar, size)),
            }
        }
    }

    held.into_iter()
        .map(|(name, storey, bar, _)| (name, onto(room, reaches, storey, bar.center())))
        .collect()
}

/// Which of the three a height belongs to: the lowest floor at or under it.
pub fn storey_at(y: f32) -> usize {
    floors()
        .iter()
        .rposition(|floor| y >= *floor - 0.6)
        .unwrap_or(0)
}

/// Where a place in the building lands on the wall.
///
/// **One mapping, used by the drawing and by whatever reads it.** The plan
/// and the mark that says where you are standing have to agree about where
/// a metre goes, and the way to be sure of that is for there to be one
/// answer rather than two.
pub fn onto(room: &Room, reaches: f32, storey: usize, at: Vec3) -> Vec3 {
    let (scale, step, first) = laid_out(room, reaches);
    let middle = footprint(room).center();

    vec3(
        first.x + step * storey as f32 + EAST * (at.x - middle.x) * scale,
        first.y + (at.z - middle.z) * scale,
        first.z,
    )
}

/// How big the mark that says where you are standing is, on the wall.
///
/// **Big enough to find without hunting.** The first go was half this and
/// read as a speck of grit on a drawing, which is worse than nothing: a
/// you-are-here you have to look for does not answer where you are.
pub const MARK: f32 = 0.055;

/// That mark: where you are, and which way you are facing.
///
/// **Two boxes and not one.** Jake, looking at the plan: how do I know
/// where I am or what I am looking at. A dot answers the first and a nose
/// out in front of it answers the second, and a plan with neither is a
/// drawing of a building you have no way into.
pub fn here(room: &Room, reaches: f32, at: Vec3, facing: Vec3) -> Vec<Aabb> {
    let storey = storey_at(at.y);
    let on = onto(room, reaches, storey, at);
    let way = glam::vec2(facing.x, facing.z);
    let way = if way.length_squared() > 1e-6 {
        way.normalize()
    } else {
        glam::vec2(0.0, 1.0)
    };
    // the mapping turns x round and leaves z alone, so the nose does too
    let nose = vec3(on.x - way.x * MARK * 2.2, on.y + way.y * MARK * 2.2, on.z);
    let square = |middle: Vec3, half: f32| {
        Aabb::new(
            vec3(
                middle.x - half,
                middle.y - half,
                middle.z.min(middle.z + OUT * RELIEF * 2.0),
            ),
            vec3(
                middle.x + half,
                middle.y + half,
                middle.z.max(middle.z + OUT * RELIEF * 2.0),
            ),
        )
    };

    vec![square(on, MARK), square(nose, MARK * 0.55)]
}

/// How deep the sheet a storey is drawn on stands off the wall, as a share
/// of the relief.
///
/// **Under one, so the rooms stand proud of their own sheet** and nothing
/// of the sheet is in a plane with anything: its face is nearer the wall
/// than theirs, and wherever a room stands the sheet is buried inside it.
pub const SHEET: f32 = 0.45;

/// The sheet each storey is drawn on: the whole building's footprint at the
/// plan's own scale, whatever that storey happens to fill of it.
///
/// **The same rectangle three times, which is the whole of it.** Jake,
/// looking at the wall: the map doesn't quite land and it's hard to read.
/// Every storey was cut at one scale and then left floating wherever its
/// own rooms sat, so the upper floor hung high and small, the cellar low
/// and small, and the ground floor filled the middle: three pale shapes at
/// three heights with bare panelling between them and nothing saying they
/// were of one building. A sheet apiece puts them on one line and keeps
/// each storey in its own true place inside it, which is also the thing a
/// reader learns from it, that the floor above covers a corner of what is
/// under it.
///
/// And it gives the drawing a ground. A cut plan is pale stone, the wall
/// behind it is lit timber of much the same brightness, and a shape only
/// reads where its edge does.
pub fn sheet(room: &Room, reaches: f32) -> Vec<Aabb> {
    let whole = footprint(room);

    (0..floors().len())
        .map(|storey| {
            let one = onto(room, reaches, storey, whole.min);
            let two = onto(room, reaches, storey, whole.max);

            Aabb::new(
                vec3(
                    one.x.min(two.x),
                    one.y.min(two.y),
                    one.z.min(one.z + OUT * RELIEF * SHEET),
                ),
                vec3(
                    one.x.max(two.x),
                    one.y.max(two.y),
                    one.z.max(one.z + OUT * RELIEF * SHEET),
                ),
            )
        })
        .collect()
}

/// How wide the lip round a sheet is, and how far off the wall it stands as
/// a share of the relief.
///
/// **A sheet with no lip is a hole in the panelling.** Three grey
/// rectangles cut into a timber wall read as three rectangles; what says
/// a drawing is hung there is a rim catching the light off the lamps either
/// side of it.
///
/// It lies on the sheet's own border rather than outside it, because the
/// band between the dado and the cornice is what set the scale and there is
/// no room over the top of it. A centimetre is about the building's own
/// outer wall at this scale, so what it covers is the wall it is standing
/// for.
pub const LIP: f32 = 0.011;
pub const LIP_OUT: f32 = 1.5;

/// That lip: four bars round each sheet.
pub fn lips(room: &Room, reaches: f32) -> Vec<Aabb> {
    sheet(room, reaches)
        .into_iter()
        .flat_map(|sheet| {
            let (low, high) = (sheet.min, sheet.max);
            // the face it is cut into, whichever end of the sheet's own
            // depth that is, and the lip standing out of it
            let face = if OUT > 0.0 { low.z } else { high.z };
            let front = face + OUT * RELIEF * LIP_OUT;
            let bar = |a: Vec3, b: Vec3| {
                Aabb::new(
                    vec3(a.x, a.y, face.min(front)),
                    vec3(b.x, b.y, face.max(front)),
                )
            };

            [
                bar(low, vec3(high.x, low.y + LIP, 0.0)),
                bar(vec3(low.x, high.y - LIP, 0.0), high),
                bar(low, vec3(low.x + LIP, high.y, 0.0)),
                bar(vec3(high.x - LIP, low.y, 0.0), high),
            ]
        })
        .collect()
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
    let whole = footprint(room);
    let mut out = Vec::new();

    for (n, floor) in floors().iter().copied().enumerate() {
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
            let one = onto(room, reaches, n, box_.min);
            let two = onto(room, reaches, n, box_.max);
            let low = vec3(
                one.x.min(two.x),
                one.y.min(two.y),
                one.z.min(one.z + OUT * RELIEF),
            );
            let high = vec3(
                one.x.max(two.x),
                one.y.max(two.y),
                one.z.max(one.z + OUT * RELIEF),
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
        let face = face(room.reaches);
        let floor = -crate::cellar::DOWN;

        assert!(
            board.min.y > floor + crate::cellar::SKIRTING
                && board.max.y < floor + crate::cellar::TALL - crate::cellar::CORNICE,
            "the board runs {:.2} to {:.2}, which is into the skirting or the cornice",
            board.min.y,
            board.max.y
        );

        // and clear of the racks either side of it, which is the whole of
        // why this stretch of wall is free
        for (at, side) in crate::cellar::racks(room.reaches) {
            if side != SIDE {
                continue;
            }

            let half = crate::cellar::RACK.z * 0.5;

            assert!(
                at.x + half <= board.min.x + 1e-4 || at.x - half >= board.max.x - 1e-4,
                "a rack runs {:.2}..{:.2} and the board is {:.2}..{:.2}",
                at.x - half,
                at.x + half,
                board.min.x,
                board.max.x
            );
        }

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
            let (back, front) = if OUT > 0.0 {
                (bar.min.z, bar.max.z)
            } else {
                (bar.max.z, bar.min.z)
            };

            assert!(
                (back - face).abs() < 1e-4 && (front - face).abs() <= RELIEF + 1e-4,
                "a bar at z {:.3}..{:.3} is not standing on the face at {:.3}",
                bar.min.z,
                bar.max.z,
                face
            );
        }
    }

    /// Spec 0115: each storey is drawn on a sheet of its own, and the three
    /// sheets are one rectangle three times.
    ///
    /// **What this holds is that they are the same.** Cut at one scale and
    /// then left to float wherever its own rooms sat, each storey hung at
    /// its own height and its own size and the wall read as three unrelated
    /// shapes. Three equal sheets in a row put every storey in its true
    /// place inside the building's own footprint, which is the thing a
    /// reader learns from a set of plans.
    #[test]
    fn each_storey_is_drawn_on_a_sheet_the_size_of_the_building() {
        let room = building();
        let board = board(room.reaches);
        let sheets = sheet(&room, room.reaches);
        let face = face(room.reaches);
        let front = |box_: &Aabb| if OUT > 0.0 { box_.max.z } else { box_.min.z };

        assert_eq!(sheets.len(), floors().len());

        for (n, sheet) in sheets.iter().enumerate() {
            assert!(
                (sheet.size().x - sheets[0].size().x).abs() < 1e-4
                    && (sheet.size().y - sheets[0].size().y).abs() < 1e-4,
                "sheet {} is {:?} and the first is {:?}",
                n,
                sheet.size(),
                sheets[0].size()
            );
            assert!(
                sheet.min.x >= board.min.x - 1e-3
                    && sheet.max.x <= board.max.x + 1e-3
                    && sheet.min.y >= board.min.y - 1e-3
                    && sheet.max.y <= board.max.y + 1e-3,
                "sheet {} runs {:?}..{:?} and the board is {:?}..{:?}",
                n,
                sheet.min,
                sheet.max,
                board.min,
                board.max
            );

            for other in sheets.iter().skip(n + 1) {
                assert!(
                    sheet.max.x <= other.min.x + 1e-4 || other.max.x <= sheet.min.x + 1e-4,
                    "two sheets overlap"
                );
            }
        }

        // **Behind the rooms and in a plane with nothing.** The sheet's own
        // face has to stand nearer the wall than anything drawn on it, or
        // the two fight where they are flush, which this building has paid
        // for seven times under other names.
        for (bar, n) in relief(&room, room.reaches) {
            let sheet = &sheets[n];

            assert!(
                bar.min.x >= sheet.min.x - 1e-3
                    && bar.max.x <= sheet.max.x + 1e-3
                    && bar.min.y >= sheet.min.y - 1e-3
                    && bar.max.y <= sheet.max.y + 1e-3,
                "a bar of storey {} runs {:?}..{:?} off a sheet of {:?}..{:?}",
                n,
                bar.min,
                bar.max,
                sheet.min,
                sheet.max
            );
            assert!(
                OUT * (front(&bar) - front(sheet)) > 1e-5,
                "storey {}'s sheet faces {:.4} and a room on it faces {:.4}",
                n,
                front(sheet),
                front(&bar)
            );
        }

        // and the lip lies on its own sheet's border, standing proud of
        // everything drawn inside it
        for (n, lip) in lips(&room, room.reaches).iter().enumerate() {
            let sheet = &sheets[n / 4];

            assert!(
                lip.min.x >= sheet.min.x - 1e-4
                    && lip.max.x <= sheet.max.x + 1e-4
                    && lip.min.y >= sheet.min.y - 1e-4
                    && lip.max.y <= sheet.max.y + 1e-4,
                "lip {} runs {:?}..{:?} off its sheet {:?}..{:?}",
                n,
                lip.min,
                lip.max,
                sheet.min,
                sheet.max
            );
            assert!(
                OUT * (front(lip) - face) > RELIEF + 1e-5
                    && ((if OUT > 0.0 { lip.min.z } else { lip.max.z }) - face).abs() < 1e-4,
                "lip {} runs {:.4}..{:.4} and the rooms reach {:.4}",
                n,
                lip.min.z,
                lip.max.z,
                face + OUT * RELIEF
            );
        }
    }

    /// Spec 0115: the three fill the wall they are given, and the building
    /// fills the sheet it is drawn on.
    ///
    /// **This is the one that holds the bound.** Without something saying
    /// where the building stops, `room_at` calls the long way out to the
    /// village and the bridge the hall, the footprint comes out 194 metres
    /// along, and the whole building is drawn at 1:115 as a smudge with
    /// every wall of it two and a half millimetres wide. Every other test
    /// here passed in that state, which is what this one is for.
    ///
    /// **Of the sheet and not of the board, which is what the sheets
    /// changed.** Three sheets tile the board by construction now, so a
    /// claim about how much of the board they cover is one the layout
    /// cannot break. What a blown footprint does is shrink the building
    /// inside its own sheet, so that is the thing to ask about.
    ///
    /// **And of the three together.** The footprint is the union of what is
    /// drawn, so the union has to fill the sheet; one storey need not, and
    /// the floor above fills a fifth of its width, which is the true thing
    /// the shared sheet is there to say.
    #[test]
    fn the_building_fills_its_sheet_and_the_three_fill_the_wall() {
        let room = building();
        let board = board(room.reaches);
        let sheets = sheet(&room, room.reaches);
        let wide = board.max.x - board.min.x;
        let (mut west, mut east) = (f32::MAX, f32::MIN);

        for sheet in sheets.iter() {
            west = west.min(sheet.min.x);
            east = east.max(sheet.max.x);
        }

        assert!(
            east - west > wide * 0.95,
            "the three fill {:.2} of a wall {:.2} wide",
            east - west,
            wide
        );

        let (mut low, mut high) = (f32::MAX, f32::MIN);
        let (mut left, mut right) = (f32::MAX, f32::MIN);

        for (bar, storey) in relief(&room, room.reaches) {
            let sheet = &sheets[storey];

            low = low.min(bar.min.y - sheet.min.y);
            high = high.max(bar.max.y - sheet.min.y);
            left = left.min(bar.min.x - sheet.min.x);
            right = right.max(bar.max.x - sheet.min.x);
        }

        let (across, along) = (sheets[0].size().x, sheets[0].size().y);

        assert!(
            high - low > along * 0.8 && right - left > across * 0.8,
            "the building fills {:.2} by {:.2} of a sheet {:.2} by {:.2}, \
             so it is a smudge on it",
            right - left,
            high - low,
            across,
            along
        );
    }

    /// Spec 0115: east is on the right as the wall is read.
    ///
    /// **Nothing else here can catch a mirrored plan.** The drawing and the
    /// mark both go through `onto`, so turning its x round turns both and
    /// every other test still passes: the mark lands on the same room, the
    /// three keep one scale, the board still holds it. What a mirror breaks
    /// is the one thing none of those ask, which is whether the drawing
    /// agrees with the building it is in.
    ///
    /// Facing this wall you are facing south, so your right hand points
    /// east, and the right-hand side of anything hung on it is the wall's
    /// own larger x. A plan is read east on the right, so east belongs
    /// there.
    ///
    /// **Written backwards first, in the nook, and it turned round when the
    /// plan moved.** The assertion there said west, on the thought that west
    /// is what is physically to your right, which is a map held up to the
    /// room rather than a drawing read as a page. That wall faced the other
    /// way, so the right answer there is the wrong one here, and `EAST` is
    /// where the two meet.
    #[test]
    fn east_is_on_the_right_as_it_is_read() {
        let room = building();
        let middle = footprint(&room).center();
        let west = onto(&room, room.reaches, 1, vec3(middle.x - 10.0, 0.0, middle.z));
        let east = onto(&room, room.reaches, 1, vec3(middle.x + 10.0, 0.0, middle.z));

        assert!(
            east.x > west.x,
            "east draws at {:.2} and west at {:.2}, so the plan is mirrored: \
             facing south your right hand points east, so the side of this \
             wall a reader calls the right is its larger x",
            east.x,
            west.x
        );

        // and north is up, which is the half of it that needs no turning
        let north = onto(&room, room.reaches, 1, vec3(middle.x, 0.0, middle.z + 10.0));
        let south = onto(&room, room.reaches, 1, vec3(middle.x, 0.0, middle.z - 10.0));

        assert!(
            north.y > south.y,
            "north draws at {:.2} and south at {:.2}, so the plan is upside down",
            north.y,
            south.y
        );
    }

    /// Spec 0115: every room the building can name is named once, on
    /// itself.
    #[test]
    fn every_room_is_named_once_and_on_itself() {
        let room = building();
        let labels = labels(&room, room.reaches);
        let mut names: Vec<&str> = labels.iter().map(|(says, _)| *says).collect();

        names.sort_unstable();

        let mut once = names.clone();

        once.dedup();

        assert_eq!(names, once, "a room is named twice: {:?}", names);

        for want in [
            "the hall",
            "the nook",
            "the gallery",
            "the cellar",
            "the baths",
            "the glass room",
        ] {
            assert!(names.contains(&want), "{} is on no plan", want);
        }

        // and each sits on what it names rather than out on the wall beside
        // it, which is the fault a weighted middle has when a room is an L
        let drawn = relief(&room, room.reaches);

        for (says, at) in labels.iter() {
            let on = drawn.iter().any(|(bar, _)| {
                bar.min.x <= at.x && bar.max.x >= at.x && bar.min.y <= at.y && bar.max.y >= at.y
            });

            assert!(
                on,
                "{} is written on bare wall at {:.2}, {:.2}",
                says, at.x, at.y
            );
        }
    }

    /// Spec 0115: the mark lands on the room you are standing in.
    ///
    /// **Which is the whole of what a you-are-here is for.** The mapping
    /// turns the building's x round, because facing this wall you face
    /// north and your right hand points west, and a sign wrong anywhere in
    /// that puts the mark in the room opposite the one you are in. Nothing
    /// you could see from a screenshot would tell you which.
    #[test]
    fn the_mark_lands_on_the_room_you_are_in() {
        let room = building();
        let standing = [
            ("the nook", vec3(-7.7, 1.0, 1.0)),
            ("the nook's south end", vec3(-7.7, 1.0, -8.0)),
            ("the hall", vec3(0.0, 1.0, 0.0)),
        ];

        for (name, at) in standing.iter() {
            let storey = storey_at(at.y);
            let floor = filled(&room, floors()[storey]);

            assert!(
                floor.iter().any(|bar| {
                    bar.min.x <= at.x && bar.max.x >= at.x && bar.min.z <= at.z && bar.max.z >= at.z
                }),
                "{} is not open floor, so this tests nothing",
                name
            );

            let mark = here(&room, room.reaches, *at, glam::Vec3::Z);
            let dot = mark[0].center();
            let under = relief(&room, room.reaches).into_iter().any(|(bar, which)| {
                which == storey
                    && bar.min.x <= dot.x
                    && bar.max.x >= dot.x
                    && bar.min.y <= dot.y
                    && bar.max.y >= dot.y
            });

            assert!(
                under,
                "standing in {} the mark lands at {:.2}, {:.2}, which is on no room of that storey",
                name, dot.x, dot.y
            );
        }
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
