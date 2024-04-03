use crate::endpoints::errors::AppError;

pub fn supported_languages() -> [Language; 26] {
    [
        Language::new("Chinese", "zh", "zh"),
        Language::new("English", "en", "en"),
        Language::new("Japanese", "jp", "jp"),
        Language::new("Korean", "kor", "ko"),
        Language::new("Thai", "th", "th"),
        Language::new("Russian", "ru", "ru"),
        Language::new("Spanish", "spa", "es"),
        Language::new("Portuguese", "pt", "pt"),
        Language::new("Arabic", "ara", "ar"),
        Language::new("French", "fra", "fr"),
        Language::new("German", "de", "de"),
        Language::new("Italian", "it", "it"),
        Language::new("Greek", "el", "el"),
        Language::new("Polish", "pl", "pl"),
        Language::new("Finnish", "fin", "fi"),
        Language::new("Czech", "cs", "cs"),
        Language::new("Romanian", "rom", "ro"),
        Language::new("Swedish", "swe", "sv"),
        Language::new("Hungarian", "hu", "hu"),
        Language::new("Vietnamese", "vie", "vi"),
        Language::new("Dutch", "nl", "nl"),
        Language::new("Danish", "dan", "da"),
        Language::new("Hindi", "hi", "hi"),
        Language::new("Bulgarian", "bul", "bg"),
        Language::new("Estonian", "est", "et"),
        Language::new("Slovak", "slo", "sk"),
    ]
}

pub struct Language {
    pub name: String,
    pub device_code: String,
    pub iso_code: String,
}

impl Language {
    pub fn new(
        name: impl Into<String>,
        device_code: impl Into<String>,
        iso_code: impl Into<String>,
    ) -> Self {
        Language {
            name: name.into(),
            device_code: device_code.into(),
            iso_code: iso_code.into(),
        }
    }
    pub fn from_device_code(device_code: &String) -> Result<Self, AppError> {
        supported_languages()
            .into_iter()
            .find(|l| l.device_code.eq(device_code))
            .ok_or(AppError::Generic(format!(
                "Language [{}] not supported",
                device_code
            )))
    }
    pub fn from_iso_code(iso_code: &String) -> Result<Self, AppError> {
        supported_languages()
            .into_iter()
            .find(|l| l.iso_code.eq(iso_code))
            .ok_or(AppError::Generic(format!(
                "Language [{}] not supported",
                iso_code
            )))
    }
}
