use util::movement_dir::MovementDir;
use util::artifact::ArtifactDependency;
use util::movement_installer::{MovementInstaller, MovementInstallerOperations};
use crate::known_artifacts::registry;

/// Installs artifacts into a specified movement directory.
pub async fn install(
    movement_dir: MovementDir,
    dependencies: Vec<ArtifactDependency>
) -> Result<MovementDir, anyhow::Error> {
    // Synchronize the movement directory.
    let movement_dir = movement_dir.sync()?;
    
    // Create a new registry instance.
    let registry = registry::Constructor::new().new_registry().await?;
    
    // Create a new MovementInstaller instance.
    let movement_installer = MovementInstaller::new();
    
    // Install artifacts into the movement directory.
    let movement_dir = movement_installer.install(
        movement_dir,
        &registry,
        dependencies
    ).await?;
    
    Ok(movement_dir)
}

/// Retrieves the current movement directory.
pub async fn get_movement_dir() -> Result<MovementDir, anyhow::Error> {
    // Get the default movement directory.
    let movement_dir = MovementDir::default();
    
    // Synchronize the movement directory.
    let movement_dir = movement_dir.sync()?;
    
    Ok(movement_dir)
}

/// Installs artifacts into the default movement directory.
pub async fn install_default(
    dependencies: Vec<ArtifactDependency>
) -> Result<MovementDir, anyhow::Error> {
    // Get the default movement directory.
    let movement_dir = MovementDir::default();
    
    // Synchronize the movement directory.
    let movement_dir = movement_dir.sync()?;
    
    // Create a new registry instance.
    let registry = registry::Constructor::new().new_registry().await?;
    
    // Create a new MovementInstaller instance.
    let movement_installer = MovementInstaller::new();
    
    // Install artifacts into the movement directory.
    let movement_dir = movement_installer.install(
        movement_dir,
        &registry,
        dependencies
    ).await?;
    
    Ok(movement_dir)
}
