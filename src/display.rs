//! What is down the sides of the gallery: the engine's own shapes, turning.
//! Spec 0002.
//!
//! **Down the sides and not across the end.** They stood in one row on the
//! far wall, opening round spec 0039's picture, and that row is what set
//! the room's width: six at 0.95 with the picture in the hole came to 10.4,
//! which put the gallery's west wall 2.8 inside the wing beside it. Jake,
//! having opened the wall between the wing and the nook: how about reshaping
//! the gallery so its objects are along the sides leading to the fractal,
//! then the room could be centered with the hall.
//!
//! So the room no longer has to be as wide as six shapes, only as wide as
//! the picture at the end of it, and `room::GALLERY_WIDE` is the hall's own
//! width instead. You walk in off the aisle, pass three pairs, and the
//! picture is what the room points at.
//!
//! Built from `blitzkit-shapes`, which is the same library the examples build
//! from, so these are the objects themselves rather than pictures of them and
//! nothing here can drift from what the examples draw.

use blitzkit::mesh::MeshData;
use glam::{vec3, Quat, Vec3};

/// How high a shape's middle sits and how fast it turns.
pub const HIGH: f32 = 1.35;
pub const TURNS: f32 = 0.35;

/// How far apart they stand along the wall, and how far off it.
///
/// **The row no longer sets the room; the room sets the row.** This used to
/// be the number that gave way: the cabinets stood 1.65 either side of the
/// middle, so anything wider than 3.3 went behind them, and when a sixth
/// shape arrived this came down from 0.72 to 0.59 and every shape in the row
/// shrank by 18 per cent to pay for it. Jake read that off the screen as the
/// plinths growing and wanting a wider berth.
///
/// Widening the hall to pay for it was tried and does not work: the aisle
/// and the sight are the same 4.0 on purpose, so the room cannot grow by as
/// much as 0.2 without things coming alive from further away.
///
/// So the wall moved instead, into a room of its own. Spec 0022. Six shapes
/// at 0.95 are 5.30 of row in a gallery 6.80 wide, which leaves three
/// quarters of a metre clear at each end, and every shape is 39 per cent
/// bigger than the squeezed ones and 32 per cent bigger than the five ever
/// were.
///
/// The row is the steps plus one shape's width, and the first sizing of this
/// forgot the shape: 4.75 was quoted where 5.30 was meant, and the test below
/// caught it on its first run, which is the whole of what it is for.
pub const APART: f32 = 0.95;

/// How much clear wall the row has to leave at each end.
///
/// It was nought, and that was the finding rather than the intent: on the
/// old wall six shapes cleared the gap by half a centimetre a side, which is
/// a row that did not fit. The gallery of spec 0022 is sized to give them
/// this instead, so a row that stops being able to leave it is a row that has
/// outgrown the room rather than one to be shrunk again.
// Measured rather than drawn: these three exist so the fit can be checked
// and are not read by anything that renders. The check is the point, since
// nothing said this row was bounded until the row stopped fitting.
#[allow(dead_code)]
pub const CLEAR: f32 = 0.6;

/// The gap the two ranks have to live in, which is the gallery's width.
///
/// It was the clear span between the two cabinets at the end of the hall's
/// rows, because the row was on the wall between them; then spec 0022's
/// gallery, because the row was as wide as the room. Now the ranks are down
/// the sides and what has to fit across here is the walkway between them.
#[allow(dead_code)]
pub fn span() -> f32 {
    crate::room::GALLERY_WIDE
}

/// How much clear wall the picture keeps either side of its own moulding.
///
/// Measured to the next shape's sweep and not to its middle, which is what
/// you are actually asking when you ask whether a thing is in the way.
#[allow(dead_code)]
pub const BESIDE: f32 = 0.4;

/// How far a shape reaches from its own middle while it is turning, which
/// is further than it is wide. See [`SWEEP`].
pub fn sweep() -> f32 {
    ACROSS * SWEEP * 0.5
}

/// How far apart one pair stands from the next, down the room.
///
/// **A bay, and not [`APART`].** That one is how close two shapes can
/// stand on one wall without their sweeps touching, which is the right
/// question for a row and the wrong one for a walk: three pairs at 0.95
/// put all six inside the first two metres of the room and left four of
/// bare plaster leading to the picture. What sets this is the pace of
/// walking past them, so it is the room's own half width and the plan
/// divides into squares.
///
/// **Or what the load room's door needs, whichever is more.** That door
/// is in the gallery's east wall, so one gap between two plinths of the
/// east rank has to hold it: two sweeps, the door, and a clear jamb.
/// `back::tests::you_can_walk_in_and_out_again` walked into a plinth
/// standing in the doorway, which is what found this.
pub fn bay() -> f32 {
    let door = sweep() * 2.0 + crate::back::WAY + CLEAR;

    door.max(crate::room::GALLERY_WIDE * 0.5)
}

/// How far down the room one side's exhibits run, clear wall at the near
/// end included.
///
/// Three a side standing opposite each other, so this is a rank's length
/// and not the whole row's. `room::gallery_deep` is this plus what it
/// takes to stand back from the picture, which is the one thing the depth
/// has to hold besides them.
pub fn alongside() -> f32 {
    CLEAR + (count() / 2 - 1) as f32 * bay() + ACROSS
}

/// How far a shape's own middle stands off the wall behind it, counting
/// its sweep.
///
/// What the walkway between the two ranks is measured against, which is
/// the question a room this narrow has that the old one did not.
///
/// Measured rather than drawn, like the three above it: nothing that
/// renders reads this, and the check is the point.
#[allow(dead_code)]
pub fn reaches_in() -> f32 {
    OFF_THE_WALL + sweep()
}

pub const OFF_THE_WALL: f32 = 0.75;

/// How wide each one is drawn, across its own widest way.
///
/// A shape turning about its middle sweeps a ball as wide as its box's space
/// diagonal, so what has to fit between two of them is the sweep rather than
/// the width. A step divided by that is the largest that never reaches its
/// neighbour.
pub const ACROSS: f32 = APART / SWEEP;

