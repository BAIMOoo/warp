use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use warpui::{AppContext, SingletonEntity};

use crate::settings::AppLocalizationSettings;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[serde(rename_all = "snake_case")]
#[schemars(
    description = "The language used for application UI.",
    rename_all = "snake_case"
)]
pub enum UiLanguage {
    #[default]
    English,
    ChineseSimplified,
}

impl UiLanguage {
    pub const ALL: &'static [Self] = &[Self::English, Self::ChineseSimplified];

    pub fn display_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::ChineseSimplified => "中文",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiStringKey {
    SettingsSectionAbout,
    SettingsSectionAccount,
    SettingsSectionMcpServers,
    SettingsSectionBillingAndUsage,
    SettingsSectionAppearance,
    SettingsSectionFeatures,
    SettingsSectionKeybindings,
    SettingsSectionPrivacy,
    SettingsSectionReferrals,
    SettingsSectionSharedBlocks,
    SettingsSectionTeams,
    SettingsSectionWarpDrive,
    SettingsSectionWarpify,
    SettingsSectionWarpAgent,
    SettingsSectionAgentProfiles,
    SettingsSectionAgentMcpServers,
    SettingsSectionKnowledge,
    SettingsSectionThirdPartyCliAgents,
    SettingsSectionCodeIndexing,
    SettingsSectionEditorAndCodeReview,
    SettingsSectionCloudEnvironments,
    SettingsSectionOzCloudApiKeys,
    SettingsUmbrellaAgents,
    SettingsUmbrellaCode,
    SettingsUmbrellaCloudPlatform,
    SettingsSearchPlaceholder,
    SettingsSearchNoResultsTitle,
    SettingsSearchNoResultsDescription,
    SettingsAccountPageTitle,
    SettingsAccountUiLanguageLabel,
    SettingsAccountUiLanguageDescription,
    SettingsAccountSignUp,
    SettingsAccountFreePlan,
    SettingsAccountComparePlans,
    SettingsAccountContactSupport,
    SettingsAccountManageBilling,
    SettingsAccountUpgradeToTurboPlan,
    SettingsAccountUpgradeToLightspeedPlan,
    SettingsAccountSettingsSync,
    SettingsAccountReferralCta,
    SettingsAccountReferAFriend,
    SettingsAccountVersionLabel,
    SettingsAccountVersionUpToDate,
    SettingsAccountVersionCheckForUpdates,
    SettingsAccountVersionCheckingForUpdate,
    SettingsAccountVersionDownloadingUpdate,
    SettingsAccountVersionUpdateAvailable,
    SettingsAccountVersionRelaunchWarp,
    SettingsAccountVersionUpdating,
    SettingsAccountVersionInstalledUpdate,
    SettingsAccountVersionUnableToInstallUpdate,
    SettingsAccountVersionUnableToLaunchUpdate,
    SettingsAccountVersionUpdateManually,
    SettingsAccountLogOut,
    SettingsDialogCancel,
    SettingsDeleteEnvironmentConfirmButton,
    SettingsDeleteEnvironmentTitle,
    SettingsTransferOwnershipTitle,
    SettingsTransferOwnershipConfirmButton,
    SettingsReferralsPageTitle,
}

#[cfg(test)]
const ALL_UI_STRING_KEYS: &[UiStringKey] = &[
    UiStringKey::SettingsSectionAbout,
    UiStringKey::SettingsSectionAccount,
    UiStringKey::SettingsSectionMcpServers,
    UiStringKey::SettingsSectionBillingAndUsage,
    UiStringKey::SettingsSectionAppearance,
    UiStringKey::SettingsSectionFeatures,
    UiStringKey::SettingsSectionKeybindings,
    UiStringKey::SettingsSectionPrivacy,
    UiStringKey::SettingsSectionReferrals,
    UiStringKey::SettingsSectionSharedBlocks,
    UiStringKey::SettingsSectionTeams,
    UiStringKey::SettingsSectionWarpDrive,
    UiStringKey::SettingsSectionWarpify,
    UiStringKey::SettingsSectionWarpAgent,
    UiStringKey::SettingsSectionAgentProfiles,
    UiStringKey::SettingsSectionAgentMcpServers,
    UiStringKey::SettingsSectionKnowledge,
    UiStringKey::SettingsSectionThirdPartyCliAgents,
    UiStringKey::SettingsSectionCodeIndexing,
    UiStringKey::SettingsSectionEditorAndCodeReview,
    UiStringKey::SettingsSectionCloudEnvironments,
    UiStringKey::SettingsSectionOzCloudApiKeys,
    UiStringKey::SettingsUmbrellaAgents,
    UiStringKey::SettingsUmbrellaCode,
    UiStringKey::SettingsUmbrellaCloudPlatform,
    UiStringKey::SettingsSearchPlaceholder,
    UiStringKey::SettingsSearchNoResultsTitle,
    UiStringKey::SettingsSearchNoResultsDescription,
    UiStringKey::SettingsAccountPageTitle,
    UiStringKey::SettingsAccountUiLanguageLabel,
    UiStringKey::SettingsAccountUiLanguageDescription,
    UiStringKey::SettingsAccountSignUp,
    UiStringKey::SettingsAccountFreePlan,
    UiStringKey::SettingsAccountComparePlans,
    UiStringKey::SettingsAccountContactSupport,
    UiStringKey::SettingsAccountManageBilling,
    UiStringKey::SettingsAccountUpgradeToTurboPlan,
    UiStringKey::SettingsAccountUpgradeToLightspeedPlan,
    UiStringKey::SettingsAccountSettingsSync,
    UiStringKey::SettingsAccountReferralCta,
    UiStringKey::SettingsAccountReferAFriend,
    UiStringKey::SettingsAccountVersionLabel,
    UiStringKey::SettingsAccountVersionUpToDate,
    UiStringKey::SettingsAccountVersionCheckForUpdates,
    UiStringKey::SettingsAccountVersionCheckingForUpdate,
    UiStringKey::SettingsAccountVersionDownloadingUpdate,
    UiStringKey::SettingsAccountVersionUpdateAvailable,
    UiStringKey::SettingsAccountVersionRelaunchWarp,
    UiStringKey::SettingsAccountVersionUpdating,
    UiStringKey::SettingsAccountVersionInstalledUpdate,
    UiStringKey::SettingsAccountVersionUnableToInstallUpdate,
    UiStringKey::SettingsAccountVersionUnableToLaunchUpdate,
    UiStringKey::SettingsAccountVersionUpdateManually,
    UiStringKey::SettingsAccountLogOut,
    UiStringKey::SettingsDialogCancel,
    UiStringKey::SettingsDeleteEnvironmentConfirmButton,
    UiStringKey::SettingsDeleteEnvironmentTitle,
    UiStringKey::SettingsTransferOwnershipTitle,
    UiStringKey::SettingsTransferOwnershipConfirmButton,
    UiStringKey::SettingsReferralsPageTitle,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalizedText {
    Static(&'static str),
    Key(UiStringKey),
}

impl LocalizedText {
    pub fn resolve(self, app: &AppContext) -> &'static str {
        match self {
            Self::Static(text) => text,
            Self::Key(key) => localized_for_app(key, app),
        }
    }

    pub fn resolve_settings(self, app: &AppContext) -> &'static str {
        match self {
            Self::Static(text) => localized_settings_text(text, app),
            Self::Key(key) => localized_for_app(key, app),
        }
    }
}

impl From<&'static str> for LocalizedText {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl From<UiStringKey> for LocalizedText {
    fn from(value: UiStringKey) -> Self {
        Self::Key(value)
    }
}

pub fn localized_for_app(key: UiStringKey, app: &AppContext) -> &'static str {
    localized(
        key,
        *AppLocalizationSettings::as_ref(app).selected_ui_language,
    )
}

pub fn localized(key: UiStringKey, language: UiLanguage) -> &'static str {
    localized_with_chinese_lookup(key, language, chinese_simplified)
}

pub fn localized_settings_text(text: &'static str, app: &AppContext) -> &'static str {
    match *AppLocalizationSettings::as_ref(app).selected_ui_language {
        UiLanguage::English => text,
        UiLanguage::ChineseSimplified => chinese_simplified_settings_text(text).unwrap_or(text),
    }
}

pub fn localized_settings_text_for_str<'a>(text: &'a str, app: &AppContext) -> Cow<'a, str> {
    match *AppLocalizationSettings::as_ref(app).selected_ui_language {
        UiLanguage::English => Cow::Borrowed(text),
        UiLanguage::ChineseSimplified => chinese_simplified_settings_text_for_str(text)
            .map(Cow::Borrowed)
            .or_else(|| {
                text.strip_prefix("Slash command: ")
                    .map(|command| Cow::Owned(format!("Slash 命令：{command}")))
            })
            .unwrap_or(Cow::Borrowed(text)),
    }
}

fn chinese_simplified_settings_text_for_str(text: &str) -> Option<&'static str> {
    CHINESE_SIMPLIFIED_SETTINGS_TEXT
        .iter()
        .find_map(|(english, chinese)| (*english == text).then_some(*chinese))
        .or_else(|| {
            CHINESE_SIMPLIFIED_SETTINGS_TEXT
                .iter()
                .find_map(|(english, chinese)| {
                    english.eq_ignore_ascii_case(text).then_some(*chinese)
                })
        })
}

fn localized_with_chinese_lookup(
    key: UiStringKey,
    language: UiLanguage,
    chinese_lookup: impl Fn(UiStringKey) -> Option<&'static str>,
) -> &'static str {
    match language {
        UiLanguage::English => english(key),
        UiLanguage::ChineseSimplified => chinese_lookup(key).unwrap_or_else(|| english(key)),
    }
}

pub fn localized_delete_environment_confirmation_description(
    environment_name: &str,
    app: &AppContext,
) -> String {
    match *AppLocalizationSettings::as_ref(app).selected_ui_language {
        UiLanguage::English => format!(
            "Are you sure you want to remove the {} environment?",
            environment_name
        ),
        UiLanguage::ChineseSimplified => format!("确定要移除 {} 环境吗？", environment_name),
    }
}

pub fn localized_transfer_ownership_confirmation_description(
    new_owner_email: &str,
    app: &AppContext,
) -> String {
    match *AppLocalizationSettings::as_ref(app).selected_ui_language {
        UiLanguage::English => format!(
            "Are you sure you want to transfer team ownership to {}? You will no longer be the owner and will not be able to take any administrative actions for this team.",
            new_owner_email
        ),
        UiLanguage::ChineseSimplified => format!(
            "确定要将团队所有权转让给 {} 吗？转让后你将不再是所有者，也无法对此团队执行任何管理操作。",
            new_owner_email
        ),
    }
}

pub fn localized_referral_invalid_email_message(invalid_email: &str, app: &AppContext) -> String {
    match *AppLocalizationSettings::as_ref(app).selected_ui_language {
        UiLanguage::English => {
            format!("Please ensure the following email is valid: {invalid_email}")
        }
        UiLanguage::ChineseSimplified => format!("请确认以下电子邮件地址有效：{invalid_email}"),
    }
}

fn english(key: UiStringKey) -> &'static str {
    match key {
        UiStringKey::SettingsSectionAbout => "About",
        UiStringKey::SettingsSectionAccount => "Account",
        UiStringKey::SettingsSectionMcpServers => "MCP Servers",
        UiStringKey::SettingsSectionBillingAndUsage => "Billing and usage",
        UiStringKey::SettingsSectionAppearance => "Appearance",
        UiStringKey::SettingsSectionFeatures => "Features",
        UiStringKey::SettingsSectionKeybindings => "Keyboard shortcuts",
        UiStringKey::SettingsSectionPrivacy => "Privacy",
        UiStringKey::SettingsSectionReferrals => "Referrals",
        UiStringKey::SettingsSectionSharedBlocks => "Shared blocks",
        UiStringKey::SettingsSectionTeams => "Teams",
        UiStringKey::SettingsSectionWarpDrive => "Warp Drive",
        UiStringKey::SettingsSectionWarpify => "Warpify",
        UiStringKey::SettingsSectionWarpAgent => "Warp Agent",
        UiStringKey::SettingsSectionAgentProfiles => "Profiles",
        UiStringKey::SettingsSectionAgentMcpServers => "MCP servers",
        UiStringKey::SettingsSectionKnowledge => "Knowledge",
        UiStringKey::SettingsSectionThirdPartyCliAgents => "Third party CLI agents",
        UiStringKey::SettingsSectionCodeIndexing => "Indexing and projects",
        UiStringKey::SettingsSectionEditorAndCodeReview => "Editor and Code Review",
        UiStringKey::SettingsSectionCloudEnvironments => "Environments",
        UiStringKey::SettingsSectionOzCloudApiKeys => "Oz Cloud API Keys",
        UiStringKey::SettingsUmbrellaAgents => "Agents",
        UiStringKey::SettingsUmbrellaCode => "Code",
        UiStringKey::SettingsUmbrellaCloudPlatform => "Cloud platform",
        UiStringKey::SettingsSearchPlaceholder => "Search",
        UiStringKey::SettingsSearchNoResultsTitle => "No settings match your search.",
        UiStringKey::SettingsSearchNoResultsDescription => {
            "You may want to try using different keywords or checking for any possible typos."
        }
        UiStringKey::SettingsAccountPageTitle => "Account",
        UiStringKey::SettingsAccountUiLanguageLabel => "App language",
        UiStringKey::SettingsAccountUiLanguageDescription => {
            "Choose the language used for Warp's application interface."
        }
        UiStringKey::SettingsAccountSignUp => "Sign up",
        UiStringKey::SettingsAccountFreePlan => "Free",
        UiStringKey::SettingsAccountComparePlans => "Compare plans",
        UiStringKey::SettingsAccountContactSupport => "Contact support",
        UiStringKey::SettingsAccountManageBilling => "Manage billing",
        UiStringKey::SettingsAccountUpgradeToTurboPlan => "Upgrade to Turbo plan",
        UiStringKey::SettingsAccountUpgradeToLightspeedPlan => "Upgrade to Lightspeed plan",
        UiStringKey::SettingsAccountSettingsSync => "Settings sync",
        UiStringKey::SettingsAccountReferralCta => {
            "Earn rewards by sharing Warp with friends & colleagues"
        }
        UiStringKey::SettingsAccountReferAFriend => "Refer a friend",
        UiStringKey::SettingsAccountVersionLabel => "Version",
        UiStringKey::SettingsAccountVersionUpToDate => "Up to date",
        UiStringKey::SettingsAccountVersionCheckForUpdates => "Check for updates",
        UiStringKey::SettingsAccountVersionCheckingForUpdate => "checking for update...",
        UiStringKey::SettingsAccountVersionDownloadingUpdate => "downloading update...",
        UiStringKey::SettingsAccountVersionUpdateAvailable => "Update available",
        UiStringKey::SettingsAccountVersionRelaunchWarp => "Relaunch Warp",
        UiStringKey::SettingsAccountVersionUpdating => "Updating...",
        UiStringKey::SettingsAccountVersionInstalledUpdate => "Installed update",
        UiStringKey::SettingsAccountVersionUnableToInstallUpdate => {
            "A new version of Warp is available but can't be installed"
        }
        UiStringKey::SettingsAccountVersionUnableToLaunchUpdate => {
            "A new version of Warp is installed but can't be launched."
        }
        UiStringKey::SettingsAccountVersionUpdateManually => "Update Warp manually",
        UiStringKey::SettingsAccountLogOut => "Log out",
        UiStringKey::SettingsDialogCancel => "Cancel",
        UiStringKey::SettingsDeleteEnvironmentConfirmButton => "Delete environment",
        UiStringKey::SettingsDeleteEnvironmentTitle => "Delete environment?",
        UiStringKey::SettingsTransferOwnershipTitle => "Transfer team ownership?",
        UiStringKey::SettingsTransferOwnershipConfirmButton => "Transfer",
        UiStringKey::SettingsReferralsPageTitle => "Invite a friend to Warp",
    }
}

fn chinese_simplified(key: UiStringKey) -> Option<&'static str> {
    CHINESE_SIMPLIFIED_STRINGS
        .iter()
        .find_map(|(candidate, value)| (*candidate == key).then_some(*value))
}

fn chinese_simplified_settings_text(text: &str) -> Option<&'static str> {
    CHINESE_SIMPLIFIED_SETTINGS_TEXT
        .iter()
        .find_map(|(candidate, value)| (*candidate == text).then_some(*value))
}

