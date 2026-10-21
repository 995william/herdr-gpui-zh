use herdr_client::{Method, protocol::ClientShellSnapshot};
use serde_json::{Value, json};

mod shortcuts;
pub(crate) use shortcuts::CTRL_SHORTCUTS;
#[cfg(test)]
pub(crate) use shortcuts::platform_default;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum Command {
    NewWindow,
    Workspace,
    NewWorktree,
    /// The focused repository's "Open worktree..." dialog.
    OpenWorktree,
    /// The focused linked worktree's delete confirmation.
    RemoveWorktree,
    /// Go To, listing only the checkouts that have a note.
    WorktreeNotes,
    /// The focused checkout's note.
    EditWorktreeNote,
    Tab,
    SplitRight,
    SplitDown,
    NextTab,
    PreviousTab,
    FocusLeft,
    FocusRight,
    FocusUp,
    FocusDown,
    NextPane,
    PreviousPane,
    Zoom,
    ClearPane,
    Find,
    FindNext,
    FindPrevious,
    CopyMode,
    AnnotateSelection,
    EditScrollback,
    ClosePane,
    CloseTab,
    /// Reopens the focused workspace's most recently closed tab.
    ReopenClosedTab,
    TabNumber(u8),
    ToggleSidebar,
    ToggleStatusBar,
    /// Moves keyboard focus to the next of the terminal, the title bar, the
    /// sidebar, and the status bar, where Tab moves between controls.
    FocusNextRegion,
    FocusPreviousRegion,
    /// Hides the window's chrome and every group but the one in use.
    ToggleZenMode,
    DevicesOverview,
    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,
    Settings,
    Keybinds,
    Sessions,
    Themes,
    WorkspacePicker,
    Palette,
    /// Reopen the palette as it was last left: query, filters, and selection.
    ResumePalette,
    /// Go to a definition in the focused pane's checkout.
    GoToSymbol,
    /// Go to a file in the focused pane's checkout.
    GoToFile,
    Detach,
    Reconnect,
    Quit,
    Logs,
    About,
    OpenNotificationTarget,
    NewBrowserTab,
    /// The focused workspace's repository's issues, pull requests, and runs.
    OpenOrchestrator,
    InstallBrowserSkill,
    SplitEditor,
    OpenCode,
    ToggleFullScreen,
    CycleWindows,
    /// Closes the window it runs in, leaving the daemon and its terminals.
    CloseWindow,
    MoveTabPrevious,
    MoveTabNext,
    RenameTab,
    LastPane,
    /// Back through the panes this endpoint has focused.
    Back,
    Forward,
    SwapLeft,
    SwapRight,
    SwapUp,
    SwapDown,
    ResizeLeft,
    ResizeRight,
    ResizeUp,
    ResizeDown,
    ResizeMode,
    RenamePane,
    MovePane,
    PreviousWorkspace,
    NextWorkspace,
    WorkspaceNumber(u8),
    RenameWorkspace,
    CloseWorkspace,
    PreviousAgent,
    NextAgent,
    AgentNumber(u8),
    ReloadConfig,
}