/// How much wider than itself a shape reaches while it is turning.
///
/// Root three, the space diagonal of its box. It was root two, the horizontal
/// one, which was right while spec 0002 turned everything about the upright
/// and only that. Spec 0004 turns one about every axis, and at root two the
/// Sierpinski tetrahedron tipped on its side reached into the Menger sponge
/// beside it and down through its own plinth.
///
/// The row cannot widen to pay for it. Five steps of 0.88 is 4.03 across and
/// the clear span between the end cabinets is 3.3, measured by standing in the
/// doorway and looking: the teapot and the hilbert tube went behind them. So
/// the shapes come down from 0.509 to 0.416 instead, which is still three
/// times the Klein bottle's old size and a little over half again the
/// teapot's.
pub const SWEEP: f32 = 1.7320508;

/// A thing on the wall: what it is called, where it stands and how big it is
/// drawn, since the shapes come out at whatever size their own arithmetic gives
/// them.
///
/// The mesh is not in here. Where a thing stands is what the room needs to
/// answer what you are looking at, and that wants checking without a window;
/// the mesh is the renderer's business and is built once beside it.
#[derive(Debug, Clone, Copy)]
pub struct Display {
    pub name: &'static str,
    pub at: Vec3,
    pub scale: f32,
    /// Half of how big it is drawn, along each axis of its own.
    ///
    /// Carried rather than looked up, because where a turned one has to sit to
    /// rest on its plinth depends on all three and the room has no mesh.
    pub half: Vec3,
}

/// What each one is, in the order they are laid out along the wall.
const SHAPES: [&str; 6] = [
    TEAPOT, KLEIN, SIERPINSKI, MENGER, HILBERT,
    // the sixth, which is a door. Spec 0021: the five before it are what
    // teach the verb, and this is the one that spends it on you.
    TESSERACT,
];

/// Each of them by name, because the row's order is one list and what a
/// shape is is another question, and matching on a string written twice is
/// two lists of one thing.
pub const TEAPOT: &str = "teapot";
pub const KLEIN: &str = "klein";
pub const SIERPINSKI: &str = "sierpinski";
pub const MENGER: &str = "menger";
pub const HILBERT: &str = "hilbert";

/// The one on the end, named because what happens when you press enter at it
/// is not what happens at the other five.
pub const TESSERACT: &str = "tesseract";

/// Which place in the row that is.
pub fn tesseract() -> usize {
    SHAPES.len() - 1
}

/// How many stand in the row.
pub fn count() -> usize {
    SHAPES.len()
}

/// Which wall the nth one stands against: -1 for the west, 1 for the east.
///
/// Alternating, so the six are three facing pairs rather than a rank of
/// three and then a rank of three: walking in you meet them two at a
/// time in the order they are listed, and the tesseract that spec 0021
/// ends on is still the last one you reach.
pub fn side(n: usize) -> f32 {
    match n.is_multiple_of(2) {
        true => -1.0,
        false => 1.0,
    }
}

/// Where the nth one stands.
///
/// **Its own function because the ceiling asks it too.** Spec 0094 hangs a
/// light over each exhibit, and a lamp that worked out where the plinths
/// are for itself is the second list this building has learnt not to keep.
/// `all_of_them` cannot answer it, because building six meshes to find out
/// where a lamp goes is a question asked every frame and answered with a
/// Menger sponge.
///
/// **Off the way in rather than off the picture.** The row used to open
/// outwards from the middle of the far wall, so the picture was what it
/// was measured from. These run from the door down, which is the order
/// you meet them in and the order they are listed in.
pub fn stands(reaches: f32, n: usize) -> Vec3 {
    let out = crate::room::GALLERY_WIDE * 0.5 - OFF_THE_WALL;
    let from = -reaches - CLEAR - ACROSS * 0.5;

    vec3(side(n) * out, HIGH, from - (n / 2) as f32 * bay())
}

/// Which shape stands in a given place.
fn shape(n: usize) -> &'static str {
    SHAPES.get(n).copied().unwrap_or("")
}

/// How far a shape's own knob can be wound, if it has one. Spec 0092.
///
/// **One number per shape and not one kind of knob per shape.** Three of
/// these are built at a depth or a thickness, so for those it is that. The
/// teapot and the Klein bottle are one mesh each however you ask for them
/// and the only thing left to change about them is whether you can see
/// through them, so for those it is nought or one. The tesseract has
/// neither: it is a door.
pub fn knob_runs(n: usize) -> Option<(i32, i32)> {
    match shape(n) {
        TEAPOT | KLEIN => Some((0, 1)),
        // six was tried and is speckle: sixteen thousand tetrahedra across
        // half a metre is a few pixels apiece, so the shape stops reading
        // as a solid and reads as grey noise on a sheet. Five is the last
        // one that is still an object
        SIERPINSKI => Some((1, 5)),
        // the engine's own cap, which its example opens at three of and
        // which is the number the shape library will build. Three is 36
        // thousand triangles and a millisecond and a half; four is 673
        // thousand and 29 milliseconds, which is a frame and three
        // quarters spent on the keypress that asks for it, once
        MENGER => Some((1, blitzkit_shapes::menger::MAX_DEPTH as i32)),
        HILBERT => Some((1, 4)),
        _ => None,
    }
}

/// Where each knob rests, which is what the row has been drawn at since it
/// went in. The row looks the same as it did until a hand moves one.
pub fn knob_rests(n: usize) -> i32 {
    match shape(n) {
        SIERPINSKI => 4,
        MENGER => 2,
        HILBERT => 2,
        _ => 0,
    }
}

/// How much thicker each step makes the hilbert tube.
///
/// The curve runs on a lattice of 0.125 at order three, so the holes close
/// at half of that: the fourth step is 0.06 and is the last one with
/// anything left to see through.
pub const HILBERT_STEP: f32 = 0.015;

