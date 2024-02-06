use core::fmt;
use once_cell::sync::Lazy;

/// Retain reference to value array.
pub static FSACTION_VALUES: Lazy<Vec<FsAction>> = Lazy::new(FsAction::values);

/// File system actions, e.g. read, write, etc.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FsAction {
    // POSIX style
    None,
    Execute,
    Write,
    WriteExecute,
    Read,
    ReadExecute,
    ReadWrite,
    All,
}

impl FsAction {
    fn values() -> Vec<Self> {
        vec![
            Self::None,
            Self::Execute,
            Self::Write,
            Self::WriteExecute,
            Self::Read,
            Self::ReadExecute,
            Self::ReadWrite,
            Self::All,
        ]
    }

    pub fn ordinal(&self) -> usize {
        *self as usize
    }

    /// Symbolic representation
    pub fn symbol(&self) -> String {
        self.to_string()
    }

    /// Return true if this action implies that action.
    pub fn implies(&self, that: Option<&FsAction>) -> bool {
        match that {
            Some(that) => self.ordinal() & that.ordinal() == that.ordinal(),
            None => false,
        }
    }

    /// AND operation.
    pub fn and(&self, that: &FsAction) -> FsAction {
        FSACTION_VALUES[self.ordinal() & that.ordinal()]
    }

    /// OR operation.
    pub fn or(&self, that: &FsAction) -> FsAction {
        FSACTION_VALUES[self.ordinal() | that.ordinal()]
    }

    /// NOT operation.
    pub fn not(&self) -> FsAction {
        FSACTION_VALUES[7 - self.ordinal()]
    }

    /// Get the FsAction enum for String representation of permissions
    pub fn get_fs_action(permission: &str) -> Option<FsAction> {
        FSACTION_VALUES
            .iter()
            .filter(|a| a.symbol() == permission)
            .next()
            .cloned()
    }
}

impl fmt::Display for FsAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "---"),
            Self::Execute => write!(f, "--x"),
            Self::Write => write!(f, "-w-"),
            Self::WriteExecute => write!(f, "-wx"),
            Self::Read => write!(f, "r--"),
            Self::ReadExecute => write!(f, "r-x"),
            Self::ReadWrite => write!(f, "rw-"),
            Self::All => write!(f, "rwx"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_action() {
        // implies
        FsAction::values()
            .iter()
            .for_each(|a| assert!(FsAction::All.implies(Some(a))));
        FsAction::values().iter().for_each(|a| {
            assert!(if *a == FsAction::None {
                FsAction::None.implies(Some(a))
            } else {
                !FsAction::None.implies(Some(a))
            });
        });
        FsAction::values().iter().for_each(|a| {
            assert!(if *a == FsAction::ReadExecute
                || *a == FsAction::Read
                || *a == FsAction::Execute
                || *a == FsAction::None
            {
                FsAction::ReadExecute.implies(Some(a))
            } else {
                !FsAction::ReadExecute.implies(Some(a))
            });
        });

        // masks
        assert!(FsAction::Execute == FsAction::Execute.and(&FsAction::ReadExecute));
        assert!(FsAction::Read == FsAction::Read.and(&FsAction::ReadExecute));
        assert!(FsAction::None == FsAction::Write.and(&FsAction::ReadExecute));

        assert!(FsAction::Read == FsAction::ReadExecute.and(&FsAction::ReadWrite));
        assert!(FsAction::None == FsAction::ReadExecute.and(&FsAction::Write));
        assert!(FsAction::WriteExecute == FsAction::All.and(&FsAction::WriteExecute));
    }
}
