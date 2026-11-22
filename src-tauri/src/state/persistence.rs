use crate::state::connection::ConnectionData;
use crate::state::workspace::LabeqWorkspace;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceState {
    pub schema_version: u32,
    pub color_palette: String,
    pub mode: String,
}

impl AppearanceState {
    pub fn load_default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            color_palette: "default".to_string(),
            mode: "light".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeneralState {
    pub schema_version: u32,
    pub active_workspace_id: Option<String>,
}

impl GeneralState {
    pub fn load_default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            active_workspace_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersistedConnection {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub mqtt_url: String,
    pub api_url: String,
    pub username: String,
    pub password: String,
}

impl From<&ConnectionData> for PersistedConnection {
    fn from(connection: &ConnectionData) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            id: connection.id.clone(),
            name: connection.name.clone(),
            mqtt_url: connection.mqtt_url.clone(),
            api_url: connection.api_url.clone(),
            username: connection.username.clone(),
            password: connection.password.clone(),
        }
    }
}

impl From<PersistedConnection> for ConnectionData {
    fn from(connection: PersistedConnection) -> Self {
        Self {
            id: connection.id,
            name: connection.name,
            mqtt_url: connection.mqtt_url,
            api_url: connection.api_url,
            username: connection.username,
            password: connection.password,
            state: crate::state::connection::ConnectionState::Disconnected,
            devices: Vec::new(),
            scripts: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub schema_version: u32,
    pub application: String,
    pub connections: Vec<String>,
    pub workspaces: Vec<String>,
}

pub struct StateStore {
    root: PathBuf,
}

impl StateStore {
    pub fn new(app_data_dir: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(app_data_dir.join("state"))?;
        fs::create_dir_all(app_data_dir.join("connections"))?;
        fs::create_dir_all(app_data_dir.join("workspaces"))?;
        fs::create_dir_all(app_data_dir.join("recovery").join("workspaces"))?;
        Ok(Self { root: app_data_dir })
    }

    pub fn load(&self) -> io::Result<(Vec<ConnectionData>, Vec<LabeqWorkspace>, GeneralState, AppearanceState)> {
        let connections = self.load_connections()?;
        let saved = self.load_workspaces()?;
        let recovery = self.load_recovery_workspaces().unwrap_or_else(|_| Vec::new());
        let mut workspaces = saved;
        for recovered in recovery {
            if !workspaces.iter().any(|workspace| workspace.id == recovered.id) {
                workspaces.push(recovered);
            }
        }
        let general = self.load_general_state().unwrap_or_else(|_| GeneralState::load_default());
        let appearance = self.load_appearance().unwrap_or_else(|_| AppearanceState::load_default());
        Ok((connections, workspaces, general, appearance))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn save_general_state(&self, state: &GeneralState) -> io::Result<()> {
        self.save_json(
            &self.root.join("state").join("state.json"),
            state,
        )
    }

    pub fn load_general_state(&self) -> io::Result<GeneralState> {
        self.load_json(&self.root.join("state").join("state.json"))
    }

    pub fn save_appearance(&self, state: &AppearanceState) -> io::Result<()> {
        self.save_json(
            &self.root.join("state").join("appearance.json"),
            state,
        )
    }

    pub fn load_appearance(&self) -> io::Result<AppearanceState> {
        self.load_json(&self.root.join("state").join("appearance.json"))
    }

    pub fn save_connection(&self, connection: &ConnectionData) -> io::Result<()> {
        let persisted = PersistedConnection::from(connection);
        let path = self.root
            .join("connections")
            .join(format!("{}.json", sanitize_file_name(&connection.id)));
        self.save_json(&path, &persisted)?;
        self.update_manifest()
    }

    pub fn delete_connection(&self, connection_id: &str) -> io::Result<()> {
        let path = self.root
            .join("connections")
            .join(format!("{}.json", sanitize_file_name(connection_id)));
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        self.update_manifest()
    }

    pub fn load_connections(&self) -> io::Result<Vec<ConnectionData>> {
        let directory = self.root.join("connections");
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut connections: Vec<ConnectionData> = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            match self.load_json::<PersistedConnection>(&path) {
                Ok(connection) => {
                    if connection.schema_version == SCHEMA_VERSION {
                        connections.push(connection.into());
                    }
                }
                Err(error) => eprintln!("Failed to load connection {}: {}", path.display(), error),
            }
        }
        connections.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(connections)
    }

    pub fn save_workspace(&self, workspace: &LabeqWorkspace) -> io::Result<()> {
        let directory = self.root
            .join("workspaces")
            .join(sanitize_file_name(&workspace.id));
        fs::create_dir_all(&directory)?;
        self.save_json(
            &directory.join("workspace.json"),
            workspace,
        )?;
        self.update_manifest()
    }

    pub fn delete_workspace(&self, workspace_id: &str) -> io::Result<()> {
        let directory = self.root
            .join("workspaces")
            .join(sanitize_file_name(workspace_id));
        match fs::remove_dir_all(directory) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        self.update_manifest()
    }

    pub fn save_workspaces(&self, workspaces: &[LabeqWorkspace]) -> io::Result<()> {
        let workspaces_root = self.root.join("workspaces");
        fs::create_dir_all(&workspaces_root)?;

        let retained_ids: std::collections::HashSet<String> =
            workspaces.iter().map(|workspace| workspace.id.clone()).collect();
        for entry in fs::read_dir(&workspaces_root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let directory_name = entry.file_name();
                let directory_name = directory_name.to_string_lossy();
                if retained_ids.iter().all(|id| sanitize_file_name(id) != directory_name) {
                    fs::remove_dir_all(entry.path())?;
                }
            }
        }

        for workspace in workspaces {
            self.save_workspace(workspace)?;
        }
        self.update_manifest()
    }

    pub fn load_workspaces(&self) -> io::Result<Vec<LabeqWorkspace>> {
        let directory = self.root.join("workspaces");
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut workspaces = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let path = entry.path().join("workspace.json");
            if !path.exists() {
                continue;
            }
            match self.load_json::<LabeqWorkspace>(&path) {
                Ok(workspace) => workspaces.push(workspace),
                Err(error) => eprintln!("Failed to load workspace {}: {}", path.display(), error),
            }
        }
        workspaces.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(workspaces)
    }

