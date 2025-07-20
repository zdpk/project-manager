use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported platforms for extension builds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Platform {
    pub os: OperatingSystem,
    pub arch: Architecture,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperatingSystem {
    Darwin,
    Linux,
    Windows,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Architecture {
    Aarch64,
    X86_64,
}

/// Platform selection for extension development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSelection {
    pub platforms: Vec<Platform>,
}

impl Platform {
    /// Create a new platform
    pub fn new(os: OperatingSystem, arch: Architecture) -> Self {
        Self { os, arch }
    }
    
    /// Get the Rust target triple for this platform
    pub fn rust_target(&self) -> &'static str {
        match (&self.os, &self.arch) {
            (OperatingSystem::Darwin, Architecture::Aarch64) => "aarch64-apple-darwin",
            (OperatingSystem::Darwin, Architecture::X86_64) => {
                // Darwin x86_64 is intentionally not supported as per requirements
                panic!("Darwin x86_64 (Intel Mac) is not supported")
            },
            (OperatingSystem::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (OperatingSystem::Linux, Architecture::X86_64) => "x86_64-unknown-linux-gnu",
            (OperatingSystem::Windows, Architecture::Aarch64) => "aarch64-pc-windows-msvc",
            (OperatingSystem::Windows, Architecture::X86_64) => "x86_64-pc-windows-msvc",
        }
    }
    
    /// Get the GitHub Actions runner OS for this platform
    pub fn github_runner_os(&self) -> &'static str {
        match self.os {
            OperatingSystem::Darwin => "macos-latest",
            OperatingSystem::Linux => "ubuntu-latest", 
            OperatingSystem::Windows => "windows-latest",
        }
    }
    
    /// Get the binary extension for this platform
    pub fn binary_extension(&self) -> &'static str {
        match self.os {
            OperatingSystem::Windows => ".exe",
            _ => "",
        }
    }
    
    /// Get the asset name for GitHub releases
    pub fn asset_name(&self, extension_name: &str) -> String {
        format!("pm-ext-{}-{}{}", extension_name, self, self.binary_extension())
    }
    
    /// Check if this platform is supported
    pub fn is_supported(&self) -> bool {
        // Apple Intel (Darwin x86_64) is not supported
        !matches!((&self.os, &self.arch), (OperatingSystem::Darwin, Architecture::X86_64))
    }
    
    /// Get all supported platforms
    pub fn all_supported() -> Vec<Platform> {
        vec![
            Platform::new(OperatingSystem::Darwin, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::Aarch64),
            Platform::new(OperatingSystem::Linux, Architecture::X86_64),
            Platform::new(OperatingSystem::Windows, Architecture::Aarch64),
            Platform::new(OperatingSystem::Windows, Architecture::X86_64),
        ]
    }
    
    /// Detect current platform
    pub fn current() -> Result<Platform> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        
        let operating_system = match os {
            "macos" => OperatingSystem::Darwin,
            "linux" => OperatingSystem::Linux,
            "windows" => OperatingSystem::Windows,
            _ => return Err(anyhow::anyhow!("Unsupported OS: {}", os)),
        };
        
        let architecture = match arch {
            "aarch64" | "arm64" => Architecture::Aarch64,
            "x86_64" | "x64" => Architecture::X86_64,
            _ => return Err(anyhow::anyhow!("Unsupported architecture: {}", arch)),
        };
        
        let platform = Platform::new(operating_system, architecture);
        
        if !platform.is_supported() {
            return Err(anyhow::anyhow!(
                "Platform {} is not supported. Apple Intel Macs are not supported - Apple Silicon required.",
                platform
            ));
        }
        
        Ok(platform)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.os, self.arch)
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatingSystem::Darwin => write!(f, "darwin"),
            OperatingSystem::Linux => write!(f, "linux"),
            OperatingSystem::Windows => write!(f, "windows"),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::Aarch64 => write!(f, "aarch64"),
            Architecture::X86_64 => write!(f, "x86_64"),
        }
    }
}

impl PlatformSelection {
    /// Create a new empty platform selection
    pub fn new() -> Self {
        Self {
            platforms: Vec::new(),
        }
    }
    
    /// Create a universal platform selection for interpreted languages
    pub fn universal() -> Self {
        // For interpreted languages (Bash, Python), we don't need specific platforms
        // but we still need at least one for the templates to work
        Self {
            platforms: vec![Platform::new(OperatingSystem::Linux, Architecture::X86_64)],
        }
    }
    
    /// Create platform selection with default platforms (current + Linux x86_64)
    pub fn default_selection() -> Result<Self> {
        let mut platforms = vec![];
        
        // Add current platform if supported
        if let Ok(current) = Platform::current() {
            platforms.push(current);
        }
        
        // Add Linux x86_64 as it's most common
        let linux_x64 = Platform::new(OperatingSystem::Linux, Architecture::X86_64);
        if !platforms.contains(&linux_x64) {
            platforms.push(linux_x64);
        }
        
        Ok(Self { platforms })
    }
    
    /// Add a platform to the selection
    pub fn add_platform(&mut self, platform: Platform) {
        if platform.is_supported() && !self.platforms.contains(&platform) {
            self.platforms.push(platform);
        }
    }
    
    /// Remove a platform from the selection
    pub fn remove_platform(&mut self, platform: &Platform) {
        self.platforms.retain(|p| p != platform);
    }
    
    /// Check if selection is valid (at least one platform)
    pub fn is_valid(&self) -> bool {
        !self.platforms.is_empty()
    }
    
