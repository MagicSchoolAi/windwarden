use crate::WindWardenError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// WindWarden configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Sort order preset: "official" (default Tailwind) or "custom"
    #[serde(default = "default_sort_order")]
    pub sort_order: String,

    /// Custom category order (used when sort_order = "custom")
    #[serde(default)]
    pub custom_order: Vec<String>,

    /// Preset regex patterns to use: "all", "react", "vue", etc.
    #[serde(default = "default_preset_regex")]
    pub preset_regex: String,

    /// Custom function names to detect (in addition to defaults)
    #[serde(default)]
    pub function_names: Vec<String>,

    /// Custom regex patterns for class extraction
    #[serde(default)]
    pub custom_regex: Vec<String>,

    /// Whether to remove null/undefined classes from output
    #[serde(default = "default_true")]
    pub remove_null_classes: bool,

    /// Whether to preserve duplicate classes (default: false, removes duplicates)
    #[serde(default)]
    pub preserve_duplicates: bool,

    /// Paths to ignore during processing
    #[serde(default = "default_ignore_paths")]
    pub ignore_paths: Vec<String>,

    /// File extensions to process
    #[serde(default = "default_file_extensions")]
    pub file_extensions: Vec<String>,

    /// Custom category definitions
    #[serde(default)]
    pub categories: HashMap<String, Vec<String>>,

    /// Maximum file size to process (in bytes)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,

    /// Number of threads to use (0 = auto-detect)
    #[serde(default)]
    pub threads: usize,

    /// Enable/disable colored output
    #[serde(default = "default_true")]
    pub colored_output: bool,

    /// Default operation mode
    #[serde(default)]
    pub default_mode: Option<String>,

    /// Git integration settings
    #[serde(default)]
    pub git: GitConfig,

    /// Safety settings for file operations
    #[serde(default)]
    pub safety: SafetyConfig,
}

/// Git-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    /// Check if file is in git before processing
    #[serde(default)]
    pub check_git_status: bool,

    /// Only process files in git index
    #[serde(default)]
    pub only_git_files: bool,

    /// Respect .gitignore patterns
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
}

/// Safety-specific configuration for file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyConfig {
    /// Use atomic file operations (write to temp file, then move)
    #[serde(default = "default_true")]
    pub atomic_writes: bool,

    /// Create backup files before overwriting
    #[serde(default)]
    pub create_backups: bool,

    /// Verify file content after writing
    #[serde(default)]
    pub verify_writes: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sort_order: default_sort_order(),
            custom_order: Vec::new(),
            preset_regex: default_preset_regex(),
            function_names: Vec::new(),
            custom_regex: Vec::new(),
            remove_null_classes: true,
            preserve_duplicates: false,
            ignore_paths: default_ignore_paths(),
            file_extensions: default_file_extensions(),
            categories: HashMap::new(),
            max_file_size: default_max_file_size(),
            threads: 0,
            colored_output: true,
            default_mode: None,
            git: GitConfig::default(),
            safety: SafetyConfig::default(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            check_git_status: false,
            only_git_files: false,
            respect_gitignore: true,
        }
    }
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            atomic_writes: true,
            create_backups: false,
            verify_writes: false,
        }
    }
}

// Default value functions
fn default_sort_order() -> String {
    "official".to_string()
}

fn default_preset_regex() -> String {
    "all".to_string()
}

fn default_true() -> bool {
    true
}

fn default_ignore_paths() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "dist".to_string(),
        "build".to_string(),
        "coverage".to_string(),
        ".git".to_string(),
        ".next".to_string(),
        ".nuxt".to_string(),
        "target".to_string(),
    ]
}

fn default_file_extensions() -> Vec<String> {
    vec![
        "tsx".to_string(),
        "jsx".to_string(),
        "ts".to_string(),
        "js".to_string(),
        "vue".to_string(),
        "svelte".to_string(),
    ]
}

fn default_max_file_size() -> usize {
    10 * 1024 * 1024 // 10MB
}

/// Configuration loading and management
pub struct ConfigManager {
    config: Config,
    config_path: Option<PathBuf>,
}

