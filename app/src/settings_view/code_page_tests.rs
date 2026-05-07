use super::{CodePageWidget, CodeSettingsPageView, InitializedFoldersMouseStates};
use crate::ai::persisted_workspace::{EnablementState, PersistedWorkspace};
use crate::ai::request_usage_model::AIRequestUsageModel;
use crate::auth::AuthStateProvider;
use crate::localization::UiLanguage;
use crate::network::NetworkStatus;
use crate::server::server_api::ServerApiProvider;
use crate::settings::{AppLocalizationSettings, PrivacySettings};
use crate::test_util::settings::initialize_settings_for_tests;
use crate::view_components::action_button::{ActionButton, SecondaryTheme};
use crate::workspaces::user_workspaces::UserWorkspaces;
use ai::index::full_source_code_embedding::manager::CodebaseIndexManager;
use ai::project_context::model::ProjectContextModel;
use lsp::supported_servers::LSPServerType;
use lsp::LspManagerModel;
use settings::Setting;
use std::path::PathBuf;
use warp_core::ui::appearance::Appearance;
use warpui::platform::WindowStyle;
use warpui::ui_components::switch::SwitchStateHandle;
use warpui::{App, SingletonEntity};

fn init_code_page_test_models(app: &mut App) {
    initialize_settings_for_tests(app);
    app.add_singleton_model(|_| ServerApiProvider::new_for_test());
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(|_| Appearance::mock());
    app.add_singleton_model(UserWorkspaces::default_mock);
    app.add_singleton_model(PrivacySettings::mock);
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(|ctx| {
        AIRequestUsageModel::new_for_test(ServerApiProvider::as_ref(ctx).get_ai_client(), ctx)
    });
    app.add_singleton_model(|ctx| {
        CodebaseIndexManager::new_for_test(ServerApiProvider::as_ref(ctx).get(), ctx)
    });
    app.add_singleton_model(PersistedWorkspace::new_for_test);
    app.add_singleton_model(|_| LspManagerModel::new());
    app.add_singleton_model(|_| ProjectContextModel::default());
}

#[test]
fn code_settings_page_renders_simplified_chinese_static_copy() {
    App::test((), |mut app| async move {
        init_code_page_test_models(&mut app);

        let (_window_id, page) = app.add_window(WindowStyle::NotStealFocus, |ctx| {
            CodeSettingsPageView::new(ctx)
        });

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });
        });

        page.update(&mut app, |page, ctx| {
            let page_text = page
                .page
                .render_page(page, ctx)
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                page_text.contains("代码库索引"),
                "Expected Chinese Codebase Indexing category title in rendered content: {page_text}"
            );

            let manual_add_directory_button =
                ctx.add_typed_action_view(|_| ActionButton::new("索引新文件夹", SecondaryTheme));
            let widget = CodePageWidget {
                switch_state: SwitchStateHandle::default(),
                auto_index_switch_state: SwitchStateHandle::default(),
                manual_add_directory_button,
            };
            let text_content = widget
                .render_initialized_folders(
                    InitializedFoldersMouseStates {
                        codebase_manual_resync: Vec::new(),
                        codebase_delete: Vec::new(),
                        lsp_rows: Vec::new(),
                        open_project_rules: Vec::new(),
                    },
                    &page.suggested_server_statuses,
                    Appearance::as_ref(ctx),
                    ctx,
                )
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                text_content.contains("已初始化/已索引文件夹"),
                "Expected Chinese initialized folders header in rendered content: {text_content}"
            );
            assert!(
                text_content.contains("尚未初始化任何文件夹"),
                "Expected Chinese empty state in rendered content: {text_content}"
            );

            let lsp_text = widget
                .render_lsp_servers_subsection(
                    &PathBuf::from("/tmp/warp-lsp-test"),
                    &[(LSPServerType::RustAnalyzer, EnablementState::No)],
                    LspManagerModel::as_ref(ctx),
                    vec![Default::default()],
                    &page.suggested_server_statuses,
                    Appearance::as_ref(ctx),
                    ctx,
                )
                .debug_text_content()
                .unwrap_or_default();
            assert!(
                lsp_text.contains("LSP 服务器"),
                "Expected Chinese LSP servers header in rendered content: {lsp_text}"
            );
        });
    })
}

#[test]
fn code_subpage_title_resolves_to_simplified_chinese() {
    App::test((), |mut app| async move {
        init_code_page_test_models(&mut app);

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });

            assert_eq!(
                crate::localization::localized_settings_text("Editor and Code Review", ctx),
                "编辑器与代码审查"
            );
            assert_eq!(
                crate::localization::localized_settings_text("Code Editor and Review", ctx),
                "代码编辑器与审查"
            );
        });
    })
}
