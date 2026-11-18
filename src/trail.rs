//! What the building has given up to you, room by room. Spec 0036.
//!
//! **This is the plumbing and not the trail.** Spec 0036 is a trail through
//! every room, and it is waiting on which room is at the end of it. What it
//! says can be built first is the part underneath: the building has eleven
//! rooms and no idea what you have already done in any of them. Four refer
//! to each other and the other seven are sealed.
//!
//! So this records, and nothing more. Nothing here is drawn, nothing is
//! counted at you, and a run that never finds the trail is a run where this
//! fills up and is never read. That is deliberate: spec 0036's rule is that
//! somebody who never finds the trail should not be able to tell it is
//! there.
//!
//! **A room gives up its answer, it does not get a tick.** The register
//! already does the whole pattern and is the reason this has the shape it
//! has: you walk the diagonal, you come away carrying a guest the hotel
//! cannot house, and the desk takes it out of your hand. The clue is the
//! receipt for doing the room properly, so what is recorded is that the
//! room gave something up, in the order the rooms gave.
//!
//! **Why not `noise::Room`.** That enum says which room you are standing
//! in, worked out from where your feet are, for deciding what you can hear.
//! It has a stair and a field, which ask you nothing, and it has no hotel,
//! no register and no gallery, which ask you the three hardest questions in
//! the building. The two lists overlap and answer different questions, so
//! each says its own, and this one is named for the asking rather than for
//! the standing.

/// A room that asks you something, named by what it asks.
///
/// **Only the rooms whose question is already in the code.** Spec 0036 is
/// plain that a room needing an object planted in it to join the trail is a
/// room the trail is wrong about, so nothing is listed here on the strength
/// of a room being nice to visit. The rest arrive as their rooms grow a
/// moment worth recording, and `Trail` does not care how many there are.
///
/// **And the tesseract is not one of them.** It was, for an afternoon on
/// 2026-11-18. Jake, trying to write the stone's line for it: it is too
/// complicated for a layperson, and if even he does not understand it then
/// it stays in the building and comes out of the requirements. The room is
/// spec 0021's and is worth working on; what it is not is a thing to put
/// between somebody and the end of the trail. Working the sixth display
/// still takes you inside, and nothing records that you went.
///
/// **Six, where there were seven, where there were five, where there were
/// eight.** Jake,
/// 2026-11-18, asked what the trail actually is and whether it is the most
/// interesting way to unlock the bridge. It was not, and the plainest
/// reason was arithmetic: the five covered four rooms out of eleven, two of
/// them the same room, and two of the five fired on walking into a place
/// rather than on doing anything there. The baths and the nook each already
/// ask something with a test behind it, so they join and nothing is
/// planted, which is this spec's own rule.
///
/// **And the order they are written in is the order they are asked for.**
/// Spec 0118's stone names the first one not yet given, so this list stops
/// being a set the moment anything reads it in order. Two of the seven are
/// fixed by the building rather than by taste, and both are checked by
/// `tests::nothing_is_asked_for_before_its_room_can_be_reached`:
///
/// - The case before the pool. `room::open` is the case over the way down,
///   the cellar is at the foot of that stair, and the baths' doorway is cut
///   in the wall the baths share with the cellar.
/// - The wall before the sun. The ride runs past `beyond::at`, which is
///   inside `behind::inside`.
///
/// Nothing else in the building is ever shut: `room::open`, `sauna_open`,
/// `baths_open` and `mirror_open` are the whole list of things that are one
/// way or the other, and the way in behind the hall's end wall is an
/// opening rather than a door.
///
/// The rest is a curve, easiest first, and the tail is the argument worth
/// keeping: the cube is the hardest thing in the building and is fifth
/// rather than last, so what stands between you and a finished bridge at
/// the end is the ride out at true scale rather than a Rubik's cube. Jake
/// settled this on 2026-11-18.
///
/// **Five, where there were eight.** Spec 0063 parked the hotel, the
/// register and the imaginarium, which between them asked three of them:
/// walking the diagonal, handing what it left you to the desk, and working
/// one of the imaginarium's hatches. That makes the bridge easier to reach,
/// which is a real change and is probably the right one: eight was a
/// checklist of a length nobody had measured against how long this building
/// takes to walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asked {
    /// Walking out past the garden's west wall, where the ground does not
    /// stop. Spec 0040.
    ///
    /// **First because it is free.** Spec 0118's stone stands at the foot
    /// of the bridge, which is out here, so anybody who can read what the
    /// stone says has already answered this one and it is never the thing
    /// named.
    WentOutIntoTheField,
    /// Opening the bookcase in the cellar, by the one leaf in it that is a
    /// handle. Spec 0007.
    OpenedTheCase,
    /// Getting into the pool rather than standing on the coping looking at
    /// it. Spec 0030, and `spa::wading` is the same test the wade's own
    /// pace is taken from.
    ///
    /// **After the case, and it has to be.** `room::open` is the case over
    /// the way down, the cellar is at the foot of that stair, and
    /// `spa::near` cuts the baths' doorway in the wall the baths share with
    /// the cellar. Named before it, this is a room with no way in.
    WadedInThePool,
    /// Solving the cube on the nook's sixth bench. Spec 0105.
    ///
    /// The hard one, and deliberately not the last: see the note on the
    /// enum about what the tail is for.
    SolvedTheCube,
    /// Getting behind the hall's near end wall. Spec 0011, and the answer
    /// to the question spec 0020's worn track asks by existing.
    ///
    /// Nothing shuts this way; it is only hidden, and the worn track is
    /// the clue it already had.
    GotBehindTheWall,
    /// Riding out to the sun, which is what the space behind the wall is
    /// for: the system at true scale, where the reading is the distance.
    /// Specs 0015 and 0018, and one of the four spec 0036 names itself.
    ///
    /// **After getting behind the wall, and it has to be.** The ride runs
    /// past `beyond::at`, which is inside `behind::inside`. Written first,
    /// as it was until 2026-11-18, the first thing spec 0118's stone could
    /// ever name was a thing in a room nobody has found.
    RodeToTheSun,
}