/// What a shape's knob is called and where it stands, for its plate.
/// Spec 0093.
///
/// **The shape's own word for it.** A depth and a thickness are not the
/// same thing, and a plate saying depth over a tube that has none is the
/// fault this spec was written to fix.
pub fn knob_says(n: usize, knob: i32) -> Option<String> {
    let (_, most) = knob_runs(n)?;

    // **No marker of its own.** This is half a sentence and is never drawn
    // on its own, so the one the plate carries covers the whole line. A
    // marker here would put two in one line of copy.
    Some(match shape(n) {
        TEAPOT | KLEIN => String::from(if knob > 0 { "see-through" } else { "solid" }),
        HILBERT => format!("thickness {} of {}", knob, most),
        _ => format!("depth {} of {}", knob, most),
    })
}

/// Whether this one is drawn see-through at that knob.
pub fn see_through(n: usize, knob: i32) -> bool {
    matches!(shape(n), TEAPOT | KLEIN) && knob > 0
}

/// Everything the engine's shape library carries, in the order it is laid out.
///
/// The depths and step counts are the ones the examples open at. Deeper is
/// prettier and slower, and a room with five of them turning at once pays for
/// all of them every frame.
/// How big each one is drawn is measured off its own mesh rather than written
/// down. Five numbers picked by eye drew the Klein bottle at 0.136 and the
/// hilbert tube at 0.522, a quarter of each other, while the library hands all
/// five back within a twentieth of the same size already.
pub fn all_of_them(reaches: f32) -> Vec<Display> {
    SHAPES
        .iter()
        .zip(meshes())
        .enumerate()
        .map(|(n, (name, mesh))| {
            let mut one = Display {
                name,
                at: stands(reaches, n),
                scale: 1.0,
                half: Vec3::ZERO,
            };
            // `fitted` settles the height, so `HIGH` above is only the
            // row's nominal line and the one below it is what is drawn
            one.fitted(&mesh);

            one
        })
        .collect()
}

impl Display {
    /// Measure one against its mesh: how much to scale it so it is the
    /// row's width, and how big it then is.
    ///
    /// **Not written down once at the start.** A knob rebuilds the mesh
    /// under a shape, and a hilbert tube wound thicker is a wider object,
    /// so a scale measured at the rest would draw it over its neighbour and
    /// a `half` measured there would sink it into its plinth. Spec 0092.
    pub fn fitted(&mut self, mesh: &MeshData) {
        let size = mesh.bounds().size();
        self.scale = ACROSS / size.max_element().max(1e-4);
        self.half = size * self.scale * 0.5;
        // and where its middle goes follows how far it reaches, so this is
        // the one account of it: the sight reads `at` and so does the draw.
        // Spec 0093.
        self.at.y = sits_at(self);
    }
}

/// The meshes, in the same order, each at where its own knob rests.
///
/// The depths and step counts are the ones the examples open at. Deeper is
/// prettier and slower, and a room with five of them turning at once pays for
/// all of them every frame.
pub fn meshes() -> Vec<MeshData> {
    (0..count()).map(|n| mesh_at(n, knob_rests(n))).collect()
}

/// One shape, built at its knob. Spec 0092.
///
/// **The one place a shape in this row is built.** The list used to be six
/// calls written out, and a knob turning three of them would have been a
/// second list saying the same thing differently, which this building has
/// learnt what happens to.
pub fn mesh_at(n: usize, knob: i32) -> MeshData {
    let knob = match knob_runs(n) {
        Some((least, most)) => knob.clamp(least, most),
        None => knob,
    };

    match shape(n) {
        TEAPOT => blitzkit_shapes::teapot::teapot(6),
        KLEIN => blitzkit_shapes::klein::klein_bottle(48, 24),
        SIERPINSKI => blitzkit_shapes::sierpinski::sierpinski(knob as u32),
        MENGER => blitzkit_shapes::menger::menger(knob as u32),
        HILBERT => blitzkit_shapes::hilbert::hilbert_tube(3, 8, knob as f32 * HILBERT_STEP),
        _ => crate::tesseract::shape(),
    }
}

/// Where a shape's lowest point is, which is where its plinth has to reach.
///
/// All five are drawn the same size, so this is one number rather than one
/// each.
pub fn stands_at() -> f32 {
    HIGH - ACROSS * 0.5
}

/// How far a shape reaches from its own middle, whichever way it is turned.
///
/// The half diagonal of its box, which is the radius of the ball it sweeps,
/// and so the one number that holds for every orientation at once.
pub fn reaches(one: &Display) -> f32 {
    one.half.length()
}

/// Where a shape's middle sits over its plinth. Spec 0093.
///
/// **One height and not one per facing.** Spec 0004 turned these about
/// every axis and left them sinking a seventh of a unit into the plinth when
/// they tipped, and spec 0005 answered that by lifting each one by however
/// far its own corners reached that frame. That is right standing still and
/// wrong the moment a hand turns it: the shape rises and falls twice a
/// revolution, which Jake read off the screen as bouncing on the plinth the
/// day the arrows could turn one.
///
/// So the lift is the sweep instead, which no turn can change. The shape
/// keeps its middle where it is however it is turned and its lowest corner
/// reaches the plinth's top only at the one orientation that stands the
/// diagonal upright. Every other way up it clears, which is what puts air
/// under it, and air under it is what the engine's own menger example has
/// and the reason that one has a shadow to look at.
pub fn sits_at(one: &Display) -> f32 {
    stands_at() + reaches(one)
}

/// The plinth under a shape: where its middle is and how big it is.
///
/// It used to be written down in the drawing code as a box 1.35 high ending at
/// 0.75, which left a shape floating a third of a unit over it. Nothing showed
/// that until the sun's shadow map was fitted to the room and the shadow on the
/// plinth's top landed clear of the thing casting it.
///
/// A little wider than the shape it carries, so a turning one does not hang
/// over the edge.
pub fn plinth_under(one: &Display) -> (Vec3, Vec3) {
    let top = stands_at();
    // the widest course and not the shaft, because what holds you off is the
    // thing you would walk into, which is the base block at its foot. Spec
    // 0111 made a pedestal of this and the box stayed as its footprint.
    let wide = ACROSS * BASE_BLOCK_WIDE;

    (vec3(one.at.x, top * 0.5, one.at.z), vec3(wide, top, wide))
}

