//! The pond early in the field: water in a stone bank, two islands standing
//! in it, and ducks on it. Spec 0129.
//!
//! **A place and not a rule.** `field` is cells whose contents come out of
//! their own coordinates, which is what makes a field endless. This is a
//! thing at a coordinate, like the village and the bridge, and the field is
//! told it is there rather than asked.
//!
//! **Nothing here is square, and nothing is a wall.** The first build was a
//! rectangle of water in a band of concrete standing out of the grass, the
//! second the same with cobbles on it, and Jake sent both back: the concrete
//! outside it made no sense and the stone borders had a geometry. The water
//! is cut to its own wandering outline now, so there is nothing to hide, and
//! the ground is what rises to meet it.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

/// How far across the water the engine simulates, and how deep it says it
/// is.
///
/// **Bigger than the pond, because the pond is cut out of it.**
/// `blitzkit::water::Water` is a rectangle and will not be anything else, so
/// what is drawn is the triangles of it that fall inside the bank. The
/// rectangle has to hold the whole outline with a cell to spare at the
/// widest part of it.
pub const WATER: Vec2 = vec2(30.0, 21.0);
pub const DEEP: f32 = 0.3;

/// Where its middle stands: west of the garden's west face, north of the gap.
/// **Far enough west that its verge is not the garden's wall.** The ground
/// rises for four metres outside the lip, and sited where the first build
/// put it that rise ran into the field's own edge at `field::starts`: a
/// walk at the pond from the east started inside the building. Found by
/// `tests::you_cannot_walk_into_the_water`, which could not get to the
/// water to be stopped by it.
pub const WEST: f32 = 21.5;
pub const NORTH: f32 = 22.35;

/// The section of the bank, from the silt at the bottom to the field at the
/// far side of the verge.
///
/// **A pond is set below the ground round it**, which is Jake's third note
/// on this and the one that undid the other two: the stones should slope up
/// from under the water to a little over the grass. The field's ground is a
/// slab that follows you and there is no hole in it to sink a pond into, so
/// what goes down is the water and what comes up is the land: the grass
/// stands [`RIM_UP`] higher at the pond's lip than it does out in the field,
/// and falls back to it over [`BERM`].
///
/// Read as a height at a distance either side of the lip, so the mesh, the
/// stones, the water's own edge and what you walk on are all one line.
/// Nought is the lip; inside it is negative.
pub const BED: f32 = 0.0;
pub const UP: f32 = 0.06;
pub const RIM_UP: f32 = 0.34;
pub const BANK_IN: f32 = 1.5;
pub const VERGE: f32 = 0.4;
pub const BERM: f32 = 3.6;

/// How high the ground stands at `d` metres out from the lip.
pub fn section(d: f32) -> f32 {
    let smooth = |t: f32| {
        let t = t.clamp(0.0, 1.0);

        t * t * (3.0 - 2.0 * t)
    };

    if d <= -BANK_IN {
        BED
    } else if d < 0.0 {
        BED + (RIM_UP - BED) * smooth((d + BANK_IN) / BANK_IN)
    } else if d <= VERGE {
        RIM_UP
    } else {
        RIM_UP * (1.0 - smooth((d - VERGE) / BERM))
    }
}

/// How far in from the lip the water's own edge lies, which is where the
/// bank passes through [`UP`].
///
/// **Asked of the section rather than written down.** The water meets the
/// bank where the bank is as high as the water, and a second number for it
/// is the water and the ground disagreeing about where the shore is.
pub fn shore() -> f32 {
    let (mut low, mut high) = (-BANK_IN, 0.0);

    for _ in 0..24 {
        let middle = (low + high) * 0.5;

        if section(middle) < UP {
            low = middle;
        } else {
            high = middle;
        }
    }

    (low + high) * 0.5
}

/// The middle of the water, at the height it stands at.
pub fn at(reaches: f32) -> Vec3 {
    vec3(
        crate::field::starts(reaches) - WEST,
        UP,
        crate::garden::at(reaches).z + NORTH,
    )
}

/// The water itself, as the engine simulates it.
///
/// **One account of it.** The ducks ride this, the drawing uploads this, and
/// a test asking where the surface is asks this.
pub fn water(reaches: f32) -> blitzkit::water::Water {
    let mut water = blitzkit::water::Water::new(at(reaches), WATER, DEEP, CELLS);

    water.damping = SETTLES;
    water.speed = RUNS;
    water.chop = CALM;
    // what this stops is a wave standing over the stones set round it
    water.rises_to = RIM_UP - UP;

    water
}

/// How many cells across the water is simulated, how fast a wave runs, and
/// how long one takes to die.
pub const CELLS: u32 = 72;
pub const SETTLES: f32 = 0.52;
pub const RUNS: f32 = 1.9;

/// How rough the surface shades, against the engine's own `water::CHOP`.
///
/// **A pond is not the sea, and from twenty metres a rough one is not even
/// water.** Spread over four hundred square metres and seen from the far
/// side, every facet of a rough one catches the sun at once and the whole
/// pond comes back the colour of the sky.
pub const CALM: f32 = 0.38;

/// How far across the pond is, and how square its outline runs.
///
/// A rounded oblong rather than an oval: the shape a pond cut into a lawn
/// takes, which is longer than it is wide and fuller at the ends than an
/// ellipse.
pub const ACROSS: Vec2 = vec2(12.2, 8.3);
pub const SQUARENESS: f32 = 2.7;

/// How far the outline wanders off that, as waves of a turn, how deep each
/// is and where it starts.
///
/// Three, of lengths that are not multiples of each other, so the outline
/// closes on itself at one turn and nothing in it repeats inside that.
pub const WANDER: [(f32, f32, f32); 3] = [(3.0, 0.075, 0.7), (5.0, 0.055, 2.3), (7.0, 0.04, 4.1)];

/// How many points the outline is said in.
pub const ROUND: usize = 128;

/// How far out the pond's lip stands at a share of the way round it.
///
/// **Polar, and that is what makes the rest of this work.** Said this way
/// the outline is a radius for every angle, so asking whether a spot is in
/// the pond is one comparison rather than a walk round a polygon: the water
/// is cut by it, the bank is built on it, the stones are set along it and
/// the field is kept off it, all from the one line.
pub fn rim_out(on: f32) -> f32 {
    use std::f32::consts::TAU;

    let turn = on * TAU;
    let (sin, cos) = turn.sin_cos();
    // the polar form of a superellipse, which at two is an ellipse and past
    // it is an oblong with the corners taken off
    let shape = ((cos / ACROSS.x).abs().powf(SQUARENESS)
        + ((sin / ACROSS.y).abs()).powf(SQUARENESS))
    .powf(-1.0 / SQUARENESS);
    let wander: f32 = WANDER
        .iter()
        .map(|(waves, deep, from)| (turn * waves + from).sin() * deep)
        .sum();

    shape * (1.0 + wander)
}

