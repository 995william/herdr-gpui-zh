//! Shared definitions for the macOS native and the Linux and Windows
//! in-window menu bars.
//! Every item dispatches the same action the palette and the keymap use.

pub(crate) mod in_window;

use crate::{
    CheckForUpdates, Quit, RunCommand, ShowLogs,
    actions::{Copy, Cut, Paste, SelectAll, SetLayout},
    config::{Layout, LayoutMode},
    controls::Command,
};
#[cfg(target_os = "macos")]
use crate::{Hide, HideOthers, Minimize, ShowAll};
#[cfg(feature = "qa-menu")]
use crate::{
    PlaySound, ShowHerdrNotDetected, ShowUpdatePreview,
    actions::{
        RingBellPreview, ShowToastPreview, ShowUpdateDownloadPreview, ShowUpdateHomebrewPreview,
    },
};
use gpui::{App, Menu, MenuItem, OsAction};
#[cfg(feature = "qa-menu")]
use herdr_client::protocol::SemanticNotificationKind;

/// Linux and Windows have no global menu bar, so each window draws its own.
pub(crate) const IN_WINDOW: bool = cfg!(any(target_os = "linux", windows));

/// Only headings are needed while the in-window menus are closed.
struct Headings(Vec<(gpui::SharedString, bool)>);
impl gpui::Global for Headings {}

/// Installs the menu bar, checking the layout the latest config picked.
pub(crate) fn install(cx: &mut App) {
    let layout = cx
        .try_global::<crate::app::InitialAppearance>()
        .map_or_else(Layout::default, |appearance| appearance.config.layout);
    let menus = menus(layout);
    cx.set_global(Headings(
        menus
            .iter()
            .map(|menu| (menu.name.clone(), menu.disabled))
            .collect(),
    ));
    cx.set_menus(menus);
}

/// View > Layout: Herdr's densities, their rounded versions, the layouts
/// with a design of their own, then Devices, which lists agents by device;
/// the one in use checked.
fn layout_menu(current: LayoutMode) -> MenuItem {
    let items = LayoutMode::ALL
        .iter()
        .enumerate()
        .flat_map(|(index, &mode)| {
            let group = matches!(index, 3 | 6 | 9).then(MenuItem::separator);
            group.into_iter().chain([
                MenuItem::action(mode.label(), SetLayout { mode }).checked(mode == current)
            ])
        });
    MenuItem::submenu(Menu::new("布局").items(items))
}

