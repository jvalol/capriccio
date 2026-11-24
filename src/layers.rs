//! Which world you are standing in. Spec 0134.
//!
//! **A fourth coordinate, and nothing more than that.** `w` beside a
//! position rather than inside it: the arcade is layer 0 and spec 0133's
//! place is layer 1, and the two stand on the same three coordinates
//! without being in each other's way. The engine is never told. What hands
//! it a scene is what knows the layer.
//!
//! **Integers, so every question about one is an equality.** Jake, deciding
//! it: *it can be integers because that makes more sense than irrationals
//! blending between things.* Nothing is half on a layer and no code has to
//! say what a body at `w = 0.4` walks into.
//!
//! **This building has done it once already.** Spec 0021's tesseract room
//! is a layer in all but name: `Arcade::inside` is its `w`, `draw` branches
//! on it and returns, the walk is handed `tesseract::solid_in_a_cell`
//! instead of the building's list, `what_i_see` gives nothing and `saying`
//! gives that room's own help. What is here is that shape, named, for a
//! place that is outdoors and so needs a sky as well.
//!
//! The `w` in `tesseract::Axis` is a different one. That picks which of
//! eight cells of one room you are in; this picks which world. Same idea at
//! two scales, which spec 0134 says out loud so that nobody has to work it
//! out.

/// Which world. The number is the coordinate.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, PartialOrd, Ord, Hash)]
pub struct Layer(pub i32);

/// The building, its field, its bridge and everything that was here before
/// spec 0134. Layer nought, and the default, so anything that does not ask
/// gets the world it always had.
pub const ARCADE: Layer = Layer(0);

/// Spec 0133's place.
pub const ELYSIUM: Layer = Layer(1);

impl Layer {
    /// The coordinate itself, for anything that wants the number.
    pub fn w(self) -> i32 {
        self.0
    }

    /// The other one, across spec 0133's glass.
    ///
    /// **Named rather than written at the crossing**, because the crossing
    /// is the one place that changes a layer and a `match` written there
    /// would be the only account of which layers exist.
    pub fn across(self) -> Layer {
        match self {
            ARCADE => ELYSIUM,
            _ => ARCADE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default is the world that was here first.
    ///
    /// **Which is what lets this be added without touching anything.**
    /// Everything that does not ask about a layer is on layer nought, so
    /// the arcade, its field and its nine hundred tests carry on exactly as
    /// they were.
    #[test]
    fn what_does_not_ask_is_in_the_arcade() {
        assert_eq!(Layer::default(), ARCADE);
        assert_eq!(ARCADE.w(), 0);
    }

    /// The glass takes you across and back, and nowhere else.
    #[test]
    fn across_and_across_again_is_where_you_started() {
        assert_eq!(ARCADE.across(), ELYSIUM);
        assert_eq!(ELYSIUM.across(), ARCADE);
        assert_eq!(ARCADE.across().across(), ARCADE);
    }
}