/// A point on the pond's lip at a share of the way round it, `out` metres
/// further out than the lip itself.
pub fn rim_at(reaches: f32, on: f32, out: f32) -> Vec3 {
    use std::f32::consts::TAU;

    let middle = at(reaches);
    let turn = on * TAU;
    let far = rim_out(on) + out;

    vec3(
        middle.x + turn.cos() * far,
        section(out),
        middle.z + turn.sin() * far,
    )
}

/// The lip, as the points of it.
pub fn rim(reaches: f32) -> Vec<Vec3> {
    (0..ROUND)
        .map(|n| rim_at(reaches, n as f32 / ROUND as f32, 0.0))
        .collect()
}

/// Whether a spot is in the pond, with `over` metres of grace outside the
/// lip.
pub fn inside(reaches: f32, spot: Vec3, over: f32) -> bool {
    use std::f32::consts::TAU;

    let middle = at(reaches);
    let off = vec2(spot.x - middle.x, spot.z - middle.z);
    let far = off.length();

    if far < 1e-4 {
        return true;
    }

    far < rim_out(off.y.atan2(off.x).rem_euclid(TAU) / TAU) + over
}

/// The whole footprint of the pond, which is what the field sows nothing on:
/// the water, the bank and the verge of mown grass round it.
pub fn holds(reaches: f32, spot: Vec3) -> bool {
    inside(reaches, spot, VERGE + BERM * 0.5)
}

/// Whether something `reach` wide at `spot` comes over the pond.
///
/// **Asked of the crown and not of the trunk.** `field::trees` learned that
/// the hard way: a line and a tree three and a half metres wide is a tree
/// planted clear of the line with a sprig the other side of it.
pub fn crowds(reaches: f32, spot: Vec3, reach: f32) -> bool {
    inside(reaches, spot, VERGE + BERM + reach)
}

/// The water, as the triangles of it that are in the pond.
///
/// **Cut to its own shore rather than hidden by something.** A triangle is
/// kept when all three of its corners are inside, so the sheet stops a cell
/// short in places and the bank shows through there, which is the wet stone
/// a pond has at its edge. Kept by the middle of the triangle instead, the
/// sheet would stand past the shore and the corners of the rectangle would
/// be back.
///
/// The count of what comes out does not change from frame to frame, because
/// the outline does not: `update_mesh` writes over what `add_mesh` put down
/// and wants it to fit.
pub fn water_mesh(reaches: f32, water: &blitzkit::water::Water) -> blitzkit::mesh::MeshData {
    let whole = water.surface();
    let edge = shore();
    let mut indices = Vec::with_capacity(whole.indices.len());

    for face in whole.indices.chunks_exact(3) {
        if face.iter().all(|n| {
            whole
                .vertices
                .get(*n as usize)
                .map(|vertex| {
                    inside(
                        reaches,
                        vec3(vertex.position[0], 0.0, vertex.position[2]),
                        edge,
                    )
                })
                .unwrap_or(false)
        }) {
            indices.extend_from_slice(face);
        }
    }

    blitzkit::mesh::MeshData::new(whole.vertices, indices)
}

/// The bank: a ring of ground running from the silt at the bottom, up
/// through the waterline, to the lip and the flat of the verge.
///
/// **One mesh and not a wall.** What a box could never be is this: a surface
/// whose outline wanders and whose section slopes, which is the whole of
/// what a pond's edge is.
pub fn bank_mesh(reaches: f32) -> blitzkit::mesh::MeshData {
    ringed(reaches, -BANK_IN, VERGE, 10, false)
}

/// And the verge: the same ring carried on out to where the field is flat
/// again, wearing the field's own grass.
pub fn verge_mesh(reaches: f32) -> blitzkit::mesh::MeshData {
    ringed(reaches, VERGE, VERGE + BERM, 8, true)
}

