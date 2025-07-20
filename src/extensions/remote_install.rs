use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

/// Install Rust extension from extracted directory
pub async fn install_rust_extension_from_extracted(ext_dir: &Path) -> Result<()> {
    // This reuses the existing logic from handle_local_install for Rust extensions
    let cargo_toml_path = ext_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(anyhow::anyhow!("Cargo.toml not found in extension directory"));
    }
    
    // Build the extension
    let output = std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(ext_dir)
        .output()
        .context("Failed to execute cargo build")?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to build Rust extension: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Find and copy the binary
    let target_dir = ext_dir.join("target/release");
    let cargo_content = fs::read_to_string(&cargo_toml_path).await?;
    let binary_name = extract_binary_name_from_cargo_toml(&cargo_content)?;
    
    let binary_path = target_dir.join(&binary_name);
    if !binary_path.exists() {
        return Err(anyhow::anyhow!("Built binary not found at: {}", binary_path.display()));
    }
    
    // Copy binary to extension directory
    let target_binary = ext_dir.join("binary");
    fs::copy(&binary_path, &target_binary).await
        .context("Failed to copy built binary")?;
    
    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_binary).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_binary, perms).await?;
    }
    
    Ok(())
}

/// Install Python extension from extracted directory
pub async fn install_python_extension_from_extracted(ext_dir: &Path) -> Result<()> {
    // Create Python wrapper script
    let python_dir = ext_dir.join("python");
    let main_py = if python_dir.exists() {
        python_dir.join("main.py")
    } else {
        ext_dir.join("main.py")
    };
    
    if !main_py.exists() {
        return Err(anyhow::anyhow!("Python main.py not found in extension"));
    }
    
    // Create wrapper script
    let wrapper_content = format!(
        r#"#!/usr/bin/env python3
import sys
import os

# Add extension directory to Python path
extension_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, extension_dir)

# Import and run the main module
try:
    if os.path.exists(os.path.join(extension_dir, "python", "main.py")):
        sys.path.insert(0, os.path.join(extension_dir, "python"))
    
    import main
    if hasattr(main, "main"):
        main.main(sys.argv[1:])
    else:
        print("Error: main.py does not have a main() function", file=sys.stderr)
        sys.exit(1)
except ImportError as e:
    print(f"Error importing main module: {{e}}", file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f"Error running extension: {{e}}", file=sys.stderr)
    sys.exit(1)
"#
    );
    
    let wrapper_path = ext_dir.join("binary");
    fs::write(&wrapper_path, wrapper_content).await
        .context("Failed to create Python wrapper script")?;
    
    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper_path, perms).await?;
    }
    
    Ok(())
}

/// Install Bash extension from extracted directory
pub async fn install_bash_extension_from_extracted(ext_dir: &Path) -> Result<()> {
    // Find main bash script
    let bash_dir = ext_dir.join("bash");
    let main_script = bash_dir.join("main.sh");
    
    if !main_script.exists() {
        return Err(anyhow::anyhow!("Bash main.sh not found in extension"));
    }
    
    // Create wrapper script
    let wrapper_content = format!(
        r#"#!/bin/bash
EXTENSION_DIR="$(cd "$(dirname "${{BASH_SOURCE[0]}}")" && pwd)"
export EXTENSION_DIR

# Source the main script
source "$EXTENSION_DIR/bash/main.sh" "$@"
"#
    );
    
    let wrapper_path = ext_dir.join("binary");
    fs::write(&wrapper_path, wrapper_content).await
        .context("Failed to create Bash wrapper script")?;
    
    // Set executable permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper_path, perms).await?;
    }
    
    Ok(())
}

/// Extract binary name from Cargo.toml
fn extract_binary_name_from_cargo_toml(content: &str) -> Result<String> {
    // Simple parsing to find [[bin]] name
    for line in content.lines() {
        if line.trim().starts_with("name =") && line.contains("=") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }
    
    // Fallback: extract from [package] name
    for line in content.lines() {
        if line.trim().starts_with("name =") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Could not extract binary name from Cargo.toml"))
}