/// A pedestal's five courses, as fractions of its whole height: the two that
/// make its base, the shaft, the echinus that flares out of it and the
/// abacus the shape stands on.
///
/// **Greco-Roman, because Jake asked for it and a box is a box.** What makes
/// a pedestal read as classical is the silhouette: something wider at the
/// foot, something wider again at the head, and a shaft between them that is
/// neither. One grey box has none of those and the room had six of them.
///
/// They sum to one, and `the_courses_fill_the_pedestal` holds them to it: a
/// course taken out of the middle would otherwise leave the shape standing
/// on air or sunk into its own abacus.
pub const BASE_BLOCK: f32 = 0.085;
pub const BASE_TORUS: f32 = 0.055;
pub const SHAFT: f32 = 0.735;
pub const ECHINUS: f32 = 0.040;
pub const CYMA: f32 = 0.035;
pub const ABACUS: f32 = 0.050;

/// And how wide each is, against the shaft's own width at its foot.
///
/// The widest is the base block, which is what sets how close two pedestals
/// come: at 1.24 of a shaft they stand 0.27 apart, where a plain box left
/// 0.32.
///
/// **It was 1.30 and 0.06 tall, and Jake read that as a plate.** A base
/// course is a block you could stub a toe on, so it lost width and gained
/// height rather than one or the other. The torus over it stopped being a
/// square step at the same time: it is turned now, with the bulge a torus
/// has, because a square step under a round shaft is a box pretending to be
/// a moulding.
pub const BASE_BLOCK_WIDE: f32 = 1.24;
pub const BASE_TORUS_WIDE: f32 = 1.10;
pub const ECHINUS_WIDE: f32 = 1.00;
pub const CYMA_WIDE: f32 = 1.08;
pub const ABACUS_WIDE: f32 = 1.20;

/// How wide the shaft is at its foot, against the course it is measured in,
/// and how much narrower it is under its echinus.
///
/// **Narrower than its own courses, which is what makes it a column.** At
/// the full width it came out half a metre across and 0.84 tall, which is a
/// drum: a classical shaft is many diameters tall and this one can only be
/// two, so what it can do is be slim against the base and the abacus rather
/// than flush with them.
///
/// A classical shaft is not a cylinder either. It narrows upward, and
/// `shaft_wide` puts a little of the swell back in so the taper is a curve
/// rather than a cone.
pub const SHAFT_WIDE: f32 = 0.80;
pub const SHAFT_TAPER: f32 = 0.12;

/// How deep the flutes are cut into the shaft and how many there are.
///
/// Sixteen rather than a Doric twenty, because the shaft is half a metre
/// across and read from a metre away: twenty at this size is a texture and
/// not a fluting.
pub const FLUTES: f32 = 16.0;
pub const FLUTE_DEEP: f32 = 0.055;

/// How wide the shaft is at a given height up it, as a fraction of its foot.
pub fn shaft_wide(up: f32) -> f32 {
    // the swell: a cone would read as a funnel, and the entasis every
    // classical shaft carries is what stops it
    let swell = (up * std::f32::consts::PI).sin() * 0.04;

    SHAFT_WIDE * (1.0 - SHAFT_TAPER * up + swell)
}

/// The three square courses of a pedestal, each as its middle and its size:
/// the two of the base and the abacus on top.
///
/// The shaft and the echinus are not here. Those two are turned, so they are
/// one mesh drawn once per pedestal rather than boxes.
pub fn courses(one: &Display) -> [(Vec3, Vec3); 3] {
    // the block at the foot, then the cyma and the abacus at the head. The
    // torus, the shaft and the echinus are turned and are one mesh.
    let tall = stands_at();
    let wide = ACROSS;
    let (x, z) = (one.at.x, one.at.z);
    let mut up = 0.0;
    let mut out = [(Vec3::ZERO, Vec3::ZERO); 3];

    for (n, (share, across)) in [
        (BASE_BLOCK, BASE_BLOCK_WIDE),
        (CYMA, CYMA_WIDE),
        (ABACUS, ABACUS_WIDE),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        // the two at the head are measured down from the top rather than up
        // through the turned courses under them: two accounts of where they
        // sit would come apart the first time a share changed
        let from = match n {
            0 => up,
            1 => 1.0 - ABACUS - CYMA,
            _ => 1.0 - ABACUS,
        };
        let high = share * tall;

        out[n] = (
            vec3(x, (from + share * 0.5) * tall, z),
            vec3(wide * across, high, wide * across),
        );

        if n == 0 {
            up += share;
        }
    }

    out
}

/// Where the turned part begins and how tall it is: the torus, the shaft and
/// the echinus together.
pub fn shaft_at(one: &Display) -> (Vec3, f32) {
    let tall = stands_at();
    let foot = BASE_BLOCK * tall;

    (
        vec3(one.at.x, foot, one.at.z),
        (BASE_TORUS + SHAFT + ECHINUS) * tall,
    )
}

/// The shaft and the echinus over it, as one turned and fluted mesh a unit
/// tall and a unit across at its foot.
///
/// **Fluted from the surface rather than cut into it.** `MeshData::surface`
/// is handed both ways round the mesh, so the flutes are what the radius
/// does with the angle and cost nothing but the steps to draw them. Boxes
/// stood in the face would be two surfaces in one plane, which this building
/// has written down seven times under other names.
pub fn shaft_mesh() -> blitzkit::mesh::MeshData {
    let whole = BASE_TORUS + SHAFT + ECHINUS;
    let torus = BASE_TORUS / whole;
    let shaft = torus + SHAFT / whole;
    let mut mesh = blitzkit::mesh::MeshData::surface(64, 72, |u, v| {
        let round = -u * std::f32::consts::TAU;

        let (wide, fluted) = if v <= torus {
            // the torus, which is a moulding and so bulges rather than
            // stepping: widest across its own middle and meeting the shaft
            // at the shaft's own width
            let up = (v / torus).clamp(0.0, 1.0);
            let bulge = (up * std::f32::consts::PI).sin();
            let from = SHAFT_WIDE + (BASE_TORUS_WIDE - SHAFT_WIDE) * 0.35;

            (
                from + (BASE_TORUS_WIDE - from) * bulge - (from - shaft_wide(0.0)) * up,
                false,
            )
        } else if v <= shaft {
            let up = (v - torus) / (shaft - torus);

            (shaft_wide(up), true)
        } else {
            // the echinus, which flares out of the shaft's own top width
            let out = (v - shaft) / (1.0 - shaft);
            let from = shaft_wide(1.0);

            (from + (ECHINUS_WIDE - from) * (out * out).min(1.0), false)
        };

        // the flutes, which stop where the echinus starts: a moulding that
        // carried them would read as a frill rather than as a capital
        let cut = if fluted {
            1.0 - FLUTE_DEEP * (0.5 + 0.5 * (round * FLUTES).cos())
        } else {
            1.0
        };

        let round_at = wide * cut * 0.5;

        vec3(round.cos() * round_at, v, round.sin() * round_at)
    });
    mesh.compute_normals();

    mesh
}

