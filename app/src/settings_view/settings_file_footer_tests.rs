use super::SettingsFooterKind;
use crate::appearance::Appearance;
use crate::localization::{localized_settings_text, UiLanguage};
use crate::settings::{AppLocalizationSettings, SettingsFileError};
use crate::test_util::settings::initialize_settings_for_tests;
use crate::ui_components::icons::Icon;
use crate::WorkspaceAction;
use settings::Setting;
use warpui::{App, SingletonEntity};

// Hidden takes precedence over everything when the feature flag is off.

#[test]
fn feature_disabled_hides_footer_regardless_of_error_or_dismissal() {
    assert_eq!(
        SettingsFooterKind::choose(false, false, false),
        SettingsFooterKind::Hidden
    );
    assert_eq!(
        SettingsFooterKind::choose(false, true, false),
        SettingsFooterKind::Hidden
    );
    assert_eq!(
        SettingsFooterKind::choose(false, false, true),
        SettingsFooterKind::Hidden
    );
    assert_eq!(
        SettingsFooterKind::choose(false, true, true),
        SettingsFooterKind::Hidden
    );
}

// ErrorAlert only appears when BOTH an error is present AND the banner is
// dismissed. The workspace banner is still in charge otherwise.

#[test]
fn error_alert_shown_only_when_error_and_banner_dismissed() {
    assert_eq!(
        SettingsFooterKind::choose(true, true, true),
        SettingsFooterKind::ErrorAlert
    );
}

#[test]
fn error_present_but_banner_not_dismissed_shows_open_button() {
    // User is still seeing the workspace banner at the top of the workspace,
    // so the nav rail should just offer the plain button.
    assert_eq!(
        SettingsFooterKind::choose(true, true, false),
        SettingsFooterKind::OpenButton
    );
}

#[test]
fn no_error_but_banner_dismissed_shows_open_button() {
    // `banner_dismissed` is sticky across error/no-error transitions in the
    // workspace today — without an error, we still want the plain button.
    assert_eq!(
        SettingsFooterKind::choose(true, false, true),
        SettingsFooterKind::OpenButton
    );
}

#[test]
fn no_error_and_banner_not_dismissed_shows_open_button() {
    assert_eq!(
        SettingsFooterKind::choose(true, false, false),
        SettingsFooterKind::OpenButton
    );
}

#[test]
fn open_button_renders_simplified_chinese_copy() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);
        app.add_singleton_model(|_| Appearance::mock());

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });

            let text_content = super::render_open_settings_file_button(
                Appearance::as_ref(ctx),
                ctx,
                Default::default(),
            )
            .debug_text_content()
            .unwrap_or_default();

            assert!(
                text_content.contains("打开设置文件"),
                "Expected Chinese open settings file label: {text_content}"
            );
        });
    })
}

#[test]
fn error_alert_action_buttons_render_simplified_chinese_copy() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);
        app.add_singleton_model(|_| Appearance::mock());

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });

            let appearance = Appearance::as_ref(ctx);
            let open_file_text = super::render_alert_action_button(
                appearance.ui_font_family(),
                appearance.theme().nonactive_ui_text_color().into_solid(),
                Default::default(),
                localized_settings_text("Open file", ctx),
                None,
                true,
                WorkspaceAction::OpenSettingsFile,
            )
            .debug_text_content()
            .unwrap_or_default();
            assert!(
                open_file_text.contains("打开文件"),
                "Expected Chinese open file action label: {open_file_text}"
            );

            let fix_with_oz_text = super::render_alert_action_button(
                appearance.ui_font_family(),
                appearance.theme().nonactive_ui_text_color().into_solid(),
                Default::default(),
                localized_settings_text("Fix with Oz", ctx),
                Some(Icon::Oz),
                false,
                WorkspaceAction::OpenSettingsFile,
            )
            .debug_text_content()
            .unwrap_or_default();
            assert!(
                fix_with_oz_text.contains("用 Oz 修复"),
                "Expected Chinese Fix with Oz action label while preserving product term: {fix_with_oz_text}"
            );
        });
    })
}

#[test]
fn error_alert_renders_localized_settings_file_error_copy() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);
        app.add_singleton_model(|_| Appearance::mock());

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });

            let (heading, description) = SettingsFileError::InvalidSettings(vec![
                "terminal.font_size".to_owned(),
            ])
            .localized_heading_and_description(ctx);

            assert_eq!(heading, "你的设置文件包含错误。");
            assert!(
                description.contains("无效值：terminal.font_size。"),
                "Expected invalid settings key to remain dynamic in localized description: {description}"
            );
            assert!(
                description.contains("正在使用默认值。"),
                "Expected localized default-value fallback copy: {description}"
            );
        });
    })
}
