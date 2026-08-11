use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSynthesisPayload {
    pub text: String,
    pub voice_name: Option<String>,
    pub rate: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSynthesisResult {
    pub text: String,
    pub ssml: String,
    pub synthesized_length_ms: u64,
    pub is_local: bool,
}

pub struct LocalSpeechSynthesizer;

impl LocalSpeechSynthesizer {
    pub fn new() -> Self {
        Self
    }

    pub fn synthesize(&self, payload: &VoiceSynthesisPayload) -> VoiceSynthesisResult {
        let clean_text = payload.text.trim();
        let ssml = format!(
            "<speak><prosody rate=\"{:.1}\" pitch=\"{:.1}st\">{}</prosody></speak>",
            payload.rate, payload.pitch, clean_text
        );
        let word_count = clean_text.split_whitespace().count() as u64;
        let estimated_duration = (word_count * 250) + 300;

        VoiceSynthesisResult {
            text: clean_text.to_string(),
            ssml,
            synthesized_length_ms: estimated_duration,
            is_local: true,
        }
    }
}

pub trait AudioInputDevice: Send + Sync {
    fn start_recording(&self, callback: Box<dyn Fn(Vec<f32>) + Send>) -> Result<(), Box<dyn std::error::Error>>;
    fn stop_recording(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait SpeechSynthesizer: Send + Sync {
    fn speak(&self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub struct StreamingVoiceManager;

impl StreamingVoiceManager {
    pub fn new() -> Self {
        Self
    }

    pub fn start_voice_assistant_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_speech_synthesis() {
        let synth = LocalSpeechSynthesizer::new();
        let payload = VoiceSynthesisPayload {
            text: "Friday AI is ready.".to_string(),
            voice_name: Some("Friday".to_string()),
            rate: 1.0,
            pitch: 1.0,
        };
        let res = synth.synthesize(&payload);
        assert!(res.is_local);
        assert_eq!(res.text, "Friday AI is ready.");
        assert!(res.synthesized_length_ms > 0);
    }
}