/// How far round the row has turned by now. All of them together, because five
/// things turning out of step with each other reads as five things rather than
/// as a row.
pub fn turned(seconds: f32) -> f32 {
    seconds * TURNS
}

/// How far a shape turns for each pixel the mouse moves, per spec 0004.
///
/// Faster than looking around, which is 0.0022. A shape is at arm's length and
/// you are turning it to see the far side, so a flick should get you there.
pub const TURN_BY: f32 = 0.008;

/// How fast a held arrow turns a shape, in radians a second. Spec 0091.
///
/// Half a turn in about two seconds, which is the same errand a flick of the
/// mouse does in one: an arrow cannot be flicked, so it is the holding that
/// has to get you to the far side without becoming a wait.
pub const ARROW_TURNS: f32 = 1.5;

/// Which way each shape is facing: the slow turn they share, and whatever a
/// hand has turned one of them to.
///
/// Spec 0004. All of it is arithmetic, so taking hold of one, turning it and
/// letting go are checked without a window.
///
/// A whole orientation rather than one angle about the upright. It was an
/// angle, and a Klein bottle turned only about its own axis never shows you
/// the handle going in through the wall, which is the one thing worth seeing.
#[derive(Debug, Clone, Default)]
pub struct Spin {
    /// Where a hand has turned each one, which is nowhere until someone does.
    by_hand: Vec<Quat>,
    /// Which one is in your hands and the way it is frozen facing.
    held: Option<(usize, Quat)>,
}

impl Spin {
    pub fn of(count: usize) -> Self {
        Self {
            by_hand: vec![Quat::IDENTITY; count],
            held: None,
        }
    }

    /// Which one is in your hands, if any.
    pub fn holding(&self) -> Option<usize> {
        self.held.map(|(n, _)| n)
    }

    /// Which way one is facing now.
    ///
    /// The row's own turn about the upright, applied over wherever a hand left
    /// this one, so a shape you have tilted goes on turning tilted rather than
    /// righting itself.
    pub fn facing(&self, n: usize, seconds: f32) -> Quat {
        match self.held {
            Some((held, at)) if held == n => at,
            _ => {
                Quat::from_rotation_y(turned(seconds))
                    * self.by_hand.get(n).copied().unwrap_or(Quat::IDENTITY)
            }
        }
    }

    /// Take hold of one, facing the way it already is, so nothing jumps.
    ///
    /// Lets go of whatever was in your hands first. Nothing in the room can
    /// reach a second one without letting go of the first, but a type that
    /// threw away the way you had just turned one is a trap waiting for the
    /// room to change.
    pub fn take(&mut self, n: usize, seconds: f32) {
        if n < self.by_hand.len() {
            self.let_go(seconds);
            self.held = Some((n, self.facing(n, seconds)));
        }
    }

    /// Turn the one in your hands, about whichever axes the hand asked for.
    /// Nothing in your hands, nothing turns.
    ///
    /// Applied over what it was already facing rather than under it, so the
    /// axes are the ones on the screen in front of you however far it has
    /// already been turned.
    pub fn turn(&mut self, by: Quat) {
        if let Some((_, at)) = self.held.as_mut() {
            *at = (by * *at).normalize();
        }
    }

    /// How far a drag turns a shape: across the screen about the upright, up
    /// and down about whichever way is right on the screen.
    pub fn drag(delta: glam::Vec2, right: Vec3) -> Quat {
        Self::turn_of(delta * TURN_BY, right)
    }

    /// The same turn asked for by the arrows, which give a direction held
    /// for a while rather than a distance moved. Spec 0091.
    pub fn held(wish: glam::Vec2, right: Vec3, dt: f32) -> Quat {
        Self::turn_of(wish * ARROW_TURNS * dt, right)
    }

    /// Which axes a turn is about, in radians, said once.
    ///
    /// The camera's right rather than a fixed axis, so turning one stays the
    /// way it looks whichever side of the aisle you are standing on.
    fn turn_of(by: glam::Vec2, right: Vec3) -> Quat {
        Quat::from_axis_angle(Vec3::Y, by.x)
            * Quat::from_axis_angle(right.normalize_or(Vec3::X), by.y)
    }

    /// Takes a hand's turn off one, so it is square to the row again.
    /// Spec 0101. The row's own slow turn goes on; what comes off is
    /// whatever somebody left it at.
    pub fn let_it_go(&mut self, n: usize) {
        if self.holding() == Some(n) {
            self.held = None;
        }

        if let Some(by_hand) = self.by_hand.get_mut(n) {
            *by_hand = Quat::IDENTITY;
        }
    }

