/*use tokio::sync::{Mutex, OnceCell};
use vosk::{Model, Recognizer};
use crate::sound::wav_utils::read_wav;

static SPEECH_RECOGNIZER: OnceCell<Mutex<Recognizer>> = OnceCell::const_new();

pub async fn recognize_speech(recording_path: &str) -> String {
    let recognizer_mutex = get_speech_recognizer().await;
    let mut recognizer = recognizer_mutex.lock().await;
    let wav = read_wav(recording_path);
    recognizer.accept_waveform(&wav).expect("Error processing audio");
    let recognized_result = recognizer.final_result().single();
    let recognized_text: String = recognized_result.as_ref().unwrap().text.into();
    recognized_text
}

async fn get_speech_recognizer() -> &'static Mutex<Recognizer> {
    SPEECH_RECOGNIZER
        .get_or_init(|| async {
            let model_path = "models/vosk-model-small-es-0.42";
            let model = Model::new(model_path).expect("Failed to load Vosk model");
            let recognizer = Recognizer::new(&model, 16000.0).expect("Failed to create Vosk recognizer");
            Mutex::new(recognizer)
    }).await
}*/