/// A ring of ground between two distances either side of the lip.
///
/// `turfed` lays the field's own tiling on it, so the verge and the field
/// are one surface rather than two greens meeting at a line.
fn ringed(
    reaches: f32,
    from: f32,
    to: f32,
    across: usize,
    turfed: bool,
) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let mut vertices = Vec::with_capacity((ROUND + 1) * (across + 1));
    let mut indices = Vec::with_capacity(ROUND * across * 6);

    for step in 0..=across {
        let d = from + (to - from) * step as f32 / across as f32;
        // the slope here, taken either side, which is the normal this ring
        // of the surface leans by
        let by = (to - from) / across as f32 * 0.5;
        let rise = (section(d + by) - section(d - by)) / (by * 2.0);

        for n in 0..ROUND {
            let on = n as f32 / ROUND as f32;
            let at = rim_at(reaches, on, d);
            let turn = on * std::f32::consts::TAU;
            // out of the pond, which is the way the ring widens
            let (sin, cos) = turn.sin_cos();
            let normal = vec3(-cos * rise, 1.0, -sin * rise).normalize();
            let uv = if turfed {
                [at.x / crate::field::TILE, at.z / crate::field::TILE]
            } else {
                [on * 24.0, step as f32]
            };

            vertices.push(Vertex::new(
                [at.x, at.y, at.z],
                [normal.x, normal.y, normal.z],
                uv,
            ));
        }
    }

    for step in 0..across {
        for n in 0..ROUND {
            let (near, far) = (step * ROUND, (step + 1) * ROUND);
            let next = (n + 1) % ROUND;
            let (a, b, c, d) = (
                (near + n) as u32,
                (near + next) as u32,
                (far + next) as u32,
                (far + n) as u32,
            );

            indices.extend_from_slice(&[a, c, b, a, d, c]);
        }
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// The silt, as a sheet laid across the bottom of the bowl.
pub fn bed_mesh(reaches: f32) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let middle = at(reaches);
    let mut vertices = vec![Vertex::new(
        [middle.x, BED, middle.z],
        [0.0, 1.0, 0.0],
        [0.5, 0.5],
    )];
    let mut indices = Vec::with_capacity(ROUND * 3);

    for n in 0..ROUND {
        let on = n as f32 / ROUND as f32;
        let turn = on * std::f32::consts::TAU;
        // a hand past where the bank's own bottom is, so the two lap rather
        // than meet at a line
        let out = rim_out(on) - BANK_IN + 0.2;

        vertices.push(Vertex::new(
            [
                middle.x + turn.cos() * out,
                BED,
                middle.z + turn.sin() * out,
            ],
            [0.0, 1.0, 0.0],
            [turn.cos() * 0.5 + 0.5, turn.sin() * 0.5 + 0.5],
        ));
        indices.extend_from_slice(&[0, 1 + n as u32, 1 + ((n + 1) % ROUND) as u32]);
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// How high the water stands against anything walking into it, and how thick
/// that is.
///
/// **The water is what stops you, which is the whole of why there is no
/// wall.** Nothing is drawn here: what is at the pond's edge to look at is
/// the stones, and what is there to walk into is the water behind them.
pub const STOPS: f32 = 0.55;
pub const STOPS_THICK: f32 = 0.4;

/// How many bands of the bank and the verge are given something to stand on.
pub const TREADS: usize = 6;

/// What you walk into here, and what you walk up.
///
/// The verge rises a third of a metre over four, which is a slope rather
/// than a step and so has to be built as a run of them: each band's top is
/// where the section says the ground is, and no two in a row are further
/// apart than `walk::STEP`.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let points = rim(reaches);
    let mut out = Vec::with_capacity(ROUND * (TREADS + 1));

    for (n, (at, next)) in points.iter().zip(points.iter().cycle().skip(1)).enumerate() {
        let on = n as f32 / ROUND as f32;
        let middle = (*at + *next) * 0.5;
        let span = (*next - *at).abs() + Vec3::splat(STOPS_THICK);

        // the water's own side, which nothing crosses
        out.push(Aabb::from_center_size(
            vec3(middle.x, RIM_UP + STOPS * 0.5, middle.z),
            vec3(span.x, STOPS, span.z),
        ));

        // and the ground out of the field, over the verge and across the
        // flat of the lip, which is the first band and was missing: a lip
        // with nothing under it is a hand's breadth of grass you fall
        // through at the one place everybody stands
        for tread in 0..=TREADS {
            let (near, far) = if tread == 0 {
                (-STOPS_THICK * 0.5, VERGE)
            } else {
                (
                    VERGE + BERM * (tread - 1) as f32 / TREADS as f32,
                    VERGE + BERM * tread as f32 / TREADS as f32,
                )
            };
            // **All four corners of the band and not two of them.** The
            // outline wanders, so the near corner at one angle and the far
            // corner at the next are not a box round the quad between
            // them: where the outline falls away quickly the box skews off
            // it and leaves a hole in the verge. Found by
            // `tests::the_verge_is_what_you_walk_on`, which walked into one.
            let next = (n + 1) as f32 / ROUND as f32;
            let corners = [
                rim_at(reaches, on, near),
                rim_at(reaches, on, far),
                rim_at(reaches, next, near),
                rim_at(reaches, next, far),
            ];
            // a finger either way, because a box round the corners of an
            // arc leaves the middle of the arc outside it
            let room = Vec3::new(0.04, 0.0, 0.04);
            let mut low = corners[0];
            let mut high = corners[0];

            for corner in &corners {
                low = low.min(*corner);
                high = high.max(*corner);
            }

            out.push(Aabb::new(
                vec3(low.x, 0.0, low.z) - room,
                vec3(high.x, section(near).max(0.01), high.z) + room,
            ));
        }
    }

    out
}
/// Where each island stands and how far across its soil is.
///
/// **Neither in the middle and neither lined up with the other**, which is
/// the garden's rule and the reason its pond is off centre and off square.
pub fn islands(reaches: f32) -> Vec<(Vec3, Vec2)> {
    let middle = at(reaches);

    // shares of the pond's own size, like the ducks' rounds and for the
    // same reason: the shape is written down once and everything in the
    // water is said against it
    ISLES
        .iter()
        .map(|(about, size)| {
            (
                vec3(
                    middle.x + about.x * ACROSS.x,
                    0.0,
                    middle.z + about.y * ACROSS.y,
                ),
                *size * ACROSS,
            )
        })
        .collect()
}

/// Where the islands stand and how big they are, as shares of the pond.
pub const ISLES: [(Vec2, Vec2); 2] = [
    (vec2(0.18, 0.02), vec2(0.60, 0.58)),
    (vec2(-0.48, 0.33), vec2(0.33, 0.34)),
];

/// The seeds the two islands' outlines wander by.
pub const ISLE_SEEDS: [u32; 2] = [0x5E1A, 0xB72D];

/// How far out an island's edge stands at a share of the way round it, as a
/// share of its own half size.
///
/// **Round and not square**, which is the whole of what Jake sent this back
/// for. An island in a pond is a heap of soil with stone set round it to
/// hold it, and a rectangle is the one shape that cannot be. Three waves of
/// different lengths, none a multiple of another, so the outline comes back
/// to itself at one turn and nothing about it repeats inside that.
pub fn isle_out(seed: u32, on: f32) -> f32 {
    use std::f32::consts::TAU;

    let phase = (seed % 997) as f32 * 0.0063;

    1.0 + (on * TAU * 3.0 + phase).sin() * 0.085
        + (on * TAU * 5.0 - phase * 2.0).sin() * 0.055
        + (on * TAU * 2.0 + phase * 3.0).sin() * 0.04
}

/// How high an island's soil stands, which is reckoned off the water rather
/// than off the ground: an island is a thing in a pond.
pub fn isle_up() -> f32 {
    UP + ISLE_OVER
}

/// One island, as a mesh: a drum with a wandering outline, standing on the
/// bed with its top at `isle_up`.
///
/// **Its own mesh rather than a box**, because the outline is the point.
/// Built about its own middle at a half metre across each way, so a game
/// scales it by the island's own size the way everything else here is
/// scaled.
pub fn isle_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    // how much of the height is the side wall rather than the top, which is
    // what gives the rim something to sit against
    const WALL: f32 = 0.34;

    crate::garden::faceted(blitzkit::mesh::MeshData::surface(44, 10, |round, up| {
        // wound the way the garden's lumps are wound, for looking at from
        // outside
        let turn = -round * TAU;
        let out = isle_out(seed, round) * 0.5;
        let (along, down) = if up <= WALL {
            (out, up / WALL)
        } else {
            (out * (1.0 - (up - WALL) / (1.0 - WALL)), 1.0)
        };

        vec3(turn.cos() * along, down, turn.sin() * along)
    }))
}

