use crate::config::{load_config, save_config};
use crate::tag_commands::{add_tags, remove_tags, list_tags, show_tags};
use crate::display::display_error;
use crate::error::PmError;
use anyhow::Result;

pub async fn handle_tag_add(project_name: &str, tags: &[String]) -> Result<()> {
    let mut config = load_config().await?;
    
    match add_tags(project_name, tags, &mut config).await {
        Ok(_) => {
            save_config(&config).await?;
            Ok(())
        }
        Err(e) => {
            display_error("Failed to add tags", &e.to_string());
            Err(PmError::TagOperationFailed.into())
        }
    }
}

pub async fn handle_tag_remove(project_name: &str, tags: &[String]) -> Result<()> {
    let mut config = load_config().await?;
    
    match remove_tags(project_name, tags, &mut config).await {
        Ok(_) => {
            save_config(&config).await?;
            Ok(())
        }
        Err(e) => {
            display_error("Failed to remove tags", &e.to_string());
            Err(PmError::TagOperationFailed.into())
        }
    }
}

pub async fn handle_tag_list() -> Result<()> {
    let config = load_config().await?;
    
    match list_tags(&config).await {
        Ok(_) => Ok(()),
        Err(e) => {
            display_error("Failed to list tags", &e.to_string());
            Err(PmError::TagOperationFailed.into())
        }
    }
}

pub async fn handle_tag_show(project_name: Option<&str>) -> Result<()> {
    let config = load_config().await?;
    
    match show_tags(project_name, &config).await {
        Ok(_) => Ok(()),
        Err(e) => {
            display_error("Failed to show tags", &e.to_string());
            Err(PmError::TagOperationFailed.into())
        }
    }
}