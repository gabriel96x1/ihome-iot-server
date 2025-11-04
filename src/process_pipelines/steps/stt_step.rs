use crate::stt::speech_recognizer::recognize_speech;

pub async fn stt_step(audio_path: String) -> String {
    recognize_speech(audio_path.as_str()).await
}