const CHINESE_SIMPLIFIED_STRINGS: &[(UiStringKey, &str)] = &[
    (UiStringKey::SettingsSectionAbout, "关于"),
    (UiStringKey::SettingsSectionAccount, "账户"),
    (UiStringKey::SettingsSectionMcpServers, "MCP 服务器"),
    (UiStringKey::SettingsSectionBillingAndUsage, "账单与用量"),
    (UiStringKey::SettingsSectionAppearance, "外观"),
    (UiStringKey::SettingsSectionFeatures, "功能"),
    (UiStringKey::SettingsSectionKeybindings, "键盘快捷键"),
    (UiStringKey::SettingsSectionPrivacy, "隐私"),
    (UiStringKey::SettingsSectionReferrals, "推荐"),
    (UiStringKey::SettingsSectionSharedBlocks, "共享块"),
    (UiStringKey::SettingsSectionTeams, "团队"),
    (UiStringKey::SettingsSectionWarpDrive, "Warp Drive"),
    (UiStringKey::SettingsSectionWarpify, "Warpify"),
    (UiStringKey::SettingsSectionWarpAgent, "Warp Agent"),
    (UiStringKey::SettingsSectionAgentProfiles, "配置文件"),
    (UiStringKey::SettingsSectionAgentMcpServers, "MCP 服务器"),
    (UiStringKey::SettingsSectionKnowledge, "知识库"),
    (
        UiStringKey::SettingsSectionThirdPartyCliAgents,
        "第三方 CLI Agent",
    ),
    (UiStringKey::SettingsSectionCodeIndexing, "索引与项目"),
    (
        UiStringKey::SettingsSectionEditorAndCodeReview,
        "编辑器与代码审查",
    ),
    (UiStringKey::SettingsSectionCloudEnvironments, "环境"),
    (
        UiStringKey::SettingsSectionOzCloudApiKeys,
        "Oz Cloud API 密钥",
    ),
    (UiStringKey::SettingsUmbrellaAgents, "Agent"),
    (UiStringKey::SettingsUmbrellaCode, "代码"),
    (UiStringKey::SettingsUmbrellaCloudPlatform, "云平台"),
    (UiStringKey::SettingsSearchPlaceholder, "搜索"),
    (
        UiStringKey::SettingsSearchNoResultsTitle,
        "没有匹配的设置。",
    ),
    (
        UiStringKey::SettingsSearchNoResultsDescription,
        "请尝试使用其他关键词，或检查是否有拼写错误。",
    ),
    (UiStringKey::SettingsAccountPageTitle, "账户"),
    (UiStringKey::SettingsAccountUiLanguageLabel, "应用语言"),
    (
        UiStringKey::SettingsAccountUiLanguageDescription,
        "选择 Warp 应用界面使用的语言。",
    ),
    (UiStringKey::SettingsAccountSignUp, "注册"),
    (UiStringKey::SettingsAccountFreePlan, "免费"),
    (UiStringKey::SettingsAccountComparePlans, "比较套餐"),
    (UiStringKey::SettingsAccountContactSupport, "联系支持"),
    (UiStringKey::SettingsAccountManageBilling, "管理账单"),
    (
        UiStringKey::SettingsAccountUpgradeToTurboPlan,
        "升级到 Turbo 套餐",
    ),
    (
        UiStringKey::SettingsAccountUpgradeToLightspeedPlan,
        "升级到 Lightspeed 套餐",
    ),
    (UiStringKey::SettingsAccountSettingsSync, "设置同步"),
    (
        UiStringKey::SettingsAccountReferralCta,
        "与朋友和同事分享 Warp 以获得奖励",
    ),
    (UiStringKey::SettingsAccountReferAFriend, "推荐朋友"),
    (UiStringKey::SettingsAccountVersionLabel, "版本"),
    (UiStringKey::SettingsAccountVersionUpToDate, "已是最新版本"),
    (
        UiStringKey::SettingsAccountVersionCheckForUpdates,
        "检查更新",
    ),
    (
        UiStringKey::SettingsAccountVersionCheckingForUpdate,
        "正在检查更新...",
    ),
    (
        UiStringKey::SettingsAccountVersionDownloadingUpdate,
        "正在下载更新...",
    ),
    (
        UiStringKey::SettingsAccountVersionUpdateAvailable,
        "有可用更新",
    ),
    (
        UiStringKey::SettingsAccountVersionRelaunchWarp,
        "重新启动 Warp",
    ),
    (UiStringKey::SettingsAccountVersionUpdating, "正在更新..."),
    (
        UiStringKey::SettingsAccountVersionInstalledUpdate,
        "更新已安装",
    ),
    (
        UiStringKey::SettingsAccountVersionUnableToInstallUpdate,
        "Warp 有新版本可用，但无法安装",
    ),
    (
        UiStringKey::SettingsAccountVersionUnableToLaunchUpdate,
        "Warp 新版本已安装，但无法启动。",
    ),
    (
        UiStringKey::SettingsAccountVersionUpdateManually,
        "手动更新 Warp",
    ),
    (UiStringKey::SettingsAccountLogOut, "退出登录"),
    (UiStringKey::SettingsDialogCancel, "取消"),
    (
        UiStringKey::SettingsDeleteEnvironmentConfirmButton,
        "删除环境",
    ),
    (UiStringKey::SettingsDeleteEnvironmentTitle, "删除环境？"),
    (
        UiStringKey::SettingsTransferOwnershipTitle,
        "转让团队所有权？",
    ),
    (UiStringKey::SettingsTransferOwnershipConfirmButton, "转让"),
    (UiStringKey::SettingsReferralsPageTitle, "邀请朋友使用 Warp"),
];