pub struct CommandInfo {
    pub command: Command,
    /// The key naming this command in the config file's `[keybindings]`.
    pub name: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[CommandInfo] = &[
    CommandInfo {
        command: Command::OpenNotificationTarget,
        name: "open_notification_target",
        label: "打开通知目标",
    },
    CommandInfo {
        command: Command::Logs,
        name: "logs",
        label: "查看日志",
    },
    CommandInfo {
        command: Command::NewWindow,
        name: "new_window",
        label: "新建窗口",
    },
    CommandInfo {
        command: Command::Workspace,
        name: "new_workspace",
        label: "新建工作区",
    },
    CommandInfo {
        command: Command::NewWorktree,
        name: "new_worktree",
        label: "新建工作树",
    },
    CommandInfo {
        command: Command::OpenWorktree,
        name: "open_worktree",
        label: "打开工作树",
    },
    CommandInfo {
        command: Command::RemoveWorktree,
        name: "remove_worktree",
        label: "删除工作树检出",
    },
    CommandInfo {
        command: Command::WorktreeNotes,
        name: "worktree_notes",
        label: "工作树备忘",
    },
    CommandInfo {
        command: Command::EditWorktreeNote,
        name: "edit_worktree_note",
        label: "编辑工作树备忘",
    },
    CommandInfo {
        command: Command::PreviousWorkspace,
        name: "previous_workspace",
        label: "上一个工作区",
    },
    CommandInfo {
        command: Command::NextWorkspace,
        name: "next_workspace",
        label: "下一个工作区",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(1),
        name: "focus_workspace_1",
        label: "Focus Workspace 1",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(2),
        name: "focus_workspace_2",
        label: "Focus Workspace 2",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(3),
        name: "focus_workspace_3",
        label: "Focus Workspace 3",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(4),
        name: "focus_workspace_4",
        label: "Focus Workspace 4",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(5),
        name: "focus_workspace_5",
        label: "Focus Workspace 5",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(6),
        name: "focus_workspace_6",
        label: "Focus Workspace 6",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(7),
        name: "focus_workspace_7",
        label: "Focus Workspace 7",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(8),
        name: "focus_workspace_8",
        label: "Focus Workspace 8",
    },
    CommandInfo {
        command: Command::WorkspaceNumber(9),
        name: "focus_workspace_9",
        label: "Focus Workspace 9",
    },
    CommandInfo {
        command: Command::RenameWorkspace,
        name: "rename_workspace",
        label: "重命名工作区",
    },
    CommandInfo {
        command: Command::CloseWorkspace,
        name: "close_workspace",
        label: "关闭工作区",
    },
    CommandInfo {
        command: Command::Tab,
        name: "new_tab",
        label: "新建标签页",
    },
    CommandInfo {
        command: Command::SplitRight,
        name: "split_right",
        label: "向右分栏",
    },
    CommandInfo {
        command: Command::SplitDown,
        name: "split_down",
        label: "向下分栏",
    },
    CommandInfo {
        command: Command::NextTab,
        name: "next_tab",
        label: "下一个标签页",
    },
    CommandInfo {
        command: Command::PreviousTab,
        name: "previous_tab",
        label: "上一个标签页",
    },
    CommandInfo {
        command: Command::MoveTabPrevious,
        name: "move_tab_previous",
        label: "向左移动标签页",
    },
    CommandInfo {
        command: Command::MoveTabNext,
        name: "move_tab_next",
        label: "向右移动标签页",
    },
    CommandInfo {
        command: Command::RenameTab,
        name: "rename_tab",
        label: "重命名标签页",
    },
    CommandInfo {
        command: Command::FocusLeft,
        name: "focus_left",
        label: "焦点左移",
    },
    CommandInfo {
        command: Command::FocusRight,
        name: "focus_right",
        label: "焦点右移",
    },
    CommandInfo {
        command: Command::FocusUp,
        name: "focus_up",
        label: "焦点上移",
    },
    CommandInfo {
        command: Command::FocusDown,
        name: "focus_down",
        label: "焦点下移",
    },
    CommandInfo {
        command: Command::NextPane,
        name: "next_pane",
        label: "下一个分栏",
    },
    CommandInfo {
        command: Command::PreviousPane,
        name: "previous_pane",
        label: "上一个分栏",
    },
    CommandInfo {
        command: Command::LastPane,
        name: "last_pane",
        label: "最后一个分栏",
    },
    CommandInfo {
        command: Command::Back,
        name: "back",
        label: "后退",
    },
    CommandInfo {
        command: Command::Forward,
        name: "forward",
        label: "前进",
    },
    CommandInfo {
        command: Command::SwapLeft,
        name: "swap_pane_left",
        label: "分栏左对调",
    },
    CommandInfo {
        command: Command::SwapRight,
        name: "swap_pane_right",
        label: "分栏右对调",
    },
    CommandInfo {
        command: Command::SwapUp,
        name: "swap_pane_up",
        label: "分栏上对调",
    },
    CommandInfo {
        command: Command::SwapDown,
        name: "swap_pane_down",
        label: "分栏下对调",
    },
    CommandInfo {
        command: Command::ResizeLeft,
        name: "resize_pane_left",
        label: "分栏向左缩放",
    },
    CommandInfo {
        command: Command::ResizeRight,
        name: "resize_pane_right",
        label: "分栏向右缩放",
    },
    CommandInfo {
        command: Command::ResizeUp,
        name: "resize_pane_up",
        label: "分栏向上缩放",
    },
    CommandInfo {
        command: Command::ResizeDown,
        name: "resize_pane_down",
        label: "分栏向下缩放",
    },
    CommandInfo {
        command: Command::ResizeMode,
        name: "resize_mode",
        label: "缩放模式",
    },
    CommandInfo {
        command: Command::RenamePane,
        name: "rename_pane",
        label: "重命名分栏",
    },
    CommandInfo {
        command: Command::MovePane,
        name: "move_pane",
        label: "移动分栏\u{2026}",
    },
    CommandInfo {
        command: Command::Zoom,
        name: "toggle_zoom",
        label: "切换分栏缩放",
    },
    CommandInfo {
        command: Command::ClearPane,
        name: "clear_pane",
        label: "清屏",
    },
    CommandInfo {
        command: Command::Find,
        name: "find",
        label: "查找",
    },
    CommandInfo {
        command: Command::FindNext,
        name: "find_next",
        label: "查找下一个",
    },
    CommandInfo {
        command: Command::FindPrevious,
        name: "find_previous",
        label: "查找上一个",
    },
    CommandInfo {
        command: Command::CopyMode,
        name: "copy_mode",
        label: "复制模式",
    },
    CommandInfo {
        command: Command::AnnotateSelection,
        name: "annotate_selection",
        label: "Annotate Selection for the Agent",
    },
    CommandInfo {
        command: Command::EditScrollback,
        name: "edit_scrollback",
        label: "Open Scrollback in Editor",
    },
    CommandInfo {
        command: Command::ClosePane,
        name: "close_pane",
        label: "关闭分栏",
    },
    CommandInfo {
        command: Command::CloseTab,
        name: "close_tab",
        label: "关闭标签页",
    },
    CommandInfo {
        command: Command::ReopenClosedTab,
        name: "reopen_closed_tab",
        label: "重新打开已关闭标签页",
    },
    CommandInfo {
        command: Command::TabNumber(1),
        name: "focus_tab_1",
        label: "Focus Tab 1",
    },
    CommandInfo {
        command: Command::TabNumber(2),
        name: "focus_tab_2",
        label: "Focus Tab 2",
    },
    CommandInfo {
        command: Command::TabNumber(3),
        name: "focus_tab_3",
        label: "Focus Tab 3",
    },
    CommandInfo {
        command: Command::TabNumber(4),
        name: "focus_tab_4",
        label: "Focus Tab 4",
    },
    CommandInfo {
        command: Command::TabNumber(5),
        name: "focus_tab_5",
        label: "Focus Tab 5",
    },
    CommandInfo {
        command: Command::TabNumber(6),
        name: "focus_tab_6",
        label: "Focus Tab 6",
    },
    CommandInfo {
        command: Command::TabNumber(7),
        name: "focus_tab_7",
        label: "Focus Tab 7",
    },
    CommandInfo {
        command: Command::TabNumber(8),
        name: "focus_tab_8",
        label: "Focus Tab 8",
    },
    CommandInfo {
        command: Command::TabNumber(9),
        name: "focus_tab_9",
        label: "Focus Tab 9",
    },
    CommandInfo {
        command: Command::PreviousAgent,
        name: "previous_agent",
        label: "Previous Agent",
    },
    CommandInfo {
        command: Command::NextAgent,
        name: "next_agent",
        label: "Next Agent",
    },
    CommandInfo {
        command: Command::AgentNumber(1),
        name: "focus_agent_1",
        label: "Focus Agent 1",
    },
    CommandInfo {
        command: Command::AgentNumber(2),
        name: "focus_agent_2",
        label: "Focus Agent 2",
    },
    CommandInfo {
        command: Command::AgentNumber(3),
        name: "focus_agent_3",
        label: "Focus Agent 3",
    },
    CommandInfo {
        command: Command::AgentNumber(4),
        name: "focus_agent_4",
        label: "Focus Agent 4",
    },
    CommandInfo {
        command: Command::AgentNumber(5),
        name: "focus_agent_5",
        label: "Focus Agent 5",
    },
    CommandInfo {
        command: Command::AgentNumber(6),
        name: "focus_agent_6",
        label: "Focus Agent 6",
    },
    CommandInfo {
        command: Command::AgentNumber(7),
        name: "focus_agent_7",
        label: "Focus Agent 7",
    },
    CommandInfo {
        command: Command::AgentNumber(8),
        name: "focus_agent_8",
        label: "Focus Agent 8",
    },
    CommandInfo {
        command: Command::AgentNumber(9),
        name: "focus_agent_9",
        label: "Focus Agent 9",
    },
    CommandInfo {
        command: Command::ToggleSidebar,
        name: "toggle_sidebar",
        label: "切换侧边栏",
    },
    CommandInfo {
        command: Command::ToggleStatusBar,
        name: "toggle_status_bar",
        label: "切换状态栏",
    },
    CommandInfo {
        command: Command::FocusNextRegion,
        name: "focus_next_region",
        label: "Focus Next Region",
    },
    CommandInfo {
        command: Command::FocusPreviousRegion,
        name: "focus_previous_region",
        label: "Focus Previous Region",
    },
    CommandInfo {
        command: Command::ToggleZenMode,
        name: "toggle_zen_mode",
        label: "切换专注模式",
    },
    CommandInfo {
        command: Command::DevicesOverview,
        name: "devices_overview",
        label: "设备概览",
    },
    CommandInfo {
        command: Command::IncreaseFontSize,
        name: "increase_font_size",
        label: "放大字号",
    },
    CommandInfo {
        command: Command::DecreaseFontSize,
        name: "decrease_font_size",
        label: "缩小字号",
    },
    CommandInfo {
        command: Command::ResetFontSize,
        name: "reset_font_size",
        label: "重置字号",
    },
    CommandInfo {
        command: Command::Settings,
        name: "settings",
        label: "设置",
    },
    CommandInfo {
        command: Command::ReloadConfig,
        name: "reload_config",
        label: "Reload Config",
    },
    CommandInfo {
        command: Command::Keybinds,
        name: "keybindings",
        label: "键盘快捷键",
    },
    CommandInfo {
        command: Command::Sessions,
        name: "sessions",
        label: "Sessions",
    },
    CommandInfo {
        command: Command::Themes,
        name: "themes",
        label: "Themes",
    },
    CommandInfo {
        command: Command::WorkspacePicker,
        name: "workspace_picker",
        label: "Go To",
    },
    CommandInfo {
        command: Command::Palette,
        name: "command_palette",
        label: "Command Palette",
    },
    CommandInfo {
        command: Command::ResumePalette,
        name: "resume_palette",
        label: "Resume Palette",
    },
    CommandInfo {
        command: Command::GoToSymbol,
        name: "go_to_symbol",
        label: "Go to Symbol",
    },
    CommandInfo {
        command: Command::GoToFile,
        name: "go_to_file",
        label: "Go to File",
    },
    CommandInfo {
        command: Command::Detach,
        name: "detach",
        label: "Detach",
    },
    CommandInfo {
        command: Command::Reconnect,
        name: "reconnect",
        label: "Reconnect",
    },
    CommandInfo {
        command: Command::Quit,
        name: "quit",
        label: "Quit",
    },
    CommandInfo {
        command: Command::About,
        name: "about",
        label: "About Herdr",
    },
    CommandInfo {
        command: Command::NewBrowserTab,
        name: "new_browser_tab",
        label: "New Browser Tab",
    },
    CommandInfo {
        command: Command::OpenOrchestrator,
        name: "open_orchestrator",
        label: "Show Issues",
    },
    CommandInfo {
        command: Command::InstallBrowserSkill,
        name: "install_browser_skill",
        label: "Install Browser Skill for Agents",
    },
    CommandInfo {
        command: Command::SplitEditor,
        name: "split_editor",
        label: "Split Editor",
    },
    CommandInfo {
        command: Command::OpenCode,
        name: "open_code",
        label: "Open VS Code",
    },
    CommandInfo {
        command: Command::ToggleFullScreen,
        name: "toggle_full_screen",
        label: "Toggle Full Screen",
    },
    CommandInfo {
        command: Command::CycleWindows,
        name: "cycle_windows",
        label: "Cycle Through Windows",
    },
    CommandInfo {
        command: Command::CloseWindow,
        name: "close_window",
        label: "Close Window",
    },
];

pub fn request(command: Command, snapshot: &ClientShellSnapshot) -> Option<(Method, Value)> {
    let workspace = snapshot
        .workspaces
        .iter()
        .find(|w| Some(&w.workspace_id) == snapshot.focused_workspace_id.as_ref());
    let tab = workspace.and_then(|w| {
        snapshot.tabs.iter().find(|t| {
            Some(&t.tab_id) == snapshot.focused_tab_id.as_ref() && t.workspace_id == w.workspace_id
        })
    });
    let pane = tab.and_then(|t| {
        snapshot.panes.iter().find(|p| {
            Some(&p.pane_id) == snapshot.focused_pane_id.as_ref()
                && p.tab_id == t.tab_id
                && p.workspace_id == t.workspace_id
        })
    });
    Some(match command {
        Command::Workspace => {
            let mut params = json!({"focus": true});
            if snapshot.focused_workspace_id.is_some() {
                params["source_workspace_id"] = json!(workspace?.workspace_id);
            }
            (Method::WorkspaceCreate, params)
        }
        Command::Tab => (
            Method::TabCreate,
            json!({"workspace_id": workspace?.workspace_id, "focus": true}),
        ),
        Command::SplitRight | Command::SplitDown => (
            Method::PaneSplit,
            json!({
                "target_pane_id": pane?.pane_id,
                "direction": if matches!(command, Command::SplitRight) { "right" } else { "down" },
                "focus": true,
            }),
        ),
        Command::NextTab | Command::PreviousTab => {
            let workspace = &workspace?.workspace_id;
            let tabs: Vec<_> = snapshot
                .tabs
                .iter()
                .filter(|t| &t.workspace_id == workspace)
                .collect();
            let index = tabs
                .iter()
                .position(|t| Some(&t.tab_id) == snapshot.focused_tab_id.as_ref())?;
            let next = if matches!(command, Command::NextTab) {
                (index + 1) % tabs.len()
            } else {
                (index + tabs.len() - 1) % tabs.len()
            };
            (Method::TabFocus, json!({"tab_id": tabs[next].tab_id}))
        }
        Command::FocusLeft | Command::FocusRight | Command::FocusUp | Command::FocusDown => (
            Method::PaneFocusDirection,
            json!({"pane_id": pane?.pane_id, "direction": direction(command)?}),
        ),
        // Herdr nudges the split by its own default step, as its TUI does.
        Command::ResizeLeft | Command::ResizeRight | Command::ResizeUp | Command::ResizeDown => (
            Method::PaneResize,
            json!({"pane_id": pane?.pane_id, "direction": direction(command)?}),
        ),
        Command::SwapLeft | Command::SwapRight | Command::SwapUp | Command::SwapDown => (
            Method::PaneSwap,
            json!({"pane_id": pane?.pane_id, "direction": direction(command)?}),
        ),
        Command::MoveTabPrevious | Command::MoveTabNext => {
            let tab = tab?;
            let tabs: Vec<_> = snapshot
                .tabs
                .iter()
                .filter(|t| t.workspace_id == tab.workspace_id)
                .collect();
            if tabs.len() <= 1 {
                return None;
            }
            let source = tabs.iter().position(|t| t.tab_id == tab.tab_id)?;
            // As Herdr's TUI moves it: the index counts the moving tab still
            // in place, and a tab at either end wraps around to the other.
            let insert_index = if command == Command::MoveTabNext {
                if source + 1 == tabs.len() {
                    0
                } else {
                    source + 2
                }
            } else if source == 0 {
                tabs.len()
            } else {
                source - 1
            };
            (
                Method::TabMove,
                json!({"tab_id": tab.tab_id, "insert_index": insert_index}),
            )
        }
        Command::NextPane | Command::PreviousPane => {
            let pane = pane?;
            let panes: Vec<_> = snapshot
                .panes
                .iter()
                .filter(|p| p.workspace_id == pane.workspace_id && p.tab_id == pane.tab_id)
                .collect();
            // Finding the current pane also guarantees a nonempty cycle.
            let index = panes.iter().position(|p| p.pane_id == pane.pane_id)?;
            let next = if command == Command::NextPane {
                (index + 1) % panes.len()
            } else {
                (index + panes.len() - 1) % panes.len()
            };
            (Method::PaneFocus, json!({"pane_id": panes[next].pane_id}))
        }
        Command::Zoom => (
            Method::PaneZoom,
            json!({"pane_id": pane?.pane_id, "mode": "toggle"}),
        ),
        Command::ClearPane => (Method::PaneClear, json!({"pane_id": pane?.pane_id})),
        Command::EditScrollback => (
            Method::PaneEditScrollback,
            json!({"pane_id": pane?.pane_id}),
        ),
        Command::ClosePane => (Method::PaneClose, json!({"pane_id": pane?.pane_id})),
        Command::CloseTab => (Method::TabClose, json!({"tab_id": tab?.tab_id})),
        // By position, as Herdr's `switch_tab` does: a tab's `number` is an
        // ID that is never reused, so it drifts from the strip once a tab closes.
        Command::TabNumber(number) => {
            let workspace = workspace?;
            let target = snapshot
                .tabs
                .iter()
                .filter(|t| t.workspace_id == workspace.workspace_id)
                .nth(usize::from(number).checked_sub(1)?)?;
            (Method::TabFocus, json!({"tab_id": target.tab_id}))
        }
        Command::NewWindow
        | Command::NewWorktree
        | Command::OpenWorktree
        | Command::RemoveWorktree
        | Command::WorktreeNotes
        | Command::EditWorktreeNote
        | Command::Find
        | Command::FindNext
        | Command::FindPrevious
        | Command::CopyMode
        | Command::AnnotateSelection
        | Command::ToggleSidebar
        | Command::ToggleStatusBar
        | Command::FocusNextRegion
        | Command::FocusPreviousRegion
        | Command::ToggleZenMode
        | Command::DevicesOverview
        | Command::IncreaseFontSize
        | Command::DecreaseFontSize
        | Command::ResetFontSize
        | Command::Settings
        | Command::Keybinds
        | Command::Sessions
        | Command::Themes
        | Command::WorkspacePicker
        | Command::Palette
        | Command::ResumePalette
        | Command::GoToSymbol
        | Command::GoToFile
        | Command::Detach
        | Command::Reconnect
        | Command::Quit
        | Command::Logs
        | Command::About
        | Command::OpenNotificationTarget
        | Command::NewBrowserTab
        | Command::OpenOrchestrator
        | Command::InstallBrowserSkill
        | Command::SplitEditor
        | Command::OpenCode
        | Command::ToggleFullScreen
        | Command::CycleWindows
        | Command::CloseWindow
        // These need state beyond the snapshot, such as the sidebar's order
        // or a dialog, so the window runs them.
        | Command::RenameTab
        | Command::ReopenClosedTab
        | Command::LastPane
        | Command::Back
        | Command::Forward
        | Command::ResizeMode
        | Command::RenamePane
        | Command::MovePane
        | Command::PreviousWorkspace
        | Command::NextWorkspace
        | Command::WorkspaceNumber(_)
        | Command::RenameWorkspace
        | Command::CloseWorkspace
        | Command::PreviousAgent
        | Command::NextAgent
        | Command::AgentNumber(_)
        | Command::ReloadConfig => return None,
    })
}

/// The pane direction a directional command names.
fn direction(command: Command) -> Option<&'static str> {
    Some(match command {
        Command::FocusLeft | Command::SwapLeft | Command::ResizeLeft => "left",
        Command::FocusRight | Command::SwapRight | Command::ResizeRight => "right",
        Command::FocusUp | Command::SwapUp | Command::ResizeUp => "up",
        Command::FocusDown | Command::SwapDown | Command::ResizeDown => "down",
        _ => return None,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