/// How far an island's soil stands over the water.
///
/// A step and not a kerb: enough that the water is plainly lower than the
/// soil, which is what tells an island from a shoal, and not so much that it
/// reads as a tub standing in a pond.
pub const ISLE_OVER: f32 = 0.2;

/// One stone set along a rim: where it sits, how big it is across, up and
/// through, which way it is turned and which cut of rock it is.
pub struct Set {
    pub at: Vec3,
    pub size: Vec3,
    pub turn: f32,
    pub cut: usize,
}

/// How far apart the stones round the water are set, how big they run, and
/// how far in from the line they wander.
///
/// **Closer together than they are wide**, so the line has no gap in it:
/// what the stones are hiding is the straight edge of a rectangle of water,
/// and one gap is a centimetre of ruled line in the middle of a pond.
///
/// **And cobbles rather than boulders.** A stone a metre across set round a
/// pond is a rockery, which is what the first run of these came out as. A
/// cobble set in mortar is a hand's breadth to two: small enough that what
/// you read is the line they make and not the stones themselves.
pub const SET_EVERY: f32 = 0.1;
pub const SET_SMALL: f32 = 0.24;
pub const SET_BIG: f32 = 0.38;
pub const SET_IN: f32 = 0.42;

/// How far a set stone's top stands over the ground it is set in.
pub const SET_PROUD: f32 = 0.075;

/// How far apart the rows of stone run up the bank, and how many of them
/// lie under the shore.
///
/// **From under the water to a little over the grass**, which is Jake's own
/// section of it. The rows are reckoned off the shore and not off the lip,
/// so that one of them is exactly where the water ends however the bank is
/// cut: a row near the waterline and not on it leaves a ring of bare bank,
/// which is a tide mark round a pond.
pub const ROW_STEP: f32 = 0.3;
pub const ROWS_UNDER: usize = 2;

/// Where the rows run, as distances out from the lip.
pub fn rows() -> Vec<f32> {
    let shore = shore();
    let mut out = Vec::new();
    let mut d = shore - ROW_STEP * ROWS_UNDER as f32;

    while d < VERGE * 0.5 {
        out.push(d);
        d += ROW_STEP;
    }

    out
}

/// Stones laid along a closed line, overlapping, each one its own size.
///
/// `points` is the line, in order and closed; `every` how far apart they are
/// set; `small` and `big` how far across they run; `into` how far each may
/// wander off the line, toward the inside.
fn laid_along(
    points: &[Vec3],
    every: f32,
    small: f32,
    big: f32,
    into: f32,
    top: f32,
    seed: u32,
) -> Vec<Set> {
    let mut out = Vec::new();
    let mut n = 0u32;
    let mut owed = 0.0f32;

    for (at, next) in points.iter().zip(points.iter().cycle().skip(1)) {
        let along = *next - *at;
        let long = along.length();

        if long <= 1e-4 {
            continue;
        }

        let way = along / long;
        // which way is into the pond, which is a quarter turn off the way
        // the line runs: the line is wound so that this points inward
        let inward = vec3(-way.z, 0.0, way.x);
        let mut on = owed;

        while on < long {
            // its own run of numbers, off its own number and the seed, so a
            // stone is the same stone every time anybody looks
            let roll = |salt: u32| {
                let mixed = (n.wrapping_mul(0x9E37_79B9) ^ seed.wrapping_mul(salt)) as f32;

                (mixed * 0.000_000_123).sin() * 0.5 + 0.5
            };
            let wide = small + (big - small) * roll(0x2545_F491);
            let foot = *at + way * on + inward * (roll(0x8363_1F2B) * into);

            // a cobble is a flattened lump, and most of it is in the
            // mortar: what stands out is `SET_PROUD` and a little either
            // way, so the line is level with lumps in it rather than a row
            // of different sized stones standing on a wall
            let deep = wide * 0.66;
            let stands = top + SET_PROUD * (0.4 + roll(0x27D4_EB2D) * 1.2);

            out.push(Set {
                at: vec3(foot.x, stands - deep * 0.5, foot.z),
                size: vec3(wide, deep, wide * 0.86),
                turn: roll(0xC2B2_AE35) * std::f32::consts::TAU,
                cut: (n as usize) % crate::garden::CUTS.len(),
            });

            n += 1;
            on += every;
        }

        owed = on - long;
    }

    out
}

/// The stones set round the water: a close run on each band of the bank,
/// from under the water to over the grass, and a looser one over them.
///
/// **Rows up the slope rather than a line at the top**, which is the section
/// Jake asked for: a pond's edge is a cobbled bank going down into it, not a
/// kerb standing round it. The close runs are what the bank is faced with
/// and so may not wander off their own band at all; the loose one is what
/// keeps them from reading as courses of brick.
pub fn set_stones(reaches: f32) -> Vec<Set> {
    let mut out = Vec::new();

    for (band, d) in rows().iter().enumerate() {
        let line: Vec<Vec3> = (0..ROUND)
            .map(|n| rim_at(reaches, n as f32 / ROUND as f32, *d))
            .collect();
        // the shore's own row is the one the eye reads as the edge of the
        // water, so it is the tightest; the rest may breathe
        let every = if *d < -0.3 {
            SET_EVERY * 1.6
        } else {
            SET_EVERY
        };

        out.extend(laid_along(
            &line,
            every,
            SET_SMALL,
            SET_BIG,
            0.0,
            section(*d),
            0x9D17u32.wrapping_mul(band as u32 + 1),
        ));
    }

    // and the bigger ones lying over them, which are what a bank like this
    // actually has: cobbles with a stone in among them every few feet
    let over: Vec<Vec3> = (0..ROUND)
        .map(|n| rim_at(reaches, n as f32 / ROUND as f32, -0.55))
        .collect();

    out.extend(laid_along(
        &over,
        SET_EVERY * 5.4,
        SET_BIG * 0.9,
        SET_BIG * 1.7,
        SET_IN,
        section(-0.55),
        0x3C71,
    ));

    out
}

