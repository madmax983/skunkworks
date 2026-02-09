use spectral_scribe::encoder::{EncoderConfig, generate_audio};
use spectral_scribe::decoder::{DecoderConfig, audio_to_spectrogram, recover_text};

#[test]
fn test_round_trip() {
    let text = "SECRET DATA 123";
    let enc_config = EncoderConfig::default();
    let dec_config = DecoderConfig::default();

    // Encode
    let samples = generate_audio(text, &enc_config);
    assert!(!samples.is_empty(), "Generated samples should not be empty");

    // Decode
    let spectrogram = audio_to_spectrogram(&samples, &dec_config);
    assert!(!spectrogram.is_empty(), "Spectrogram should not be empty");

    let recovered = recover_text(&spectrogram, &dec_config);

    // Check
    assert_eq!(recovered.trim(), text, "Recovered text must match original");
}
