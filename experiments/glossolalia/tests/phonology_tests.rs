use glossolalia::phonology::{GrimmsLaw, Manner, Phoneme, Place, Rule, Voice, VowelShift, Word};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn test_phoneme_parsing() {
    let p = Phoneme::from_char('p').unwrap();
    assert_eq!(p.voice, Voice::Voiceless);
    assert_eq!(p.manner, Manner::Stop);
    assert_eq!(p.place, Place::Labial);
}

#[test]
fn test_grimms_law_p_to_f() {
    let mut word = Word::new("pater");
    let mut rng = StdRng::seed_from_u64(42);
    let grimms = GrimmsLaw;

    for _ in 0..100 {
        grimms.apply(&mut word, &mut rng);
    }

    let s = word.to_string_word();
    assert!(s.contains('f') || s.contains('s') || s.contains('h'));
}

#[test]
fn test_vowel_shift() {
    let mut word = Word::new("a");
    let shift = VowelShift;
    let mut rng = StdRng::seed_from_u64(123);

    let mut changed = false;
    for _ in 0..50 {
        if shift.apply(&mut word, &mut rng) {
            changed = true;
        }
    }

    assert!(changed);
    assert_ne!(word.to_string_word(), "a");
}