/// The outline of an island, as the points of it.
pub fn isle_rim(reaches: f32, which: usize) -> Vec<Vec3> {
    let isles = islands(reaches);
    let Some((middle, size)) = isles.get(which) else {
        return Vec::new();
    };
    let seed = ISLE_SEEDS[which % ISLE_SEEDS.len()];
    let half = *size * 0.5;

    (0..48)
        .map(|n| {
            // wound the other way round from the pond's rim, because what is
            // inside an island is the island: the stones lean out of it
            let on = 1.0 - n as f32 / 48.0;
            let turn = on * std::f32::consts::TAU;
            let out = isle_out(seed, on);

            vec3(
                middle.x + turn.cos() * half.x * out,
                0.0,
                middle.z + turn.sin() * half.y * out,
            )
        })
        .collect()
}

/// The stones set round the islands, which are the pond's own in a smaller
/// size.
pub fn isle_stones(reaches: f32) -> Vec<Set> {
    let mut out = Vec::new();

    for (which, (middle, _)) in islands(reaches).into_iter().enumerate() {
        let rim = isle_rim(reaches, which);
        let seed = ISLE_SEEDS[which % ISLE_SEEDS.len()];

        // two rows, the lower one standing in the water the way the pond's
        // own do, so an island is banked rather than kerbed
        for (band, (down, out_by)) in [(0.0f32, 0.0f32), (ISLE_OVER + 0.06, -0.26)]
            .iter()
            .copied()
            .enumerate()
        {
            let line: Vec<Vec3> = rim
                .iter()
                .map(|spot| {
                    let off = vec2(spot.x - middle.x, spot.z - middle.z);
                    let far = off.length().max(1e-4);

                    vec3(
                        middle.x + off.x / far * (far + out_by),
                        0.0,
                        middle.z + off.y / far * (far + out_by),
                    )
                })
                .collect();

            out.extend(laid_along(
                &line,
                SET_EVERY * 0.9,
                SET_SMALL * 0.72,
                SET_BIG * 0.8,
                0.0,
                isle_up() - down,
                seed.wrapping_mul(band as u32 * 7919 + 1),
            ));
        }

        out.extend(laid_along(
            &rim,
            SET_EVERY * 4.0,
            SET_BIG * 0.8,
            SET_BIG * 1.3,
            SET_IN * 0.3,
            isle_up(),
            seed.wrapping_mul(7919),
        ));
    }

    out
}

/// The trees standing on the islands.
///
/// **Written down rather than sown.** Five trees on two islands is a place,
/// and a place is a list: the field's own rule would put a different wood
/// here every time either island moved.
pub fn trees(reaches: f32) -> Vec<crate::field::Stood> {
    let isles = islands(reaches);
    let kinds = crate::field::kinds();
    let mut out = Vec::new();

    // **Small, because an island is looked at across the water.** The field
    // stands these at three quarters to one and a third, which is a tree you
    // walk under. Five of those on nine metres of soil is a thicket with a
    // pond behind it: from the near kerb they filled the sky and the water
    // was a strip under them.
    //
    // **Oak and aspen, and not the fir.** `field::kinds` has three and the
    // field stands all of them, which is right for a wood you look at from
    // a distance. An island is looked at across twenty metres of water with
    // nothing else on it: an evergreen there is a green wall in a field
    // that has turned, and what the photograph has on its island is bare
    // wood and the leaves that came off it.
    for (isle, off, turn, size, kind) in [
        (0usize, vec2(-2.4, -0.6), 0.7, 0.72, 1usize),
        (0, vec2(0.9, 1.1), 2.4, 0.58, 2),
        (0, vec2(2.7, -1.0), 4.1, 0.66, 1),
        (1, vec2(-1.1, 0.4), 1.8, 0.52, 2),
        (1, vec2(1.2, -0.5), 5.0, 0.44, 1),
    ] {
        let Some((middle, _)) = isles.get(isle) else {
            continue;
        };
        if kinds.get(kind).is_none() {
            continue;
        }

        out.push(crate::field::Stood {
            at: vec3(middle.x + off.x, isle_up(), middle.z + off.y),
            turn,
            size,
            what: crate::field::What::Tree(kind),
        });
    }

    out
}

/// How many patches of leaves lie on the bigger island.
///
/// **What is on an island in November is leaves and not soil.** Scattered
/// thinly they read as litter on a beach; what the photograph has is a floor
/// of them with the bed showing through in patches.
pub const LEAVES: usize = 260;

/// The leaf fall on the islands.
///
/// What is on an island in November is what came off the trees standing on
/// it, which is the whole of why the soil is not bare.
pub fn litter(reaches: f32) -> Vec<crate::field::Stood> {
    let mut out = Vec::new();

    for (n, (middle, size)) in islands(reaches).into_iter().enumerate() {
        let many = if n == 0 { LEAVES } else { LEAVES / 2 };
        let half = size * 0.5;

        for leaf in 0..many {
            // a scatter that is the same every run, off the leaf's own
            // number: an island whose litter moved when anything else did
            // would be an island that twinkles
            let seed = (n * 977 + leaf * 31 + 7) as f32;
            // **Round the island rather than across a rectangle**, because
            // the island is not one any more. The root is what keeps a
            // scatter from crowding the middle: an angle and a radius taken
            // flat puts half of them in the inner quarter.
            let round = (seed * 0.7891).sin() * 0.5 + 0.5;
            let far = ((seed * 1.3177).cos() * 0.5 + 0.5).sqrt() * 0.88;
            let turn = (seed * 2.1119).sin() * std::f32::consts::PI;
            let (along, across) = (
                (round * std::f32::consts::TAU).cos() * far,
                (round * std::f32::consts::TAU).sin() * far,
            );

            out.push(crate::field::Stood {
                // on the soil and not over it: what lays a patch of
                // leaves the hair's breadth clear of the ground it is on is
                // the drawing, the same hair's breadth the field's own
                // litter is laid at
                at: vec3(
                    middle.x + along * half.x,
                    isle_up(),
                    middle.z + across * half.y,
                ),
                turn,
                size: 1.05 + (seed * 0.5531).sin().abs() * 0.65,
                what: crate::field::What::Litter,
            });
        }
    }

    out
}

/// How long a duck is, how wide and how deep in the body, and how much of it
/// stands out of the water.
///
/// A mallard is about half a metre nose to tail with a third of that under
/// the surface. What makes a duck on water read as a duck is the waterline:
/// a body floating with its back dry and its belly gone.
pub const DUCK_LONG: f32 = 0.44;
pub const DUCK_WIDE: f32 = 0.21;
pub const DUCK_DEEP: f32 = 0.19;
pub const DUCK_RIDES: f32 = 0.055;