impl ConfigManager {
    /// Create a new ConfigManager with default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            config_path: None,
        }
    }

    /// Create a new ConfigManager with specific config and path
    pub fn new_with_config(config: Config, config_path: Option<PathBuf>) -> Self {
        Self {
            config,
            config_path,
        }
    }

    /// Load configuration from the filesystem
    pub fn load_from_directory(start_dir: &Path) -> Result<Self, WindWardenError> {
        let config_path = Self::find_config_file(start_dir)?;

        let config = if let Some(path) = &config_path {
            Self::load_config_file(path)?
        } else {
            Config::default()
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Find configuration file by walking up the directory tree
    fn find_config_file(start_dir: &Path) -> Result<Option<PathBuf>, WindWardenError> {
        let config_names = [
            ".windwarden.json",
            "windwarden.json",
            ".windwarden.config.json",
        ];

        let mut current_dir = start_dir;

        loop {
            for config_name in &config_names {
                let config_path = current_dir.join(config_name);
                if config_path.exists() {
                    return Ok(Some(config_path));
                }
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => break,
            }
        }

        Ok(None)
    }

    /// Load and parse configuration file
    pub fn load_config_file(path: &Path) -> Result<Config, WindWardenError> {
        let content = fs::read_to_string(path)
            .map_err(|e| WindWardenError::from_io_error(e, Some(&path.display().to_string())))?;

        let config: Config = serde_json::from_str(&content).map_err(|e| {
            WindWardenError::config_error(format!(
                "Invalid configuration in {}: {}",
                path.display(),
                e
            ))
        })?;

        Self::validate_config(&config)?;

        Ok(config)
    }

    /// Validate configuration values
    fn validate_config(config: &Config) -> Result<(), WindWardenError> {
        // Validate sort_order
        if !["official", "custom"].contains(&config.sort_order.as_str()) {
            return Err(WindWardenError::config_error(format!(
                "Invalid sort_order '{}'. Must be 'official' or 'custom'",
                config.sort_order
            )));
        }

        // If custom sort order, ensure custom_order is provided and valid
        if config.sort_order == "custom" {
            if config.custom_order.is_empty() {
                return Err(WindWardenError::config_error(
                    "custom_order must be provided when sort_order is 'custom'",
                ));
            }

            // Validate that all categories in custom_order are known categories
            let available_categories = ConfigManager::get_available_categories();
            let available_set: std::collections::HashSet<&String> =
                available_categories.iter().collect();

            for category in &config.custom_order {
                if !available_set.contains(category) {
                    return Err(WindWardenError::config_error(format!(
                        "Unknown category '{}' in custom_order. Available categories: {}",
                        category,
                        available_categories.join(", ")
                    )));
                }
            }
        }

        // Validate preset_regex
        let valid_presets = ["all", "react", "vue", "svelte", "angular"];
        if !valid_presets.contains(&config.preset_regex.as_str()) {
            return Err(WindWardenError::config_error(format!(
                "Invalid preset_regex '{}'. Valid options: {}",
                config.preset_regex,
                valid_presets.join(", ")
            )));
        }

        // Validate file extensions
        for ext in &config.file_extensions {
            if ext.is_empty() {
                return Err(WindWardenError::config_error(
                    "File extensions cannot be empty",
                ));
            }
        }

        // Validate max_file_size
        if config.max_file_size == 0 {
            return Err(WindWardenError::config_error(
                "max_file_size must be greater than 0",
            ));
        }

        // Validate custom regex patterns
        for regex_pattern in &config.custom_regex {
            if let Err(e) = regex::Regex::new(regex_pattern) {
                return Err(WindWardenError::config_error(format!(
                    "Invalid custom regex '{}': {}",
                    regex_pattern, e
                )));
            }
        }

        // Validate function names
        for func_name in &config.function_names {
            if func_name.is_empty() {
                return Err(WindWardenError::config_error(
                    "Function names cannot be empty",
                ));
            }
            if func_name.contains(char::is_whitespace) {
                return Err(WindWardenError::config_error(format!(
                    "Function name '{}' cannot contain whitespace",
                    func_name
                )));
            }
        }

        // Validate thread count
        if config.threads > 1024 {
            return Err(WindWardenError::config_error(format!(
                "Thread count {} is too high (max: 1024)",
                config.threads
            )));
        }

        // Validate default_mode if provided
        if let Some(ref mode) = config.default_mode {
            let valid_modes = ["format", "check", "diff"];
            if !valid_modes.contains(&mode.as_str()) {
                return Err(WindWardenError::config_error(format!(
                    "Invalid default_mode '{}'. Valid options: {}",
                    mode,
                    valid_modes.join(", ")
                )));
            }
        }

        Ok(())
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the path of the loaded configuration file
    pub fn config_path(&self) -> Option<&PathBuf> {
        self.config_path.as_ref()
    }

    /// Merge configuration with CLI arguments
    pub fn merge_with_cli_args(&mut self, cli_args: &crate::cli::Cli) {
        // Override config with CLI arguments where provided
        if cli_args.stdin {
            // stdin mode doesn't use file-based config much
        }

        // Additional CLI overrides can be added here
    }

    /// Save current configuration to file
    pub fn save_config(&self, path: &Path) -> Result<(), WindWardenError> {
        let content = serde_json::to_string_pretty(&self.config).map_err(|e| {
            WindWardenError::config_error(format!("Failed to serialize config: {}", e))
        })?;

        fs::write(path, content)
            .map_err(|e| WindWardenError::from_io_error(e, Some(&path.display().to_string())))?;

        Ok(())
    }

    /// Create a default configuration file
    pub fn create_default_config(path: &Path) -> Result<(), WindWardenError> {
        let config = Config::default();
        let content = serde_json::to_string_pretty(&config).map_err(|e| {
            WindWardenError::config_error(format!("Failed to serialize default config: {}", e))
        })?;

        fs::write(path, content)
            .map_err(|e| WindWardenError::from_io_error(e, Some(&path.display().to_string())))?;

        Ok(())
    }

    /// Get effective function names (defaults + custom)
    pub fn get_function_names(&self) -> Vec<String> {
        // Use the same defaults as the parser visitor
        let mut names = vec![
            "cn".to_string(),
            "twMerge".to_string(),
            "clsx".to_string(),
            "classNames".to_string(),
            "classList".to_string(),
            "cva".to_string(),
        ];

        names.extend(self.config.function_names.clone());
        names.sort();
        names.dedup();
        names
    }

    /// Get effective ignore patterns
    pub fn get_ignore_patterns(&self) -> Vec<String> {
        self.config.ignore_paths.clone()
    }

    /// Check if a file should be processed based on extension
    pub fn should_process_extension(&self, extension: &str) -> bool {
        self.config
            .file_extensions
            .iter()
            .any(|ext| ext == extension)
    }

    /// Check if a file size is within limits
    pub fn is_file_size_allowed(&self, size: usize) -> bool {
        size <= self.config.max_file_size
    }

    /// Get available Tailwind categories for custom sort order
    pub fn get_available_categories() -> Vec<String> {
        crate::sorter::TailwindSorter::get_default_category_order()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.sort_order, "official");
        assert!(config.remove_null_classes);
        assert!(!config.preserve_duplicates);
        assert!(config.file_extensions.contains(&"tsx".to_string()));
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(ConfigManager::validate_config(&config).is_ok());

        // Test invalid sort_order
        config.sort_order = "invalid".to_string();
        assert!(ConfigManager::validate_config(&config).is_err());

        // Test custom order without custom_order
        config.sort_order = "custom".to_string();
        assert!(ConfigManager::validate_config(&config).is_err());

        // Fix with custom_order
        config.custom_order = vec!["layout".to_string(), "spacing".to_string()];
        assert!(ConfigManager::validate_config(&config).is_ok());
    }

    #[test]
    fn test_function_name_validation() {
        let mut config = Config::default();

        // Test empty function name
        config.function_names = vec!["".to_string()];
        assert!(ConfigManager::validate_config(&config).is_err());

        // Test function name with whitespace
        config.function_names = vec!["my func".to_string()];
        assert!(ConfigManager::validate_config(&config).is_err());

        // Test valid function name
        config.function_names = vec!["myFunc".to_string(), "anotherFunc".to_string()];
        assert!(ConfigManager::validate_config(&config).is_ok());
    }

    #[test]
    fn test_thread_count_validation() {
        let mut config = Config::default();

        // Test excessive thread count
        config.threads = 2000;
        assert!(ConfigManager::validate_config(&config).is_err());

        // Test reasonable thread count
        config.threads = 16;
        assert!(ConfigManager::validate_config(&config).is_ok());

        // Test zero (auto-detect)
        config.threads = 0;
        assert!(ConfigManager::validate_config(&config).is_ok());
    }

    #[test]
    fn test_default_mode_validation() {
        let mut config = Config::default();

        // Test invalid mode
        config.default_mode = Some("invalid".to_string());
        assert!(ConfigManager::validate_config(&config).is_err());

        // Test valid modes
        config.default_mode = Some("format".to_string());
        assert!(ConfigManager::validate_config(&config).is_ok());

        config.default_mode = Some("check".to_string());
        assert!(ConfigManager::validate_config(&config).is_ok());

        config.default_mode = Some("diff".to_string());
        assert!(ConfigManager::validate_config(&config).is_ok());

        // Test None (no default)
        config.default_mode = None;
        assert!(ConfigManager::validate_config(&config).is_ok());
    }

    #[test]
    fn test_config_file_discovery() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("src/components");
        fs::create_dir_all(&nested_dir).unwrap();

        // Create config in root
        let config_path = temp_dir.path().join(".windwarden.json");
        let config = Config::default();
        let content = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, content).unwrap();

        // Search from nested directory should find it
        let found = ConfigManager::find_config_file(&nested_dir).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), config_path);
    }

    #[test]
    fn test_function_names() {
        let manager = ConfigManager::new();
        let function_names = manager.get_function_names();
        assert!(function_names.contains(&"cn".to_string()));
        assert!(function_names.contains(&"clsx".to_string()));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.sort_order, parsed.sort_order);
    }
}