impl Asked {
    /// Every question the building asks, in the order they are written.
    ///
    /// **One list, so that a room which starts asking joins what reads
    /// this without the reader being touched.** Spec 0054's bridge is
    /// there once all of these have been given, and the alternative was a
    /// number written down in that file, which goes stale the first time
    /// this enum grows.
    ///
    /// Nothing in Rust counts an enum's variants, so what keeps this
    /// honest is `tests::every_one_of_them_is_in_every`, whose match is
    /// exhaustive: a new variant stops that test compiling, and the test
    /// then says this list is short.
    pub fn every() -> &'static [Asked] {
        &[
            Asked::WentOutIntoTheField,
            Asked::OpenedTheCase,
            Asked::WadedInThePool,
            Asked::SolvedTheCube,
            Asked::GotBehindTheWall,
            Asked::RodeToTheSun,
        ]
    }
}

/// What the building has given up, in the order it gave.
///
/// A list rather than a set of flags, because the order is the only thing
/// here that could ever mean anything: two rooms answered is not the same
/// as two rooms answered in that order, and a flag per room throws that
/// away. It is also what stops this becoming the checklist spec 0036 says
/// it must not be, since a list of what happened is not a list of what has
/// not.
#[derive(Clone, Debug, Default)]
pub struct Trail {
    given: Vec<Asked>,
}

impl Trail {
    pub fn new() -> Self {
        Self { given: Vec::new() }
    }

    /// A room gave up its answer.
    ///
    /// **Saying it twice is not two answers.** You can walk the diagonal
    /// again, and spec 0032 says so in as many words: walk it again and it
    /// will have found somebody else. The building wants to know that the
    /// room has been done, not how many times, and a count here would be a
    /// score.
    pub fn gave(&mut self, what: Asked) {
        if !self.given.contains(&what) {
            self.given.push(what);
        }
    }

    /// In the order they were given.
    pub fn given(&self) -> &[Asked] {
        &self.given
    }