/// How big a duck's head is, how far it stands over the body, how thick the
/// neck under it is and how long the bill on it.
pub const HEAD_WIDE: f32 = 0.115;
pub const NECK_UP: f32 = 0.155;
pub const NECK_THICK: f32 = 0.058;
pub const BILL_LONG: f32 = 0.082;

/// How far forward of the middle of the body the neck stands.
pub const NECK_ON: f32 = 0.11;

/// How fat a duck's body is a given way along itself, nought at the tail and
/// one at the bill end.
///
/// Full at the breast and drawn out to the tail, which is a duck seen from
/// above: the widest part is forward of the middle, not at it.
pub fn duck_waist(along: f32) -> f32 {
    let on = along.clamp(0.0, 1.0);

    (on * (1.0 - on) * 4.0).powf(0.62) * (0.55 + 0.45 * on)
}

/// How far the back of a duck rises a given way along itself.
///
/// The tail end lifts clear of the water, which is the line everybody draws
/// when they draw a duck.
pub fn duck_rise(along: f32) -> f32 {
    let on = along.clamp(0.0, 1.0);

    (1.0 - on).powi(3) * 0.55
}

/// A duck's body, built once and stood many times.
///
/// **Nose on +z**, which is where this building puts the front of anything
/// that goes somewhere: a heading is `dx.atan2(dz)` and
/// `Quat::from_rotation_y` takes +z to it. The koi are laid the same way.
pub fn duck_mesh() -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    blitzkit::mesh::MeshData::surface(26, 16, |along, round| {
        let waist = duck_waist(along);
        let (sin, cos) = (round * TAU).sin_cos();

        vec3(
            cos * waist * DUCK_WIDE * 0.5,
            duck_rise(along) * DUCK_DEEP + sin * waist * DUCK_DEEP * 0.5,
            (along - 0.5) * DUCK_LONG,
        )
    })
}

/// One duck's circuit: what it goes round, how far out, how many turns a
/// second and where on the round it started.
pub struct Round {
    /// The middle of the round, as a share of the pond's own size.
    ///
    /// **Shares and not metres**, because the pond's size is written down
    /// once in [`ACROSS`] and a duck drawn in metres swims out of it the
    /// first time anybody changes the shape. Three of these had to be moved
    /// by hand after the outline was given more wander, which is the fault
    /// this takes away.
    pub about: Vec2,
    /// How far out from that middle it goes, each way, in the same shares:
    /// an ellipse, because the pond is longer than it is wide and so is
    /// everything on it.
    pub out: Vec2,
    /// Turns a second, signed: a negative one goes round the other way.
    pub turns: f32,
    /// Where on its own round it is when the field opens, in turns.
    pub from: f32,
    /// How big this one is against the others.
    pub size: f32,
    /// Whether it is a drake, which is the one that has a green head.
    pub drake: bool,
}

/// The ducks, each on a slow round of its own.
///
/// **Six, at six different speeds, none of them a divisor of another.** Two
/// ducks going round at rates that divide meet in the same place every time,
/// which is the one thing a pond full of birds never does.
pub const ROUNDS: [Round; 6] = [
    // the south water, which is the open half: the big island is north of it
    // and the way you come at the pond is from the south
    Round {
        about: vec2(-0.296, -0.560),
        out: vec2(0.310, 0.083),
        turns: 0.021,
        from: 0.12,
        size: 1.0,
        drake: true,
    },
    Round {
        about: vec2(0.169, -0.539),
        out: vec2(0.141, 0.052),
        turns: -0.017,
        from: 0.63,
        size: 0.92,
        drake: false,
    },
    Round {
        about: vec2(-0.508, -0.404),
        out: vec2(0.113, 0.124),
        turns: 0.013,
        from: 0.41,
        size: 1.04,
        drake: true,
    },
    // the east end, past the big island
    Round {
        about: vec2(0.606, 0.000),
        out: vec2(0.056, 0.435),
        turns: -0.026,
        from: 0.88,
        size: 0.88,
        drake: false,
    },
    // and the north water, between the islands and the far bank
    Round {
        about: vec2(0.106, 0.545),
        out: vec2(0.261, 0.095),
        turns: 0.019,
        from: 0.27,
        size: 0.96,
        drake: false,
    },
    Round {
        about: vec2(-0.240, 0.560),
        out: vec2(0.070, 0.083),
        turns: -0.011,
        from: 0.74,
        size: 1.0,
        drake: true,
    },
];

/// Where a duck is, which way it is pointing and how big it is.
pub struct Swims {
    pub at: Vec3,
    pub heading: f32,
    pub size: f32,
    pub drake: bool,
}

/// How hard a duck pushes the water it is on.
///
/// Per second, not per frame, so the wake is the same wake whatever the room
/// is managing. Gentle: six birds stirring hard is a chop, and what this pond
/// mostly is, is sky lying on water with something moving across it.
pub const STIRS: f32 = 0.22;

