use super::{
    derive_agent_attribution_toggle_state, AISettingsPageView, AgentAttributionToggleState,
    AgentsWidget, UsageWidget,
};
use crate::ai::blocklist::BlocklistAIPermissions;
use crate::ai::execution_profiles::profiles::AIExecutionProfilesModel;
use crate::ai::llms::LLMPreferences;
use crate::ai::mcp::TemplatableMCPServerManager;
use crate::ai::request_usage_model::AIRequestUsageModel;
use crate::auth::{AuthManager, AuthStateProvider};
use crate::cloud_object::model::persistence::CloudModel;
use crate::localization::UiLanguage;
use crate::network::NetworkStatus;
use crate::server::cloud_objects::update_manager::UpdateManager;
use crate::server::server_api::ServerApiProvider;
use crate::server::sync_queue::SyncQueue;
use crate::settings::{AppLocalizationSettings, PrivacySettings};
use crate::settings_view::keybindings::KeybindingChangedNotifier;
use crate::settings_view::settings_page::SettingsWidget;
use crate::test_util::settings::initialize_settings_for_tests;
use crate::workspaces::team_tester::TeamTesterStatus;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::workspaces::workspace::AdminEnablementSetting;
use crate::LaunchMode;
use settings::Setting;
use warp_core::ui::appearance::Appearance;
use warpui::elements::MouseStateHandle;
use warpui::platform::WindowStyle;
use warpui::{App, SingletonEntity, View};

fn init_ai_page_test_models(app: &mut App) {
    initialize_settings_for_tests(app);
    app.add_singleton_model(|_| ServerApiProvider::new_for_test());
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(AuthManager::new_for_test);
    app.add_singleton_model(|_| Appearance::mock());
    app.add_singleton_model(|_| KeybindingChangedNotifier::mock());
    app.add_singleton_model(SyncQueue::mock);
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(TeamTesterStatus::mock);
    app.add_singleton_model(UpdateManager::mock);
    app.add_singleton_model(CloudModel::mock);
    app.add_singleton_model(|_| TemplatableMCPServerManager::default());
    app.add_singleton_model(PrivacySettings::mock);
    app.add_singleton_model(UserWorkspaces::default_mock);
    app.add_singleton_model(|ctx| {
        AIExecutionProfilesModel::new(&LaunchMode::new_for_unit_test(), ctx)
    });
    app.add_singleton_model(|ctx| {
        AIRequestUsageModel::new_for_test(ServerApiProvider::as_ref(ctx).get_ai_client(), ctx)
    });
    app.add_singleton_model(LLMPreferences::new);
    app.add_singleton_model(BlocklistAIPermissions::new);
}

#[test]
fn ai_settings_page_renders_simplified_chinese_static_copy() {
    App::test((), |mut app| async move {
        init_ai_page_test_models(&mut app);

        let (_window_id, page) =
            app.add_window(WindowStyle::NotStealFocus, AISettingsPageView::new);

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });
        });

        page.read(&app, |page, ctx| {
            let appearance = Appearance::as_ref(ctx);
            let usage_text = UsageWidget::default()
                .render(page, appearance, ctx)
                .debug_text_content()
                .unwrap_or_default();
            let agents_text = AgentsWidget::default()
                .render(page, appearance, ctx)
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                usage_text.contains("用量"),
                "Expected Chinese usage header in rendered content: {usage_text}"
            );
            assert!(
                agents_text.contains("权限"),
                "Expected Chinese permissions header in rendered content: {agents_text}"
            );
            assert!(
                agents_text.contains("模型"),
                "Expected Chinese model header in rendered content: {agents_text}"
            );
            assert!(
                agents_text.contains("Agent"),
                "Expected Agent product term to remain untranslated: {agents_text}"
            );
            let profile_text = page.profile_views[0]
                .as_ref(ctx)
                .render(ctx)
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                profile_text.contains("编辑"),
                "Expected Chinese edit button in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("模型"),
                "Expected Chinese models header in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("基础模型："),
                "Expected Chinese base model label in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("权限"),
                "Expected Chinese permissions header in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("应用代码 diff："),
                "Expected Chinese permission label in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("由 Agent 决定"),
                "Expected Chinese permission value in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("始终询问"),
                "Expected Chinese permission value in profile content: {profile_text}"
            );
            assert!(
                profile_text.contains("目录允许列表："),
                "Expected Chinese allowlist label in profile content: {profile_text}"
            );
            assert!(
                !profile_text.contains("Base model:"),
                "Expected profile labels to hot-render through localized text: {profile_text}"
            );
        });
    })
}