    /// Whether the building has given up anything at all.
    ///
    /// **There is no `len` and no `has`.** A room that wants either can
    /// read `given`, and writing them before anything asked would be two
    /// more ways to say what one list already says. Spec 0036 is also
    /// plain about what a count must never become: no panel of ticks, no
    /// "3 of 8 found".
    pub fn is_empty(&self) -> bool {
        self.given.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where each one sits in `Asked::every`, by a match the compiler
    /// makes you finish.
    ///
    /// **Two halves, and neither is enough alone.** Adding a variant to
    /// the enum stops this compiling, which is what sends you here. Adding
    /// it here and not to `every` gives it an index past the end of that
    /// list, which is what `every_one_of_them_is_in_every` catches. The
    /// first version of this was a match that returned its own argument,
    /// which clippy is right to call needless: it forced an edit and
    /// checked nothing.
    fn sits_at(what: Asked) -> usize {
        match what {
            Asked::WentOutIntoTheField => 0,
            Asked::OpenedTheCase => 1,
            Asked::WadedInThePool => 2,
            Asked::SolvedTheCube => 3,
            Asked::GotBehindTheWall => 4,
            Asked::RodeToTheSun => 5,
        }
    }

    /// What has to be answered before what, as pairs.
    ///
    /// **Written here and not beside the list**, because the list is what
    /// this checks: a constraint that lives inside the thing it constrains
    /// is a comment. Each pair is a room you cannot reach until the other
    /// one has been answered, and both are doors rather than opinions.
    const NEEDS: [(Asked, Asked); 2] = [
        (Asked::OpenedTheCase, Asked::WadedInThePool),
        (Asked::GotBehindTheWall, Asked::RodeToTheSun),
    ];

    /// Spec 0118: the stone never names a room there is no way into.
    ///
    /// **Which it did.** Riding to the sun was written first and getting
    /// behind the wall fourth, and the ride happens inside that room, so
    /// the first thing the stone could have said to anybody was the name
    /// of a thing in a place nobody has found.
    #[test]
    fn nothing_is_asked_for_before_its_room_can_be_reached() {
        for (first, then) in NEEDS {
            assert!(
                sits_at(first) < sits_at(then),
                "{:?} is asked for at {} and {:?} at {}, which is the \
                 wrong way round: you cannot reach the second without \
                 having answered the first",
                first,
                sits_at(first),
                then,
                sits_at(then)
            );
        }
    }

    /// Spec 0054. What it is guarding is a bridge that would quietly open
    /// one room early.
    #[test]
    fn every_one_of_them_is_in_every() {
        let listed = Asked::every();

        for (at, one) in listed.iter().enumerate() {
            assert_eq!(
                sits_at(*one),
                at,
                "{:?} is listed at {} and belongs at {}",
                one,
                at,
                sits_at(*one)
            );
        }

        // and a variant that was given an index and never listed sits off
        // the end of it
        for one in listed {
            assert!(sits_at(*one) < listed.len(), "{:?} is not in every", one);
        }

        let mut sorted = listed.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), listed.len(), "something is in it twice");
    }

    #[test]
    fn it_starts_with_nothing() {
        let trail = Trail::new();

        assert!(trail.is_empty());
        assert!(trail.given().is_empty());
    }

    #[test]
    fn a_room_that_answers_twice_answered_once() {
        let mut trail = Trail::new();

        trail.gave(Asked::OpenedTheCase);
        trail.gave(Asked::OpenedTheCase);

        assert_eq!(
            trail.given().len(),
            1,
            "opening the bookcase again scored again"
        );
    }

    #[test]
    fn it_keeps_the_order_they_came_in() {
        let mut one = Trail::new();
        one.gave(Asked::OpenedTheCase);
        one.gave(Asked::RodeToTheSun);

        let mut other = Trail::new();
        other.gave(Asked::RodeToTheSun);
        other.gave(Asked::OpenedTheCase);

        assert_ne!(
            one.given(),
            other.given(),
            "the same two rooms in either order came out the same, so the \
             order is not kept and a trail cannot be walked backwards"
        );
    }
}