    pub fn save_recovery_workspaces(&self, workspaces: &[LabeqWorkspace]) -> io::Result<()> {
        let recovery_root = self.root.join("recovery").join("workspaces");
        fs::create_dir_all(&recovery_root)?;

        let retained_ids: std::collections::HashSet<String> =
            workspaces.iter().map(|workspace| workspace.id.clone()).collect();
        for entry in fs::read_dir(&recovery_root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let directory_name = entry.file_name();
                let directory_name = directory_name.to_string_lossy();
                if retained_ids.iter().all(|id| sanitize_file_name(id) != directory_name) {
                    fs::remove_dir_all(entry.path())?;
                }
            }
        }

        for workspace in workspaces {
            let directory = recovery_root.join(sanitize_file_name(&workspace.id));
            fs::create_dir_all(&directory)?;
            self.save_json(&directory.join("workspace.json"), workspace)?;
        }
        Ok(())
    }

    pub fn load_recovery_workspaces(&self) -> io::Result<Vec<LabeqWorkspace>> {
        let directory = self.root.join("recovery").join("workspaces");
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut workspaces = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let path = entry.path().join("workspace.json");
            if !path.exists() {
                continue;
            }
            match self.load_json::<LabeqWorkspace>(&path) {
                Ok(workspace) => workspaces.push(workspace),
                Err(error) => eprintln!("Failed to load recovery workspace {}: {}", path.display(), error),
            }
        }
        workspaces.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(workspaces)
    }

    pub fn clear_recovery(&self) -> io::Result<()> {
        let directory = self.root.join("recovery").join("workspaces");
        if directory.exists() {
            fs::remove_dir_all(&directory)?;
        }
        fs::create_dir_all(&directory)
    }

    pub fn clear_workspace_recovery(&self, workspace_id: &str) -> io::Result<()> {
        let directory = self.root
            .join("recovery")
            .join("workspaces")
            .join(sanitize_file_name(workspace_id));
        match fs::remove_dir_all(directory) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn save_workspace_file(&self, workspace: &LabeqWorkspace, path: &Path) -> io::Result<()> {
        self.save_json(path, workspace)
    }

    pub fn load_workspace_file(&self, path: &Path) -> io::Result<LabeqWorkspace> {
        self.load_json(path)
    }

    pub fn update_manifest(&self) -> io::Result<()> {
        let connections = self.load_connections()?
            .into_iter()
            .map(|connection| connection.id)
            .collect();
        let workspaces = self.load_workspaces()?
            .into_iter()
            .map(|workspace| workspace.id)
            .collect();
        let manifest = Manifest {
            schema_version: SCHEMA_VERSION,
            application: "labeq-monitor".to_string(),
            connections,
            workspaces,
        };
        self.save_json(&self.root.join("manifest.json"), &manifest)
    }

    pub fn export_to(&self, destination: &Path) -> io::Result<()> {
        if destination.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Export destination already exists",
            ));
        }
        copy_directory(&self.root, destination)
    }

    pub fn import_from(&self, source: &Path, replace: bool) -> io::Result<()> {
        if !source.join("manifest.json").exists() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Import source has no manifest.json",
            ));
        }
        if replace {
            for directory in ["state", "connections", "workspaces"] {
                let destination = self.root.join(directory);
                if destination.exists() {
                    fs::remove_dir_all(&destination)?;
                }
                fs::create_dir_all(&destination)?;
            }
        }
        for directory in ["state", "connections", "workspaces"] {
            let source_directory = source.join(directory);
            if source_directory.exists() {
                copy_directory_contents(&source_directory, &self.root.join(directory))?;
            }
        }
        copy_file_if_exists(&source.join("manifest.json"), &self.root.join("manifest.json"))?;
        self.update_manifest()
    }

    fn save_json<T: Serialize>(&self, path: &Path, value: &T) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_vec_pretty(value)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let temp = path.with_extension("json.tmp");
        fs::write(&temp, json)?;
        match fs::rename(&temp, path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists
                || error.kind() == io::ErrorKind::PermissionDenied =>
            {
                fs::remove_file(path)?;
                fs::rename(&temp, path)?;
            }
            Err(error) => {
                let _ = fs::remove_file(&temp);
                return Err(error);
            }
        }
        Ok(())
    }

    fn load_json<T: for<'de> Deserialize<'de>>(&self, path: &Path) -> io::Result<T> {
        let data = fs::read(path)?;
        serde_json::from_slice(&data)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }
}

fn sanitize_file_name(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn copy_file_if_exists(source: &Path, destination: &Path) -> io::Result<()> {
    if source.exists() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, destination)?;
    }
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    copy_directory_contents(source, destination)
}

fn copy_directory_contents(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory_contents(&source_path, &destination_path)?;
        } else {
            fs::copy(source_path, destination_path)?;
        }
    }
    Ok(())
}
