use std::fmt;
use std::sync::Arc;

#[derive(Clone, Default)]
pub enum Action {
    Shutdown,
    ChangeTitle(String),
    /// OSC 52: a program requested to store text in the system clipboard.
    ClipboardStore(String),
    /// OSC 52: a program requested the clipboard contents. The formatter
    /// transforms the clipboard text into the escape sequence to write back
    /// to the PTY.
    ClipboardLoad(Arc<dyn Fn(&str) -> String + Sync + Send + 'static>),
    #[default]
    Ignore,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Shutdown => write!(f, "Shutdown"),
            Action::ChangeTitle(t) => write!(f, "ChangeTitle({t})"),
            Action::ClipboardStore(t) => write!(f, "ClipboardStore({t})"),
            Action::ClipboardLoad(_) => write!(f, "ClipboardLoad(<formatter>)"),
            Action::Ignore => write!(f, "Ignore"),
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Action::Shutdown, Action::Shutdown) => true,
            (Action::ChangeTitle(a), Action::ChangeTitle(b)) => a == b,
            (Action::ClipboardStore(a), Action::ClipboardStore(b)) => a == b,
            (Action::Ignore, Action::Ignore) => true,
            // ClipboardLoad contains a closure — two instances are never equal.
            _ => false,
        }
    }
}
