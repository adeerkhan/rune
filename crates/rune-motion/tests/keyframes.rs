//! An empty keyframe track and a clock stepped the wrong way.

use rune_motion::prelude::*;
use rune_motion::{Keyframes, Prop, PropClass};

const X: Prop<f32> = Prop::custom(64, PropClass::Spatial);
const N: NodeId = NodeId(7);

#[test]
fn an_empty_track_plays_nothing_and_leaves_the_property_alone() {
    let mut store = AnimationStore::new();
    store.set(N, X, 3.0);
    let before = store.sample(N, X).unwrap();

    // An empty track has no frames to play: the property keeps its value and nothing animates.
    let track = store.play_keyframes(N, X, &Keyframes::<f32>::new());
    assert!(store.contains(track), "the caller gets a track back");
    assert_eq!(store.sample(N, X).unwrap(), before, "the value did not move");
    assert!(!store.is_animating(N, X), "nothing is animating");
}

#[test]
fn an_empty_track_on_a_property_without_one_does_not_panic() {
    let mut store = AnimationStore::new();
    assert!(!store.has(N, X));
    let track = store.play_keyframes(N, X, &Keyframes::<f32>::new());
    assert!(store.contains(track));
    assert_eq!(store.sample(N, X).unwrap(), 0.0, "it starts at the property's own value");
    assert!(!store.is_animating(N, X));
}

#[test]
fn a_manual_clock_needs_a_finite_rate() {
    for hz in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let r = std::panic::catch_unwind(|| Clock::manual(hz));
        assert!(r.is_err(), "{hz} is not a rate a clock can keep");
    }
    assert!(std::panic::catch_unwind(|| Clock::manual(60.0)).is_ok());
}

#[test]
fn stepping_a_display_clock_leaves_it_where_it_is() {
    // A display clock is driven by `frame(host)`, which has the host timestamp `step` lacks.
    let mut clock = Clock::display();
    let first = clock.step();
    assert_eq!(clock.step(), first, "stepping it changes nothing");
}