#[test]
fn ai_settings_mcp_lists_render_simplified_chinese_static_copy() {
    App::test((), |mut app| async move {
        init_ai_page_test_models(&mut app);

        let (_window_id, page) =
            app.add_window(WindowStyle::NotStealFocus, AISettingsPageView::new);

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });
        });

        page.read(&app, |page, ctx| {
            let appearance = Appearance::as_ref(ctx);
            let ai_settings = crate::settings::AISettings::as_ref(ctx);
            let agents_widget = AgentsWidget::default();

            let allowlist_text = agents_widget
                .render_mcp_list(
                    "MCP allowlist",
                    "Allow the Warp Agent to call these MCP servers.",
                    &page.mcp_allowlist_dropdown,
                    Vec::new(),
                    Vec::<MouseStateHandle>::new(),
                    super::AISettingsPageAction::RemoveFromMCPAllowlist,
                    ai_settings,
                    appearance,
                    ctx,
                )
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                allowlist_text.contains("MCP 允许列表"),
                "Expected Chinese MCP allowlist label in rendered content: {allowlist_text}"
            );
            assert!(
                allowlist_text.contains("允许 Warp Agent 调用这些 MCP 服务器"),
                "Expected Chinese MCP allowlist description in rendered content: {allowlist_text}"
            );

            let denylist_text = agents_widget
                .render_mcp_list(
                    "MCP denylist",
                    "The Warp Agent will always ask for permission before calling any MCP servers on this list.",
                    &page.mcp_denylist_dropdown,
                    Vec::new(),
                    Vec::<MouseStateHandle>::new(),
                    super::AISettingsPageAction::RemoveFromMCPDenylist,
                    ai_settings,
                    appearance,
                    ctx,
                )
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                denylist_text.contains("MCP 拒绝列表"),
                "Expected Chinese MCP denylist label in rendered content: {denylist_text}"
            );
            assert!(
                denylist_text.contains("Warp Agent 调用此列表中的任何 MCP 服务器前都会请求权限"),
                "Expected Chinese MCP denylist description in rendered content: {denylist_text}"
            );
        });
    })
}

#[test]
fn respect_user_setting_returns_user_pref_unlocked() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        true,
        true,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: false,
            is_disabled: false,
        }
    );
}

#[test]
fn respect_user_setting_with_user_off_returns_unchecked_unlocked() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        false,
        true,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: false,
            is_forced_by_org: false,
            is_disabled: false,
        }
    );
}

#[test]
fn team_enable_locks_toggle_on_regardless_of_user_pref() {
    let state = derive_agent_attribution_toggle_state(&AdminEnablementSetting::Enable, false, true);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}

#[test]
fn team_disable_locks_toggle_off_regardless_of_user_pref() {
    let state = derive_agent_attribution_toggle_state(&AdminEnablementSetting::Disable, true, true);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: false,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}

#[test]
fn ai_globally_disabled_marks_toggle_disabled_but_not_forced() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        true,
        false,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: false,
            is_disabled: true,
        }
    );
}

#[test]
fn team_force_takes_precedence_over_global_ai_disabled() {
    let state =
        derive_agent_attribution_toggle_state(&AdminEnablementSetting::Enable, false, false);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}