/// The ducks, where they are this moment.
///
/// Each rides the water it is on rather than a line drawn at nought, which is
/// what `water` is handed over for: a duck on a wake sits on the wake.
pub fn ducks(reaches: f32, since: f32, water: &blitzkit::water::Water) -> Vec<Swims> {
    use std::f32::consts::TAU;

    let middle = at(reaches);

    ROUNDS
        .iter()
        .map(|round| {
            let round_to = (round.from + since * round.turns) * TAU;
            let way = round.turns.signum();
            // shares of the pond, turned into metres here and nowhere else
            let about = round.about * ACROSS;
            let out = round.out * ACROSS;
            let (x, z) = (
                middle.x + about.x + round_to.cos() * out.x,
                middle.z + about.y + round_to.sin() * out.y,
            );
            // the tangent of the ellipse it is going round, which is not the
            // tangent of a circle unless the two radii agree
            let (dx, dz) = (-round_to.sin() * out.x * way, round_to.cos() * out.y * way);

            Swims {
                at: vec3(x, water.height_at(x, z) - DUCK_DEEP * DUCK_RIDES, z),
                heading: dx.atan2(dz),
                size: round.size,
                drake: round.drake,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REACHES: f32 = 20.0;

    /// Walks a body from `from` toward `to` for `seconds` and says where it
    /// got to.
    ///
    /// The field's own ground as well as the pond's, because a walk that
    /// only knows the bank falls through the grass it starts on.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid: Vec<Aabb> = solid(REACHES)
            .into_iter()
            .chain(crate::field::solid(REACHES, from))
            .collect();
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    /// How far out the pond ever reaches, which is what a sweep round it is
    /// measured against.
    fn out_to() -> f32 {
        (0..ROUND * 4)
            .map(|n| rim_out(n as f32 / (ROUND * 4) as f32))
            .fold(0.0f32, f32::max)
    }

    /// Spec 0129: the pond is north of the way there and clear of the
    /// bridge's deck.
    #[test]
    fn the_pond_keeps_clear_of_the_way_and_the_bridge() {
        let middle = at(REACHES);
        let out = out_to() + VERGE + BERM;
        let trail = crate::trail::Trail::default();
        let deck = crate::bridge::solid(REACHES, crate::bridge::reach(&trail));

        for x in -30..=30 {
            for z in -30..=30 {
                let spot = vec3(
                    middle.x + x as f32 * out / 30.0,
                    0.0,
                    middle.z + z as f32 * out / 30.0,
                );

                if !holds(REACHES, spot) {
                    continue;
                }

                assert!(
                    !crate::village::on_the_way(REACHES, spot)
                        && !crate::village::along_the_path(REACHES, spot),
                    "the pond is on the way there at {} {}",
                    spot.x,
                    spot.z
                );

                for box_ in &deck {
                    assert!(
                        spot.x < box_.min.x
                            || spot.x > box_.max.x
                            || spot.z < box_.min.z
                            || spot.z > box_.max.z,
                        "the pond is under the bridge at {} {}",
                        spot.x,
                        spot.z
                    );
                }
            }
        }
    }

    /// Spec 0129: you cannot walk into the water, from any side.
    ///
    /// Walked rather than measured, which is what the pool in the baths
    /// taught this building: four rises each under a step add up to a wall
    /// you cannot climb, and four boxes each higher than a step add up to
    /// nothing if there is a gap between two of them.
    ///
    /// **And the walk has to go up the verge first**, which is the whole of
    /// what this build changed: the ground rises a third of a metre to the
    /// lip, and if that rise were a step rather than a slope the walk would
    /// stop at the bottom of it and this would pass for the wrong reason.
    #[test]
    fn you_cannot_walk_into_the_water() {
        let middle = at(REACHES);
        let out = out_to() + VERGE + BERM + 2.0;

        for eighth in 0..8 {
            let turn = eighth as f32 / 8.0 * std::f32::consts::TAU;
            let from = vec3(
                middle.x + turn.cos() * out,
                0.0,
                middle.z + turn.sin() * out,
            );
            let ended = walked(from, vec3(middle.x, 0.0, middle.z), 14.0);

            assert!(
                !inside(REACHES, ended, -0.1),
                "a walk from {} radians ended in the water at {} {}",
                turn,
                ended.x,
                ended.z
            );
            // and it got somewhere: a walk stopped at the foot of the verge
            // proves nothing about the water
            assert!(
                inside(REACHES, ended, VERGE + BERM),
                "a walk from {} radians never reached the pond at all",
                turn
            );
        }
    }

    /// Spec 0129: the lip is ringed the whole way round, and what stands at
    /// it is higher than a step.
    #[test]
    fn nothing_gets_over_the_lip() {
        let middle = at(REACHES);
        let ring = solid(REACHES);
        let stops: Vec<&Aabb> = ring
            .iter()
            .filter(|box_| box_.max.y > RIM_UP + crate::walk::STEP)
            .collect();

        assert!(
            stops.len() >= ROUND,
            "only {} of the ring stands over the lip",
            stops.len()
        );

        // every way out of the middle, at a fifth of a degree
        for step in 0..1800 {
            let turn = step as f32 * std::f32::consts::TAU / 1800.0;
            let (sin, cos) = turn.sin_cos();
            let mut met = false;

            for n in 0..((out_to() + 1.0) * 100.0) as u32 {
                let spot = vec3(
                    middle.x + cos * n as f32 * 0.01,
                    0.0,
                    middle.z + sin * n as f32 * 0.01,
                );

                if stops.iter().any(|box_| {
                    spot.x >= box_.min.x
                        && spot.x <= box_.max.x
                        && spot.z >= box_.min.z
                        && spot.z <= box_.max.z
                }) {
                    met = true;
                    break;
                }
            }

            assert!(met, "nothing stands in the way out at {} radians", turn);
        }
    }

    /// Spec 0129: what you walk on across the verge is the ground that is
    /// drawn there.
    ///
    /// **Not whether it can be climbed**, which was the first thing asked
    /// here and is true of anything: the whole verge rises 0.34 and
    /// `walk::STEP` is 0.42, so a single box would let anybody up it. What
    /// the treads are for is that your feet are on the grass rather than a
    /// third of a metre over it, and that is what this measures.
    #[test]
    fn the_verge_is_what_you_walk_on() {
        let solid = solid(REACHES);
        let mut worst = 0.0f32;

        for n in 0..ROUND {
            let on = (n as f32 + 0.5) / ROUND as f32;

            for step in 0..=12 {
                let d = BERM * step as f32 / 12.0;
                let spot = rim_at(REACHES, on, d);
                let ground = section(d);
                let stood = solid
                    .iter()
                    .filter(|box_| {
                        spot.x >= box_.min.x
                            && spot.x <= box_.max.x
                            && spot.z >= box_.min.z
                            && spot.z <= box_.max.z
                            && box_.max.y <= RIM_UP + 0.01
                    })
                    .map(|box_| box_.max.y)
                    .fold(0.0f32, f32::max);

                worst = worst.max((stood - ground).abs());
            }
        }

        assert!(
            worst < crate::walk::STEP * 0.5,
            "what you stand on is {} off the ground that is drawn",
            worst
        );
    }

    /// Spec 0129: the outline wanders rather than being one smooth curve.
    ///
    /// **Which is what Jake asked for in as many words**: it should look
    /// natural with a curved perimeter. A superellipse on its own is curved
    /// and reads as a swimming pool, because every pond anybody has seen
    /// goes in and out. Counted as the turns in the radius round one lap:
    /// the shape's own four corners give eight, and what the waves add on
    /// top is the wandering.
    #[test]
    fn the_outline_wanders() {
        let many = ROUND * 8;
        let out: Vec<f32> = (0..many).map(|n| rim_out(n as f32 / many as f32)).collect();
        let mut turns = 0;

        for n in 0..many {
            let (before, here, after) = (out[(n + many - 1) % many], out[n], out[(n + 1) % many]);

            if (here - before).signum() != (after - here).signum() {
                turns += 1;
            }
        }

        // the shape on its own turns four times in a lap, at its own two
        // ends and its own two sides. Anything past that is the wander.
        assert!(
            turns >= 8,
            "the outline turns {} times in a lap, which is a smooth curve",
            turns
        );
    }

    /// Spec 0129: the pond has no straight edge anywhere.
    ///
    /// **What it is cut out of has four.** The water the engine simulates is
    /// a rectangle, and every build before this one showed some of it: a
    /// band of stone round the whole thing, then the same with its corners
    /// cut off. What is drawn now is the triangles of it inside the
    /// outline, and this is what says so.
    #[test]
    fn the_water_is_cut_to_its_own_shore() {
        let Some(mut renderer) =
            pollster::block_on(blitzkit::renderer::Renderer::offscreen(16, 16))
        else {
            eprintln!("no adapter here, so nothing was cut");
            return;
        };
        let _ = &mut renderer;
        let water = water(REACHES);
        let mesh = water_mesh(REACHES, &water);
        let whole = water.surface();

        assert!(
            mesh.indices.len() < whole.indices.len(),
            "the water was not cut at all"
        );
        // a rounded oblong inscribed in the rectangle it is cut from keeps
        // about two fifths of it: much less and the outline has collapsed
        assert!(
            mesh.indices.len() * 3 > whole.indices.len(),
            "the cut took {} of {} triangles, which is not a pond",
            mesh.indices.len() / 3,
            whole.indices.len() / 3
        );

        for n in &mesh.indices {
            let spot = mesh.vertices[*n as usize].position;

            assert!(
                inside(REACHES, vec3(spot[0], 0.0, spot[2]), shore()),
                "a corner of the water stands at {} {}, outside the shore",
                spot[0],
                spot[2]
            );
        }
    }

    /// Spec 0129: both islands stand in the water, clear of its edge and of
    /// each other.
    #[test]
    fn the_islands_stand_in_the_water() {
        let isles = islands(REACHES);

        assert_eq!(isles.len(), 2, "the pond has {} islands", isles.len());

        let mut rims = Vec::new();

        for which in 0..isles.len() {
            let rim = isle_rim(REACHES, which);

            assert!(
                rim.len() > 24,
                "an island's outline is {} points",
                rim.len()
            );

            for spot in &rim {
                assert!(
                    inside(REACHES, *spot, shore() - 1.0),
                    "an island reaches {} {}, which is at the water's edge",
                    spot.x,
                    spot.z
                );
            }

            rims.push(rim);
        }

        for one in &rims[0] {
            for two in &rims[1] {
                assert!(
                    one.distance(*two) > 2.0,
                    "the two islands meet at {} {}",
                    one.x,
                    one.z
                );
            }
        }
    }

    /// Spec 0129: the field sows nothing in the pond, and no tree leans over
    /// it.
    ///
    /// The fault this is written against is the one spec 0127 fixed for the
    /// way there and spec 0117 for the bridge's abutment: a thing standing
    /// in the field that the field was never told about grows grass through
    /// its middle.
    #[test]
    fn nothing_of_the_field_grows_in_the_pond() {
        let middle = at(REACHES);
        let mut sown = 0;

        for cell in crate::field::cells_round(vec3(middle.x, 0.0, middle.z), 40.0) {
            for stood in crate::field::cover(REACHES, cell) {
                assert!(
                    !holds(REACHES, stood.at),
                    "the field sowed something at {} {}, which is on the pond",
                    stood.at.x,
                    stood.at.z
                );
                sown += 1;
            }

            for stood in crate::field::trees(REACHES, cell) {
                let crate::field::What::Tree(kind) = stood.what else {
                    continue;
                };
                let kinds = crate::field::kinds();
                let reach = kinds
                    .get(kind)
                    .map(|tree| crate::field::crown_reach(tree, stood.size))
                    .unwrap_or(0.0);

                assert!(
                    !crowds(REACHES, stood.at, reach),
                    "a tree at {} {} leans over the water",
                    stood.at.x,
                    stood.at.z
                );
            }
        }

        assert!(
            sown > 100,
            "only {} things were sown round the pond, so this proved nothing",
            sown
        );
    }

    /// Spec 0129: the water is seen to end in stone, the whole way round.
    ///
    /// **Because what it actually ends in is a cut edge.** The sheet stops
    /// where the bank rises through it, and the stones of that row are what
    /// the eye reads as the shore: a gap in them is a hand's breadth of
    /// bare bank in the middle of a pond.
    #[test]
    fn the_water_ends_in_stone() {
        let set = set_stones(REACHES);
        let line: Vec<Vec3> = (0..ROUND * 2)
            .map(|n| rim_at(REACHES, n as f32 / (ROUND * 2) as f32, shore()))
            .collect();

        for spot in &line {
            assert!(
                set.iter().any(|stone| {
                    (spot.x - stone.at.x).hypot(spot.z - stone.at.z)
                        < stone.size.x.min(stone.size.z) * 0.5
                }),
                "the water's edge is bare at {} {}",
                spot.x,
                spot.z
            );
        }
    }

    /// Spec 0129: a duck is on the water all the way round its circuit,
    /// rather than beside it or under it.
    #[test]
    fn a_duck_rides_the_water_it_is_on() {
        let middle = at(REACHES);
        let water = water(REACHES);
        let isles = islands(REACHES);

        for tenth in 0..240 {
            let since = tenth as f32 * 2.5;

            for (n, duck) in ducks(REACHES, since, &water).into_iter().enumerate() {
                assert!(
                    inside(REACHES, duck.at, shore() - DUCK_LONG),
                    "duck {} is off the water at {} {}",
                    n,
                    duck.at.x,
                    duck.at.z
                );
                assert!(
                    (duck.at.y - middle.y).abs() < 0.2,
                    "duck {} rides at {} against water at {}",
                    n,
                    duck.at.y,
                    middle.y
                );

                // and not through an island, asked of its outline rather
                // than of a rectangle round it, which is the one thing in
                // the water a duck could swim into
                for which in 0..isles.len() {
                    for spot in isle_rim(REACHES, which) {
                        assert!(
                            (duck.at.x - spot.x).hypot(duck.at.z - spot.z) > DUCK_LONG,
                            "duck {} is on island {} at {} {}",
                            n,
                            which,
                            duck.at.x,
                            duck.at.z
                        );
                    }
                }
            }
        }
    }
}
