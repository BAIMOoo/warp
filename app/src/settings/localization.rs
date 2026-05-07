use crate::localization::UiLanguage;
use settings::{
    macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud,
};

define_settings_group!(AppLocalizationSettings, settings: [
    selected_ui_language: SelectedUiLanguage {
        type: UiLanguage,
        default: UiLanguage::English,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "appearance.ui_language",
        description: "The language used for Warp's application interface.",
    },
]);

#[cfg(test)]
mod tests {
    use super::*;
    use settings::Setting;

    #[test]
    fn default_ui_language_is_english() {
        assert_eq!(SelectedUiLanguage::default_value(), UiLanguage::English);
    }

    #[test]
    fn ui_language_serializes_as_snake_case() {
        let value = serde_json::to_value(UiLanguage::ChineseSimplified).unwrap();
        assert_eq!(value, serde_json::json!("chinese_simplified"));
    }
}
