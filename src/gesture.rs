//! What somebody does while they are standing at a stop, per spec 0067.
//!
//! **The robot is the moment they arrive.** A round walks somebody to the
//! fire and then sets every joint to nought, so a figure that read as a
//! person while it was moving becomes a mannequin the instant it gets where
//! it was going. Standing is most of what anybody in this village is doing
//! at any second, so it is most of what they are.
//!
//! Jake, 2026-11-14: it only lands if it reads as natural and not robotic,
//! as though the motion was recorded by a human actor. Nothing here is
//! recorded and nothing here will be mistaken for it. What is written down
//! is the handful of things that actually separate a person standing from a
//! machine paused, each of them a rule rather than a keyframe.
//!
//! **Nothing is ever still.** A body holding a pose is breathing, swaying a
//! little, and carrying its weight on one foot until that foot complains.
//! Under every errand below there is an idle that never stops, and the
//! errand is added to it rather than replacing it.
//!
//! **The joints do not start together.** A shoulder leads and the elbow
//! follows it a moment later, which is the difference between an arm and a
//! lever. Every chain here is given that lag.
//!
//! **No two repetitions match.** The fourth stir is not the third stir. Each
//! turn of a cycle takes its own length and its own reach off a hash of who
//! is doing it and which turn it is.
//!
//! **And it starts and stops at rest.** The gesture fades up over the first
//! moments at a stop and back down before they leave, so nobody begins
//! mid-stroke or walks away with their arm out.

use std::f32::consts::{PI, TAU};

/// What somebody is at a stop for.
///
/// **Named for the errand and not for the motion.** `Stir` rather than
/// `swing the forearm`, because what the village wanted was a cook, and the
/// day the pot moves the cook should still be cooking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errand {
    /// Standing about: at the end of a street, waiting for somebody.
    Wait,
    /// A pot over a fire.
    Stir,
    /// Something heavy off a wagon and onto the next thing.
    Lift,
    /// A bucket up a well, hand over hand.
    Draw,
    /// A horse's neck, which is slower than any of the others and stops to
    /// think halfway through.
    Tend,
    /// At a door: reach, and in.
    Knock,
    /// Down to the ground and up again: putting something on a fire, or
    /// taking it off the floor. Spec 0077.
    Stoop,
}

/// Every joint this building can actually move on a standing figure, which
/// is fewer than a body has.
///
/// **No torso and no neck.** Both are drawn on the body's own yaw at a fixed
/// height, so leaning or looking about would be a change to every head
/// fitting in `main.rs` rather than a number here. `turn` leans on the whole
/// figure instead, which is cruder and is what there is. Spec 0067.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Stance {
    /// Right leg first, as the drawing counts them.
    pub hip: [f32; 2],
    pub knee: [f32; 2],
    pub shoulder: [f32; 2],
    pub elbow: [f32; 2],
    /// How far the hips are off where a standing body would hold them, in
    /// metres, which everything above them rides on.
    pub rise: f32,
    /// How far off the stop's own facing they have turned, in radians.
    pub turn: f32,
    /// How far the body is leaning forward from the hips, in radians.
    ///
    /// **The one thing that stops a gesture being an arm on a post.**
    /// Somebody reaching over a pot or lifting off a wagon takes their
    /// shoulders with them; a figure that holds a plumb torso while the arm
    /// goes out is doing neither of those and is visibly doing neither.
    pub lean: f32,
}

/// A number between -1 and 1 off two things that do not divide.
///
/// Not random: a stop is drawn again every frame and has to answer the same
/// way, and a picture test has to answer the same way tomorrow.
/// **`rem_euclid` and not `fract`.** Rust keeps the sign through `fract`, so
/// the obvious spelling of this returns -3 to 1 rather than -1 to 1: a
/// quarter of the cycle lengths below came out negative, which ran the
/// cycles backwards, and a quarter of the reaches came out inside out. It
/// looked like five separate faults in five separate errands.
fn wiggle(who: f32, what: f32) -> f32 {
    ((who * 12.9898 + what * 78.233).sin() * 43758.547).rem_euclid(1.0) * 2.0 - 1.0
}

