use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum PmError {
    ConfigLoadFailed,
    ConfigSaveFailed,
    InvalidPath,
    ProjectNotFound,
    ProjectPathNotFound,
    DuplicateProject,
    NoProjectsFound,
    DirectoryChangeFailed,
    DirectoryCreationFailed,
    EditorLaunchFailed,
    InitializationFailed,
    TagOperationFailed,
    ValidationFailed(String),
    GitOperationFailed,
    OperationCancelled,
}

impl fmt::Display for PmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PmError::ConfigLoadFailed => write!(f, "Failed to load configuration"),
            PmError::ConfigSaveFailed => write!(f, "Failed to save configuration"),
            PmError::InvalidPath => write!(f, "Invalid project path"),
            PmError::ProjectNotFound => write!(f, "Project not found"),
            PmError::ProjectPathNotFound => write!(f, "Project path no longer exists"),
            PmError::DuplicateProject => write!(f, "Project already exists"),
            PmError::NoProjectsFound => write!(f, "No projects found"),
            PmError::DirectoryChangeFailed => write!(f, "Failed to change directory"),
            PmError::DirectoryCreationFailed => write!(f, "Failed to create directory"),
            PmError::EditorLaunchFailed => write!(f, "Failed to launch editor"),
            PmError::InitializationFailed => write!(f, "Failed to initialize PM"),
            PmError::TagOperationFailed => write!(f, "Tag operation failed"),
            PmError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            PmError::GitOperationFailed => write!(f, "Git operation failed"),
            PmError::OperationCancelled => write!(f, "Operation cancelled by user"),
        }
    }
}

impl std::error::Error for PmError {}

pub fn handle_error(error: anyhow::Error, context: &str) -> ! {
    eprintln!("❌ {}: {}", context, error);
    std::process::exit(1);
}

/// Handle inquire errors, specifically checking for interruption (Ctrl-C)
/// Returns true if the operation should continue, false if cancelled
pub fn handle_inquire_error<T>(result: Result<T, inquire::InquireError>) -> anyhow::Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(inquire::InquireError::OperationCanceled) => {
            println!("\n💭 Operation cancelled");
            Err(PmError::OperationCancelled.into())
        }
        Err(inquire::InquireError::OperationInterrupted) => {
            println!("\n💭 Operation cancelled");
            Err(PmError::OperationCancelled.into())
        }
        Err(e) => Err(e.into()),
    }
}

#[allow(dead_code)]
pub trait ErrorContext<T> {
    fn with_pm_context(self, context: &str) -> Result<T, anyhow::Error>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_pm_context(self, context: &str) -> Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::Error::new(e).context(context.to_string()))
    }
}
