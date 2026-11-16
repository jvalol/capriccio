//! What is on the far wall: the engine's own shapes, turning. Spec 0002.
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

/// The gap the row has to live in, which is the gallery's width.
///
/// It used to be the gap between the inner faces of the two cabinets at the
/// end of the rows, because the row was on the wall between them. Spec 0022
/// moved the wall.
#[allow(dead_code)]
pub fn span() -> f32 {
    crate::room::GALLERY_WIDE
}

/// How wide the six stand, from the outside of one to the outside of the
/// other, with no account of what is between them.
///
/// What the room is deep enough to see, which is a different question from
/// how wide the room is. See `room::gallery_deep`.
pub fn shapes() -> f32 {
    (SHAPES.len() - 1) as f32 * APART + ACROSS
}

/// How much clear wall the picture keeps either side of its own moulding.
///
/// Measured to the next shape's sweep and not to its middle, which is what
/// you are actually asking when you ask whether a thing is in the way.
pub const BESIDE: f32 = 0.4;

/// Where the innermost shape of each half stands.
///
/// **Spec 0039 hung the one that does not stop behind the six that do, and
/// from anywhere far enough back to see all of it, three of them were
/// across the middle of it.** So the row opens instead: three either side
/// and the picture in the hole, with clear floor in front of it and the
/// way in from the hall pointing straight at it.
pub fn inside() -> f32 {
    crate::mandel::framed() * 0.5 + BESIDE + sweep()
}

/// How far a shape reaches from its own middle while it is turning, which
/// is further than it is wide. See [`SWEEP`].
pub fn sweep() -> f32 {
    ACROSS * SWEEP * 0.5
}

/// How wide the row actually is, from the outside of one shape to the other.
///
/// Measured and not drawn, which is the family three constants above are in
/// already: it exists so the fit can be checked. The ceiling used to divide
/// it to place three lamps, and spec 0105 hung one over each exhibit off
/// `along` instead, so nothing that renders reads this now.
#[allow(dead_code)]
pub fn row() -> f32 {
    (inside() + (SHAPES.len() / 2 - 1) as f32 * APART) * 2.0 + ACROSS
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

/// Where the nth one stands along the row: out from the picture a step at a
/// time, each half its own way.
///
/// **Its own function because the ceiling asks it too.** Spec 0105 hangs a
/// light over each exhibit, and a lamp that worked out where the plinths
/// are for itself is the second list this building has learnt not to keep.
/// `all_of_them` cannot answer it, because building six meshes to find out
/// where a lamp goes is a question asked every frame and answered with a
/// Menger sponge.
pub fn along(n: usize) -> f32 {
    let each = count() / 2;
    let (side, out) = if n < each {
        (-1.0, (each - 1 - n) as f32)
    } else {
        (1.0, (n - each) as f32)
    };

    side * (inside() + out * APART)
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
pub fn all_of_them(wall: f32) -> Vec<Display> {
    SHAPES
        .iter()
        .zip(meshes())
        .enumerate()
        .map(|(n, (name, mesh))| {
            let mut one = Display {
                name,
                at: vec3(along(n), HIGH, wall + OFF_THE_WALL),
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
    let wide = ACROSS * 1.15;

    (vec3(one.at.x, top * 0.5, one.at.z), vec3(wide, top, wide))
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

    /// Spec 0002 and 0021: the row fits between the cabinets at the end of
    /// the rows, with room to spare at each end.
    ///
    /// **The test that was missing.** What bounds this row is where those
    /// cabinets stand, and nothing said so, so when a sixth shape arrived the
    /// row was made to fit by shrinking every shape in it by 18 per cent and
    /// the gate stayed green. Jake read it off the screen instead.
    #[test]
    fn the_row_fits_between_the_end_cabinets() {
        let span = span();
        let row = row();

        assert!(
            row + CLEAR * 2.0 <= span,
            "{} shapes at {} apart is {:.2} of row in a gap of {:.2}, which leaves {:.2} at each end",
            SHAPES.len(),
            APART,
            row,
            span,
            (span - row) * 0.5
        );

        // and the outermost shape really is inside the gap, measured off the
        // row rather than off the arithmetic above
        let on = all_of_them(-6.0);
        let edge = on
            .iter()
            .map(|one| one.at.x.abs() + ACROSS * 0.5)
            .fold(0.0f32, f32::max);

        assert!(
            edge <= span * 0.5 - CLEAR,
            "the end shape reaches {:.2} and the clear half gap is {:.2}",
            edge,
            span * 0.5 - CLEAR
        );
    }

    /// Spec 0002: there is a display for each shape the library carries.
    #[test]
    fn every_shape_is_on_the_wall() {
        let on = all_of_them(-6.0);
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
        let on = all_of_them(-6.0);

        for (n, one) in on.iter().enumerate() {
            assert!(
                (one.at.z + 6.0 - OFF_THE_WALL).abs() < 1e-4,
                "{} is not off the wall",
                one.name
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
        for (one, mesh) in all_of_them(-6.0).iter().zip(meshes()) {
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

        for one in all_of_them(-6.0).iter() {
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
        for one in all_of_them(-6.0).iter() {
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
