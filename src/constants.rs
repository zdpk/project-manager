// Application constants
pub const APP_NAME: &str = "pm";
pub const CONFIG_FILENAME: &str = "config.yml";
pub const CONFIG_SCHEMA_PATH: &str = "schemas/config.schema.json";
pub const CONFIG_VERSION: &str = "1.0";

// Legacy support
pub const LEGACY_CONFIG_FILENAME: &str = "config.json";

// Default values
pub const DEFAULT_WORKSPACE_DIR: &str = "~/workspace";
pub const DEFAULT_EDITOR: &str = "hx";
pub const DEFAULT_RECENT_DAYS: i64 = 7;

// Time constants
pub const GIT_UPDATE_INTERVAL_HOURS: i64 = 1;

// Display constants
pub const PROJECT_NAME_WIDTH: usize = 20;
pub const PROJECT_TAGS_WIDTH: usize = 30;
pub const PROJECT_TIME_WIDTH: usize = 20;

// Error messages
pub const ERROR_CONFIG_LOAD: &str = "Failed to load configuration";
pub const ERROR_CONFIG_SAVE: &str = "Failed to save configuration";
pub const ERROR_INVALID_PATH: &str = "Invalid project path";
pub const ERROR_PROJECT_NOT_FOUND: &str = "Project not found";
pub const ERROR_DUPLICATE_PROJECT: &str = "Project already exists";
pub const ERROR_DIRECTORY_CHANGE: &str = "Failed to change directory";
pub const ERROR_EDITOR_LAUNCH: &str = "Failed to launch editor";

// Success messages
pub const SUCCESS_PROJECT_ADDED: &str = "Project added successfully";
pub const SUCCESS_PROJECT_SWITCHED: &str = "Project switched";
pub const SUCCESS_PM_INITIALIZED: &str = "PM initialized successfully";

// Suggestion messages
pub const SUGGESTION_CHECK_PATH: &str = "Check if the path is correct";
pub const SUGGESTION_CREATE_DIRECTORY: &str = "Create the directory first";
pub const SUGGESTION_USE_PM_LS: &str = "Use 'pm ls' to see all available projects";
pub const SUGGESTION_ADD_FIRST_PROJECT: &str = "Add your first project with: pm add <path>";
pub const SUGGESTION_INSTALL_HELIX: &str = "Install Helix editor: https://helix-editor.com/";
pub const SUGGESTION_USE_NO_EDITOR: &str = "Use --no-editor flag to skip editor";
pub const SUGGESTION_SET_EDITOR_ENV: &str = "Set EDITOR environment variable to your preferred editor";