/// The menu bar, with `layout`'s mode checked under View > Layout.
pub(crate) fn menus(layout: Layout) -> Vec<Menu> {
    vec![
        Menu {
            name: "Herdr".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "关于 Herdr",
                    RunCommand {
                        command: Command::About,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "命令面板",
                    RunCommand {
                        command: Command::Palette,
                    },
                ),
                MenuItem::action(
                    "设置",
                    RunCommand {
                        command: Command::Settings,
                    },
                ),
                MenuItem::action(
                    "键盘快捷键",
                    RunCommand {
                        command: Command::Keybinds,
                    },
                ),
                MenuItem::action("检查更新...", CheckForUpdates),
                // macOS reserves Hide for every app; without these items
                // Cmd-H has no key equivalent and does nothing.
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Herdr", Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Others", HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("Show All", ShowAll),
                MenuItem::separator(),
                MenuItem::action("退出 Herdr", Quit),
            ],
        },
        Menu {
            name: "文件".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "新建工作区",
                    RunCommand {
                        command: Command::Workspace,
                    },
                ),
                MenuItem::action(
                    "新建工作树...",
                    RunCommand {
                        command: Command::NewWorktree,
                    },
                ),
                MenuItem::action(
                    "新建标签页",
                    RunCommand {
                        command: Command::Tab,
                    },
                ),
                MenuItem::action(
                    "快速跳转",
                    RunCommand {
                        command: Command::WorkspacePicker,
                    },
                ),
                MenuItem::action(
                    "前往符号...",
                    RunCommand {
                        command: Command::GoToSymbol,
                    },
                ),
                MenuItem::action(
                    "前往文件...",
                    RunCommand {
                        command: Command::GoToFile,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "关闭分栏...",
                    RunCommand {
                        command: Command::ClosePane,
                    },
                ),
                MenuItem::action(
                    "关闭标签页...",
                    RunCommand {
                        command: Command::CloseTab,
                    },
                ),
                MenuItem::action(
                    "重新打开已关闭标签页",
                    RunCommand {
                        command: Command::ReopenClosedTab,
                    },
                ),
            ],
        },
        // OS actions also route these items to native text fields, such as
        // the file panels, the way every macOS app's Edit menu does.
        Menu {
            name: "编辑".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("剪切", Cut, OsAction::Cut),
                MenuItem::os_action("复制", Copy, OsAction::Copy),
                MenuItem::os_action("粘贴", Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::os_action("全选", SelectAll, OsAction::SelectAll),
            ],
        },
        Menu {
            name: "视图".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "放大字号",
                    RunCommand {
                        command: Command::IncreaseFontSize,
                    },
                ),
                MenuItem::action(
                    "缩小字号",
                    RunCommand {
                        command: Command::DecreaseFontSize,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "重置字号",
                    RunCommand {
                        command: Command::ResetFontSize,
                    },
                ),
                MenuItem::separator(),
                layout_menu(layout.mode),
                MenuItem::action(
                    "设备概览",
                    RunCommand {
                        command: Command::DevicesOverview,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "切换全屏",
                    RunCommand {
                        command: Command::ToggleFullScreen,
                    },
                ),
                MenuItem::action(
                    "切换专注模式",
                    RunCommand {
                        command: Command::ToggleZenMode,
                    },
                ),
            ],
        },
        Menu {
            name: "终端".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "向右垂直分栏",
                    RunCommand {
                        command: Command::SplitRight,
                    },
                ),
                MenuItem::action(
                    "向下水平分栏",
                    RunCommand {
                        command: Command::SplitDown,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "下一个标签页",
                    RunCommand {
                        command: Command::NextTab,
                    },
                ),
                MenuItem::action(
                    "上一个标签页",
                    RunCommand {
                        command: Command::PreviousTab,
                    },
                ),
                MenuItem::action(
                    "后退",
                    RunCommand {
                        command: Command::Back,
                    },
                ),
                MenuItem::action(
                    "前进",
                    RunCommand {
                        command: Command::Forward,
                    },
                ),
                MenuItem::action(
                    "切换分栏缩放",
                    RunCommand {
                        command: Command::Zoom,
                    },
                ),
                MenuItem::action(
                    "清屏",
                    RunCommand {
                        command: Command::ClearPane,
                    },
                ),
                MenuItem::action(
                    "查找",
                    RunCommand {
                        command: Command::Find,
                    },
                ),
                MenuItem::action(
                    "查找下一个",
                    RunCommand {
                        command: Command::FindNext,
                    },
                ),
                MenuItem::action(
                    "查找上一个",
                    RunCommand {
                        command: Command::FindPrevious,
                    },
                ),
                MenuItem::action(
                    "复制模式",
                    RunCommand {
                        command: Command::CopyMode,
                    },
                ),
                MenuItem::action(
                    "在编辑器中打开历史输出",
                    RunCommand {
                        command: Command::EditScrollback,
                    },
                ),
                MenuItem::action(
                    "打开通知目标",
                    RunCommand {
                        command: Command::OpenNotificationTarget,
                    },
                ),
                MenuItem::action(
                    "切换侧边栏",
                    RunCommand {
                        command: Command::ToggleSidebar,
                    },
                ),
                MenuItem::action(
                    "切换状态栏",
                    RunCommand {
                        command: Command::ToggleStatusBar,
                    },
                ),
                MenuItem::action(
                    "打开 VS Code",
                    RunCommand {
                        command: Command::OpenCode,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "重新连接",
                    RunCommand {
                        command: Command::Reconnect,
                    },
                ),
            ],
        },
        Menu {
            name: "窗口".into(),
            disabled: false,
            items: vec![
                // Without this item Cmd-M has no key equivalent and does
                // nothing; the pane zoom above is unrelated to the window.
                #[cfg(target_os = "macos")]
                MenuItem::action("Minimize", Minimize),
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                MenuItem::action(
                    "新建窗口",
                    RunCommand {
                        command: Command::NewWindow,
                    },
                ),
                MenuItem::action(
                    "在窗口间切换",
                    RunCommand {
                        command: Command::CycleWindows,
                    },
                ),
                MenuItem::action(
                    "关闭窗口",
                    RunCommand {
                        command: Command::CloseWindow,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("日志", ShowLogs),
            ],
        },
        #[cfg(feature = "qa-menu")]
        Menu {
            name: "QA".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Show herdr non-detected modal", ShowHerdrNotDetected),
                MenuItem::action("Show app update available", ShowUpdatePreview),
                MenuItem::action(
                    "Show update download progress (50%)",
                    ShowUpdateDownloadPreview,
                ),
                MenuItem::action("Show Homebrew update progress", ShowUpdateHomebrewPreview),
                MenuItem::action("Play Sound", PlaySound),
                MenuItem::action("Ring Bell in 3 Seconds", RingBellPreview),
                #[cfg(any(target_os = "macos", windows))]
                MenuItem::action(
                    "Enable badge",
                    crate::actions::SetBadgePreview { enabled: true },
                ),
                #[cfg(any(target_os = "macos", windows))]
                MenuItem::action(
                    "Disable badge preview",
                    crate::actions::SetBadgePreview { enabled: false },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Show NeedsAttention toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::NeedsAttention,
                    },
                ),
                MenuItem::action(
                    "Show Finished toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::Finished,
                    },
                ),
                MenuItem::action(
                    "Show UpdateInstalled toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::UpdateInstalled,
                    },
                ),
                MenuItem::action(
                    "Show Custom toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::Custom,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Send notification in 3 seconds",
                    crate::actions::ShowSystemNotificationPreview,
                ),
            ],
        },
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