    /// Let go, leaving it where you left it rather than where it would have
    /// been. It carries on from there.
    pub fn let_go(&mut self, seconds: f32) {
        if let Some((n, at)) = self.held.take() {
            if let Some(by_hand) = self.by_hand.get_mut(n) {
                *by_hand = (Quat::from_rotation_y(turned(seconds)).inverse() * at).normalize();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0002 and 0021: the two ranks leave a walkway between them, and
    /// the picture they lead to has clear wall either side of it.
    ///
    /// **The test that was missing, in its second shape.** What bounded
    /// the old row was where the hall's end cabinets stood, and nothing
    /// said so, so when a sixth shape arrived the row was made to fit by
    /// shrinking every shape in it by 18 per cent and the gate stayed
    /// green. The row is down the sides now and what binds has changed
    /// with it: across the room it is the walkway, and on the end wall it
    /// is the picture. Both, because either alone is a room you cannot use
    /// or a room with nothing to look at.
    #[test]
    fn the_ranks_leave_a_way_through_and_the_picture_a_wall() {
        let walkway = span() - reaches_in() * 2.0;

        assert!(
            walkway > crate::RADIUS * 4.0,
            "{} across less {:.2} a side of exhibit leaves a walkway of \
             {:.2}, against a body of {:.2}",
            span(),
            reaches_in(),
            walkway,
            crate::RADIUS * 2.0
        );

        let beside = (span() - crate::mandel::framed()) * 0.5;

        assert!(
            beside >= CLEAR,
            "the picture is {:.2} framed on a wall of {:.2}, which leaves \
             {:.2} either side against a clearance of {}",
            crate::mandel::framed(),
            span(),
            beside,
            CLEAR
        );

        // and the ranks really are off the walls and inside the room's own
        // depth, measured off where they stand rather than off the
        // arithmetic above
        let reaches = 9.5;
        let on = all_of_them(reaches);
        let (near, far) = (-reaches, -reaches - crate::room::gallery_deep());

        for one in &on {
            assert!(
                (one.at.x.abs() - (span() * 0.5 - OFF_THE_WALL)).abs() < 1e-4,
                "{} stands at x {:.3} rather than {:.2} off a side wall",
                one.name,
                one.at.x,
                OFF_THE_WALL
            );
            assert!(
                one.at.z + ACROSS * 0.5 <= near - CLEAR + 1e-4
                    && one.at.z - ACROSS * 0.5 >= far + CLEAR - 1e-4,
                "{} stands at z {:.3}, and the room runs {:.2} to {:.2} \
                 wanting {} clear at each end",
                one.name,
                one.at.z,
                near,
                far,
                CLEAR
            );
        }
    }

    /// Spec 0002: they are three facing pairs, in the order you meet them.
    ///
    /// **Because a rank of three and then a rank of three is the same six
    /// and a different room.** Alternating is what makes the walk down the
    /// middle pass them two at a time, and it is what keeps the tesseract
    /// of spec 0021 at the end rather than halfway along one side.
    #[test]
    fn they_stand_in_pairs_down_the_sides() {
        let on = all_of_them(9.5);

        for pair in on.chunks(2) {
            assert_eq!(pair.len(), 2, "an odd one out of the pairing");
            assert!(
                (pair[0].at.z - pair[1].at.z).abs() < 1e-4,
                "{} and {} are a pair and stand at z {:.3} and {:.3}",
                pair[0].name,
                pair[1].name,
                pair[0].at.z,
                pair[1].at.z
            );
            assert!(
                pair[0].at.x < 0.0 && pair[1].at.x > 0.0,
                "{} and {} are both on the {} side",
                pair[0].name,
                pair[1].name,
                match pair[0].at.x < 0.0 {
                    true => "west",
                    false => "east",
                }
            );
        }

        // **The deepest pair and not the deepest one**, since the two of a
        // pair stand level and which of them a sort calls first is the
        // order they were listed in, not a fact about the room.
        let deepest = on.iter().map(|one| one.at.z).fold(f32::INFINITY, f32::min);

        assert!(
            (on[tesseract()].at.z - deepest).abs() < 1e-4,
            "the tesseract stands at z {:.3} and the last pair at {:.3}",
            on[tesseract()].at.z,
            deepest
        );
    }

    /// Spec 0002: there is a display for each shape the library carries.
    #[test]
    fn every_shape_is_on_the_wall() {
        let on = all_of_them(9.5);
        let names: Vec<&str> = on.iter().map(|one| one.name).collect();

        assert_eq!(
            names,
            [
                "teapot",
                "klein",
                "sierpinski",
                "menger",
                "hilbert",
                "tesseract"
            ],
            "the wall carries {:?}",
            names
        );
        let built = meshes();
        assert_eq!(
            built.len(),
            on.len(),
            "a shape with no mesh or the other way"
        );
        for (one, mesh) in on.iter().zip(&built) {
            assert!(
                !mesh.vertices.is_empty(),
                "{} came out with no mesh at all",
                one.name
            );
            assert!(one.scale > 0.0);
        }
    }

    /// Spec 0002: and they stand clear of each other.
    #[test]
    fn they_stand_clear() {
        let on = all_of_them(9.5);

        for (n, one) in on.iter().enumerate() {
            assert!(
                (span() * 0.5 - one.at.x.abs() - OFF_THE_WALL).abs() < 1e-4,
                "{} stands {:.3} off its side wall rather than {}",
                one.name,
                span() * 0.5 - one.at.x.abs(),
                OFF_THE_WALL
            );

            for other in &on[n + 1..] {
                let apart = one.at.distance(other.at);
                assert!(
                    apart > ACROSS,
                    "{} and {} are {} apart and are drawn {} across",
                    one.name,
                    other.name,
                    apart,
                    ACROSS
                );
            }
        }
    }

    /// Spec 0002: all five come out the same size, whatever their own
    /// arithmetic gave them.
    ///
    /// Five scales picked by eye drew the Klein bottle at 0.136 and the
    /// hilbert tube at 0.522, and nothing failed.
    #[test]
    fn they_are_all_the_same_size() {
        for (one, mesh) in all_of_them(9.5).iter().zip(meshes()) {
            let drawn = mesh.bounds().size().max_element() * one.scale;

            assert!(
                (drawn - ACROSS).abs() < 1e-3,
                "{} is drawn {} across where the others are {}",
                one.name,
                drawn,
                ACROSS
            );
        }
    }

    /// Spec 0002: and a turning one never reaches the one beside it.
    ///
    /// The sweep rather than the width, since a shape turning about its middle
    /// covers a circle as wide as its diagonal. Checked against the gap
    /// because that is what it has to fit in.
    #[test]
    fn a_turning_one_keeps_to_itself() {
        let sweep = ACROSS * SWEEP;

        assert!(
            sweep <= APART,
            "a shape sweeps {} across a gap of {}",
            sweep,
            APART
        );
    }

    /// Spec 0093: no turn moves a shape, and no turn puts it through its
    /// plinth.
    ///
    /// **Both halves, because either alone is met by something wrong.**
    /// Spec 0005 lifted each shape by how far it reached that frame, which
    /// never collides and moves the shape twice a revolution, and Jake read
    /// Spec 0111: the six courses fill the pedestal exactly, in order, and
    /// the shape stands on the top of the abacus.
    ///
    /// **Because a course is a share of a whole and nothing checks arithmetic
    /// that is spread over six constants.** Take one out of the middle and
    /// the shape is left standing on air or sunk into its own abacus, and
    /// the only thing that would say so is a picture.
    ///
    /// The turned part is checked against the boxes either side of it rather
    /// than against its own constants, because what has to be true is that
    /// they meet: an open-ended skin that stops short of the block leaves a
    /// gap you can see under the torus.
    #[test]
    fn the_courses_fill_the_pedestal() {
        let shares = BASE_BLOCK + BASE_TORUS + SHAFT + ECHINUS + CYMA + ABACUS;

        assert!(
            (shares - 1.0).abs() < 1e-4,
            "the six courses come to {} of a pedestal",
            shares
        );

        let one = Display {
            name: "teapot",
            at: vec3(1.0, HIGH, -2.0),
            scale: 1.0,
            half: Vec3::ZERO,
        };
        let tall = stands_at();
        let [block, cyma, abacus] = courses(&one);
        let (foot, turned) = shaft_at(&one);

        let bottom = |(at, size): (Vec3, Vec3)| at.y - size.y * 0.5;
        let top = |(at, size): (Vec3, Vec3)| at.y + size.y * 0.5;

        assert!(
            bottom(block).abs() < 1e-4,
            "the base block starts {} off the floor",
            bottom(block)
        );
        assert!(
            (top(abacus) - tall).abs() < 1e-4,
            "the abacus tops out at {} where a shape stands at {}",
            top(abacus),
            tall
        );
        assert!(
            (bottom(abacus) - top(cyma)).abs() < 1e-4,
            "the cyma and the abacus do not meet"
        );

        // and the turned part fills the whole of what is left between them
        assert!(
            (foot.y - top(block)).abs() < 1e-4,
            "the turned part starts at {} and the block tops out at {}",
            foot.y,
            top(block)
        );
        assert!(
            (foot.y + turned - bottom(cyma)).abs() < 1e-4,
            "the turned part ends at {} and the cyma starts at {}",
            foot.y + turned,
            bottom(cyma)
        );

        // the base block is the widest, which is what `plinth_under` hands
        // out as the footprint and what decides how close two of them stand
        let (_, box_) = plinth_under(&one);

        for (what, course) in [("cyma", cyma), ("abacus", abacus)] {
            assert!(
                course.1.x <= block.1.x + 1e-4,
                "the {} is wider than the base block, so the footprint is wrong",
                what
            );
        }

        assert!(
            (box_.x - block.1.x).abs() < 1e-4,
            "the footprint is {} and the widest course is {}",
            box_.x,
            block.1.x
        );
    }

    /// that off the screen as bouncing. Leaving it where it stands never
    /// moves it and sinks it a seventh of a unit in when it tips, which is
    /// the fault spec 0005 was written for. The lift that does both is the
    /// sweep, and this says so by walking the turns.
    ///
    /// And the closest approach has to be nought, or a lift of any size at
    /// all would pass: a shape held a metre up never collides either, and
    /// what it looks like is a row that forgot its plinths.
    #[test]
    fn no_turn_moves_a_shape_or_puts_it_through_its_plinth() {
        let quarter = std::f32::consts::FRAC_PI_2;

        for one in all_of_them(9.5).iter() {
            let (at, size) = plinth_under(one);
            let top = at.y + size.y * 0.5;

            // a spread, and then the one that stands the box's own diagonal
            // upright, which is where a box hangs lowest
            let mut facings = vec![
                Quat::IDENTITY,
                Quat::from_rotation_x(quarter),
                Quat::from_rotation_z(quarter),
                Quat::from_rotation_x(0.7) * Quat::from_rotation_y(1.2),
                Quat::from_rotation_x(quarter * 0.5) * Quat::from_rotation_z(quarter * 0.5),
                Quat::from_rotation_arc(one.half.normalize_or(Vec3::Y), Vec3::Y),
            ];
            for step in 0..24 {
                let turn = step as f32 * 0.26;
                facings.push(Quat::from_rotation_x(turn) * Quat::from_rotation_z(turn * 0.61));
            }

            let mut closest = f32::MAX;

            for facing in facings {
                let m = glam::Mat3::from_quat(facing);
                let reach = one.half.x * m.x_axis.y.abs()
                    + one.half.y * m.y_axis.y.abs()
                    + one.half.z * m.z_axis.y.abs();
                let bottom = one.at.y - reach;

                assert!(
                    bottom > top - 1e-4,
                    "{} turned {:?} reaches {:.4} into a plinth whose top is {:.4}",
                    one.name,
                    facing,
                    bottom,
                    top
                );

                closest = closest.min(bottom - top);
            }

            assert!(
                closest < 1e-3,
                "{} never comes nearer its plinth than {:.4}, so it is hanging rather than standing over it",
                one.name,
                closest
            );
        }
    }

    /// Spec 0002: and the plinth is wider than what it carries, so a turning
    /// shape does not hang over the edge.
    #[test]
    fn a_plinth_is_wider_than_its_shape() {
        for one in all_of_them(9.5).iter() {
            let (_, size) = plinth_under(one);

            assert!(
                size.x > ACROSS && size.z > ACROSS,
                "{}'s plinth is {} across under a shape {} across",
                one.name,
                size.x,
                ACROSS
            );
            assert!(
                size.x < APART,
                "{}'s plinth is {} across in a gap of {}",
                one.name,
                size.x,
                APART
            );
        }
    }

    /// How far apart two ways of facing are, in radians.
    fn apart(one: Quat, other: Quat) -> f32 {
        one.angle_between(other)
    }

    /// A drag of the mouse, with the camera looking down the room.
    fn drag(x: f32, y: f32) -> Quat {
        Spin::drag(glam::vec2(x, y), Vec3::X)
    }

    /// Spec 0004: taking hold of one and letting go leaves it where it was.
    #[test]
    fn letting_go_does_not_jump() {
        let mut spin = Spin::of(5);
        let was = spin.facing(2, 3.0);

        spin.take(2, 3.0);
        assert!(
            apart(spin.facing(2, 3.0), was) < 1e-4,
            "it jumped on taking"
        );

        spin.let_go(3.0);
        assert!(
            apart(spin.facing(2, 3.0), was) < 1e-4,
            "it jumped on letting go"
        );
    }

    /// Spec 0004: a held one does not turn on its own.
    #[test]
    fn a_held_one_holds_still() {
        let mut spin = Spin::of(5);
        spin.take(1, 2.0);
        let at = spin.facing(1, 2.0);

        for later in [2.5, 4.0, 10.0] {
            assert!(
                apart(spin.facing(1, later), at) < 1e-4,
                "it drifted by {}",
                later
            );
        }
    }

    /// Spec 0004: a free one does.
    #[test]
    fn a_free_one_turns() {
        let spin = Spin::of(5);

        assert!(
            apart(spin.facing(0, 2.0), spin.facing(0, 1.0)) > 1e-3,
            "it stopped"
        );
    }

    /// Spec 0004: a shape turns about every axis, not only the upright.
    ///
    /// A Klein bottle turned only about its own axis never shows you the
    /// handle going in through the wall, which is the one thing worth seeing.
    #[test]
    fn it_turns_in_every_direction() {
        let mut spin = Spin::of(5);
        spin.take(0, 0.0);

        // dragging across takes the upright with it and leaves it upright
        spin.turn(drag(100.0, 0.0));
        let across = spin.facing(0, 0.0);
        assert!(
            (across * Vec3::Y).dot(Vec3::Y) > 0.999,
            "a drag across the screen tipped it: up is now {:?}",
            across * Vec3::Y
        );
        assert!(
            (across * Vec3::Z).dot(Vec3::Z) < 0.999,
            "a drag across the screen turned nothing"
        );

        // and dragging up or down tips it
        spin.turn(drag(0.0, 100.0));
        let tipped = spin.facing(0, 0.0);
        assert!(
            (tipped * Vec3::Y).dot(Vec3::Y) < 0.9,
            "a drag down the screen did not tip it"
        );
    }

    /// Spec 0004: turning moves the one in your hands and no other.
    #[test]
    fn turning_moves_only_the_held_one() {
        let mut spin = Spin::of(5);
        let others: Vec<Quat> = (0..5).map(|n| spin.facing(n, 1.0)).collect();

        spin.take(3, 1.0);
        spin.turn(drag(80.0, 40.0));

        assert!(
            apart(spin.facing(3, 1.0), others[3]) > 1e-3,
            "the held one did not turn"
        );
        for n in [0, 1, 2, 4] {
            assert!(
                apart(spin.facing(n, 1.0), others[n]) < 1e-4,
                "{} moved as well",
                n
            );
        }
    }

    /// Spec 0004: letting go carries on from where you left it.
    #[test]
    fn it_carries_on_from_where_you_left_it() {
        let mut spin = Spin::of(5);
        spin.take(0, 1.0);
        spin.turn(drag(120.0, 60.0));
        let left_at = spin.facing(0, 1.0);
        spin.let_go(1.0);

        assert!(
            apart(spin.facing(0, 1.0), left_at) < 1e-4,
            "it moved the moment you let go"
        );
        let on = apart(spin.facing(0, 2.0), left_at);
        assert!(
            (on - TURNS).abs() < 1e-3,
            "a second later it had gone {} rather than {}",
            on,
            TURNS
        );
    }

    /// Spec 0004: and the tilt you gave it survives the turning.
    #[test]
    fn a_tilted_one_goes_on_turning_tilted() {
        let mut spin = Spin::of(5);
        spin.take(0, 1.0);
        spin.turn(drag(0.0, 100.0));
        let tipped = spin.facing(0, 1.0) * Vec3::Y;
        spin.let_go(1.0);

        // the row turns about the upright, so a tilt stays as far off upright
        // as it was however long it goes on turning
        for later in [1.5, 3.0, 7.0] {
            let up = spin.facing(0, later) * Vec3::Y;
            assert!(
                (up.y - tipped.y).abs() < 1e-3,
                "at {} it had righted itself to {:?}",
                later,
                up
            );
        }
    }

    /// Spec 0004: nothing can be turned while nothing is held.
    #[test]
    fn turning_nothing_turns_nothing() {
        let mut spin = Spin::of(5);
        let before: Vec<Quat> = (0..5).map(|n| spin.facing(n, 1.0)).collect();

        spin.turn(drag(200.0, 200.0));

        for (n, was) in before.iter().enumerate() {
            assert!(apart(spin.facing(n, 1.0), *was) < 1e-4, "{} moved", n);
        }
    }

    /// Spec 0004: only one is held at a time.
    #[test]
    fn only_one_is_held() {
        let mut spin = Spin::of(5);

        assert_eq!(spin.holding(), None);
        spin.take(1, 1.0);
        assert_eq!(spin.holding(), Some(1));
        spin.turn(drag(70.0, 0.0));
        let first = spin.facing(1, 1.0);

        spin.take(4, 1.0);
        assert_eq!(spin.holding(), Some(4), "the second did not take over");
        assert!(
            apart(spin.facing(1, 1.0), first) < 1e-4,
            "taking the second threw away where the first was turned to"
        );

        spin.let_go(1.0);
        assert_eq!(spin.holding(), None);
    }

    /// Spec 0002: each one turns.
    #[test]
    fn they_turn() {
        assert_eq!(turned(0.0), 0.0);
        assert!(turned(1.0) > 0.0);
        assert!(turned(2.0) > turned(1.0), "it stopped");

        // and goes right round in a reasonable while rather than a minute
        let round = std::f32::consts::TAU / TURNS;
        assert!(
            (6.0..40.0).contains(&round),
            "a turn takes {} seconds",
            round
        );
    }
}