/// Nought up to one, with the ends flat, which is how a body starts and
/// stops a movement.
fn eased(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

/// Which turn of a cycle this is and how far through it, with every turn its
/// own length.
///
/// **The jitter is in the length and not only in the reach**, because a
/// motion that is always the same length is a metronome whatever its size.
fn turn_of(who: f32, stood: f32, every: f32) -> (f32, f32) {
    let mut from = 0.0;
    let mut n = 0.0;

    loop {
        let long = every * (1.0 + wiggle(who, n * 3.7) * 0.22);

        if stood < from + long || n > 64.0 {
            return (n, ((stood - from) / long).clamp(0.0, 1.0));
        }

        from += long;
        n += 1.0;
    }
}

/// What a body does when it is not doing anything: breathe, sway, and change
/// feet.
///
/// **The weight shift is the big one.** Standing on both feet evenly is a
/// thing people manage for a few seconds at a time; the rest of it is one
/// hip taking the load while the other leg goes soft, and swapping when that
/// gets tiring. It moves the hips, both knees and the whole body's facing at
/// once, which is why it reads as a person deciding rather than as a part
/// oscillating.
fn idle(who: f32, stood: f32) -> Stance {
    let mut out = Stance::default();

    // breath, which is four millimetres and the only thing here that is
    // regular, because breathing is
    let breath = ((stood / 3.7 + who) * TAU).sin();

    out.rise += breath * 0.004;

    // and a sway that never repeats, off two periods that do not divide
    out.turn += ((stood * 0.31 + who * 2.0).sin() * 0.013) + ((stood * 0.17 + who).sin() * 0.009);
    out.rise += (stood * 0.23 + who * 3.0).sin() * 0.003;

    // the foot they are standing on, which changes every six seconds or so
    let (n, through) = turn_of(who, stood, 6.5);
    let on = (n as usize) % 2;
    let other = 1 - on;
    // eased at both ends, so the change of feet is a movement and not a
    // switch
    let over = eased(through * 3.0) * (1.0 - eased(through * 3.0 - 2.0));
    let lean = 0.35 + wiggle(who, n * 1.3) * 0.25;

    out.knee[other] += (0.10 + wiggle(who, n * 5.1) * 0.04) * over;
    out.hip[other] += 0.07 * over;
    out.hip[on] -= 0.02 * over;
    out.rise -= 0.013 * over;
    out.turn += (if on == 0 { 1.0 } else { -1.0 }) * 0.045 * lean * over;
    // and the arm on the loose side hangs a little further back
    out.shoulder[other] += 0.06 * over;
    // and the shoulders come back a little over the loaded hip
    out.lean -= 0.012 * over;

    out
}

/// The two arms of a swing, with the elbow lagging the shoulder.
///
/// **The lag is the whole of it.** Driven off one phase the arm is a lever
/// with a hinge in it; a tenth of a cycle between the two and it is an arm.
fn swing(through: f32, lag: f32) -> (f32, f32) {
    let lead = ((through * TAU) - PI * 0.5).sin() * 0.5 + 0.5;
    let follow = (((through - lag).rem_euclid(1.0) * TAU) - PI * 0.5).sin() * 0.5 + 0.5;

    (lead, follow)
}

/// What somebody at a stop is doing with themselves at this second.
///
/// `stood` is how long they have been there and `stays` how long they are
/// staying, both in seconds. `who` is anything that tells one person from
/// another and stays the same from frame to frame.
pub fn standing(errand: Errand, who: f32, stood: f32, stays: f32) -> Stance {
    let mut out = idle(who, stood);

    // **Up on arriving and down before leaving.** A gesture at full size on
    // the first frame is somebody who was already stirring before they got
    // there, and one that stops dead is somebody switched off.
    let settled = eased(stood / 0.8) * eased((stays - stood) / 0.7);

    if settled <= 0.0 || errand == Errand::Wait {
        if errand == Errand::Wait {
            waiting(&mut out, who, stood, settled);
        }

        return out;
    }

    match errand {
        Errand::Wait => {}
        Errand::Stoop => {
            let (n, through) = turn_of(who, stood, 4.6);
            let deep = 1.0 + wiggle(who, n * 3.1) * 0.18;

            // **Down and up inside the first third, and the rest of the
            // turn upright.** Somebody who is bent over most of the time
            // is a machine folding; somebody putting a log on a fire goes
            // down once, does the thing, and straightens.
            //
            // The shape the rest of this module uses, `eased(t * k)` against
            // `1 - eased(t * k - (k - 1))`, holds its middle for `(k - 2) / k`
            // of the turn, which is the wrong way round for this: a bigger
            // `k` is a longer plateau. So the pulse runs on a clock of its
            // own that finishes a third of the way through.
            let dip = (through / 0.34).min(1.0);
            let down = eased(dip * 2.0) * (1.0 - eased(dip * 2.0 - 1.0));

            // **The knees take a third of it and the hips the rest.** A
            // body that folds at the hips alone with straight legs is a
            // bow rather than a stoop, which is what the first build of
            // this looked like: `lean` is the only fold this rig has and
            // reaching the floor with it needs about 1.2 radians, which
            // puts a head where its own knees are.
            out.lean += 0.42 * deep * down * settled;
            out.rise -= 0.20 * deep * down * settled;

            for leg in 0..2 {
                out.knee[leg] += (0.46 + wiggle(who, n + leg as f32) * 0.06) * down * settled;
                out.hip[leg] += 0.16 * down * settled;
            }

            // the working arm goes down with the body, and the other goes
            // back a little for balance
            out.shoulder[0] += (0.14 + 0.26 * down) * settled;
            out.elbow[0] += (0.26 - 0.18 * down) * settled;
            out.shoulder[1] += (0.05 - 0.20 * down) * settled;
        }
        Errand::Stir => {
            let (n, through) = turn_of(who, stood, 1.9);
            let (shoulder, elbow) = swing(through, 0.12);
            let reach = 1.0 + wiggle(who, n * 2.3) * 0.18;

            // **The working arm, held out over the pot and not swinging
            // beside the hip.** The first build put the shoulder at a
            // quarter of a radian, which is an arm hanging down with a
            // twitch in it: a hand over a pot a stride away wants the
            // shoulder most of the way forward and the elbow folded, so the
            // forearm is the thing lying across the fire. What moves is
            // small and on top of that.
            out.shoulder[0] += (0.82 + shoulder * 0.17) * reach * settled;
            out.elbow[0] += (0.95 + elbow * 0.35) * reach * settled;
            // the other hand rests on something, and is not a mirror of it
            out.shoulder[1] += 0.10 * settled;
            out.elbow[1] += (0.5 + (stood * 0.4 + who).sin() * 0.06) * settled;
            // a cook leans over the pot, and sways with the stroke rather
            // than holding the lean still
            out.rise -= 0.035 * settled;
            out.lean += (0.20 + shoulder * 0.05) * settled;
        }
        Errand::Lift => {
            let (n, through) = turn_of(who, stood, 3.4);
            let (shoulder, elbow) = swing(through, 0.16);
            let reach = 1.0 + wiggle(who, n * 4.1) * 0.2;

            // both arms, because it is heavy, and the near one leads
            for side in 0..2 {
                let lag = if side == 0 { 0.0 } else { 0.05 };
                let (shoulder, elbow) = swing((through - lag).rem_euclid(1.0), 0.16);

                out.shoulder[side] += (0.15 + shoulder * 0.55) * reach * settled;
                out.elbow[side] += (0.55 + elbow * 0.75) * reach * settled;
            }

            let _ = (shoulder, elbow);

            // the knees take it at the bottom of the lift
            let down = (1.0 - (through * TAU).cos()) * 0.5;

            out.knee[0] += 0.22 * down * settled;
            out.knee[1] += 0.20 * down * settled;
            out.rise -= 0.09 * down * settled;
            // **Bent to it at the bottom and upright at the top**, which is
            // most of what says the thing has weight.
            out.lean += (0.08 + down * 0.30) * settled;
        }
        Errand::Draw => {
            let (n, through) = turn_of(who, stood, 1.5);
            let reach = 1.0 + wiggle(who, n * 1.7) * 0.15;

            // hand over hand, so the two arms are half a turn apart and
            // neither is ever where the other is
            for side in 0..2 {
                let at = (through + side as f32 * 0.5).rem_euclid(1.0);
                let (shoulder, elbow) = swing(at, 0.1);

                out.shoulder[side] += (0.30 + shoulder * 0.42) * reach * settled;
                out.elbow[side] += (1.25 - elbow * 0.85) * reach * settled;
            }

            out.rise -= 0.02 * settled;
            out.lean += 0.14 * settled;
        }
        Errand::Tend => {
            let (n, through) = turn_of(who, stood, 2.6);
            let (shoulder, elbow) = swing(through, 0.14);
            // **It stops to think.** Every third turn or so the hand comes
            // to rest where it is for a beat, which no amount of smoothing
            // does for a cycle that never pauses.
            let resting = if wiggle(who, n * 6.7) > 0.35 {
                0.25
            } else {
                1.0
            };
            let reach = (1.0 + wiggle(who, n * 2.9) * 0.2) * resting;

            out.shoulder[0] += (0.42 + shoulder * 0.20) * reach * settled;
            out.elbow[0] += (0.55 + elbow * 0.30) * reach * settled;
            out.shoulder[1] += -0.05 * settled;
            out.turn += (through - 0.5) * 0.06 * settled;
            out.lean += 0.07 * reach * settled;
        }
        Errand::Knock => {
            // once, not a cycle: reach for the latch, then let the arm down
            let reach = eased(stood / 0.9) * (1.0 - eased((stood - 1.4) / 0.9));

            out.shoulder[0] += 0.85 * reach;
            out.elbow[0] += 0.95 * reach;
            out.lean += 0.10 * reach;
            // and afterwards they stand there with their hands down, which
            // the idle under this is already doing
        }
    }

    out
}

/// Standing about, which is its own errand and not the absence of one.
///
/// Somebody waiting at the end of a street looks round, because a person
/// with nothing to do looks at things. Held for a beat and brought back,
/// rather than swept, which is what makes it a glance.
fn waiting(out: &mut Stance, who: f32, stood: f32, settled: f32) {
    let (n, through) = turn_of(who, stood, 4.2);
    let away = wiggle(who, n * 8.3);

    if away.abs() < 0.3 {
        return;
    }

    let held = eased(through * 4.0) * (1.0 - eased(through * 4.0 - 3.0));

    out.turn += away * 0.33 * held * settled;
    out.shoulder[0] += 0.05 * held;
}

/// How long after arriving an errand lets go of what it is carrying, per
/// spec 0078, or `None` for an errand that never does.
///
/// **The bottom of the first dip.** `Stoop` turns on 4.6 seconds and its
/// dip finishes a third of the way through, so the lowest point is around
/// 0.78 of a second in. The length jitters by a fifth per person and per
/// turn, which at the distance anybody watches this from is not a thing
/// you could see: what matters is that the hand opens while it is down
/// rather than while the body is upright.
///
/// Everything else here holds what it has until it walks away, which is
/// what `None` means and is what every errand did before this.
pub fn lets_go_at(errand: Errand) -> Option<f32> {
    match errand {
        Errand::Stoop => Some(0.78),
        _ => None,
    }
}

/// What carrying something does to a body that is not standing still, per
/// spec 0077.
///
/// **A weight in one hand is not a thing the hand does.** Somebody with a
/// full pail leans away from it, lets that arm hang long and straight, and
/// puts the other one out a little for the counterweight. None of that is
/// in the gait, which knows how far they have walked and nothing else, so
/// it is added on top of whatever the walking gives.
///
/// `side` is the hand it is in, as the drawing counts them.
pub fn laden(side: usize, how_heavy: f32) -> Stance {
    let mut out = Stance::default();
    let away = if side == 0 { -1.0 } else { 1.0 };

    // the body tips away from the load, which is what carrying one looks
    // like from across a street
    out.turn += away * 0.035 * how_heavy;
    out.rise -= 0.012 * how_heavy;
    // the loaded arm hangs long, and the other comes out to balance it
    out.shoulder[side] -= 0.10 * how_heavy;
    out.elbow[side] -= 0.16 * how_heavy;
    out.shoulder[1 - side] += 0.14 * how_heavy;
    out.elbow[1 - side] += 0.10 * how_heavy;

    out
}

/// What somebody sitting on a bench is doing, per spec 0074.
///
/// **Its own function and not a flag on [`standing`].** A seated body's
/// idle is a different idle: the big thing about standing is the weight
/// going from one foot to the other, and somebody on a bench has put their
/// weight down. The errands are no use either, since `Lift` dips both knees
/// and `Draw` works two arms over a well, and neither is a thing you do
/// sitting in a hall.
///
/// The drop to the seat is not in here. This gives the pose; where the
/// bench is is the drawing's, because only the drawing knows how tall this
/// particular body is.
///
/// `stays` is how long they are sitting, which is only used to settle at
/// both ends, the way every gesture in this module does. `seat` is how
/// high the thing under them is and `tall` how big this body is, because a
/// seated pose is the one pose in this module that depends on both.
pub fn sitting(who: f32, stood: f32, stays: f32, seat: f32, tall: f32) -> Stance {
    let mut out = Stance::default();
    let settled = eased(stood / 1.1) * eased((stays - stood) / 0.9);

    // **The thigh takes whatever angle puts the feet on the floor**, and
    // the knee keeps the shin plumb under it. Spec 0046's own convention:
    // positive is forward at the hip, and the knee is subtracted from it,
    // so `hip - knee` of nought is a shin straight down.
    //
    // Level was the first build and the steading's benches are 0.52 where
    // a shin is 0.447, which left the feet seven centimetres off the
    // floor: near enough to read as a mistake and not far enough to read
    // as dangling. Where a seat is lower than a shin this comes out
    // negative and the knees rise instead, which is the same arithmetic
    // and the same thing a low stool does to you.
    let tilt = ((seat - crate::people::SHIN_TALL * tall) / (crate::people::THIGH_TALL * tall))
        .clamp(-1.0, 1.0)
        .asin();

    for leg in 0..2 {
        out.hip[leg] = PI * 0.5 - tilt;
        out.knee[leg] = PI * 0.5 - tilt;
    }

    // breath, the same four millimetres as standing
    out.rise += ((stood / 3.7 + who) * TAU).sin() * 0.004;

    // **And a shift on the seat rather than between the feet.** Nobody
    // holds one position on a bench either. It leans the body a little,
    // takes one knee out further than the other and turns the whole figure
    // with it, which is the same three things the standing shift moves and
    // none of the same joints.
    let (n, through) = turn_of(who, stood, 8.5);
    let on = (n as usize) % 2;
    let over = eased(through * 3.0) * (1.0 - eased(through * 3.0 - 2.0));
    let how_far = 0.5 + wiggle(who, n * 2.9) * 0.4;

    out.hip[on] += (0.10 + wiggle(who, n * 6.1) * 0.05) * over;
    out.knee[on] -= 0.12 * over;
    out.turn += (if on == 0 { 1.0 } else { -1.0 }) * 0.05 * how_far * over;
    out.lean += (0.04 + 0.05 * how_far) * over;
    // and the hands follow the body rather than hanging where they were
    out.shoulder[on] += 0.07 * over;

    // a sway under all of it, off two periods that do not divide
    out.turn += ((stood * 0.27 + who * 2.0).sin() * 0.011) + ((stood * 0.15 + who).sin() * 0.008);
    // and somebody sitting in a hall looks about, which is the one thing
    // `waiting` already says how to do
    waiting(&mut out, who + 1.7, stood, settled);

    out
}

/// What a horse tied to a rail is doing with itself, per spec 0070.
///
/// **The head is the big one, where a person's is the weight shift.** A
/// person stands on two feet and manages it for a few seconds at a time, so
/// what says they are alive is the hip. A horse stands on four and is
/// stable: its stance says very little and its head says everything, on a
/// metre of neck where two centimetres at the shoulder is ten at the muzzle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Hitched {
    /// How far the head has dropped below where it is carried, in metres.
    pub head_drops: f32,
    /// How far the head has swung off the body's own line, in metres.
    pub head_aside: f32,
    /// How far the tail is swung off where it hangs, in radians about the
    /// horse's own forward.
    pub tail: f32,
    /// How far each hind leg is resting, nought to one, as the drawing
    /// counts the two sides.
    pub rests: [f32; 2],
    /// How far the body is off where it stands, in metres, which the legs
    /// do not take and everything above them does.
    pub rise: f32,
}

