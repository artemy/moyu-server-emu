use async_openai::config::OpenAIConfig;
use async_openai::error::{ApiError, OpenAIError};
use async_openai::types::{
    AudioInput, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
    CreateSpeechRequestArgs, CreateTranscriptionRequestArgs, SpeechModel, SpeechResponseFormat,
    Voice,
};
use async_openai::Client;
use bytes::Bytes;

#[derive(Clone)]
pub struct OpenAiClient {
    client: Client<OpenAIConfig>,
}

pub struct Conversation {
    pub request_text: String,
    pub request_lang: String,
    pub response_text: String,
    pub response_lang: String,
    pub response: Bytes,
}

impl OpenAiClient {
    const SYSTEM_PROMPT: &'static str = "You are an intelligent assistant built-in to a small handheld device. Please answer the questions in concise manner.";
    pub fn new(api_key: String) -> Self {
        OpenAiClient {
            client: Client::with_config(OpenAIConfig::default().with_api_key(api_key)),
        }
    }

    pub async fn translate(
        &self,
        buffer: Bytes,
        from: &String,
        to: &String,
    ) -> Result<Conversation, OpenAIError> {
        let transcription = self.speech_to_text(buffer, from).await?;

        let prompt = format!(
            "Please translate this text into language {}: {}",
            to, transcription
        );
        let translation = self.chat(&prompt).await?;

        self.text_to_speech(&translation, None)
            .await
            .map(|bytes| Conversation {
                request_text: transcription,
                request_lang: from.to_owned(),
                response_text: translation,
                response_lang: to.to_owned(),
                response: bytes,
            })
    }

    pub async fn qa(&self, buffer: Bytes, from: &String) -> Result<Conversation, OpenAIError> {
        let transcription = self.speech_to_text(buffer, from).await?;

        let translation = self.chat(&transcription).await?;

        self.text_to_speech(&translation, None)
            .await
            .map(|bytes| Conversation {
                request_text: transcription,
                request_lang: from.to_owned(),
                response_text: translation,
                response_lang: from.to_owned(),
                response: bytes,
            })
    }

    async fn speech_to_text(&self, buffer: Bytes, from: &String) -> Result<String, OpenAIError> {
        let transcribe = CreateTranscriptionRequestArgs::default()
            .file(AudioInput::from_bytes(String::from("source.ogg"), buffer))
            .model("whisper-1")
            .language(from.to_owned())
            .build()?;

        self.client
            .audio()
            .transcribe(transcribe)
            .await
            .map(|response| response.text)
            .map(|response| {
                log::debug!("Transcription response: {}", response);
                response
            })
    }
    async fn chat(&self, prompt: &String) -> Result<String, OpenAIError> {
        let chat_request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-4o")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(Self::SYSTEM_PROMPT)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(ChatCompletionRequestUserMessageContent::from(
                        prompt.to_string(),
                    ))
                    .build()?
                    .into(),
            ])
            .build()?;

        self.client
            .chat()
            .create(chat_request)
            .await
            .and_then(|response| {
                response
                    .choices
                    .first()
                    .and_then(|choice| choice.clone().message.content)
                    .ok_or(OpenAIError::ApiError(ApiError {
                        message: "No response provided".into(),
                        r#type: None,
                        param: None,
                        code: None,
                    }))
            })
            .map(|response| {
                log::debug!("Chat response: {}", response);
                response
            })
    }

    pub async fn text_to_speech(
        &self,
        text: &String,
        speed: Option<f32>,
    ) -> Result<Bytes, OpenAIError> {
        let speech_request = CreateSpeechRequestArgs::default()
            .model(SpeechModel::Tts1)
            .input(text)
            .voice(Voice::Nova)
            .response_format(SpeechResponseFormat::Mp3)
            .speed(speed.unwrap_or(1.0))
            .build()?;

        self.client
            .audio()
            .speech(speech_request)
            .await
            .map(|res| res.bytes)
    }
}