    /// Get platforms grouped by OS for display
    pub fn platforms_by_os(&self) -> std::collections::HashMap<OperatingSystem, Vec<Architecture>> {
        let mut grouped = std::collections::HashMap::new();
        
        for platform in &self.platforms {
            grouped
                .entry(platform.os.clone())
                .or_insert_with(Vec::new)
                .push(platform.arch.clone());
        }
        
        grouped
    }
    
    /// Interactive platform selection using inquire
    pub fn interactive_selection() -> Result<Self> {
        println!("🎯 Select target platforms for your extension (minimum 1 required):");
        
        let all_platforms = Platform::all_supported();
        let mut selected_platforms = Vec::new();
        
        // Group platforms by OS for better UX
        let macos_platforms: Vec<_> = all_platforms.iter()
            .filter(|p| p.os == OperatingSystem::Darwin)
            .collect();
        let linux_platforms: Vec<_> = all_platforms.iter()
            .filter(|p| p.os == OperatingSystem::Linux)
            .collect();
        let windows_platforms: Vec<_> = all_platforms.iter()
            .filter(|p| p.os == OperatingSystem::Windows)
            .collect();
        
        // macOS selection
        if !macos_platforms.is_empty() {
            println!("\n🍎 macOS:");
            for platform in macos_platforms {
                let prompt = format!("  {} ({})", 
                    match platform.arch {
                        Architecture::Aarch64 => "Apple Silicon",
                        Architecture::X86_64 => "Intel (Not Supported)",
                    },
                    platform.arch
                );
                
                if platform.is_supported() {
                    if inquire::Confirm::new(&prompt).with_default(true).prompt()? {
                        selected_platforms.push(platform.clone());
                    }
                } else {
                    println!("  ❌ {}", prompt);
                }
            }
        }
        
        // Linux selection
        if !linux_platforms.is_empty() {
            println!("\n🐧 Linux:");
            for platform in linux_platforms {
                let prompt = format!("  {} ({})", 
                    match platform.arch {
                        Architecture::Aarch64 => "ARM64",
                        Architecture::X86_64 => "x86_64",
                    },
                    platform.arch
                );
                
                let default = platform.arch == Architecture::X86_64; // Default to x86_64
                if inquire::Confirm::new(&prompt).with_default(default).prompt()? {
                    selected_platforms.push(platform.clone());
                }
            }
        }
        
        // Windows selection
        if !windows_platforms.is_empty() {
            println!("\n🪟 Windows:");
            for platform in windows_platforms {
                let prompt = format!("  {} ({})", 
                    match platform.arch {
                        Architecture::Aarch64 => "ARM64",
                        Architecture::X86_64 => "x86_64",
                    },
                    platform.arch
                );
                
                let default = platform.arch == Architecture::X86_64; // Default to x86_64
                if inquire::Confirm::new(&prompt).with_default(default).prompt()? {
                    selected_platforms.push(platform.clone());
                }
            }
        }
        
        let selection = Self {
            platforms: selected_platforms,
        };
        
        if !selection.is_valid() {
            return Err(anyhow::anyhow!(
                "At least one platform must be selected. Please run the command again and select platforms."
            ));
        }
        
        println!("\n✅ Selected platforms:");
        for platform in &selection.platforms {
            println!("  • {}", platform);
        }
        
        Ok(selection)
    }
}

impl Default for PlatformSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_creation() {
        let platform = Platform::new(OperatingSystem::Darwin, Architecture::Aarch64);
        assert_eq!(platform.os, OperatingSystem::Darwin);
        assert_eq!(platform.arch, Architecture::Aarch64);
    }
    
    #[test]
    fn test_rust_targets() {
        let darwin_arm = Platform::new(OperatingSystem::Darwin, Architecture::Aarch64);
        assert_eq!(darwin_arm.rust_target(), "aarch64-apple-darwin");
        
        let linux_x64 = Platform::new(OperatingSystem::Linux, Architecture::X86_64);
        assert_eq!(linux_x64.rust_target(), "x86_64-unknown-linux-gnu");
        
        let windows_x64 = Platform::new(OperatingSystem::Windows, Architecture::X86_64);
        assert_eq!(windows_x64.rust_target(), "x86_64-pc-windows-msvc");
    }
    
    #[test]
    fn test_asset_names() {
        let darwin_arm = Platform::new(OperatingSystem::Darwin, Architecture::Aarch64);
        assert_eq!(darwin_arm.asset_name("hooks"), "pm-ext-hooks-darwin-aarch64");
        
        let windows_x64 = Platform::new(OperatingSystem::Windows, Architecture::X86_64);
        assert_eq!(windows_x64.asset_name("hooks"), "pm-ext-hooks-windows-x86_64.exe");
    }
    
    #[test]
    fn test_platform_support() {
        let darwin_arm = Platform::new(OperatingSystem::Darwin, Architecture::Aarch64);
        assert!(darwin_arm.is_supported());
        
        let darwin_x64 = Platform::new(OperatingSystem::Darwin, Architecture::X86_64);
        assert!(!darwin_x64.is_supported()); // Intel Mac not supported
        
        let linux_x64 = Platform::new(OperatingSystem::Linux, Architecture::X86_64);
        assert!(linux_x64.is_supported());
    }
    
    #[test]
    fn test_platform_selection() {
        let mut selection = PlatformSelection::new();
        assert!(!selection.is_valid()); // Empty selection is invalid
        
        let platform = Platform::new(OperatingSystem::Linux, Architecture::X86_64);
        selection.add_platform(platform.clone());
        assert!(selection.is_valid());
        assert!(selection.platforms.contains(&platform));
        
        selection.remove_platform(&platform);
        assert!(!selection.is_valid());
    }
}