/// What a horse at the rail is doing at this second.
///
/// `who` is anything that tells one animal from another and stays the same
/// from frame to frame. `since` is the clock.
pub fn hitched(who: f32, since: f32) -> Hitched {
    let mut out = Hitched::default();

    // **Breath**, which is the only regular thing here, because breathing
    // is. Five seconds to the cycle against the people's 3.7, because a
    // horse at rest breathes slower than a person standing.
    out.rise += ((since / 5.0 + who) * TAU).sin() * 0.006;

    // **The head, dropping and coming back on its own time.** Eased at both
    // ends and away from the bottom of its reach for most of a turn, so it
    // is a horse lowering its head rather than a head on a spring.
    let (n, through) = turn_of(who, since, 9.0);
    let reach = 0.10 + wiggle(who, n * 2.3) * 0.07;

    out.head_drops += reach * eased(through * 2.5) * (1.0 - eased(through * 2.5 - 1.5));
    // and a bob under it that never stops, off two periods that do not
    // divide, like the people's sway
    out.head_drops += (since * 0.37 + who).sin() * 0.010;
    out.head_aside +=
        ((since * 0.21 + who * 2.0).sin() * 0.030) + ((since * 0.13 + who).sin() * 0.022);

    // **The tail swishes in bouts rather than swinging.** A tail on a
    // steady cycle is the metronome this module is about. A horse swishes
    // at a fly: two or three strokes that die away, then ten or twenty
    // seconds of nothing.
    let (n, through) = turn_of(who + 3.0, since, 13.0);
    let bout = 0.11;

    if through < bout {
        let t = through / bout;

        out.tail = ((t * TAU * 2.5).sin())
            * (1.0 - t)
            * eased(t * 6.0)
            * (0.20 + wiggle(who, n * 4.7) * 0.09);
    }

    // **A hind leg rests, which is the four-legged weight shift.** One hind
    // hoof tipped back with the load off it, swapped every half minute or
    // so, with both down in between. Only a hind leg: a foreleg cocked the
    // same way is a lame horse.
    let (n, through) = turn_of(who + 7.0, since, 31.0);
    let on = (n as usize) % 2;
    let over = eased(through * 3.0) * (1.0 - eased(through * 3.0 - 2.0));

    out.rests[on] = over;
    out.rise -= 0.010 * over;

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVERY: Errand = Errand::Wait;

    fn all() -> [Errand; 6] {
        [
            Errand::Wait,
            Errand::Stir,
            Errand::Lift,
            Errand::Draw,
            Errand::Tend,
            Errand::Knock,
        ]
    }

    fn apart(one: &Stance, two: &Stance) -> f32 {
        let mut most: f32 = 0.0;

        for n in 0..2 {
            most = most.max((one.hip[n] - two.hip[n]).abs());
            most = most.max((one.knee[n] - two.knee[n]).abs());
            most = most.max((one.shoulder[n] - two.shoulder[n]).abs());
            most = most.max((one.elbow[n] - two.elbow[n]).abs());
        }

        most.max((one.turn - two.turn).abs())
            .max((one.lean - two.lean).abs())
            .max((one.rise - two.rise).abs() * 10.0)
    }

    /// How far apart two horses' poses are, in roughly comparable units:
    /// the metres scaled up so a centimetre of head counts like a tenth of
    /// a radian of tail.
    fn apart_hitched(one: &Hitched, two: &Hitched) -> f32 {
        ((one.head_drops - two.head_drops).abs() * 10.0)
            .max((one.head_aside - two.head_aside).abs() * 10.0)
            .max((one.tail - two.tail).abs())
            .max((one.rests[0] - two.rests[0]).abs())
            .max((one.rests[1] - two.rests[1]).abs())
            .max((one.rise - two.rise).abs() * 10.0)
    }

    /// Spec 0077: a stoop goes down once a turn and spends most of the
    /// turn upright.
    ///
    /// **Which is what makes it a stoop and not a machine folding.**
    /// Somebody putting a log on a fire goes down, does the thing, and
    /// straightens; the first build of this bent far enough to put a head
    /// past its own toes and did it every few seconds.
    #[test]
    fn a_stoop_is_a_thing_you_do_once_and_get_up_from() {
        for who in [0.3f32, 2.4, 5.1] {
            let mut down = 0;
            let mut deepest: f32 = 0.0;

            for step in 0..2_000 {
                let stood = 1.5 + step as f32 * 0.05;
                let pose = standing(Errand::Stoop, who, stood, 110.0);

                if pose.lean > 0.12 {
                    down += 1;
                }

                deepest = deepest.max(pose.lean);
            }

            let share = down as f32 / 2_000.0;

            assert!(
                share < 0.5,
                "they are bent over {:.0}% of the time, which is not a stoop",
                share * 100.0
            );
            assert!(
                (0.3..0.7).contains(&deepest),
                "the deepest they bend is {:.2} radians, which is either a \
                 nod or a dive",
                deepest
            );
        }
    }

    /// Spec 0077: and a load leans a body away from itself.
    #[test]
    fn a_load_leans_a_body_away_from_it() {
        for side in 0..2 {
            let laden = laden(side, 1.0);
            let away = if side == 0 { -1.0 } else { 1.0 };

            assert!(
                laden.turn * away > 0.0,
                "a load in hand {} turns the body {:+.3}, which is toward it",
                side,
                laden.turn
            );
            // the loaded arm hangs longer than the free one, which is what
            // says which hand it is in
            assert!(
                laden.shoulder[side] < laden.shoulder[1 - side],
                "the loaded arm is raised {:+.3} and the free one {:+.3}",
                laden.shoulder[side],
                laden.shoulder[1 - side]
            );
        }

        // and nothing of it for somebody carrying nothing
        assert_eq!(laden(0, 0.0), Stance::default(), "an empty hand leans");
    }

    /// Spec 0074: a seated leg is a thigh forward and a shin straight down
    /// under it, with the foot on the floor.
    ///
    /// **Worked out rather than looked at**, because seven centimetres of
    /// float is the size of fault that reads as wrong in a picture and is
    /// never obvious in one. The first build sat everybody with a level
    /// thigh, which on this building's own benches is exactly that.
    #[test]
    fn a_seated_foot_reaches_the_floor() {
        let (thigh, shin) = (crate::people::THIGH_TALL, crate::people::SHIN_TALL);

        for tall in [0.94f32, 1.0, 1.06] {
            for seat in [0.40f32, 0.45, 0.52, 0.58] {
                let pose = sitting(0.3, 0.0, 9.0, seat, tall);

                for leg in 0..2 {
                    // the shin hangs off the thigh's own line by
                    // `hip - knee`, so nought is plumb
                    assert!(
                        (pose.hip[leg] - pose.knee[leg]).abs() < 1e-5,
                        "the shin is {:.3} off plumb on a seat of {:.2}",
                        pose.hip[leg] - pose.knee[leg],
                        seat
                    );

                    // and the foot lands where the floor is
                    let tilt = PI * 0.5 - pose.hip[leg];
                    let foot = seat - thigh * tall * tilt.sin() - shin * tall;

                    assert!(
                        foot.abs() < 0.01,
                        "a foot lands {:.3} off the floor on a seat of {:.2} \
                         under a body of {:.2}",
                        foot,
                        seat,
                        tall
                    );
                }
            }
        }
    }

    /// Spec 0074, and the same claim as every other pose in here: a body
    /// that stops moving the moment it sits down is the fault this module
    /// exists for.
    #[test]
    fn nobody_sitting_is_actually_still() {
        for who in [0.3f32, 1.7, 4.9] {
            let mut most: f32 = 0.0;

            for step in 0..200 {
                let stood = 1.0 + step as f32 * 0.1;
                let now = sitting(who, stood, 24.0, 0.52, 1.0);
                let next = sitting(who, stood + 0.1, 24.0, 0.52, 1.0);

                most = most.max(apart(&now, &next));
            }

            assert!(
                most > 0.004,
                "somebody sitting moves at most {:.5} in a tenth of a second",
                most
            );
        }
    }

    /// The same fault as the people's, one spec later: spec 0070 is here
    /// because the horses were the last thing in the village holding a
    /// pose.
    #[test]
    fn a_hitched_horse_is_never_still() {
        for who in [0.7f32, 3.0, 5.4] {
            let mut most: f32 = 0.0;

            for step in 0..600 {
                let since = step as f32 * 0.1;

                most = most.max(apart_hitched(
                    &hitched(who, since),
                    &hitched(who, since + 0.1),
                ));
            }

            assert!(
                most > 0.004,
                "a horse moves at most {:.5} in a tenth of a second, which is a sawhorse",
                most
            );
        }
    }

    /// **The argument the whole spec rests on.** A horse on four legs says
    /// nothing with its stance, so if the head is not the biggest thing
    /// moving then nothing anybody can see is moving at all.
    #[test]
    fn a_horse_says_most_of_it_with_its_head() {
        for who in [0.7f32, 3.0] {
            let mut head: f32 = 0.0;
            let mut body: f32 = 0.0;

            for step in 0..1200 {
                let one = hitched(who, step as f32 * 0.1);

                head = head.max(one.head_drops.abs()).max(one.head_aside.abs());
                body = body.max(one.rise.abs());
            }

            assert!(
                head > body * 4.0,
                "the head moves {:.4} and the body {:.4}, which is a horse \
                 bobbing rather than a horse standing",
                head,
                body
            );
        }
    }

    /// A tail on a steady cycle is the metronome this module exists to
    /// avoid, so it has to be still most of the time and quick when it
    /// goes.
    #[test]
    fn a_tail_swishes_in_bouts_rather_than_swinging() {
        for who in [0.7f32, 3.0, 5.4] {
            let mut moving = 0;
            let mut most: f32 = 0.0;

            for step in 0..2000 {
                let tail = hitched(who, step as f32 * 0.05).tail;

                if tail.abs() > 0.02 {
                    moving += 1;
                }

                most = most.max(tail.abs());
            }

            let share = moving as f32 / 2000.0;

            assert!(
                share < 0.25,
                "the tail is moving {:.0}% of the time, which is a pendulum",
                share * 100.0
            );
            assert!(
                share > 0.02 && most > 0.1,
                "the tail moves {:.0}% of the time and reaches {:.3}, which is a \
                 tail nailed on",
                share * 100.0,
                most
            );
        }
    }

    /// A horse rests a hind leg. Two at once is a horse sitting down, and
    /// the fore legs never do it at all, which is drawn rather than said
    /// here: this holds the half of it that is a number.
    #[test]
    fn a_horse_rests_one_hind_leg_at_a_time() {
        for who in [0.7f32, 3.0, 5.4] {
            let mut each = [false, false];

            for step in 0..6000 {
                let rests = hitched(who, step as f32 * 0.1).rests;

                assert!(
                    rests[0] <= 0.0 || rests[1] <= 0.0,
                    "both hind legs are resting at once, at {:.1} seconds",
                    step as f32 * 0.1
                );

                for n in 0..2 {
                    assert!(
                        (0.0..=1.0).contains(&rests[n]),
                        "a leg is resting {:.3}, which is not a share",
                        rests[n]
                    );

                    each[n] |= rests[n] > 0.5;
                }
            }

            assert!(
                each[0] && each[1],
                "over ten minutes it rested {:?}, so it favours one leg for ever",
                each
            );
        }
    }

    /// Two animals at one rail driven off one clock, which is what the
    /// village has.
    #[test]
    fn two_horses_are_not_the_same_horse() {
        let (one, two) = (0.7f32, 3.0f32);
        let mut most: f32 = 0.0;

        for step in 0..1200 {
            let since = step as f32 * 0.1;

            most = most.max(apart_hitched(&hitched(one, since), &hitched(two, since)));
        }

        assert!(
            most > 0.1,
            "two horses are never more than {:.4} apart, which is one horse drawn twice",
            most
        );
    }

    /// The fault this whole module is for: a figure that arrives and freezes.
    #[test]
    fn nobody_standing_still_is_actually_still() {
        let _ = EVERY;

        for errand in all() {
            for who in [0.3f32, 1.7, 4.9] {
                let mut most: f32 = 0.0;

                for step in 0..60 {
                    let stood = 1.0 + step as f32 * 0.1;
                    let now = standing(errand, who, stood, 9.0);
                    let next = standing(errand, who, stood + 0.1, 9.0);

                    most = most.max(apart(&now, &next));
                }

                assert!(
                    most > 0.004,
                    "{:?} moves at most {:.5} in a tenth of a second, which is a statue",
                    errand,
                    most
                );
            }
        }
    }

    /// And a figure that does the same thing over and over is the other
    /// half of the same fault.
    #[test]
    fn no_two_turns_of_a_cycle_are_the_same() {
        for (errand, every) in [
            (Errand::Stir, 1.9),
            (Errand::Lift, 3.4),
            (Errand::Draw, 1.5),
            (Errand::Tend, 2.6),
        ] {
            let who = 2.1;
            let mut most: f32 = 0.0;

            for step in 0..40 {
                let stood = 1.5 + step as f32 * 0.05;
                let now = standing(errand, who, stood, 30.0);
                let again = standing(errand, who, stood + every, 30.0);

                most = most.max(apart(&now, &again));
            }

            assert!(
                most > 0.02,
                "{:?} comes back to the same pose a cycle later, within {:.4}",
                errand,
                most
            );
        }
    }

    /// Arriving and leaving are movements too.
    #[test]
    fn a_gesture_fades_up_and_down_rather_than_switching_on() {
        for errand in all() {
            let stays = 6.0;
            let who = 0.9;
            let arriving = standing(errand, who, 0.0, stays);
            let settled = standing(errand, who, 3.0, stays);
            let leaving = standing(errand, who, stays, stays);
            let resting = idle(who, 0.0);

            assert!(
                apart(&arriving, &resting) < 0.05,
                "{:?} is already underway on the frame they arrive",
                errand
            );
            assert!(
                apart(&leaving, &idle(who, stays)) < 0.05,
                "{:?} is still going on the frame they leave",
                errand
            );

            // **Over the stay and not at one instant**, because `Tend`
            // rests its hand for a beat every few turns on purpose, and a
            // sample that lands in one of those says the errand does
            // nothing when what it is doing is pausing.
            if errand != Errand::Wait && errand != Errand::Knock {
                let most = (0..50)
                    .map(|step| {
                        let stood = 1.0 + step as f32 * 0.08;

                        apart(&standing(errand, who, stood, stays), &idle(who, stood))
                    })
                    .fold(0.0f32, f32::max);

                assert!(most > 0.2, "{:?} never actually does anything", errand);
            }

            let _ = settled;
        }
    }

    /// A shoulder leads and an elbow follows, which is what tells an arm
    /// from a lever.
    ///
    /// **Two signals rather than two peaks.** Comparing the instant each one
    /// is furthest over sounds like the same question and is not: a cycle
    /// that pauses, as `Tend` does on purpose, puts both maxima in whatever
    /// quiet stretch the idle happens to peak in, and the test reads that as
    /// one hinge. Normalising both curves and asking how far apart they stay
    /// does not care where the maxima are.
    #[test]
    fn the_shoulder_and_the_elbow_do_not_turn_together() {
        for errand in [Errand::Stir, Errand::Lift, Errand::Draw, Errand::Tend] {
            let who = 1.3;
            let taken: Vec<(f32, f32)> = (0..400)
                .map(|step| {
                    let now = standing(errand, who, 2.0 + step as f32 * 0.02, 30.0);

                    (now.shoulder[0], now.elbow[0])
                })
                .collect();

            let span = |pick: fn(&(f32, f32)) -> f32| {
                let low = taken.iter().map(pick).fold(f32::MAX, f32::min);
                let high = taken.iter().map(pick).fold(f32::MIN, f32::max);

                (low, (high - low).max(1e-6))
            };
            let (shoulder_low, shoulder_span) = span(|at| at.0);
            let (elbow_low, elbow_span) = span(|at| at.1);
            let apart = taken
                .iter()
                .map(|(shoulder, elbow)| {
                    ((shoulder - shoulder_low) / shoulder_span - (elbow - elbow_low) / elbow_span)
                        .abs()
                })
                .sum::<f32>()
                / taken.len() as f32;

            assert!(
                apart > 0.05,
                "{:?} moves its shoulder and its elbow as one signal, {:.4} apart on \
                 average, which is one hinge and not two",
                errand,
                apart
            );
        }
    }

    /// And none of it bends anywhere a body does not.
    #[test]
    fn nothing_bends_the_wrong_way_or_too_far() {
        for errand in all() {
            for who in [0.2f32, 2.6, 5.4] {
                for step in 0..400 {
                    let stood = step as f32 * 0.025;
                    let now = standing(errand, who, stood, 10.0);

                    for n in 0..2 {
                        assert!(
                            (-0.2..=0.6).contains(&now.knee[n]),
                            "{:?} bends a knee to {:.2}",
                            errand,
                            now.knee[n]
                        );
                        assert!(
                            (-0.1..=2.4).contains(&now.elbow[n]),
                            "{:?} bends an elbow to {:.2}",
                            errand,
                            now.elbow[n]
                        );
                        assert!(
                            (-0.8..=1.6).contains(&now.shoulder[n]),
                            "{:?} swings a shoulder to {:.2}",
                            errand,
                            now.shoulder[n]
                        );
                    }

                    assert!(
                        now.rise.abs() < 0.2,
                        "{:?} moves the hips {:.3} metres",
                        errand,
                        now.rise
                    );
                    assert!(
                        (-0.1..=0.55).contains(&now.lean),
                        "{:?} leans {:.2}, which is a bow or a backflip",
                        errand,
                        now.lean
                    );
                    assert!(
                        now.turn.abs() < 0.6,
                        "{:?} turns {:.2} off their facing",
                        errand,
                        now.turn
                    );
                }
            }
        }
    }
}