const CHINESE_SIMPLIFIED_SETTINGS_TEXT: &[(&str, &str)] = &[
    ("General", "通用"),
    ("Session", "会话"),
    ("Keys", "按键"),
    ("Text Editing", "文本编辑"),
    ("Terminal Input", "终端输入"),
    ("Terminal", "终端"),
    ("Notifications", "通知"),
    ("Workflows", "工作流"),
    ("System", "系统"),
    ("Window", "窗口"),
    ("Input", "输入"),
    ("Blocks", "块"),
    ("Text", "文本"),
    ("Tabs", "标签页"),
    ("Full-screen Apps", "全屏应用"),
    ("Privacy", "隐私"),
    ("Subshells", "子 shell"),
    (
        "Subshells supported: bash, zsh, and fish.",
        "支持的子 shell：bash、zsh 和 fish。",
    ),
    ("SSH", "SSH"),
    (
        "Warpify your interactive SSH sessions.",
        "为你的交互式 SSH 会话启用 Warpify。",
    ),
    ("Warpify", "Warpify"),
    ("Billing and usage", "账单与用量"),
    ("Codebase Indexing", "代码库索引"),
    ("Codebase indexing", "代码库索引"),
    ("Code Editor and Review", "代码编辑器与审查"),
    ("Editor and Code Review", "编辑器与代码审查"),
    ("Shared blocks", "共享块"),
    ("Reset to default", "重置为默认值"),
    ("Import", "导入"),
    ("Reset to Warp defaults", "重置为 Warp 默认值"),
    (
        "Select a settings profile to import:",
        "选择要导入的设置配置文件：",
    ),
    ("Looking for settings to import...", "正在查找可导入的设置..."),
    (
        "Some settings will take effect when you open a new session.",
        "部分设置会在你打开新会话时生效。",
    ),
    ("Theme", "主题"),
    ("Theme,", "主题，"),
    ("1 other setting", "1 项其他设置"),
    ("{count} other settings", "{count} 项其他设置"),
    ("Option as Meta", "Option 作为 Meta"),
    ("Mouse/Scroll Reporting", "鼠标/滚动报告"),
    ("Font", "字体"),
    ("Default Shell", "默认 shell"),
    ("Working Directory", "工作目录"),
    ("Global hotkey", "全局热键"),
    ("Window Dimensions", "窗口尺寸"),
    ("Copy On Select", "选中时复制"),
    ("Window Opacity", "窗口不透明度"),
    ("Cursor Blinking", "光标闪烁"),
    ("Add directory color", "添加目录颜色"),
    ("+ Add directory…", "+ 添加目录…"),
    ("Open settings file", "打开设置文件"),
    ("Open file", "打开文件"),
    ("Fix with Oz", "用 Oz 修复"),
    ("Search repos", "搜索仓库"),
    ("Search tabs...", "搜索标签页..."),
    (" + Add new repo", " + 添加新仓库"),
    (
        "Search sessions, agents, files...",
        "搜索会话、Agent、文件...",
    ),
    ("Search Warp Drive", "搜索 Warp Drive"),
    ("Split pane right", "向右拆分窗格"),
    ("Split pane left", "向左拆分窗格"),
    ("Split pane down", "向下拆分窗格"),
    ("Split pane up", "向上拆分窗格"),
    ("Close pane", "关闭窗格"),
    (
        "Your settings file contains an error.",
        "你的设置文件包含错误。",
    ),
    (
        "Your settings file contains errors.",
        "你的设置文件包含多个错误。",
    ),
    (
        "Couldn't parse due to invalid syntax",
        "因语法无效而无法解析",
    ),
    ("Open the file to fix it.", "打开文件进行修复。"),
    ("Invalid value for", "无效值："),
    ("The default value is being used.", "正在使用默认值。"),
    ("Invalid values for:", "无效值："),
    ("Default values are being used.", "正在使用默认值。"),
    ("Click to learn more in docs", "点击在文档中了解更多"),
    (
        "This setting is not synced to your other devices",
        "此设置不会同步到你的其他设备",
    ),
    (
        "To use Warp Drive, please create an account.",
        "要使用 Warp Drive，请先创建账户。",
    ),
    (
        "Warp Drive is a workspace in your terminal where you can save Workflows, Notebooks, Prompts, and Environment Variables for personal use or to share with a team.",
        "Warp Drive 是终端中的工作区，你可以在其中保存 Workflows、Notebooks、Prompts 和 Environment Variables，供个人使用或与团队共享。",
    ),
    (
        "Configure whether Warp attempts to “Warpify” (add support for blocks, input modes, etc) certain shells. ",
        "配置 Warp 是否尝试对特定 shell 执行 Warpify（添加对块、输入模式等的支持）。",
    ),
    ("Learn more", "了解更多"),
    ("Added commands", "已添加命令"),
    ("Denylisted commands", "拒绝列表中的命令"),
    ("Warpify SSH Sessions", "Warpify SSH 会话"),
    ("Install SSH extension", "安装 SSH 扩展"),
    (
        "Controls the installation behavior for Warp's SSH extension when a remote host doesn't have it installed.",
        "控制远程主机未安装 Warp SSH 扩展时的安装行为。",
    ),
    ("Use Tmux Warpification", "使用 Tmux Warpification"),
    (
        "The tmux ssh wrapper works in many situations where the default one does not, but may require you to hit a button to warpify. Takes effect in new tabs.",
        "tmux ssh 包装器可在默认方式不适用的许多场景下工作，但可能需要你点击按钮来执行 Warpify。对新标签页生效。",
    ),
    ("Denylisted hosts", "拒绝列表中的主机"),
    ("command (supports regex)", "命令（支持 regex）"),
    ("host (supports regex)", "主机（支持 regex）"),
    ("Always ask", "始终询问"),
    ("Always install", "始终安装"),
    ("Never install", "从不安装"),
    (
        "You don't have any shared blocks yet.",
        "你还没有任何共享块。",
    ),
    ("Getting blocks...", "正在获取块..."),
    (
        "Failed to load blocks. Please try again.",
        "无法加载块。请重试。",
    ),
    ("Deleting...", "正在删除..."),
    ("Unshare", "取消共享"),
    ("Unshare block", "取消共享块"),
    (
        "Are you sure you want to unshare this block?\n\nIt will no longer be accessible by link and will be permanently deleted from Warp servers.",
        "确定要取消共享此块吗？\n\n它将无法再通过链接访问，并会从 Warp 服务器永久删除。",
    ),
    ("Link copied.", "链接已复制。"),
    ("Block was successfully unshared.", "块已成功取消共享。"),
    (
        "Failed to unshare block. Please try again.",
        "无法取消共享块。请重试。",
    ),
    ("Failed to load referral code.", "无法加载推荐码。"),
    ("Copy link", "复制链接"),
    ("Send", "发送"),
    ("Sending...", "正在发送..."),
    ("Loading...", "正在加载..."),
    ("Successfully sent emails.", "邮件已成功发送。"),
    (
        "Failed to send emails. Please try again.",
        "无法发送邮件。请重试。",
    ),
    (
        "Get exclusive Warp goodies when you refer someone*",
        "推荐他人即可获得 Warp 专属好礼*",
    ),
    ("Current referral", "当前推荐"),
    ("Current referrals", "当前推荐"),
    ("Link", "链接"),
    ("Email", "电子邮件"),
    ("Please enter an email.", "请输入电子邮件地址。"),
    (
        "Sign up to participate in Warp's referral program",
        "注册以参与 Warp 推荐计划",
    ),
    ("Executed on:", "执行时间："),
    ("Exclusive theme", "专属主题"),
    ("Keycaps + stickers", "键帽 + 贴纸"),
    ("T-shirt", "T 恤"),
    ("Notebook", "笔记本"),
    ("Baseball cap", "棒球帽"),
    ("Hoodie", "连帽衫"),
    ("Premium Hydro Flask", "高端 Hydro Flask"),
    ("Backpack", "背包"),
    ("Certain restrictions apply.", "适用某些限制。"),
    (
        " If you have any questions about the referral program, please contact referrals@warp.dev.",
        " 如果你对推荐计划有任何疑问，请联系 referrals@warp.dev。",
    ),
    (
        "Search by name or by keys (ex. \"cmd d\")",
        "按名称或按键搜索（例如 \"cmd d\"）",
    ),
    (
        "This shortcut conflicts with other keybinds",
        "此快捷键与其他键绑定冲突",
    ),
    ("Default", "默认"),
    ("Clear", "清除"),
    ("Save", "保存"),
    ("Press new keyboard shortcut", "按下新的键盘快捷键"),
    (
        "Add your own custom keybindings to existing actions below.",
        "在下方为现有操作添加自定义键绑定。",
    ),
    ("Use", "使用"),
    (
        "to reference these keybindings in a side pane at anytime.",
        "可随时在侧边窗格中参考这些键绑定。",
    ),
    (
        "Keyboard shortcuts are not synced to the cloud",
        "键盘快捷键不会同步到云端",
    ),
    ("Configure keyboard shortcuts", "配置键盘快捷键"),
    ("Command", "命令"),
    // Keybindings command descriptions surfaced in Settings > Keyboard shortcuts.
    ("(Experimental) Toggle classic completions mode", "（实验性）切换经典补全模式"),
    ("Activate next pane", "激活下一个窗格"),
    ("Activate next tab", "激活下一个标签页"),
    ("Activate previous pane", "激活上一个窗格"),
    ("Activate previous tab", "激活上一个标签页"),
    ("Accept Autosuggestion", "接受自动建议"),
    ("accept autosuggestion", "接受自动建议"),
    ("Accept Prompt Suggestion", "接受提示建议"),
    ("accept prompt suggestion", "接受提示建议"),
    ("Accept Prompt Suggestions", "接受提示建议"),
    ("accept prompt suggestions", "接受提示建议"),
    ("About Warp", "关于 Warp"),
    ("Agent conversation list view", "Agent 对话列表视图"),
    ("Appearance...", "外观..."),
    ("Add current folder as project", "将当前文件夹添加为项目"),
    ("Attach Selected Block as Agent Context", "将所选块附加为 Agent 上下文"),
    ("Attach Selected Text as Agent Context", "将所选文本附加为 Agent 上下文"),
    ("Attach Selection as Agent Context", "将所选内容附加为 Agent 上下文"),
    ("Add cursor above", "在上方添加光标"),
    ("Add cursor below", "在下方添加光标"),
    ("Add repository", "添加仓库"),
    ("Add selection for next occurrence", "为下一处匹配添加选择"),
    ("Alternate terminal paste", "备用终端粘贴"),
    ("Ask Warp AI", "询问 Warp AI"),
    ("Ask Warp AI about Selection", "就所选内容询问 Warp AI"),
    ("Ask Warp AI about last block", "就上一个块询问 Warp AI"),
    ("Backward tabulation within an executing command", "在执行中的命令内反向制表"),
    ("Bookmark selected block", "为所选块添加书签"),
    ("Cancel active process", "取消活动进程"),
    ("Check for updates", "检查更新"),
    ("Clear Blocks", "清除块"),
    ("Clear and reset AI context menu query", "清除并重置 AI 上下文菜单查询"),
    ("Clear command editor", "清空命令编辑器"),
    ("Clear screen", "清屏"),
    ("Clear selected lines", "清除所选行"),
    ("Close", "关闭"),
    ("Close Current Session", "关闭当前会话"),
    ("Close Warp AI", "关闭 Warp AI"),
    ("Close Window", "关闭窗口"),
    ("Close focused panel", "关闭已聚焦面板"),
    ("Close tabs to the right", "关闭右侧标签页"),
    ("close tabs below", "关闭下方标签页"),
    ("Close all tabs", "关闭所有标签页"),
    ("Close other tabs", "关闭其他标签页"),
    ("Close saved tabs", "关闭已保存的标签页"),
    ("Close the current tab", "关闭当前标签页"),
    ("Command Palette", "命令面板"),
    ("Configure Keyboard Shortcuts...", "配置键盘快捷键..."),
    ("Configure Warpify...", "配置 Warpify..."),
    ("Command Search", "命令搜索"),
    ("Configure Global Hotkey", "配置全局热键"),
    ("Copy access token to clipboard", "将访问令牌复制到剪贴板"),
    ("Copy and clear selected lines", "复制并清除所选行"),
    ("Copy command", "复制命令"),
    ("Copy command and output", "复制命令和输出"),
    ("Copy command output", "复制命令输出"),
    ("Copy git branch", "复制 git 分支"),
    ("Copy text or cancel active process", "复制文本或取消活动进程"),
    ("Copy rich-text buffer", "复制富文本缓冲区"),
    ("Copy rich-text selection", "复制富文本选择"),
    ("Create New Window", "创建新窗口"),
    ("Create a new personal folder", "新建个人文件夹"),
    ("Create a new personal notebook", "新建个人 Notebook"),
    ("Create a new personal prompt", "新建个人 Prompt"),
    ("Create a new personal workflow", "新建个人 Workflow"),
    ("Create a new team folder", "新建团队文件夹"),
    ("Create a new team notebook", "新建团队 Notebook"),
    ("Create a new team prompt", "新建团队 Prompt"),
    ("Create a new team workflow", "新建团队 Workflow"),
    ("Create new personal environment variables", "新建个人环境变量"),
    ("Create new tab", "新建标签页"),
    ("Create new team environment variables", "新建团队环境变量"),
    ("Create new project", "创建新项目"),
    ("Create or edit link", "创建或编辑链接"),
    ("Cursor at buffer end", "光标移至缓冲区末尾"),
    ("Cursor at buffer start", "光标移至缓冲区开头"),
    ("Cut", "剪切"),
    ("Cut all left", "剪切左侧全部内容"),
    ("Cut all right", "剪切右侧全部内容"),
    ("Cut word left", "剪切左侧单词"),
    ("Cut word right", "剪切右侧单词"),
    ("De-select shell commands", "取消选择 shell 命令"),
    ("Decrease font size", "减小字体大小"),
    ("Decrease notebook font size", "减小 Notebook 字体大小"),
    ("Decrease zoom level", "降低缩放级别"),
    ("[Debug] Enter Onboarding State", "[Debug] 进入 Onboarding 状态"),
    (
        "[Debug] Log review comment send status for active tab",
        "[Debug] 记录当前标签页的 review comment 发送状态",
    ),
    (
        "[Debug] View first-time user experience",
        "[Debug] 查看首次用户体验",
    ),
    (
        "[Debug] Open Build Plan Migration Modal",
        "[Debug] 打开 Build Plan 迁移模态框",
    ),
    (
        "[Debug] Reset Build Plan Migration Modal State",
        "[Debug] 重置 Build Plan 迁移模态框状态",
    ),
    (
        "[Debug] Un-dismiss AWS login banner",
        "[Debug] 取消隐藏 AWS 登录横幅",
    ),
    ("[Debug] Open Oz Launch Modal", "[Debug] 打开 Oz 启动模态框"),
    (
        "[Debug] Reset Oz Launch Modal State",
        "[Debug] 重置 Oz 启动模态框状态",
    ),
    (
        "[Debug] Open OpenWarp Launch Modal",
        "[Debug] 打开 OpenWarp 启动模态框",
    ),
    (
        "[Debug] Reset OpenWarp Launch Modal State",
        "[Debug] 重置 OpenWarp 启动模态框状态",
    ),
    (
        "[Debug] Install OpenCode Warp plugin",
        "[Debug] 安装 OpenCode Warp 插件",
    ),
    (
        "[Debug] Use local OpenCode Warp plugin (testing only)",
        "[Debug] 使用本地 OpenCode Warp 插件（仅测试）",
    ),
    (
        "[Debug] Open Session Config Modal",
        "[Debug] 打开会话配置模态框",
    ),
    (
        "[Debug] Start HOA Onboarding Flow",
        "[Debug] 启动 HOA Onboarding 流程",
    ),
    (
        "[Debug] Onboarding Callout: WarpInput - Terminal",
        "[Debug] Onboarding 提示：WarpInput - Terminal",
    ),
    (
        "[Debug] Onboarding Callout: WarpInput - Project",
        "[Debug] Onboarding 提示：WarpInput - Project",
    ),
    (
        "[Debug] Onboarding Callout: WarpInput - No Project",
        "[Debug] Onboarding 提示：WarpInput - 无 Project",
    ),
    (
        "[Debug] Onboarding Callout: Modality - Project",
        "[Debug] Onboarding 提示：Modality - Project",
    ),
    (
        "[Debug] Onboarding Callout: Modality - No Project",
        "[Debug] Onboarding 提示：Modality - 无 Project",
    ),
    (
        "[Debug] Onboarding Callout: Modality - Terminal",
        "[Debug] Onboarding 提示：Modality - Terminal",
    ),
    (
        "[Debug] Generate codebase index",
        "[Debug] 生成代码库索引",
    ),
    ("Delete", "删除"),
    ("Delete all left", "删除左侧全部内容"),
    ("Delete all right", "删除右侧全部内容"),
    ("Delete to line end within an executing command", "在执行中的命令内删除到行尾"),
    ("Delete to line start within an executing command", "在执行中的命令内删除到行首"),
    ("Delete word left", "删除左侧单词"),
    ("Delete word left within an executing command", "在执行中的命令内删除左侧单词"),
    ("Delete word right", "删除右侧单词"),
    ("Dump debug info", "转储调试信息"),
    ("Dump heap profile (can only be done once)", "转储堆 profile（只能执行一次）"),
    ("Edit Code Diff", "编辑代码 Diff"),
    ("Edit Prompt", "编辑 Prompt"),
    ("Edit requested command", "编辑请求的命令"),
    ("End", "End"),
    ("Exit Vim insert mode", "退出 Vim 插入模式"),
    ("Expand selected blocks above", "向上展开所选块"),
    ("Expand selected blocks below", "向下展开所选块"),
    ("Export all Warp Drive objects", "导出所有 Warp Drive 对象"),
    ("Find in Notebook", "在 Notebook 中查找"),
    ("Find in Terminal", "在终端中查找"),
    ("Find in code editor", "在代码编辑器中查找"),
    ("Find the next occurrence of your search query", "查找搜索查询的下一处匹配"),
    ("Find the previous occurrence of your search query", "查找搜索查询的上一处匹配"),
    ("Find within selected block", "在所选块中查找"),
    ("Focus Terminal Input From Warp AI", "从 Warp AI 聚焦终端输入"),
    ("Focus Terminal Input from File", "从文件聚焦终端输入"),
    ("Focus Terminal Input from Notebook", "从 Notebook 聚焦终端输入"),
    ("Focus next match", "聚焦下一处匹配"),
    ("Focus previous match", "聚焦上一处匹配"),
    ("Focus terminal input", "聚焦终端输入"),
    ("Fold", "折叠"),
    ("Fold selected ranges", "折叠所选范围"),
    ("Global Search", "全局搜索"),
    ("Go to line", "跳转到行"),
    ("Hide All Windows", "隐藏所有窗口"),
    ("Hide Dedicated Hotkey Window", "隐藏专用热键窗口"),
    ("History Search", "历史搜索"),
    ("Home", "Home"),
    ("Import External Settings", "导入外部设置"),
    ("Import To Personal Drive", "导入到个人 Drive"),
    ("Import To Team Drive", "导入到团队 Drive"),
    ("Increase font size", "增大字体大小"),
    ("Increase notebook font size", "增大 Notebook 字体大小"),
    ("Increase zoom level", "提高缩放级别"),
    ("Insert Command Correction", "插入命令纠错"),
    ("Insert last word of previous command", "插入上一条命令的最后一个单词"),
    ("Insert newline", "插入换行"),
    ("Insert non-expanding space", "插入非扩展空格"),
    ("Inspect Command", "检查命令"),
    ("Install Oz CLI command", "安装 Oz CLI 命令"),
    ("Install update and relaunch", "安装更新并重新启动"),
    ("Initiate project for warp", "为 warp 初始化项目"),
    ("Invite People...", "邀请成员..."),
    ("Join our Slack community (opens external link)", "加入我们的 Slack 社区（打开外部链接）"),
    ("Jump to latest agent task", "跳转到最新 Agent 任务"),
    ("Launch configuration palette", "启动配置面板"),
    ("Left Panel: Agent conversations", "左侧面板：Agent 对话"),
    ("Left Panel: Global search", "左侧面板：全局搜索"),
    ("Left Panel: Project explorer", "左侧面板：项目浏览器"),
    ("Left Panel: Warp Drive", "左侧面板：Warp Drive"),
    ("Load agent mode conversation (from debug link in clipboard)", "加载 Agent 模式对话（来自剪贴板中的调试链接）"),
    ("Log editor state", "记录编辑器状态"),
    ("Move Backward One Subword", "向后移动一个子词"),
    ("Move tab left", "向左移动标签页"),
    ("Move tab right", "向右移动标签页"),
    ("move tab down", "向下移动标签页"),
    ("move tab up", "向上移动标签页"),
    ("Move Backward One Word", "向后移动一个单词"),
    ("Move Forward One Subword", "向前移动一个子词"),
    ("Move Forward One Word", "向前移动一个单词"),
    ("Move backward one word", "向后移动一个单词"),
    ("Move cursor down", "向下移动光标"),
    ("Move cursor end within an executing command", "在执行中的命令内将光标移至末尾"),
    ("Move cursor home within an executing command", "在执行中的命令内将光标移至开头"),
    ("Move cursor left", "向左移动光标"),
    ("Move cursor one word to the left within an executing command", "在执行中的命令内将光标向左移动一个单词"),
    ("Move cursor one word to the right within an executing command", "在执行中的命令内将光标向右移动一个单词"),
    ("Move cursor right", "向右移动光标"),
    ("Move cursor to the bottom", "将光标移至底部"),
    ("Move cursor to the top", "将光标移至顶部"),
    ("Move cursor up", "向上移动光标"),
    ("Move forward one word", "向前移动一个单词"),
    ("Move to end of line", "移至行尾"),
    ("Move to end of paragraph", "移至段落末尾"),
    ("Move to line end", "移至行尾"),
    ("Move to line start", "移至行首"),
    ("Move to start of line", "移至行首"),
    ("Move to start of paragraph", "移至段落开头"),
    ("Move to the end of the buffer", "移至缓冲区末尾"),
    ("Move to the end of the paragraph", "移至段落末尾"),
    ("Move to the start of the buffer", "移至缓冲区开头"),
    ("Move to the start of the paragraph", "移至段落开头"),
    ("Navigation Palette", "导航面板"),
    ("New Personal Environment Variables", "新建个人环境变量"),
    ("New Personal Folder", "新建个人文件夹"),
    ("New Personal Notebook", "新建个人 Notebook"),
    ("New Personal Prompt", "新建个人 Prompt"),
    ("New Personal Workflow", "新建个人 Workflow"),
    ("New Agent Tab", "新建 Agent 标签页"),
    ("New Cloud Agent Tab", "新建 Cloud Agent 标签页"),
    ("New Terminal Tab", "新建终端标签页"),
    ("New Team Environment Variables", "新建团队环境变量"),
    ("New Team Folder", "新建团队文件夹"),
    ("New Team Notebook", "新建团队 Notebook"),
    ("New Team Prompt", "新建团队 Prompt"),
    ("New Team Workflow", "新建团队 Workflow"),
    ("New agent conversation", "新建 Agent 对话"),
    ("New File", "新建文件"),
    ("Open AI Rules", "打开 AI 规则"),
    ("Open AI Command Suggestions", "打开 AI 命令建议"),
    ("Open Settings: Account", "打开设置：账户"),
    ("Open Settings: Features", "打开设置：功能"),
    ("Open Left Panel", "打开左侧面板"),
    ("Open MCP Servers", "打开 MCP Servers"),
    ("Open Settings: About", "打开设置：关于"),
    ("Open Settings: AI", "打开设置：AI"),
    ("Open Settings: Appearance", "打开设置：外观"),
    ("Open Settings: Billing and usage", "打开设置：账单与用量"),
    ("Open Settings: Code", "打开设置：代码"),
    ("Open Settings: Environments", "打开设置：环境"),
    ("Open Settings: Keyboard Shortcuts", "打开设置：键盘快捷键"),
    ("Open Settings: MCP Servers", "打开设置：MCP Servers"),
    ("Open Team Settings", "打开团队设置"),
    ("Open Settings: Privacy", "打开设置：隐私"),
    ("Open Settings: Referrals", "打开设置：推荐"),
    ("Open Settings: Shared Blocks", "打开设置：共享块"),
    ("Open Settings: Teams", "打开设置：团队"),
    ("Open Settings: Warpify", "打开设置：Warpify"),
    ("Open completions menu", "打开补全菜单"),
    ("Open global search", "打开全局搜索"),
    ("Open block context menu", "打开块上下文菜单"),
    ("Open keybindings editor", "打开键盘快捷键编辑器"),
    ("Open repository", "打开仓库"),
    ("Open tab configs menu", "打开标签页配置菜单"),
    ("Open settings", "打开设置"),
    ("Open theme picker", "打开主题选择器"),
    ("Open view tree debugger", "打开视图树调试器"),
    ("Paste", "粘贴"),
    ("Quit Warp", "退出 Warp"),
    ("Redo", "重做"),
    ("Reinput selected commands", "重新输入所选命令"),
    ("Reinput selected commands as root", "以 root 身份重新输入所选命令"),
    ("Reload file", "重新加载文件"),
    ("Remove the previous character", "删除前一个字符"),
    ("Rename the current tab", "重命名当前标签页"),
    ("Reopen closed session", "重新打开已关闭会话"),
    ("Reset Zoom", "重置缩放"),
    ("Reset font size to default", "将字体大小重置为默认值"),
    ("Reset notebook font size", "重置 Notebook 字体大小"),
    ("Reset zoom level to default", "将缩放级别重置为默认值"),
    ("Resize pane > Move divider down", "调整窗格大小 > 向下移动分隔线"),
    ("Resize pane > Move divider left", "调整窗格大小 > 向左移动分隔线"),
    ("Resize pane > Move divider right", "调整窗格大小 > 向右移动分隔线"),
    ("Resize pane > Move divider up", "调整窗格大小 > 向上移动分隔线"),
    ("Restart Warp AI", "重启 Warp AI"),
    ("Run selected commands", "运行所选命令"),
    ("Sample Process", "采样进程"),
    ("Save all unsaved files in code review", "保存代码审查中所有未保存的文件"),
    ("Save file as", "文件另存为"),
    ("Save new launch configuration", "保存新的启动配置"),
    ("Save workflow", "保存 Workflow"),
    ("Settings", "设置"),
    ("Scroll terminal output down one line", "将终端输出向下滚动一行"),
    ("Scroll terminal output up one line", "将终端输出向上滚动一行"),
    ("Scroll to bottom of selected block", "滚动到所选块底部"),
    ("Scroll to top of selected block", "滚动到所选块顶部"),
    ("Select To Line End", "选择到行尾"),
    ("Select To Line Start", "选择到行首"),
    ("Select all", "全选"),
    ("Select next block", "选择下一个块"),
    ("Select all blocks", "选择所有块"),
    ("Select and move to the bottom", "选择并移动到底部"),
    ("Select and move to the top", "选择并移动到顶部"),
    ("Select down", "向下选择"),
    ("Select next command", "选择下一条命令"),
    ("Select one character to the left", "向左选择一个字符"),
    ("Select one character to the right", "向右选择一个字符"),
    ("Select one subword to the left", "向左选择一个子词"),
    ("Select one subword to the right", "向右选择一个子词"),
    ("Select one word to the left", "向左选择一个单词"),
    ("Select one word to the right", "向右选择一个单词"),
    ("Select previous block", "选择上一个块"),
    ("Select previous command", "选择上一条命令"),
    ("Select shell command at cursor", "选择光标处的 shell 命令"),
    ("Select the closest bookmark down", "选择下方最近的书签"),
    ("Select the closest bookmark up", "选择上方最近的书签"),
    ("Select to end of line", "选择到行尾"),
    ("Select to end of paragraph", "选择到段落末尾"),
    ("Select to start of line", "选择到行首"),
    ("Select to start of paragraph", "选择到段落开头"),
    ("Select up", "向上选择"),
    ("Send feedback (opens external link)", "发送反馈（打开外部链接）"),
    ("Send feedback with Oz", "使用 Oz 发送反馈"),
    ("Set Input Mode to Agent Mode", "将输入模式设为 Agent 模式"),
    ("Set Input Mode to Terminal Mode", "将输入模式设为终端模式"),
    ("Setup Guide", "设置指南"),
    ("Share current session", "共享当前会话"),
    ("Share pane", "共享窗格"),
    ("Share selected block", "共享所选块"),
    ("Show Dedicated Hotkey Window", "显示专用热键窗口"),
    ("Show History", "显示历史记录"),
    ("Show Warp network log", "显示 Warp 网络日志"),
    ("Show find bar in code review", "在代码审查中显示查找栏"),
    ("Stop Synchronizing Any Panes", "停止同步所有窗格"),
    ("Stop sharing current session", "停止共享当前会话"),
    ("Switch Focus to Left Panel", "将焦点切换到左侧面板"),
    ("Switch Focus to Right Panel", "将焦点切换到右侧面板"),
    ("Switch panes down", "切换到下方窗格"),
    ("Switch panes left", "切换到左侧窗格"),
    ("Switch panes right", "切换到右侧窗格"),
    ("Switch panes up", "切换到上方窗格"),
    ("Switch to 1st tab", "切换到第 1 个标签页"),
    ("Switch to 2nd tab", "切换到第 2 个标签页"),
    ("Switch to 3rd tab", "切换到第 3 个标签页"),
    ("Switch to 4th tab", "切换到第 4 个标签页"),
    ("Switch to 5th tab", "切换到第 5 个标签页"),
    ("Switch to 6th tab", "切换到第 6 个标签页"),
    ("Switch to 7th tab", "切换到第 7 个标签页"),
    ("Switch to 8th tab", "切换到第 8 个标签页"),
    ("Switch to last tab", "切换到最后一个标签页"),
    ("Switch to next tab", "切换到下一个标签页"),
    ("Switch to previous tab", "切换到上一个标签页"),
    ("Take control of running command", "接管正在运行的命令"),
    ("Terminal session", "终端会话"),
    ("Toggle Agent conversation list view", "切换 Agent 对话列表视图"),
    ("Toggle Auto-execute Mode", "切换自动执行模式"),
    ("Toggle CLI Agent Rich Input", "切换 CLI Agent 富文本输入"),
    ("Toggle Files Palette", "切换文件面板"),
    ("Toggle Maximize Active Pane", "切换当前窗格最大化"),
    ("Toggle Hide CLI Responses", "切换隐藏 CLI 响应"),
    ("Toggle Maximize Code Review Panel", "切换代码审查面板最大化"),
    ("Toggle Mouse Reporting", "切换鼠标报告"),
    ("Toggle PTY Recording for Session", "切换会话 PTY 录制"),
    ("Toggle Queue Next Prompt", "切换排队下一条 Prompt"),
    ("Toggle Resource Center", "切换资源中心"),
    ("Toggle Sticky Command Header in Active Pane", "切换当前窗格的粘性命令标题"),
    ("Toggle Synchronizing All Panes in All Tabs", "切换所有标签页中所有窗格的同步"),
    ("Toggle Synchronizing All Panes in Current Tab", "切换当前标签页中所有窗格的同步"),
    ("Toggle Warp Drive", "切换 Warp Drive"),
    ("Toggle block filter on selected or last block", "切换所选块或最后一个块的过滤器"),
    ("Toggle Warp AI", "切换 Warp AI"),
    ("Toggle case-sensitive search", "切换区分大小写搜索"),
    ("Toggle code review", "切换代码审查"),
    ("Toggle command palette", "切换命令面板"),
    ("Toggle comment", "切换注释"),
    ("Toggle fullscreen", "切换全屏"),
    ("Toggle inline code styling", "切换行内代码样式"),
    ("Toggle keyboard shortcuts", "切换键盘快捷键"),
    ("Toggle navigation palette", "切换导航面板"),
    ("Toggle notification mailbox", "切换通知信箱"),
    ("Toggle project explorer", "切换项目浏览器"),
    ("Toggle regular expression search", "切换正则表达式搜索"),
    ("Toggle vertical tabs panel", "切换垂直标签页面板"),
    ("Toggle resource center", "切换资源中心"),
    ("Toggle rich-text debug mode", "切换富文本调试模式"),
    ("Toggle sticky command header", "切换粘性命令标题"),
    ("Toggle strikethrough styling", "切换删除线样式"),
    ("Toggle team workflows modal", "切换团队 Workflows 模态框"),
    ("Toggle the agent management view", "切换 Agent 管理视图"),
    ("Toggle underline styling", "切换下划线样式"),
    ("Trigger Auto Detection", "触发自动检测"),
    ("Trigger a panic (for testing sentry-rust)", "触发 panic（用于测试 sentry-rust）"),
    ("Turn notifications off", "关闭通知"),
    ("Turn notifications on", "开启通知"),
    ("Undo", "撤销"),
    ("Unfold", "展开"),
    ("Uninstall Oz CLI command", "卸载 Oz CLI 命令"),
    ("Write current codebase index snapshot", "写入当前代码库索引快照"),
    ("View Shared Blocks...", "查看共享块..."),
    ("Warp Drive", "Warp Drive"),
    ("View Warp logs", "查看 Warp 日志"),
    ("View latest changelog", "查看最新更新日志"),
    ("View privacy policy (opens external link)", "查看隐私政策（打开外部链接）"),
    ("View user docs (opens external link)", "查看用户文档（打开外部链接）"),
    ("Warpify ssh session", "Warpify SSH 会话"),
    ("Warpify subshell", "Warpify 子 shell"),
    ("Zoom In", "放大"),
    ("Zoom Out", "缩小"),
    ("[a11y] Set concise accessibility announcements", "[a11y] 设置简洁辅助功能播报"),
    ("[a11y] Set verbose accessibility announcements", "[a11y] 设置详细辅助功能播报"),
    ("Personal", "个人"),
    ("Enterprise", "企业"),
    (
        "Enterprise secret redaction cannot be modified.",
        "无法修改企业密钥遮盖规则。",
    ),
    (
        "No enterprise regexes have been configured by your organization.",
        "你的组织尚未配置任何企业 regex。",
    ),
    ("Recommended", "推荐"),
    ("Add all", "全部添加"),
    ("Enabled by your organization.", "已由你的组织启用。"),
    ("Secret redaction", "密钥遮盖"),
    (
        "When this setting is enabled, Warp will scan blocks, the contents of Warp Drive objects, and Oz prompts for potential sensitive information and prevent saving or sending this data to any servers. You can customize this list via regexes.",
        "启用此设置后，Warp 会扫描块、Warp Drive 对象内容和 Oz prompts 中可能存在的敏感信息，并阻止保存这些数据或将其发送到任何服务器。你可以通过 regex 自定义此列表。",
    ),
    ("Secret visual redaction mode", "密钥视觉遮盖模式"),
    (
        "Choose how secrets are visually presented in the block list while keeping them searchable. This setting only affects what you see in the block list.",
        "选择密钥在块列表中的视觉呈现方式，同时保留搜索能力。此设置只影响你在块列表中看到的内容。",
    ),
    ("Custom secret redaction", "自定义密钥遮盖"),
    (
        "Use regex to define additional secrets or data you'd like to redact. This will take effect when the next command runs. You can use the inline (?i) flag as a prefix to your regex to make it case-insensitive.",
        "使用 regex 定义你想要遮盖的其他密钥或数据。此设置会在下一个命令运行时生效。你可以在 regex 前添加内联 (?i) 标志，使其不区分大小写。",
    ),
    ("Add regex", "添加 regex"),
    ("Add regex pattern", "添加 regex 模式"),
    ("Name (optional)", "名称（可选）"),
    ("Regex pattern", "Regex 模式"),
    ("Invalid regex", "无效的 regex"),
    ("e.g. \"Google API Key\"", "例如 \"Google API Key\""),
    ("Asterisks", "星号"),
    ("Strikethrough", "删除线"),
    ("Always show secrets", "始终显示密钥"),
    (
        "Your administrator has enabled zero data retention for your team. User generated content will never be collected.",
        "你的管理员已为团队启用零数据保留。用户生成的内容永远不会被收集。",
    ),
    ("Help improve Warp", "帮助改进 Warp"),
    (
        "App analytics help us make the product better for you. We only collect app usage metadata, never console input or output.",
        "应用分析可帮助我们为你改进产品。我们只收集应用使用元数据，绝不会收集控制台输入或输出。",
    ),
    (
        "App analytics help us make the product better for you. We may collect certain console interactions to improve Warp's AI capabilities.",
        "应用分析可帮助我们为你改进产品。我们可能会收集某些控制台交互，以改进 Warp 的 AI 能力。",
    ),
    (
        "On the free tier, analytics must be enabled to use AI features.",
        "在免费套餐中，必须启用分析才能使用 AI 功能。",
    ),
    (
        "Read more about Warp's use of data",
        "详细了解 Warp 如何使用数据",
    ),
    (
        "This setting is managed by your organization.",
        "此设置由你的组织管理。",
    ),
    ("Send crash reports", "发送崩溃报告"),
    (
        "Crash reports assist with debugging and stability improvements.",
        "崩溃报告有助于调试和提升稳定性。",
    ),
    (
        "Store AI conversations in the cloud",
        "将 AI 对话存储在云端",
    ),
    (
        "Agent conversations can be shared with others and are retained when you log in on different devices. This data is only stored for product functionality, and Warp will not use it for analytics.",
        "Agent 对话可以与他人共享，并会在你登录不同设备时保留。这些数据仅为产品功能而存储，Warp 不会将其用于分析。",
    ),
    (
        "Agent conversations are only stored locally on your machine, are lost upon logout, and cannot be shared. Note: conversation data for ambient agents are still stored in the cloud.",
        "Agent 对话只会存储在本机，退出登录后会丢失，且无法共享。注意：ambient agents 的对话数据仍会存储在云端。",
    ),
    ("Plan", "套餐"),
    ("Sign up", "注册"),
    ("Free", "免费"),
    ("Compare plans", "比较套餐"),
    ("Manage billing", "管理账单"),
    ("Contact support", "联系支持"),
    ("Open admin panel", "打开管理面板"),
    ("Overage spending limit", "超额用量支出上限"),
    ("Monthly spending limit", "每月支出上限"),
    ("Load more", "加载更多"),
    ("Overview", "概览"),
    ("Usage History", "用量历史"),
    ("Cloud agent trial", "Cloud agent 试用"),
    ("1 credit remaining", "剩余 1 个 credit"),
    ("{credits} credits remaining", "剩余 {credits} 个 credits"),
    ("New agent", "新建 Agent"),
    ("Buy more", "购买更多"),
    ("View details on overage usage", "查看超额用量详情"),
    (
        "Enable premium model usage overages",
        "启用高级模型用量超额",
    ),
    (
        "Premium model usage overages are enabled",
        "高级模型用量超额已启用",
    ),
    (
        "Premium model usage overages are not enabled",
        "高级模型用量超额未启用",
    ),
    (
        "Continue using premium models beyond your plan's limits. Usage is charged in $20 increments up to your spending limit, with any remaining balance charged on your scheduled billing date.",
        "超出套餐限制后仍可继续使用高级模型。用量会按 20 美元为单位收费，最高不超过你的支出上限，剩余余额会在计划账单日期收取。",
    ),
    (
        "Ask a team admin to enable overages for more AI usage.",
        "请团队管理员启用超额用量，以获得更多 AI 用量。",
    ),
    ("Not set", "未设置"),
    (
        "Sets the monthly overage spending limit beyond the plan amount",
        "设置套餐额度之外的每月超额支出上限",
    ),
    ("Monthly overage spending limit", "每月超额支出上限"),
    ("Add-on credits", "附加 credits"),
    ("Switch to the Build plan", "切换到 Build 套餐"),
    ("Upgrade to the Build plan", "升级到 Build 套餐"),
    (" to purchase add-on credits.", " 以购买附加 credits。"),
    (
        "Contact your Account Executive for more add-on credits.",
        "请联系你的 Account Executive 获取更多附加 credits。",
    ),
    (
        "Contact a team admin to purchase add-on credits.",
        "请联系团队管理员购买附加 credits。",
    ),
    (
        "Add-on credits are purchased in prepaid packages that roll over each billing cycle and expire after one year. The more you purchase, the better the per-credit rate. Once your base plan credits are used, add-on credits will be consumed.",
        "附加 credits 以预付包形式购买，会在每个账单周期结转，并在一年后过期。购买越多，单个 credit 价格越优惠。当基础套餐 credits 用完后，将开始消耗附加 credits。",
    ),
    (
        "Purchased add-on credits are shared across your team.",
        "已购买的附加 credits 会在你的团队中共享。",
    ),
    (
        "Sets the monthly limit spent on add-on credits",
        "设置附加 credits 的每月支出上限",
    ),
    ("Monthly spend limit", "每月支出上限"),
    ("Purchased this month", "本月已购买"),
    ("1 credit", "1 个 credit"),
    ("{credits} credits", "{credits} 个 credits"),
    ("your selected", "你选择的"),
    ("Auto reload", "自动充值"),
    (
        "When enabled, auto reload will automatically purchase {auto_reload_amount} credits when your add-on credit balance reaches 100 credits remaining.",
        "启用后，当附加 credit 余额降至 100 个 credits 时，自动充值会自动购买 {auto_reload_amount} 个 credits。",
    ),
    ("Buying…", "正在购买..."),
    ("Buy", "购买"),
    (
        "Auto reload is disabled, as the next reload would exceed your monthly spend limit. Increase your limit to use auto reload.",
        "自动充值已禁用，因为下一次充值会超过你的每月支出上限。提高上限后即可使用自动充值。",
    ),
    (
        "Restricted due to billing issue. Update your payment method to purchase add-on credits.",
        "因账单问题受限。请更新付款方式以购买附加 credits。",
    ),
    (
        "Auto reload is disabled due to recent failed reload. Please update your payment method and try again.",
        "自动充值已禁用，因为最近一次充值失败。请更新付款方式后重试。",
    ),
    ("One-time purchase", "一次性购买"),
    (
        "Reloading would exceed your monthly limit. ",
        "充值会超过你的每月上限。",
    ),
    ("Increase your limit", "提高上限"),
    (" to continue.", " 后继续。"),
    ("Total overages", "超额用量总计"),
    ("Usage resets on {date}", "用量将在 {date} 重置"),
    (
        "Your credit limit is prorated because you joined midway through the billing cycle.",
        "你的 credit 上限已按比例折算，因为你是在账单周期中途加入的。",
    ),
    (
        "This credit limit is prorated because this user joined midway through the billing cycle.",
        "此 credit 上限已按比例折算，因为该用户是在账单周期中途加入的。",
    ),
    ("Restricted due to billing issue", "因账单问题受限"),
    ("Unlimited", "无限制"),
    ("Credits", "Credits"),
    (
        "This is the {refresh_duration} limit of AI credits for your account.",
        "这是你账户 AI credits 的 {refresh_duration} 上限。",
    ),
    ("Last 30 days", "过去 30 天"),
    ("No usage history", "暂无用量历史"),
    (
        "Kick off an agent task to view usage history here.",
        "启动一个 Agent 任务后，即可在此查看用量历史。",
    ),
    (
        "Usage reporting is currently limited",
        "当前用量报告能力有限",
    ),
    (
        "Enterprise credit usage isn't fully available in this view yet. For the most accurate spend tracking, ",
        "Enterprise credit 用量尚未在此视图中完整显示。如需最准确的支出跟踪，",
    ),
    ("visit the admin panel", "请访问管理面板"),
    (
        "Enterprise credit usage isn't fully available in this view yet. Contact a team admin for detailed usage reporting.",
        "Enterprise credit 用量尚未在此视图中完整显示。请联系团队管理员获取详细用量报告。",
    ),
    ("Usage", "用量"),
    ("Resets {time}", "{time} 重置"),
    ("Sort by", "排序依据"),
    ("A to Z", "A 到 Z"),
    ("Z to A", "Z 到 A"),
    ("Usage ascending", "用量升序"),
    ("Usage descending", "用量降序"),
    ("Team total", "团队总计"),
    (" to regain access to AI features.", " 以恢复 AI 功能访问。"),
    (
        "Contact your team admin to resolve billing issues.",
        "请联系你的团队管理员解决账单问题。",
    ),
    (
        " for a more flexible pricing model.",
        " 以使用更灵活的定价模式。",
    ),
    (" or ", " 或 "),
    ("bring your own key", "使用你自己的密钥"),
    (
        " for increased access to AI features.",
        " 以获得更多 AI 功能访问。",
    ),
    ("Upgrade to Turbo plan", "升级到 Turbo 套餐"),
    ("Upgrade to Lightspeed plan", "升级到 Lightspeed 套餐"),
    ("Upgrade", "升级"),
    (" to get more AI usage.", " 以获得更多 AI 用量。"),
    ("Upgrade to Max", "升级到 Max"),
    (" for more AI credits.", " 以获得更多 AI credits。"),
    ("Switch to Business", "切换到 Business"),
    (
        " for security features like SSO and automatically applied zero data retention.",
        " 以使用 SSO 和自动应用的零数据保留等安全功能。",
    ),
    ("Upgrade to Enterprise", "升级到 Enterprise"),
    (
        " for custom limits and dedicated support.",
        " 以获得自定义上限和专属支持。",
    ),
    (" for more AI usage.", " 以获得更多 AI 用量。"),
    (
        " for more credits and access to more models.",
        " 以获得更多 credits 并访问更多模型。",
    ),
    (
        "Warp will prevent use of premium models when this dollar limit is reached. Resets on a monthly basis.",
        "达到此美元上限后，Warp 将阻止使用高级模型。该上限每月重置。",
    ),
    (
        "Note that AI credits made near your chosen limit may exceed it by a few dollars.",
        "请注意，接近所选上限时产生的 AI credits 可能会超出几美元。",
    ),
    (
        "Please enter a valid currency amount",
        "请输入有效的货币金额",
    ),
    (
        "Please enter a price between $0.01 and $10,000,000",
        "请输入 $0.01 到 $10,000,000 之间的价格",
    ),
    ("Cancel", "取消"),
    ("Update", "更新"),
    ("Team name", "团队名称"),
    ("Your new team name", "你的新团队名称"),
    (
        "When you create a team, you can collaborate on agent-driven development by sharing cloud agent runs, environments, automations, and artifacts. You can also create a shared knowledge store for teammates and agents alike.",
        "创建团队后，你可以通过共享 cloud agent runs、environments、automations 和 artifacts 来协作进行 Agent 驱动开发。你也可以为队友和 Agent 创建共享知识库。",
    ),
    ("Leave team", "离开团队"),
    ("Delete team", "删除团队"),
    ("Create", "创建"),
    ("Domains, comma separated", "域名，用逗号分隔"),
    ("Emails, comma separated", "电子邮件，用逗号分隔"),
    ("Set", "设置"),
    ("Invite", "邀请"),
    ("Cancel invite", "取消邀请"),
    ("Transfer ownership", "转让所有权"),
    ("Demote from admin", "移除管理员身份"),
    ("Promote to admin", "提升为管理员"),
    ("Remove from team", "从团队中移除"),
    (
        "Some of the provided domains are invalid, or have already been added.",
        "提供的部分域名无效，或已添加。",
    ),
    (
        "As an admin, you can choose whether to enable or disable the ability for team members to invite others by invitation link.",
        "作为管理员，你可以选择是否允许团队成员通过邀请链接邀请他人。",
    ),
    (
        "Only allow users with emails at specific domains to join your team through the invite link.",
        "仅允许使用特定域名电子邮件的用户通过邀请链接加入你的团队。",
    ),
    (
        "Email invitations are valid for 7 days.",
        "电子邮件邀请有效期为 7 天。",
    ),
    (
        "Some of the provided email addresses are invalid, already invited, or members of the team.",
        "提供的部分电子邮件地址无效、已被邀请，或已是团队成员。",
    ),
    ("You are offline.", "你当前离线。"),
    (
        "You've reached the team member limit for your plan. Upgrade to add more teammates.",
        "你已达到当前套餐的团队成员上限。升级后可添加更多队友。",
    ),
    (
        "You've reached the team member limit for your plan. Contact support@warp.dev to add more teammates.",
        "你已达到当前套餐的团队成员上限。请联系 support@warp.dev 添加更多队友。",
    ),
    (
        "You've reached the team member limit for your plan. Contact a team admin to add more teammates.",
        "你已达到当前套餐的团队成员上限。请联系团队管理员添加更多队友。",
    ),
    (
        "Team invites have been restricted due to a payment issue. Please contact support@warp.dev to restore access.",
        "由于付款问题，团队邀请已受限。请联系 support@warp.dev 恢复访问。",
    ),
    (
        "Team invites have been restricted due to a payment issue. Please contact a team admin to restore access.",
        "由于付款问题，团队邀请已受限。请联系团队管理员恢复访问。",
    ),
    (
        "Team invites have been restricted due to a subscription payment issue.",
        "由于订阅付款问题，团队邀请已受限。",
    ),
    ("Please ", "请"),
    ("update your payment information", "更新你的付款信息"),
    (" to restore access.", "以恢复访问。"),
    (
        "You've exceeded the team member limit for your plan. Please contact support@warp.dev to upgrade your team.",
        "你已超过当前套餐的团队成员上限。请联系 support@warp.dev 升级你的团队。",
    ),
    (
        "You've exceeded the team member limit for your plan. Contact a team admin to upgrade your team.",
        "你已超过当前套餐的团队成员上限。请联系团队管理员升级你的团队。",
    ),
    (
        "You've exceeded the team member limit for your plan. Upgrade to add more teammates.",
        "你已超过当前套餐的团队成员上限。升级后可添加更多队友。",
    ),
    ("Failed to send invite", "发送邀请失败"),
    ("Toggled invite links", "已切换邀请链接"),
    ("Failed to toggle invite links", "切换邀请链接失败"),
    ("Reset invite links", "已重置邀请链接"),
    ("Failed to reset invite links", "重置邀请链接失败"),
    ("Deleted invite", "已删除邀请"),
    ("Failed to delete invite", "删除邀请失败"),
    ("Failed to add domain restriction", "添加域名限制失败"),
    ("Failed to delete domain restriction", "删除域名限制失败"),
    (
        "Failed to generate upgrade link. Please contact us at feedback@warp.dev",
        "生成升级链接失败。请通过 feedback@warp.dev 联系我们",
    ),
    (
        "Failed to generate billing link. Please contact us at feedback@warp.dev",
        "生成账单链接失败。请通过 feedback@warp.dev 联系我们",
    ),
    ("Toggled team discoverability", "已切换团队可发现性"),
    (
        "Failed to toggle team discoverability",
        "切换团队可发现性失败",
    ),
    ("Successfully joined team", "已成功加入团队"),
    ("Successfully joined {team_name}", "已成功加入 {team_name}"),
    ("Failed to join team", "加入团队失败"),
    (
        "Successfully transferred team ownership",
        "已成功转让团队所有权",
    ),
    ("Failed to transfer team ownership", "转让团队所有权失败"),
    (
        "Successfully updated team member role",
        "已成功更新团队成员角色",
    ),
    ("Failed to update team member role", "更新团队成员角色失败"),
    ("Error leaving team", "离开团队出错"),
    ("Successfully left team", "已成功离开团队"),
    ("Successfully renamed team", "已成功重命名团队"),
    ("Failed to rename team", "重命名团队失败"),
    ("Link copied to clipboard!", "链接已复制到剪贴板！"),
    ("Failed to load invite link.", "无法加载邀请链接。"),
    ("Invalid domains: {count}", "无效域名：{count}"),
    (
        "Domain restrictions added: {count}",
        "已添加域名限制：{count}",
    ),
    ("Invalid emails: {count}", "无效电子邮件：{count}"),
    ("Your invite is on the way!", "你的邀请正在发送！"),
    (
        "Your {count} invites are on the way!",
        "你的 {count} 封邀请正在发送！",
    ),
    (
        "You'll be charged for a portion of the team member's usage of Warp.",
        "你将为该团队成员的一部分 Warp 使用量付费。",
    ),
    (
        "Your admin will be charged for a portion of the team member's usage of Warp.",
        "你的管理员将为该团队成员的一部分 Warp 使用量付费。",
    ),
    (
        "Additional members are billed at your plan's per-user rate: ${monthly_cost}/month or ${yearly_cost}/year, depending on your billing interval. {prorated_message}",
        "额外成员会按你的套餐单用户价格计费：每月 ${monthly_cost} 或每年 ${yearly_cost}，取决于你的账单周期。{prorated_message}",
    ),
    (
        "Additional members are billed at your plan's per-user rate. {prorated_message}",
        "额外成员会按你的套餐单用户价格计费。{prorated_message}",
    ),
    ("Team members", "团队成员"),
    ("PAST DUE", "逾期"),
    ("UNPAID", "未付款"),
    ("Upgrade to Build", "升级到 Build"),
    ("Free plan usage limits", "免费套餐用量限制"),
    ("Plan usage limits", "套餐用量限制"),
    ("Shared Notebooks", "共享 Notebooks"),
    ("Shared Workflows", "共享 Workflows"),
    ("Invite by Link", "通过链接邀请"),
    ("Reset links", "重置链接"),
    ("Invite by Email", "通过电子邮件邀请"),
    ("Team Members", "团队成员"),
    ("Restrict by domain", "按域名限制"),
    ("Remove domain", "移除域名"),
    ("Make team discoverable", "允许发现团队"),
    (
        "Allow Warp users with an @{domain} email to find and join the team.",
        "允许使用 @{domain} 邮箱的 Warp 用户找到并加入该团队。",
    ),
    (
        "Allow Warp users with the same email domain as you to find and join the team.",
        "允许与你使用相同电子邮件域名的 Warp 用户找到并加入该团队。",
    ),
    ("Manage plan", "管理套餐"),
    ("EXPIRED", "已过期"),
    ("PENDING", "待处理"),
    ("OWNER", "所有者"),
    ("ADMIN", "管理员"),
    ("Teams", "团队"),
    ("Create a team", "创建团队"),
    (
        "Or, join an existing team within your company",
        "或者，加入公司内已有团队",
    ),
    ("1 teammate", "1 位队友"),
    ("{count} teammates", "{count} 位队友"),
    (
        "Join this team and start collaborating on workflows, notebooks, and more.",
        "加入此团队，开始在 workflows、notebooks 等内容上协作。",
    ),
    ("Join", "加入"),
    (
        "Contact Admin to request access",
        "联系管理员以请求访问权限",
    ),
    ("Search MCP Servers", "搜索 MCP 服务器"),
    ("Add", "添加"),
    (
        "Add MCP servers to extend the Warp Agent's capabilities. MCP servers expose data sources or tools to agents through a standardized interface, essentially acting like plugins. Add a custom server, or use the presets to get started with popular servers. You can also find team servers that have been shared with you here. ",
        "添加 MCP 服务器以扩展 Warp Agent 的能力。MCP 服务器通过标准化接口向 Agent 暴露数据源或工具，本质上类似插件。你可以添加自定义服务器，或使用预设快速开始使用热门服务器。你也可以在这里找到与你共享的团队服务器。",
    ),
    ("Learn more.", "了解更多。"),
    ("Available to install", "可安装"),
    (
        "Auto-spawn servers from third-party agents",
        "自动启动第三方 Agent 中的服务器",
    ),
    (
        "Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually in the \"Detected from\" sections below. ",
        "自动从全局范围的第三方 AI Agent 配置文件（例如你的主目录中）检测并启动 MCP 服务器。在仓库内检测到的服务器不会自动启动，必须在下方“检测来源”区块中逐个启用。",
    ),
    (
        "Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually from the MCP settings page. ",
        "自动从全局范围的第三方 AI Agent 配置文件（例如你的主目录中）检测并启动 MCP 服务器。在仓库内检测到的服务器不会自动启动，必须从 MCP 设置页面逐个启用。",
    ),
    ("See supported providers.", "查看支持的提供方。"),
    ("My MCPs", "我的 MCP"),
    ("Shared by Warp and {name}", "由 Warp 和 {name} 共享"),
    (
        "Shared by Warp and from other devices",
        "由 Warp 和其他设备共享",
    ),
    ("Shared from Warp", "来自 Warp 的共享"),
    ("Detected from {provider}", "检测来源：{provider}"),
    (
        "Once you add a MCP server, it will be shown here.",
        "添加 MCP 服务器后，它会显示在这里。",
    ),
    ("No search results found", "未找到搜索结果"),
    ("Detected from config file", "从配置文件检测到"),
    ("Shared by: {creator}", "共享者：{creator}"),
    ("Shared by a team member", "由团队成员共享"),
    ("From another device", "来自其他设备"),
    ("MCP server updated", "MCP 服务器已更新"),
    ("Offline", "离线"),
    ("Starting server...", "正在启动服务器..."),
    ("Authenticating...", "正在认证..."),
    ("Shutting down...", "正在关闭..."),
    ("No tools available", "没有可用工具"),
    ("{count} tools available", "{count} 个工具可用"),
    ("Show logs", "显示日志"),
    ("Log out", "退出登录"),
    ("Share server", "共享服务器"),
    ("Edit", "编辑"),
    ("View logs", "查看日志"),
    ("Edit config", "编辑配置"),
    ("Set up", "设置"),
    ("Server update available", "有可用的服务器更新"),
    ("Add New MCP Server", "添加新的 MCP 服务器"),
    ("Edit {name} MCP Server", "编辑 {name} MCP 服务器"),
    ("Edit MCP Server", "编辑 MCP 服务器"),
    ("Edit Variables", "编辑变量"),
    ("Delete MCP", "删除 MCP"),
    (
        "Only team admins and the creator of the MCP server can edit the MCP server.",
        "只有团队管理员和 MCP 服务器创建者可以编辑此 MCP 服务器。",
    ),
    (
        "This MCP server contains secrets. Visit Settings > Privacy to modify your secret redaction settings.",
        "此 MCP 服务器包含密钥。请前往“设置 > 隐私”修改密钥遮盖设置。",
    ),
    ("No MCP Server specified.", "未指定 MCP 服务器。"),
    (
        "Cannot add multiple MCP servers while editing a single server.",
        "编辑单个服务器时不能添加多个 MCP 服务器。",
    ),
    ("Install {name}", "安装 {name}"),
    ("Install", "安装"),
    ("Shared from team", "来自团队共享"),
    ("No MCP server selected", "未选择 MCP 服务器"),
    ("Update {name}", "更新 {name}"),
    (
        "This server has {count} updates available, which would you like to proceed with?",
        "此服务器有 {count} 个可用更新，你想继续使用哪一个？",
    ),
    ("Update from {publisher}", "来自 {publisher} 的更新"),
    ("another device", "其他设备"),
    ("a team member", "团队成员"),
    ("Version {new_version}", "版本 {new_version}"),
    ("No updates available", "没有可用更新"),
    ("Delete MCP server?", "删除 MCP 服务器？"),
    (
        "This will uninstall and remove this MCP server from all your devices.",
        "这将从你的所有设备卸载并移除此 MCP 服务器。",
    ),
    ("Delete shared MCP server?", "删除共享的 MCP 服务器？"),
    (
        "This will not only delete this MCP server for yourself, but also uninstall and remove this MCP server from Warp and across all of your teammates' devices.",
        "这不仅会为你自己删除此 MCP 服务器，还会从 Warp 以及所有队友的设备中卸载并移除此 MCP 服务器。",
    ),
    (
        "Remove shared MCP server from team?",
        "从团队中移除共享的 MCP 服务器？",
    ),
    (
        "This will uninstall and remove this MCP server from Warp and across all of your teammates' devices.",
        "这将从 Warp 以及所有队友的设备中卸载并移除此 MCP 服务器。",
    ),
    (
        "Successfully logged out of {name} MCP server",
        "已成功退出 {name} MCP 服务器",
    ),
    (
        "Successfully logged out of MCP server",
        "已成功退出 MCP 服务器",
    ),
    (
        "Finish the current MCP install before opening another install link.",
        "请先完成当前 MCP 安装，再打开其他安装链接。",
    ),
    ("Unknown MCP server '{name}'", "未知 MCP 服务器“{name}”"),
    (
        "MCP server '{name}' cannot be installed from this link.",
        "无法通过此链接安装 MCP 服务器“{name}”。",
    ),
    ("New API key", "新建 API 密钥"),
    ("Save your key", "保存你的密钥"),
    ("API key deleted", "API 密钥已删除"),
    (
        "Create and manage API keys to allow other Oz cloud agents to access your Warp account.\nFor more information, visit the ",
        "创建和管理 API 密钥，以允许其他 Oz cloud agents 访问你的 Warp 账户。\n如需更多信息，请访问",
    ),
    ("Documentation.", "文档。"),
    ("Oz Cloud API Keys", "Oz Cloud API 密钥"),
    ("+ Create API Key", "+ 创建 API 密钥"),
    ("Name", "名称"),
    ("Key", "密钥"),
    ("Scope", "范围"),
    ("Created", "创建时间"),
    ("Last used", "上次使用"),
    ("Expires at", "过期时间"),
    ("Team", "团队"),
    ("No API Keys", "没有 API 密钥"),
    (
        "Create a key to manage external access to Warp",
        "创建密钥以管理对 Warp 的外部访问",
    ),
    (
        "This API key is tied to your user and can make requests against your Warp account.",
        "此 API 密钥与你的用户绑定，可向你的 Warp 账户发起请求。",
    ),
    (
        "This API key is tied to your team and can make requests on behalf of your team.",
        "此 API 密钥与你的团队绑定，可代表你的团队发起请求。",
    ),
    ("1 day", "1 天"),
    ("30 days", "30 天"),
    ("90 days", "90 天"),
    (
        "Unable to create a team API key because there is no current team.",
        "无法创建团队 API 密钥，因为当前没有团队。",
    ),
    (
        "Failed to create API key. Please try again.",
        "创建 API 密钥失败。请重试。",
    ),
    (
        "This secret key is shown only once. Copy and store it securely.",
        "此密钥只会显示一次。请复制并安全保存。",
    ),
    ("Copied", "已复制"),
    ("Copy", "复制"),
    ("Done", "完成"),
    ("Type", "类型"),
    ("Expiration", "过期时间"),
    ("Creating…", "正在创建..."),
    ("Create key", "创建密钥"),
    ("Secret key copied.", "密钥已复制。"),
    ("Warp API Key", "Warp API Key"),
    (
        "Failed to delete API key. Please try again.",
        "删除 API 密钥失败。请重试。",
    ),
    ("Network log console", "网络日志控制台"),
    (
        "We've built a native console that allows you to view all communications from Warp to external servers to ensure you feel comfortable that your work is always kept safe.",
        "我们内置了原生控制台，可让你查看 Warp 与外部服务器之间的所有通信，帮助你确认工作内容始终安全。",
    ),
    ("View network logging", "查看网络日志"),
    ("Manage your data", "管理你的数据"),
    (
        "At any time, you may choose to delete your Warp account permanently. You will no longer be able to use Warp.",
        "你可以随时选择永久删除你的 Warp 账户。删除后你将无法继续使用 Warp。",
    ),
    ("Visit the data management page", "访问数据管理页面"),
    ("Privacy policy", "隐私政策"),
    ("Read Warp's privacy policy", "阅读 Warp 隐私政策"),
    ("(default)", "（默认）"),
    ("Themes", "主题"),
    ("Icon", "图标"),
    ("Panes", "窗格"),
    ("Cursor", "光标"),
    ("Create your own custom theme", "创建你自己的自定义主题"),
    ("Light", "浅色"),
    ("Dark", "深色"),
    ("Current theme", "当前主题"),
    ("Sync with OS", "与操作系统同步"),
    (
        "Automatically switch between light and dark themes when your system does.",
        "当系统切换浅色和深色主题时自动同步切换。",
    ),
    ("Customize your app icon", "自定义应用图标"),
    (
        "Changing the app icon requires the app to be bundled.",
        "更改应用图标需要应用已打包。",
    ),
    (
        "You may need to restart Warp for MacOS to apply the preferred icon style.",
        "你可能需要重启 Warp 才能让 macOS 应用首选图标样式。",
    ),
    (
        "Open new windows with custom size",
        "使用自定义尺寸打开新窗口",
    ),
    ("Columns", "列"),
    ("Rows", "行"),
    ("Window Opacity:", "窗口不透明度："),
    (
        "Transparency is not supported with your graphics drivers.",
        "你的图形驱动不支持透明效果。",
    ),
    (
        "The selected graphics settings may not support rendering transparent windows.",
        "所选图形设置可能不支持渲染透明窗口。",
    ),
    (
        " Try changing the settings for the graphics backend or integrated GPU in Features > System.",
        " 请尝试在“功能 > 系统”中更改图形后端或集成 GPU 设置。",
    ),
    ("Window Blur Radius:", "窗口模糊半径："),
    (
        "Use Window Blur (Acrylic texture)",
        "使用窗口模糊（亚克力纹理）",
    ),
    (
        "The selected hardware may not support rendering transparent windows.",
        "所选硬件可能不支持渲染透明窗口。",
    ),
    (
        "Tools panel visibility is consistent across tabs",
        "工具面板可见性在所有标签页中保持一致",
    ),
    ("Input type", "输入类型"),
    ("Shell (PS1)", "Shell (PS1)"),
    ("Input position", "输入位置"),
    ("Pin to the bottom (Warp mode)", "固定到底部（Warp 模式）"),
    ("Pin to the top (Reverse mode)", "固定到顶部（反向模式）"),
    ("Start at the top (Classic mode)", "从顶部开始（经典模式）"),
    ("Dim inactive panes", "调暗非活动窗格"),
    ("Focus follows mouse", "焦点跟随鼠标"),
    ("Compact mode", "紧凑模式"),
    (
        "Show Jump to Bottom of Block button",
        "显示“跳到底部块”按钮",
    ),
    ("Show block dividers", "显示块分隔线"),
    ("Agent font", "Agent 字体"),
    ("Match terminal", "匹配终端"),
    ("Line height", "行高"),
    ("Terminal font", "终端字体"),
    ("View all available system fonts", "查看所有可用系统字体"),
    ("Font weight", "字体粗细"),
    ("Font size (px)", "字体大小（px）"),
    ("Notebook font size", "Notebook 字体大小"),
    ("Use thin strokes", "使用细笔画"),
    ("On low-DPI displays", "在低 DPI 显示器上"),
    ("On high-DPI displays", "在高 DPI 显示器上"),
    ("Enforce minimum contrast", "强制最小对比度"),
    ("Only for named colors", "仅用于命名颜色"),
    ("Never", "从不"),
    ("Always", "始终"),
    ("Show ligatures in terminal", "在终端中显示连字"),
    ("Ligatures may reduce performance", "连字可能会降低性能"),
    ("Cursor type", "光标类型"),
    (
        "Cursor type is disabled in Vim mode",
        "光标类型在 Vim 模式下已禁用",
    ),
    ("Blinking cursor", "闪烁光标"),
    ("Tab close button position", "标签页关闭按钮位置"),
    ("Right", "右侧"),
    ("Left", "左侧"),
    ("Show tab indicators", "显示标签页指示器"),
    ("Show code review button", "显示代码审查按钮"),
    (
        "Preserve active tab color for new tabs",
        "为新标签页保留当前活动标签页颜色",
    ),
    ("Use vertical tab layout", "使用垂直标签页布局"),
    (
        "Show vertical tabs panel in restored windows",
        "在恢复的窗口中显示垂直标签页面板",
    ),
    (
        "When enabled, reopening or restoring a window opens the vertical tabs panel even if it was closed when the window was last saved.",
        "启用后，重新打开或恢复窗口时会打开垂直标签页面板，即使上次保存窗口时它是关闭的。",
    ),
    (
        "Use latest user prompt as conversation title in tab names",
        "在标签页名称中使用最新用户提示作为对话标题",
    ),
    (
        "Show the latest user prompt instead of the generated conversation title for Oz and third-party agent sessions in vertical tabs.",
        "在垂直标签页中，对 Oz 和第三方 Agent 会话显示最新用户提示，而不是生成的对话标题。",
    ),
    ("Header toolbar layout", "标题栏工具栏布局"),
    ("Directory tab colors", "目录标签页颜色"),
    (
        "Automatically color tabs based on the directory or repo you're working in.",
        "根据你正在使用的目录或仓库自动为标签页着色。",
    ),
    ("Default (no color)", "默认（无颜色）"),
    ("Show the tab bar", "显示标签栏"),
    (
        "Use custom padding in alt-screen",
        "在 alt-screen 中使用自定义内边距",
    ),
    ("Uniform padding (px)", "统一内边距（px）"),
    ("Zoom", "缩放"),
    (
        "Adjusts the default zoom level across all windows",
        "调整所有窗口的默认缩放级别",
    ),
    ("When windowed", "窗口模式时"),
    ("Only on hover", "仅悬停时"),
    ("Disabled", "已禁用"),
    ("Dedicated hotkey window", "专用热键窗口"),
    ("Show/hide all windows", "显示/隐藏所有窗口"),
    ("Active Screen", "当前屏幕"),
    ("Pin to top", "固定到顶部"),
    ("Pin to bottom", "固定到底部"),
    ("Pin to left", "固定到左侧"),
    ("Pin to right", "固定到右侧"),
    ("After all tabs", "在所有标签页之后"),
    ("After current tab", "在当前标签页之后"),
    ("Activate previous/next tab", "激活上一个/下一个标签页"),
    ("Cycle most recent session", "循环切换最近使用的会话"),
    ("Cycle most recent tab", "循环切换最近使用的标签页"),
    ("Width %", "宽度 %"),
    ("Height %", "高度 %"),
    (
        "Autohides on loss of keyboard focus",
        "失去键盘焦点时自动隐藏",
    ),
    ("When a command takes longer than", "当命令运行时间超过"),
    ("seconds to complete", "秒后"),
    ("Keybinding", "键绑定"),
    ("Click to set global hotkey", "点击设置全局热键"),
    ("Change keybinding", "更改键绑定"),
    ("Open links in desktop app", "在桌面应用中打开链接"),
    (
        "Automatically open links in desktop app whenever possible.",
        "尽可能自动在桌面应用中打开链接。",
    ),
    (
        "Restore windows, tabs, and panes on startup",
        "启动时恢复窗口、标签页和窗格",
    ),
    (
        "Window positions won't be restored on Wayland. ",
        "在 Wayland 上不会恢复窗口位置。",
    ),
    ("See docs.", "查看文档。"),
    ("Show sticky command header", "显示粘性命令标题"),
    ("Show tooltip on click on links", "点击链接时显示工具提示"),
    (
        "Show warning before quitting/logging out",
        "退出或注销前显示警告",
    ),
    (
        "Start Warp at login (requires macOS 13+)",
        "登录时启动 Warp（需要 macOS 13+）",
    ),
    ("Start Warp at login", "登录时启动 Warp"),
    ("Quit when all windows are closed", "关闭所有窗口时退出"),
    (
        "Show changelog toast after updates",
        "更新后显示更新日志 toast",
    ),
    ("Allowed Values: 1-20", "允许值：1-20"),
    (
        "Lines scrolled by mouse wheel interval",
        "鼠标滚轮每次滚动的行数",
    ),
    (
        "Supports floating point values between 1 and 20.",
        "支持 1 到 20 之间的浮点值。",
    ),
    ("Auto open code review panel", "自动打开代码审查面板"),
    (
        "When this setting is on, the code review panel will open on the first accepted diff of a conversation",
        "启用此设置后，代码审查面板会在一次对话中首次接受 diff 时打开",
    ),
    ("Warp is the default terminal", "Warp 是默认终端"),
    ("Make Warp the default terminal", "将 Warp 设为默认终端"),
    ("Maximum rows in a block", "块中的最大行数"),
    (
        "Setting the limit above 100k lines may impact performance. Maximum rows supported is {max_rows}.",
        "将限制设置为超过 100k 行可能会影响性能。支持的最大行数为 {max_rows}。",
    ),
    ("Warp SSH Wrapper", "Warp SSH 包装器"),
    (
        "This change will take effect in new sessions",
        "此更改将在新会话中生效",
    ),
    (
        "Receive desktop notifications from Warp",
        "接收来自 Warp 的桌面通知",
    ),
    (
        "Notify when an agent completes a task",
        "Agent 完成任务时通知",
    ),
    (
        "Notify when a command or agent needs your attention to continue",
        "命令或 Agent 需要你处理才能继续时通知",
    ),
    ("Play notification sounds", "播放通知声音"),
    ("Show in-app agent notifications", "显示应用内 Agent 通知"),
    ("Toast notifications stay visible for", "Toast 通知保持可见"),
    ("seconds", "秒"),
    ("Default shell for new sessions", "新会话的默认 shell"),
    ("Working directory for new sessions", "新会话的工作目录"),
    (
        "Confirm before closing shared session",
        "关闭共享会话前确认",
    ),
    ("Global hotkey:", "全局热键："),
    ("Not supported on Wayland. ", "Wayland 上不支持。"),
    (
        "Autocomplete quotes, parentheses, and brackets",
        "自动补全引号、圆括号和方括号",
    ),
    ("Error underlining for commands", "为命令显示错误下划线"),
    ("Syntax highlighting for commands", "为命令启用语法高亮"),
    ("Open completions menu as you type", "输入时打开补全菜单"),
    ("Suggest corrected commands", "建议修正后的命令"),
    ("Expand aliases as you type", "输入时展开别名"),
    ("Middle-click to paste", "中键点击粘贴"),
    (
        "Edit code and commands with Vim keybindings",
        "使用 Vim 键绑定编辑代码和命令",
    ),
    (
        "Set unnamed register as system clipboard",
        "将未命名寄存器设为系统剪贴板",
    ),
    ("Show Vim status bar", "显示 Vim 状态栏"),
    (
        "Enable '@' context menu in terminal mode",
        "在终端模式中启用 '@' 上下文菜单",
    ),
    (
        "Enable slash commands in terminal mode",
        "在终端模式中启用斜杠命令",
    ),
    (
        "Outline codebase symbols for '@' context menu",
        "为 '@' 上下文菜单列出代码库符号",
    ),
    ("Show terminal input message line", "显示终端输入消息行"),
    (
        "Show autosuggestion keybinding hint",
        "显示自动建议键绑定提示",
    ),
    ("Show autosuggestion ignore button", "显示自动建议忽略按钮"),
    ("→ accepts autosuggestions.", "→ 接受自动建议。"),
    ("accepts autosuggestions.", "接受自动建议。"),
    ("Completions open as you type.", "输入时会打开补全。"),
    (
        "Completions open as you type (or {keybinding}).",
        "输入时会打开补全（或按 {keybinding}）。",
    ),
    (
        "Opening the completion menu is unbound.",
        "打开补全菜单未绑定快捷键。",
    ),
    ("opens completion menu.", "打开补全菜单。"),
    ("Tab key behavior", "Tab 键行为"),
    ("Ctrl+Tab behavior:", "Ctrl+Tab 行为："),
    ("Enable Mouse Reporting", "启用鼠标报告"),
    ("Enable Scroll Reporting", "启用滚动报告"),
    ("Enable Focus Reporting", "启用焦点报告"),
    ("Use Audible Bell", "使用声音铃声"),
    (
        "Characters considered part of a word",
        "视为单词一部分的字符",
    ),
    ("Double-click smart selection", "双击智能选择"),
    ("Show help block in new sessions", "在新会话中显示帮助块"),
    ("Copy on select", "选择时复制"),
    ("New tab placement", "新标签页位置"),
    ("Default mode for new sessions", "新会话的默认模式"),
    ("Cloud Oz", "Cloud Oz"),
    ("Tab Config", "标签页配置"),
    ("Local Docker Sandbox", "本地 Docker 沙盒"),
    (
        "Show Global Workflows in Command Search (ctrl-r)",
        "在命令搜索（ctrl-r）中显示全局 Workflows",
    ),
    ("Honor Linux selection clipboard", "遵循 Linux 选择剪贴板"),
    (
        "Whether the Linux primary clipboard should be supported.",
        "是否支持 Linux 主剪贴板。",
    ),
    (
        "Prefer rendering new windows with integrated GPU (low power)",
        "优先使用集成 GPU 渲染新窗口（低功耗）",
    ),
    (
        "Use Wayland for window management",
        "使用 Wayland 进行窗口管理",
    ),
    ("Enables the use of Wayland", "启用 Wayland"),
    (
        "Enabling this setting disables global hotkey support. When disabled, text may be blurry if your Wayland compositor is using fraction scaling (ex: 125%).",
        "启用此设置会禁用全局热键支持。禁用时，如果你的 Wayland 合成器使用分数缩放（例如 125%），文本可能会模糊。",
    ),
    (
        "Restart Warp for changes to take effect.",
        "重启 Warp 后更改才会生效。",
    ),
    ("Preferred graphics backend", "首选图形后端"),
    ("Executable path", "可执行文件路径"),
    ("Custom", "自定义"),
    ("Home directory", "主目录"),
    ("Previous session's directory", "上一个会话的目录"),
    ("Custom directory", "自定义目录"),
    ("Directory path", "目录路径"),
    ("New window", "新窗口"),
    ("New tab", "新标签页"),
    ("Split pane", "拆分窗格"),
    ("Advanced", "高级"),
    ("New Tab", "新标签页"),
    ("Split Pane", "拆分窗格"),
    ("Default App", "默认应用"),
    (
        "Choose an editor to open file links",
        "选择用于打开文件链接的编辑器",
    ),
    (
        "Choose an editor to open files from the code review panel, project explorer, and global search",
        "选择用于从代码审查面板、项目浏览器和全局搜索打开文件的编辑器",
    ),
    (
        "Choose a layout to open files in Warp",
        "选择在 Warp 中打开文件的布局",
    ),
    (
        "Group files into single editor pane",
        "将文件分组到单个编辑器窗格",
    ),
    (
        "When this setting is on, any files opened in the same tab will be automatically grouped into a single editor pane.",
        "启用此设置后，同一标签页中打开的所有文件都会自动分组到单个编辑器窗格。",
    ),
    (
        "Open Markdown files in Warp's Markdown Viewer by default",
        "默认在 Warp 的 Markdown Viewer 中打开 Markdown 文件",
    ),
    (
        "Enable reopening of closed sessions",
        "启用重新打开已关闭会话",
    ),
    ("Grace period (seconds)", "宽限期（秒）"),
    ("Environments", "环境"),
    (
        "Environments define where your ambient agents run. Set one up in minutes via GitHub (recommended), Warp-assisted setup, or manual configuration.",
        "环境用于定义 ambient agents 的运行位置。可通过 GitHub（推荐）、Warp 辅助设置或手动配置在几分钟内完成设置。",
    ),
    ("Search environments...", "搜索环境..."),
    ("Successfully updated environment", "环境已成功更新"),
    ("Successfully created environment", "环境已成功创建"),
    ("Environment deleted successfully", "环境已成功删除"),
    ("Successfully shared environment", "环境已成功共享"),
    (
        "Failed to share environment with team",
        "无法与团队共享环境",
    ),
    (
        "Unable to create environment: not logged in.",
        "无法创建环境：尚未登录。",
    ),
    (
        "Unable to save: environment no longer exists.",
        "无法保存：环境已不存在。",
    ),
    (
        "Unable to share environment: you are not currently on a team.",
        "无法共享环境：你当前不在团队中。",
    ),
    (
        "Unable to share environment: environment is not yet synced.",
        "无法共享环境：环境尚未同步。",
    ),
    ("No environments match your search.", "没有匹配搜索的环境。"),
    ("Shared by Warp and your team", "由 Warp 和你的团队共享"),
    ("Authorize", "授权"),
    ("Get started", "开始使用"),
    ("Quick setup", "快速设置"),
    ("Suggested", "推荐"),
    (
        "Select the GitHub repositories you’d like to work with and we’ll suggest a base image and config",
        "选择你要使用的 GitHub 仓库，我们会建议基础镜像和配置",
    ),
    ("Use the agent", "使用 Agent"),
    (
        "Choose a locally set up project and we’ll help you set up an environment based on it",
        "选择一个本地已设置的项目，我们会基于它帮助你设置环境",
    ),
    (
        "You haven’t set up any environments yet.",
        "你还没有设置任何环境。",
    ),
    (
        "Choose how you’d like to set up your environment:",
        "选择设置环境的方式：",
    ),
    ("Image", "镜像"),
    ("Repos", "仓库"),
    ("Setup commands", "设置命令"),
    ("Last edited", "上次编辑"),
    ("never", "从未"),
    ("View my runs", "查看我的运行"),
    ("Share", "共享"),
    ("Edit environment", "编辑环境"),
    ("Create environment", "创建环境"),
    ("New environment", "新建环境"),
    ("Environment name", "环境名称"),
    ("Save environment", "保存环境"),
    ("Share with team", "与团队共享"),
    (
        "Personal environments cannot be used with external integrations or team API keys. For the best experience, use shared environments.",
        "个人环境不能用于外部集成或团队 API 密钥。为获得最佳体验，请使用共享环境。",
    ),
    ("Setup command(s)", "设置命令"),
    (
        "Setup commands run independently. Each command runs from the workspace root (/workspace). If a command depends on the previous one, combine them with &&.",
        "设置命令会独立运行。每条命令都从工作区根目录（/workspace）运行。如果某条命令依赖上一条命令，请使用 && 合并它们。",
    ),
    (
        "e.g. cd my-repo && pip install -r requirements.txt",
        "例如 cd my-repo && pip install -r requirements.txt",
    ),
    (
        "e.g., this environment is for all front end focused agents",
        "例如，此环境供所有专注前端的 Agent 使用",
    ),
    ("Description", "描述"),
    ("{count} / {max} characters", "{count} / {max} 个字符"),
    ("Repo(s)", "仓库"),
    ("Auth with GitHub", "使用 GitHub 授权"),
    ("Failed to load GitHub repos", "无法加载 GitHub 仓库"),
    ("Failed to load GitHub repositories", "无法加载 GitHub 仓库"),
    (
        "Enter repos (owner/repo format)",
        "输入仓库（owner/repo 格式）",
    ),
    ("Paste repo URL(s)", "粘贴仓库 URL"),
    (
        "Type owner/repo and press Enter to add, or select from dropdown.",
        "输入 owner/repo 并按 Enter 添加，或从下拉列表中选择。",
    ),
    ("Missing a repo?", "缺少某个仓库？"),
    ("Configure access on GitHub", "在 GitHub 上配置访问权限"),
    ("No repositories found", "未找到仓库"),
    ("Docker image reference", "Docker 镜像引用"),
    ("Open image at {url}", "在 {url} 打开镜像"),
    ("Generating…", "正在生成..."),
    ("Suggest image", "建议镜像"),
    (
        "Warp will suggest a Docker image based on your selected repositories.",
        "Warp 会根据你选择的仓库建议 Docker 镜像。",
    ),
    ("Failed to suggest a Docker image", "无法建议 Docker 镜像"),
    (
        "Unknown response from suggestCloudEnvironmentImage",
        "suggestCloudEnvironmentImage 返回未知响应",
    ),
    ("Authenticate", "授权"),
    (
        "You need to grant access to your GitHub repos to suggest a Docker image",
        "需要授予 GitHub 仓库访问权限，才能建议 Docker 镜像",
    ),
    (
        "We couldn't find a good match. We recommend using a custom Docker image for these repos.",
        "未找到合适匹配。建议为这些仓库使用自定义 Docker 镜像。",
    ),
    ("Add repo", "添加仓库"),
    ("Selected repos", "已选仓库"),
    ("No repos selected yet", "尚未选择仓库"),
    ("Available indexed repos", "可用的已索引仓库"),
    (
        "Loading locally indexed repos…",
        "正在加载本地已索引仓库...",
    ),
    (
        "No locally indexed repos found yet. Index a repo, then try again.",
        "尚未找到本地已索引仓库。请先索引一个仓库，然后重试。",
    ),
    (
        "Local repo selection is unavailable in this build.",
        "此构建中不可使用本地仓库选择。",
    ),
    (
        "All locally indexed repos are already selected.",
        "所有本地已索引仓库都已选择。",
    ),
    (
        "Selected folder is not a Git repository: {path}",
        "所选文件夹不是 Git 仓库：{path}",
    ),
    ("No directory selected", "未选择目录"),
    (
        "Select locally indexed repos to provide context for the environment creation agent.",
        "选择本地已索引仓库，为环境创建 Agent 提供上下文。",
    ),
    (
        "Select repos to provide context for the environment creation agent.",
        "选择仓库，为环境创建 Agent 提供上下文。",
    ),
    ("Select repos for your environment", "为你的环境选择仓库"),
    ("Warp Agent", "Warp Agent"),
    (
        "Your organization disallows AI when the active pane contains content from a remote session",
        "当活动窗格包含远程会话内容时，你的组织不允许使用 AI",
    ),
    (
        "To use AI features, please create an account.",
        "要使用 AI 功能，请创建账户。",
    ),
    (
        "Resets {formatted_next_refresh_time}",
        "{formatted_next_refresh_time} 重置",
    ),
    ("Next Command", "Next Command"),
    ("Prompt Suggestions", "Prompt Suggestions"),
    ("Suggested Code Banners", "建议代码横幅"),
    ("Natural Language Autosuggestions", "自然语言自动建议"),
    ("Shared Block Title Generation", "共享块标题生成"),
    (
        "Commit & Pull Request Generation",
        "Commit 与 Pull Request 生成",
    ),
    (
        "Let AI suggest the next command to run based on your command history, outputs, and common workflows.",
        "允许 AI 根据你的命令历史、输出和常见工作流建议下一条要运行的命令。",
    ),
    (
        "Let AI suggest natural language prompts, as inline banners in the input, based on recent commands and their outputs.",
        "允许 AI 根据最近的命令及其输出，在输入区以内联横幅形式建议自然语言提示。",
    ),
    (
        "Let AI suggest code diffs and queries as inline banners in the blocklist, based on recent commands and their outputs.",
        "允许 AI 根据最近的命令及其输出，在 blocklist 中以内联横幅形式建议代码 diff 和查询。",
    ),
    (
        "Let AI suggest natural language autosuggestions, based on recent commands and their outputs.",
        "允许 AI 根据最近的命令及其输出提供自然语言自动建议。",
    ),
    (
        "Let AI generate a title for your shared block based on the command and output.",
        "允许 AI 根据命令和输出为共享块生成标题。",
    ),
    (
        "Let AI generate commit messages and pull request titles and descriptions.",
        "允许 AI 生成 commit message 以及 pull request 的标题和描述。",
    ),
    ("Agents", "Agent"),
    (
        "Set the boundaries for how your Agent operates. Choose what it can access, how much autonomy it has, and when it must ask for your approval. You can also fine-tune behavior around natural language input, codebase awareness, and more.",
        "设置 Agent 的运行边界。选择它可以访问什么、拥有多少自主权，以及何时必须请求你的批准。你还可以微调自然语言输入、代码库感知等行为。",
    ),
    ("Profiles", "配置文件"),
    (
        "Profiles let you define how your Agent operates — from the actions it can take and when it needs approval, to the models it uses for tasks like coding and planning. You can also scope them to individual projects.",
        "配置文件可定义 Agent 的运行方式，包括可执行的操作、何时需要批准，以及用于编码和规划等任务的模型。你也可以将其限定到单个项目。",
    ),
    ("Models", "模型"),
    ("Context window (tokens)", "上下文窗口（token）"),
    ("Permissions", "权限"),
    ("Apply code diffs", "应用代码 diff"),
    ("Read files", "读取文件"),
    ("Execute commands", "执行命令"),
    ("Interact with running commands", "与运行中的命令交互"),
    (
        "Some of your permissions are managed by your workspace.",
        "你的部分权限由工作区管理。",
    ),
    ("Agent decides", "由 Agent 决定"),
    ("Always allow", "始终允许"),
    ("Ask on first write", "首次写入时询问"),
    ("Read only", "只读"),
    ("Supervised", "受监督"),
    ("Allow in specific directories", "允许特定目录"),
    (
        "The Agent chooses the safest path: acting on its own when confident, and asking for approval when uncertain.",
        "Agent 会选择最安全的路径：有把握时自行操作，不确定时请求批准。",
    ),
    (
        "Give the Agent full autonomy  — no manual approval ever required.",
        "给予 Agent 完全自主权，无需手动批准。",
    ),
    (
        "Require explicit approval before the Agent takes any action.",
        "Agent 执行任何操作前都需要明确批准。",
    ),
    (
        "The agent will ask for permission the first time it needs to interact with a running command. After that, it will continue automatically for the rest of that command.",
        "Agent 第一次需要与正在运行的命令交互时会请求权限。之后，该命令剩余过程会自动继续。",
    ),
    (
        "The agent will always ask for permission to interact with a running command.",
        "Agent 每次与正在运行的命令交互前都会请求权限。",
    ),
    (
        "This model serves as the primary engine behind the Warp Agent. It powers most interactions and invokes other models for tasks like planning or code generation when necessary. Warp may automatically switch to alternate models based on model availability or for auxiliary tasks such as conversation summarization.",
        "此模型是 Warp Agent 的主要引擎，负责大多数交互，并在规划或代码生成等任务需要时调用其他模型。Warp 可能会根据模型可用性，或为了会话摘要等辅助任务自动切换到备用模型。",
    ),
    ("Show model picker in prompt", "在提示中显示模型选择器"),
    ("Base model", "基础模型"),
    ("Codebase Context", "代码库上下文"),
    (
        "Allow the Warp Agent to generate an outline of your codebase that can be used for context. No code is ever stored on our servers. ",
        "允许 Warp Agent 生成可用作上下文的代码库概要。任何代码都不会存储在我们的服务器上。",
    ),
    ("Call MCP servers", "调用 MCP 服务器"),
    (
        "You haven't added any MCP servers yet. Once you do, you'll be able to control how much autonomy the Warp Agent has when interacting with them. ",
        "你还没有添加任何 MCP 服务器。添加后，你可以控制 Warp Agent 与这些服务器交互时拥有多少自主权。",
    ),
    ("Add a server", "添加服务器"),
    ("learn more about MCPs.", "了解更多 MCP 信息。"),
    ("MCP allowlist", "MCP 允许列表"),
    (
        "Allow the Warp Agent to call these MCP servers.",
        "允许 Warp Agent 调用这些 MCP 服务器。",
    ),
    ("MCP denylist", "MCP 拒绝列表"),
    (
        "The Warp Agent will always ask for permission before calling any MCP servers on this list.",
        "Warp Agent 调用此列表中的任何 MCP 服务器前都会请求权限。",
    ),
    ("Show input hint text", "显示输入提示文本"),
    ("Show agent tips", "显示 Agent 提示"),
    (
        "Include agent-executed commands in history",
        "将 Agent 执行的命令纳入历史记录",
    ),
    ("Encountered an incorrect detection? ", "遇到错误检测？"),
    (
        " Encountered an incorrect input detection? ",
        " 遇到错误输入检测？",
    ),
    ("Let us know", "告诉我们"),
    (
        "Autodetect agent prompts in terminal input",
        "自动检测终端输入中的 Agent 提示",
    ),
    (
        "Autodetect terminal commands in agent input",
        "自动检测 Agent 输入中的终端命令",
    ),
    (
        "Enabling natural language detection will detect when natural language is written in the terminal input, and then automatically switch to Agent Mode for AI queries.",
        "启用自然语言检测后，如果终端输入中写入了自然语言，将自动切换到 Agent Mode 进行 AI 查询。",
    ),
    ("Natural language detection", "自然语言检测"),
    ("Natural language denylist", "自然语言拒绝列表"),
    (
        "Commands listed here will never trigger natural language detection.",
        "此处列出的命令永远不会触发自然语言检测。",
    ),
    (
        "Add MCP servers to extend the Warp Agent's capabilities. MCP servers expose data sources or tools to agents through a standardized interface, essentially acting like plugins. ",
        "添加 MCP 服务器以扩展 Warp Agent 的能力。MCP 服务器通过标准化接口向 Agent 暴露数据源或工具，本质上类似插件。",
    ),
    ("Manage MCP servers", "管理 MCP 服务器"),
    ("Rules", "规则"),
    (
        "Rules help the Warp Agent follow your conventions, whether for codebases or specific workflows. ",
        "规则帮助 Warp Agent 遵循你的约定，无论是针对代码库还是特定工作流。",
    ),
    ("Manage rules", "管理规则"),
    ("Suggested Rules", "建议规则"),
    (
        "Let AI suggest rules to save based on your interactions.",
        "允许 AI 根据你的交互建议要保存的规则。",
    ),
    (
        "Warp Drive as agent context",
        "将 Warp Drive 用作 Agent 上下文",
    ),
    (
        "The Warp Agent can leverage your Warp Drive Contents to tailor responses to your personal and team developer workflows and environments. This includes any Workflows, Notebooks, and Environment Variables.",
        "Warp Agent 可以利用你的 Warp Drive 内容，为个人和团队开发工作流及环境定制响应。这包括 Workflows、Notebooks 和 Environment Variables。",
    ),
    ("Voice Input", "语音输入"),
    (
        "Voice input allows you to control Warp by speaking directly to your terminal (powered by ",
        "语音输入允许你直接对终端说话来控制 Warp（由 ",
    ),
    (").", "）。"),
    ("Key for Activating Voice Input", "激活语音输入的按键"),
    ("Press and hold to activate.", "按住以激活。"),
    ("Voice", "语音"),
    ("Other", "其他"),
    (
        "Show Oz changelog in new conversation view",
        "在新会话视图中显示 Oz 更新日志",
    ),
    ("Show \"Use Agent\" footer", "显示“Use Agent”页脚"),
    (
        "Shows hint to use the \"Full Terminal Use\"-enabled agent in long running commands.",
        "在长时间运行的命令中显示使用已启用“Full Terminal Use”的 Agent 的提示。",
    ),
    (
        "Show conversation history in tools panel",
        "在工具面板中显示会话历史",
    ),
    ("Agent thinking display", "Agent 思考显示"),
    (
        "Controls how reasoning/thinking traces are displayed.",
        "控制 reasoning/thinking 轨迹的显示方式。",
    ),
    ("Show & collapse", "显示并折叠"),
    ("Always show", "始终显示"),
    ("Never show", "从不显示"),
    (
        "Preferred layout when opening existing agent conversations",
        "打开现有 Agent 会话时的首选布局",
    ),
    ("Show coding agent toolbar", "显示编码 Agent 工具栏"),
    (
        "Show a toolbar with quick actions when running coding agents like ",
        "运行 ",
    ),
    (", or ", " 或 "),
    ("Third party CLI agents", "第三方 CLI Agent"),
    (
        "Auto show/hide Rich Input based on agent status",
        "根据 Agent 状态自动显示/隐藏 Rich Input",
    ),
    (
        "Requires the Warp plugin for your coding agent",
        "需要为你的编码 Agent 安装 Warp 插件",
    ),
    (
        "Auto open Rich Input when a coding agent session starts",
        "编码 Agent 会话开始时自动打开 Rich Input",
    ),
    (
        "Auto dismiss Rich Input after prompt submission",
        "提交提示后自动关闭 Rich Input",
    ),
    ("Commands that enable the toolbar", "启用工具栏的命令"),
    (
        "Add regex patterns to show the coding agent toolbar for matching commands.",
        "添加 regex 模式，以便对匹配命令显示编码 Agent 工具栏。",
    ),
    (
        "This option is enforced by your organization's settings and cannot be customized.",
        "此选项由你的组织设置强制执行，无法自定义。",
    ),
    ("Enable agent attribution", "启用 Agent 署名"),
    ("Agent Attribution", "Agent 署名"),
    (
        "Oz can add attribution to commit messages and pull requests it creates",
        "Oz 可以在其创建的 commit message 和 pull request 中添加署名",
    ),
    (
        "Computer use in Cloud Agents",
        "Cloud Agents 中的计算机使用",
    ),
    ("Experimental", "实验性"),
    (
        "Enable computer use in cloud agent conversations started from the Warp app.",
        "在从 Warp 应用启动的 cloud agent 会话中启用计算机使用。",
    ),
    ("Orchestration", "编排"),
    (
        "Enable multi-agent orchestration, allowing the agent to spawn and coordinate parallel sub-agents.",
        "启用多 Agent 编排，允许 Agent 启动并协调并行子 Agent。",
    ),
    ("API Keys", "API Keys"),
    (
        "Use your own API keys from model providers for the Warp Agent to use. API keys are stored locally and never synced to the cloud. Using auto models or models from providers you have not provided API keys for will consume Warp credits.",
        "使用模型提供商的自有 API keys 供 Warp Agent 使用。API keys 会存储在本地，绝不会同步到云端。使用自动模型或未提供 API keys 的提供商模型会消耗 Warp credits。",
    ),
    ("Contact sales", "联系销售"),
    (
        " to enable bringing your own API keys on your Enterprise plan.",
        " 以在你的 Enterprise 套餐中启用自带 API keys。",
    ),
    (" to use your own API keys.", " 以使用你自己的 API keys。"),
    (
        "Ask your team's admin to upgrade to the Build plan to use your own API keys.",
        "请你的团队管理员升级到 Build 套餐，以使用你自己的 API keys。",
    ),
    ("Warp credit fallback", "Warp credit 兜底"),
    (
        "When enabled, agent requests may be routed to one of Warp's provided models in the event of an error. Warp will prioritize using your API keys over your Warp credits.",
        "启用后，Agent 请求在发生错误时可能会路由到 Warp 提供的某个模型。Warp 会优先使用你的 API keys，而不是 Warp credits。",
    ),
    ("Refresh", "刷新"),
    (
        "Warp loads and sends local AWS CLI credentials for Bedrock-supported models. This setting is managed by your organization.",
        "Warp 会为 Bedrock 支持的模型加载并发送本地 AWS CLI 凭据。此设置由你的组织管理。",
    ),
    (
        "Warp loads and sends local AWS CLI credentials for Bedrock-supported models.",
        "Warp 会为 Bedrock 支持的模型加载并发送本地 AWS CLI 凭据。",
    ),
    ("AWS Bedrock", "AWS Bedrock"),
    ("Use AWS Bedrock credentials", "使用 AWS Bedrock 凭据"),
    ("Login Command", "登录命令"),
    ("AWS Profile", "AWS Profile"),
    ("Automatically run login command", "自动运行登录命令"),
    (
        "When enabled, the login command will run automatically when AWS Bedrock credentials expire.",
        "启用后，当 AWS Bedrock 凭据过期时会自动运行登录命令。",
    ),
    ("Code", "代码"),
    ("Initialization Settings", "初始化设置"),
    (
        "Warp can automatically index code repositories as you navigate them, helping agents quickly understand context and provide solutions. Code is never stored on the server. If a codebase is unable to be indexed, Warp can still navigate your codebase and gain insights via grep and find tool calling.",
        "Warp 可以在你浏览代码仓库时自动为其建立索引，帮助 Agent 快速理解上下文并提供解决方案。代码永远不会存储在服务器上。如果代码库无法建立索引，Warp 仍可通过 grep 和 find 工具调用浏览代码库并获得洞察。",
    ),
    (
        "To exclude specific files or directories from indexing, add them to the .warpindexingignore file in your repository directory. These files will still be accessible to AI features, but they won't be included in codebase embeddings.",
        "要从索引中排除特定文件或目录，请将它们添加到仓库目录中的 .warpindexingignore 文件。这些文件仍可被 AI 功能访问，但不会包含在代码库 embeddings 中。",
    ),
    ("Index new folders by default", "默认索引新文件夹"),
    (
        "When set to true, Warp will automatically index code repositories as you navigate them - helping agents quickly understand context and provide targeted solutions.",
        "设为 true 时，Warp 会在你浏览代码仓库时自动建立索引，帮助 Agent 快速理解上下文并提供有针对性的解决方案。",
    ),
    (
        "Team admins have disabled codebase indexing.",
        "团队管理员已禁用代码库索引。",
    ),
    (
        "Team admins have enabled codebase indexing.",
        "团队管理员已启用代码库索引。",
    ),
    (
        "AI Features must be enabled to use codebase indexing.",
        "必须启用 AI 功能才能使用代码库索引。",
    ),
    (
        "You have reached the maximum number of codebase indices for your plan. Delete existing indices to auto-index new codebases.",
        "你已达到当前套餐的代码库索引数量上限。请删除现有索引以自动索引新的代码库。",
    ),
    ("Index new folder", "索引新文件夹"),
    ("Initialized / indexed folders", "已初始化/已索引文件夹"),
    (
        "No folders have been initialized yet.",
        "尚未初始化任何文件夹。",
    ),
    ("Open project rules", "打开项目规则"),
    ("INDEXING", "索引"),
    ("No index created", "未创建索引"),
    (
        "Discovered {total_nodes} chunks",
        "已发现 {total_nodes} 个 chunk",
    ),
    (
        "Syncing - {completed_nodes} / {total_nodes}",
        "正在同步 - {completed_nodes} / {total_nodes}",
    ),
    ("Syncing...", "正在同步..."),
    ("Synced", "已同步"),
    ("Codebase too large", "代码库过大"),
    ("Stale", "已过期"),
    ("No index built", "未构建索引"),
    ("LSP SERVERS", "LSP 服务器"),
    ("Installed", "已安装"),
    ("Installing...", "正在安装..."),
    ("Checking...", "正在检查..."),
    ("Available for download", "可下载"),
    ("Restart server", "重启服务器"),
    ("Available", "可用"),
    ("Busy", "忙碌"),
    ("Stopped", "已停止"),
    ("Not running", "未运行"),
    (
        "Show a button in the top right of the window to toggle the code review panel.",
        "在窗口右上角显示一个按钮，用于切换代码审查面板。",
    ),
    (
        "Show diff stats on code review button",
        "在代码审查按钮上显示 diff 统计",
    ),
    (
        "Show lines added and removed counts on the code review button.",
        "在代码审查按钮上显示新增和删除行数。",
    ),
    ("Project explorer", "项目浏览器"),
    (
        "Adds an IDE-style project explorer / file tree to the left side tools panel.",
        "在左侧工具面板中添加 IDE 风格的项目浏览器/文件树。",
    ),
    ("Global file search", "全局文件搜索"),
    (
        "Adds global file search to the left side tools panel.",
        "在左侧工具面板中添加全局文件搜索。",
    ),
    ("Active AI", "Active AI"),
    ("Add Profile", "添加配置文件"),
    ("Commands, comma separated", "命令，以逗号分隔"),
    ("Delete environment", "删除环境"),
    ("Failed", "失败"),
    ("Knowledge", "知识库"),
    ("Launch agent", "启动 Agent"),
    ("MCP Servers", "MCP 服务器"),
    ("Retry", "重试"),
    ("Select MCP servers", "选择 MCP 服务器"),
    ("Toolbar layout", "工具栏布局"),
    ("MODELS", "模型"),
    ("PERMISSIONS", "权限"),
    ("Base model:", "基础模型："),
    ("Full terminal use:", "完整终端使用："),
    ("Computer use:", "计算机使用："),
    ("Apply code diffs:", "应用代码 diff："),
    ("Read files:", "读取文件："),
    ("Execute commands:", "执行命令："),
    ("Interact with running commands:", "与运行中的命令交互："),
    ("Ask questions:", "提问："),
    ("Call MCP servers:", "调用 MCP 服务器："),
    ("Call web tools:", "调用 Web 工具："),
    (
        "Auto-sync plans to Warp Drive:",
        "自动同步计划到 Warp Drive：",
    ),
    ("Directory allowlist:", "目录允许列表："),
    ("Command allowlist:", "命令允许列表："),
    ("Command denylist:", "命令拒绝列表："),
    ("MCP allowlist:", "MCP 允许列表："),
    ("MCP denylist:", "MCP 拒绝列表："),
    ("None", "无"),
    ("Unknown", "未知"),
    ("Never ask", "从不询问"),
    ("Ask unless auto-approve", "除非自动批准，否则询问"),
    ("On", "开启"),
    ("Off", "关闭"),
    ("Changes will apply to new windows.", "更改将应用于新窗口。"),
    ("e.g. ls .*", "例如 ls .*"),
    ("e.g. rm .*", "例如 rm .*"),
    ("e.g. ~/code-repos/repo", "例如 ~/code-repos/repo"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_keys_have_english_values() {
        for key in ALL_UI_STRING_KEYS {
            assert!(
                !localized(*key, UiLanguage::English).is_empty(),
                "{key:?} is missing English text"
            );
        }
    }

    #[test]
    fn all_keys_have_chinese_values() {
        for key in ALL_UI_STRING_KEYS {
            assert!(
                !localized(*key, UiLanguage::ChineseSimplified).is_empty(),
                "{key:?} is missing Simplified Chinese text"
            );
        }
    }

    #[test]
    fn language_option_labels_are_stable() {
        assert_eq!(
            UiLanguage::ALL,
            &[UiLanguage::English, UiLanguage::ChineseSimplified]
        );
        assert_eq!(UiLanguage::English.display_name(), "English");
        assert_eq!(UiLanguage::ChineseSimplified.display_name(), "中文");
        assert_eq!(
            UiLanguage::ALL
                .iter()
                .map(|language| language.display_name())
                .collect::<Vec<_>>(),
            vec!["English", "中文"]
        );
    }

    #[test]
    fn chinese_simplified_missing_key_falls_back_to_english() {
        assert_eq!(
            localized_with_chinese_lookup(
                UiStringKey::SettingsSearchPlaceholder,
                UiLanguage::ChineseSimplified,
                |_| None,
            ),
            "Search"
        );
    }

    #[test]
    fn settings_static_text_falls_back_when_no_translation_exists() {
        assert_eq!(
            chinese_simplified_settings_text("not a localized settings string"),
            None
        );
    }

    #[test]
    fn settings_static_text_has_chinese_values_for_shared_labels() {
        assert_eq!(
            chinese_simplified_settings_text("Reset to default"),
            Some("重置为默认值")
        );
        assert_eq!(
            chinese_simplified_settings_text("Open settings file"),
            Some("打开设置文件")
        );
        assert_eq!(chinese_simplified_settings_text("Import"), Some("导入"));
        assert_eq!(
            chinese_simplified_settings_text("Reset to Warp defaults"),
            Some("重置为 Warp 默认值")
        );
        assert_eq!(
            chinese_simplified_settings_text("Add directory color"),
            Some("添加目录颜色")
        );
        assert_eq!(
            chinese_simplified_settings_text("Full-screen Apps"),
            Some("全屏应用")
        );
        assert_eq!(
            chinese_simplified_settings_text("Workflows"),
            Some("工作流")
        );
        assert_eq!(
            chinese_simplified_settings_text("Warpify SSH Sessions"),
            Some("Warpify SSH 会话")
        );
        assert_eq!(
            chinese_simplified_settings_text("You don't have any shared blocks yet."),
            Some("你还没有任何共享块。")
        );
        assert_eq!(
            chinese_simplified_settings_text("Get exclusive Warp goodies when you refer someone*"),
            Some("推荐他人即可获得 Warp 专属好礼*")
        );
        assert_eq!(
            chinese_simplified_settings_text("Secret redaction"),
            Some("密钥遮盖")
        );
        assert_eq!(
            chinese_simplified_settings_text("Store AI conversations in the cloud"),
            Some("将 AI 对话存储在云端")
        );
        assert_eq!(
            chinese_simplified_settings_text("Privacy policy"),
            Some("隐私政策")
        );
        assert_eq!(
            chinese_simplified_settings_text("Cancel invite"),
            Some("取消邀请")
        );
        assert_eq!(
            chinese_simplified_settings_text("Failed to load invite link."),
            Some("无法加载邀请链接。")
        );
        assert_eq!(
            chinese_simplified_settings_text("Search MCP Servers"),
            Some("搜索 MCP 服务器")
        );
        assert_eq!(
            chinese_simplified_settings_text("Delete MCP server?"),
            Some("删除 MCP 服务器？")
        );
        assert_eq!(
            chinese_simplified_settings_text("No tools available"),
            Some("没有可用工具")
        );
        assert_eq!(
            chinese_simplified_settings_text("Add Profile"),
            Some("添加配置文件")
        );
        assert_eq!(
            chinese_simplified_settings_text("Select MCP servers"),
            Some("选择 MCP 服务器")
        );
        assert_eq!(
            chinese_simplified_settings_text("Changes will apply to new windows."),
            Some("更改将应用于新窗口。")
        );
        assert_eq!(
            chinese_simplified_settings_text("Launch agent"),
            Some("启动 Agent")
        );
        assert_eq!(
            chinese_simplified_settings_text("AWS Bedrock"),
            Some("AWS Bedrock")
        );
        assert_eq!(
            chinese_simplified_settings_text("Warp API Key"),
            Some("Warp API Key")
        );
        assert_eq!(
            chinese_simplified_settings_text("Codebase indexing"),
            Some("代码库索引")
        );
        assert_eq!(
            chinese_simplified_settings_text("Search repos"),
            Some("搜索仓库")
        );
        assert_eq!(
            chinese_simplified_settings_text("Search tabs..."),
            Some("搜索标签页...")
        );
        assert_eq!(
            chinese_simplified_settings_text(" + Add new repo"),
            Some(" + 添加新仓库")
        );
        assert_eq!(
            chinese_simplified_settings_text("Search sessions, agents, files..."),
            Some("搜索会话、Agent、文件...")
        );
        assert_eq!(
            chinese_simplified_settings_text("Split pane right"),
            Some("向右拆分窗格")
        );
        assert_eq!(
            chinese_simplified_settings_text("Split pane left"),
            Some("向左拆分窗格")
        );
        assert_eq!(
            chinese_simplified_settings_text("Split pane down"),
            Some("向下拆分窗格")
        );
        assert_eq!(
            chinese_simplified_settings_text("Split pane up"),
            Some("向上拆分窗格")
        );
        assert_eq!(
            chinese_simplified_settings_text("Close pane"),
            Some("关闭窗格")
        );
        assert_eq!(
            chinese_simplified_settings_text(" Encountered an incorrect input detection? "),
            Some(" 遇到错误输入检测？")
        );
        assert_eq!(
            chinese_simplified_settings_text("Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually from the MCP settings page. "),
            Some("自动从全局范围的第三方 AI Agent 配置文件（例如你的主目录中）检测并启动 MCP 服务器。在仓库内检测到的服务器不会自动启动，必须从 MCP 设置页面逐个启用。")
        );
    }

    #[test]
    fn settings_static_text_has_no_duplicate_english_entries() {
        let mut seen = std::collections::HashSet::new();
        for (english, _) in CHINESE_SIMPLIFIED_SETTINGS_TEXT {
            assert!(
                seen.insert(*english),
                "{english} has duplicate translations"
            );
        }
    }
}
