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
/// **Five, where there were six.** The field went on 2026-11-18, the day it
/// arrived. It was written first on the grounds that it was free, spec
/// 0118's stone standing out in the field, so anybody who could read the
/// stone had answered it already and it was never the line the stone named.
/// Jake, looking at it in the game: by the time you'd see it you've already
/// gotten there. A rung everybody is standing on before they can learn the
/// ladder is there is not a rung, and the count it padded was the only thing
/// it did.
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
    /// Walking in to the sun at the middle of spec 0015's room, the one
    /// with the planets and the moons going round it.
    ///
    /// **The first sun and not the far one.** It was the ride arriving at
    /// spec 0018's sun, which is thirty metres past a door, and then where
    /// you stand out there, which is four metres past the same door. Jake,
    /// 2026-11-21: the first sun in the other room is the far more
    /// impressive place to register having arrived at the sun. It is
    /// eighty metres in from the way in, through the whole model, and what
    /// you arrive at is a sun with the planets turning round it rather
    /// than a ball on a slab.
    ///
    /// **After getting behind the wall, and it has to be.** This sun is at
    /// the middle of that room. Written first, as it was until 2026-11-18,
    /// the first thing spec 0118's stone could ever name was a thing in a
    /// room nobody has found.
    ReachedTheSun,
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
    /// What this station is called when it is written down. Spec 0120.
    ///
    /// **A name and not a number.** The file is a list of what somebody
    /// did and is meant to be readable, which is the only register this
    /// building has; and a number would be an index into `every`, which
    /// has been reordered twice in a fortnight and would silently turn an
    /// old file into a different trail.
    ///
    /// **A match and not a table**, so a station added to this enum stops
    /// it compiling rather than being quietly unwritable.
    pub fn named(self) -> &'static str {
        match self {
            Asked::OpenedTheCase => "opened-the-case",
            Asked::WadedInThePool => "waded-in-the-pool",
            Asked::SolvedTheCube => "solved-the-cube",
            Asked::GotBehindTheWall => "got-behind-the-wall",
            Asked::ReachedTheSun => "reached-the-sun",
        }
    }

    /// The station of that name, or nothing.
    ///
    /// **Nothing rather than an error, and that is the whole of what this
    /// is for.** `Asked` has changed three times in a fortnight: the
    /// tesseract came out of it, the field came out of it, and the order
    /// became a curve. A file written by an arcade of a fortnight ago has
    /// to open, and a name this one no longer knows is a room that no
    /// longer asks rather than a broken save.
    ///
    /// Read off `every` rather than matched, so there is one list of the
    /// names and it is `named`.
    pub fn of_name(name: &str) -> Option<Asked> {
        Asked::every()
            .iter()
            .find(|one| one.named() == name)
            .copied()
    }

    pub fn every() -> &'static [Asked] {
        &[
            Asked::OpenedTheCase,
            Asked::WadedInThePool,
            Asked::SolvedTheCube,
            Asked::GotBehindTheWall,
            Asked::ReachedTheSun,
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

    /// What this trail looks like written down. Spec 0120.
    ///
    /// One station per line, by name, in the order they were given, which
    /// is the only thing in here that means anything. A trailing newline,
    /// so the file is a file rather than a line.
    pub fn written(&self) -> String {
        self.given
            .iter()
            .map(|one| format!("{}\n", one.named()))
            .collect()
    }

    /// And a trail read back out of that. Spec 0120.
    ///
    /// **Nothing in here can fail.** A name nothing answers to is
    /// skipped, a blank line is skipped, surrounding space is ignored,
    /// and a station written twice is given once, which `gave` already
    /// does. What a file cannot do is stop the building opening, so there
    /// is no error to return and nothing to decide at the call.
    pub fn read_back(text: &str) -> Self {
        let mut out = Self::new();

        for line in text.lines() {
            if let Some(one) = Asked::of_name(line.trim()) {
                out.gave(one);
            }
        }

        out
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
            Asked::OpenedTheCase => 0,
            Asked::WadedInThePool => 1,
            Asked::SolvedTheCube => 2,
            Asked::GotBehindTheWall => 3,
            Asked::ReachedTheSun => 4,
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
        (Asked::GotBehindTheWall, Asked::ReachedTheSun),
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

    /// Spec 0120: what is written comes back in the order it went.
    ///
    /// **Not in `every` order**, which is what a set of flags would give
    /// back and is the one thing the record is not: the kerbs and the
    /// ledger both read the order, so a round trip that sorted would
    /// repaint somebody's bridge.
    #[test]
    fn what_is_written_comes_back_in_the_order_it_went() {
        let mut trail = Trail::new();

        trail.gave(Asked::ReachedTheSun);
        trail.gave(Asked::OpenedTheCase);
        trail.gave(Asked::SolvedTheCube);

        let back = Trail::read_back(&trail.written());

        assert_eq!(
            back.given(),
            trail.given(),
            "a trail written out and read back is a different trail"
        );
        assert_ne!(
            back.given(),
            &[
                Asked::OpenedTheCase,
                Asked::SolvedTheCube,
                Asked::ReachedTheSun
            ],
            "it came back in `every` order, so the order was not kept and \
             the one thing in the record that means anything is gone"
        );
    }

    /// Spec 0120: a file from an older arcade still opens.
    ///
    /// **Which is a thing that has already happened three times.** The
    /// tesseract came out of `Asked` on 2026-11-18, the field came out the
    /// same day, and the order became a curve. A save written before any
    /// of those names a station this arcade no longer has.
    #[test]
    fn a_file_from_an_older_arcade_still_opens() {
        let older = "reached-the-sun\nturned-the-sixth-display\nopened-the-case\n";
        let back = Trail::read_back(older);

        assert_eq!(
            back.given(),
            &[Asked::ReachedTheSun, Asked::OpenedTheCase],
            "a name this arcade does not know took the rest of the file \
             with it"
        );
    }

    /// Spec 0120: nothing in a file can stop the building opening.
    ///
    /// Blank lines, space round a name, a name twice, and a file that is
    /// not a trail at all.
    #[test]
    fn nothing_written_in_it_can_break_it() {
        for (text, wants) in [
            ("", 0),
            ("\n\n\n", 0),
            ("  opened-the-case  \n", 1),
            ("opened-the-case\nopened-the-case\n", 1),
            ("{\"given\": [1, 2, 3]}", 0),
            ("\u{0}\u{1}\u{2}", 0),
        ] {
            assert_eq!(
                Trail::read_back(text).given().len(),
                wants,
                "{:?} read back as {} stations",
                text,
                Trail::read_back(text).given().len()
            );
        }
    }

    /// Spec 0120: every station can be written down and read back.
    ///
    /// **The names are their own list and this is what holds it.**
    /// `named` is an exhaustive match, so a station added to the enum
    /// stops that compiling; what nothing else catches is two stations
    /// given the same name, which would make one of them unreadable.
    #[test]
    fn every_station_has_a_name_of_its_own() {
        let mut names: Vec<&'static str> = Asked::every().iter().map(|one| one.named()).collect();
        let many = names.len();

        names.sort_unstable();
        names.dedup();

        assert_eq!(many, names.len(), "two stations share a name");

        for one in Asked::every() {
            assert_eq!(
                Asked::of_name(one.named()),
                Some(*one),
                "{:?} does not come back from its own name",
                one
            );
            assert!(
                !one.named().trim().is_empty(),
                "{:?} has no name to write down",
                one
            );
        }
    }

    #[test]
    fn it_keeps_the_order_they_came_in() {
        let mut one = Trail::new();
        one.gave(Asked::OpenedTheCase);
        one.gave(Asked::ReachedTheSun);

        let mut other = Trail::new();
        other.gave(Asked::ReachedTheSun);
        other.gave(Asked::OpenedTheCase);

        assert_ne!(
            one.given(),
            other.given(),
            "the same two rooms in either order came out the same, so the \
             order is not kept and a trail cannot be walked backwards"
        );
    }
}
