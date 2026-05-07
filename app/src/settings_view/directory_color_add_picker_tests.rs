use std::collections::HashMap;
use std::path::{Path, PathBuf};

use warp_core::ui::theme::AnsiColorIdentifier;

use super::compute_candidate_paths;
use crate::{
    localization::{localized_settings_text, UiLanguage},
    settings::AppLocalizationSettings,
    test_util::settings::initialize_settings_for_tests,
    workspace::tab_settings::{DirectoryTabColor, DirectoryTabColors},
};
use settings::Setting;
use warpui::{App, SingletonEntity};

fn colors(entries: &[(&str, DirectoryTabColor)]) -> DirectoryTabColors {
    let map: HashMap<String, DirectoryTabColor> = entries
        .iter()
        .map(|(path, color)| ((*path).to_string(), *color))
        .collect();
    DirectoryTabColors(map)
}

fn all_exist(_: &Path) -> bool {
    true
}

#[test]
fn test_union_dedupes_across_sources() {
    let indexed = vec![PathBuf::from("/nonexistent/repo_a")];
    let persisted = vec![PathBuf::from("/nonexistent/repo_a")];
    let existing = DirectoryTabColors::default();

    let candidates = compute_candidate_paths(indexed, persisted, &existing, all_exist);

    assert_eq!(candidates, vec![PathBuf::from("/nonexistent/repo_a")]);
}

#[test]
fn test_filters_out_existing_non_suppressed_entries() {
    let indexed = vec![
        PathBuf::from("/nonexistent/unassigned"),
        PathBuf::from("/nonexistent/colored"),
        PathBuf::from("/nonexistent/fresh"),
    ];
    let existing = colors(&[
        ("/nonexistent/unassigned", DirectoryTabColor::Unassigned),
        (
            "/nonexistent/colored",
            DirectoryTabColor::Color(AnsiColorIdentifier::Red),
        ),
    ]);

    let candidates = compute_candidate_paths(indexed, Vec::<PathBuf>::new(), &existing, all_exist);

    assert_eq!(candidates, vec![PathBuf::from("/nonexistent/fresh")]);
}

#[test]
fn test_retains_suppressed_entries_as_candidates() {
    let indexed = vec![PathBuf::from("/nonexistent/suppressed_repo")];
    let existing = colors(&[(
        "/nonexistent/suppressed_repo",
        DirectoryTabColor::Suppressed,
    )]);

    let candidates = compute_candidate_paths(indexed, Vec::<PathBuf>::new(), &existing, all_exist);

    assert_eq!(
        candidates,
        vec![PathBuf::from("/nonexistent/suppressed_repo")]
    );
}

#[test]
fn test_non_existent_paths_are_dropped() {
    let indexed = vec![
        PathBuf::from("/nonexistent/a"),
        PathBuf::from("/nonexistent/b"),
    ];
    let existing = DirectoryTabColors::default();

    let candidates = compute_candidate_paths(indexed, Vec::<PathBuf>::new(), &existing, |p| {
        p == Path::new("/nonexistent/b")
    });

    assert_eq!(candidates, vec![PathBuf::from("/nonexistent/b")]);
}

#[test]
fn test_worktree_paths_are_kept() {
    let indexed = vec![
        PathBuf::from("/users/alice/.warp-dev/worktrees/warp-internal/feature_a"),
        PathBuf::from("/users/alice/.warp-dev/worktrees/warp-internal/feature_b"),
        PathBuf::from("/users/alice/code/primary-repo"),
    ];
    let existing = DirectoryTabColors::default();

    let candidates = compute_candidate_paths(indexed, Vec::<PathBuf>::new(), &existing, all_exist);

    assert_eq!(
        candidates,
        vec![
            PathBuf::from("/users/alice/.warp-dev/worktrees/warp-internal/feature_a"),
            PathBuf::from("/users/alice/.warp-dev/worktrees/warp-internal/feature_b"),
            PathBuf::from("/users/alice/code/primary-repo"),
        ]
    );
}

#[test]
fn test_results_are_sorted_alphabetically_by_canonical_key() {
    let indexed = vec![
        PathBuf::from("/nonexistent/zulu"),
        PathBuf::from("/nonexistent/alpha"),
    ];
    let persisted = vec![PathBuf::from("/nonexistent/mango")];
    let existing = DirectoryTabColors::default();

    let candidates = compute_candidate_paths(indexed, persisted, &existing, all_exist);

    assert_eq!(
        candidates,
        vec![
            PathBuf::from("/nonexistent/alpha"),
            PathBuf::from("/nonexistent/mango"),
            PathBuf::from("/nonexistent/zulu"),
        ]
    );
}

#[test]
fn test_empty_inputs_produce_empty_output() {
    let existing = DirectoryTabColors::default();

    let candidates = compute_candidate_paths(
        Vec::<PathBuf>::new(),
        Vec::<PathBuf>::new(),
        &existing,
        all_exist,
    );

    assert!(candidates.is_empty());
}

#[test]
fn test_static_copy_resolves_to_simplified_chinese() {
    App::test((), |mut app| async move {
        initialize_settings_for_tests(&mut app);

        app.update(|ctx| {
            AppLocalizationSettings::handle(ctx).update(ctx, |settings, ctx| {
                settings
                    .selected_ui_language
                    .set_value(UiLanguage::ChineseSimplified, ctx)
                    .unwrap();
            });

            assert_eq!(
                localized_settings_text("Add directory color", ctx),
                "添加目录颜色"
            );
            assert_eq!(
                localized_settings_text("+ Add directory…", ctx),
                "+ 添加目录…"
            );
        });
    })
}
