use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::code_gen::generator::generate_code;
use crate::error::{FlowError, Result};
use crate::graph::container::{ContainerGraph, Viewport};
use crate::serializer::json::GraphDocument;

/// 编辑器工程内部数据文件夹名称。
///
/// 该文件夹位于项目根目录下，保存每个 `.code` 文件对应的节点图 JSON。
/// 以 `.` 开头，在文件树中默认被隐藏，避免与游戏加载的真实文件混淆。
pub const EDITOR_DIR: &str = ".cm2editor";

/// 默认代码文件名。
pub const DEFAULT_CODE_FILE: &str = "main";

/// 任务 `meta.json` 数据模型。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissionMeta {
    /// 多语言标题，键为语言代码（如 `En`、`Ja`）。
    #[serde(default)]
    pub title: HashMap<String, String>,
    /// 多语言描述。
    #[serde(default)]
    pub description: HashMap<String, String>,
    /// 任务是否默认启用。
    #[serde(default, rename = "defaultactive")]
    pub default_active: bool,
    /// 设置菜单项列表。
    #[serde(default)]
    pub settings: Vec<Setting>,
}

impl MissionMeta {
    /// 创建带默认英文标题的任务元数据。
    pub fn with_title(title: &str) -> Self {
        let mut map = HashMap::new();
        map.insert("En".to_string(), title.to_string());
        Self {
            title: map,
            description: HashMap::new(),
            default_active: false,
            settings: Vec::new(),
        }
    }
}

/// `meta.json` 中的单条设置项。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum Setting {
    /// 纯文本说明标签。
    Label { title: String },
    /// 整数输入。
    Integer {
        name: String,
        title: String,
        #[serde(default)]
        minvalue: Option<i64>,
        #[serde(default)]
        maxvalue: Option<i64>,
        default: i64,
    },
    /// 浮点数输入。
    Float {
        name: String,
        title: String,
        #[serde(default)]
        minvalue: Option<f64>,
        #[serde(default)]
        maxvalue: Option<f64>,
        default: f64,
    },
    /// 布尔开关。
    Boolean {
        name: String,
        title: String,
        default: bool,
    },
    /// 字符串输入。
    String {
        name: String,
        title: String,
        #[serde(default)]
        default: String,
    },
    /// 枚举下拉框。
    Enum {
        name: String,
        title: String,
        options: Vec<String>,
        default: usize,
    },
}

/// 工程内的一个 `.code` 文件及其对应的节点图。
#[derive(Debug, Clone)]
pub struct CodeFile {
    /// 不含扩展名的文件名，如 `main`。
    pub name: String,
    /// 节点图文档，包含画布状态。
    pub graph_doc: GraphDocument,
    /// 从当前节点图生成的 `.code` 代码。
    pub generated_code: String,
    /// 文本编辑器中显示的 `.code` 代码。
    pub code_text: String,
    /// `code_text` 是否被用户手动修改，保存时优先使用。
    pub code_text_dirty: bool,
}

impl CodeFile {
    /// 在项目根目录下生成 `.code` 文件路径。
    pub fn code_path(&self, root: &Path) -> PathBuf {
        root.join(format!("{}.code", self.name))
    }

    /// 在编辑器内部目录中生成对应的 JSON 路径。
    pub fn graph_path(&self, root: &Path) -> PathBuf {
        root.join(EDITOR_DIR)
            .join(format!("{}.code.json", self.name))
    }

    /// 重新从节点图生成代码，并覆盖当前编辑器文本（丢弃手动修改）。
    pub fn regenerate_code(&mut self) -> Result<()> {
        self.generated_code = generate_code(&self.graph_doc.graph)?;
        self.code_text = self.generated_code.clone();
        self.code_text_dirty = false;
        Ok(())
    }

    /// 更新容器化节点图与画布状态，并重新生成代码。
    pub fn update_container_graph(&mut self, graph: ContainerGraph, viewport: Viewport) -> Result<()> {
        self.graph_doc.graph = graph;
        self.graph_doc.viewport = viewport;
        self.regenerate_code()
    }

    /// 保存 `.code` 文件到磁盘。
    ///
    /// 如果用户手动编辑过代码文本，则优先写入手动编辑的内容；否则写入从节点图生成的代码。
    fn write_code(&self, root: &Path) -> Result<()> {
        let path = self.code_path(root);
        let content = if self.code_text_dirty {
            self.code_text.clone()
        } else {
            self.generated_code.clone()
        };
        std::fs::write(path, content)?;
        Ok(())
    }

    /// 保存内部节点图 JSON 到磁盘。
    fn write_graph(&self, root: &Path) -> Result<()> {
        let path = self.graph_path(root);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = self.graph_doc.to_json_pretty()?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// 一个完整的 Custom Missions 2 工程。
///
/// 工程 = 文件夹 + `meta.json` + 一个或多个 `.code` 文件 + 编辑器内部 `.code.json`。
#[derive(Debug, Clone)]
pub struct Project {
    /// 工程根目录。
    pub root: PathBuf,
    /// `meta.json` 数据。
    pub meta: MissionMeta,
    /// 所有 `.code` 文件。
    pub code_files: Vec<CodeFile>,
    /// 当前在编辑器中打开的 `.code` 文件名。
    pub active_code: String,
    /// `meta.json` 的文本编辑器内容。
    pub meta_text: String,
    /// `meta_text` 是否无法解析为 `MissionMeta`。
    pub meta_text_invalid: bool,
}

impl Project {
    /// 在指定父目录下创建新工程。
    ///
    /// `parent` 为父目录，`name` 为工程文件夹名称。会创建文件夹、默认 `meta.json`、
    /// 默认 `main.code` 及其内部节点图。
    pub fn create(parent: &Path, name: &str) -> Result<Self> {
        let root = parent.join(name);
        std::fs::create_dir_all(&root)?;

        let mut meta = MissionMeta::with_title(name);
        meta.default_active = false;

        let graph_doc = GraphDocument::from_graph(
            ContainerGraph::default_main(),
            Value::Object(serde_json::Map::new()),
            Viewport::default(),
            Vec::new(),
        );
        let mut code_file = CodeFile {
            name: DEFAULT_CODE_FILE.to_string(),
            graph_doc,
            generated_code: String::new(),
            code_text: String::new(),
            code_text_dirty: false,
        };
        code_file.regenerate_code()?;

        let meta_text = serde_json::to_string_pretty(&meta)?;

        let mut project = Self {
            root,
            meta,
            code_files: vec![code_file],
            active_code: DEFAULT_CODE_FILE.to_string(),
            meta_text,
            meta_text_invalid: false,
        };
        project.save()?;
        Ok(project)
    }

    /// 打开已有工程文件夹。
    ///
    /// 读取 `meta.json` 和所有 `.code` 文件。如果缺少内部节点图 JSON，则为该 `.code`
    /// 文件创建空节点图，并将 `.code` 文件内容保留为手动编辑文本（不会立即覆盖）。
    pub fn open(root: PathBuf) -> Result<Self> {
        if !root.is_dir() {
            return Err(FlowError::Validation(format!(
                "Project path is not a directory: {}",
                root.display()
            )));
        }

        let meta = load_meta(&root)?;
        let meta_text = serde_json::to_string_pretty(&meta)?;

        let code_names = collect_code_files(&root)?;
        let mut code_files = Vec::new();
        for name in code_names {
            code_files.push(load_code_file(&root, &name)?);
        }

        if code_files.is_empty() {
            // 没有 .code 文件时，自动创建一个空的 main.code。
            let mut code_file = CodeFile {
                name: DEFAULT_CODE_FILE.to_string(),
                graph_doc: GraphDocument::from_graph(
                    ContainerGraph::default_main(),
                    Value::Object(serde_json::Map::new()),
                    Viewport::default(),
                    Vec::new(),
                ),
                generated_code: String::new(),
                code_text: String::new(),
                code_text_dirty: false,
            };
            code_file.regenerate_code()?;
            code_files.push(code_file);
        }

        let active_code = code_files[0].name.clone();

        Ok(Self {
            root,
            meta,
            code_files,
            active_code,
            meta_text,
            meta_text_invalid: false,
        })
    }

    /// 保存整个工程：写入 `meta.json`、所有 `.code` 文件和内部节点图 JSON。
    pub fn save(&mut self) -> Result<()> {
        self.save_meta()?;
        for code_file in &self.code_files {
            code_file.write_code(&self.root)?;
            code_file.write_graph(&self.root)?;
        }
        Ok(())
    }

    /// 仅保存 `meta.json`。
    fn save_meta(&self) -> Result<()> {
        let path = self.root.join("meta.json");
        let json = serde_json::to_string_pretty(&self.meta)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 导出工程到目标文件夹（通常是游戏的 `CustomMissions2` 目录）。
    ///
    /// 会在目标目录下创建一个与工程同名的子文件夹，复制 `meta.json`、所有 `.code` 文件
    /// 以及 `Images/` 等资源目录。
    pub fn export(&self, destination: &Path) -> Result<()> {
        let project_name = self
            .root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "UnknownProject".to_string());
        let target = destination.join(project_name);
        std::fs::create_dir_all(&target)?;

        // 复制 meta.json
        std::fs::copy(self.root.join("meta.json"), target.join("meta.json"))?;

        // 复制所有 .code 文件
        for code_file in &self.code_files {
            let src = code_file.code_path(&self.root);
            let dst = code_file.code_path(&target);
            std::fs::copy(src, dst)?;
        }

        // 复制 Images/ 等资源目录（如果存在）
        copy_assets(&self.root, &target)?;
        Ok(())
    }

    /// 获取当前激活的 `.code` 文件。
    pub fn active_code_file(&self) -> Option<&CodeFile> {
        self.code_files.iter().find(|c| c.name == self.active_code)
    }

    /// 获取当前激活的 `.code` 文件的可变引用。
    pub fn active_code_file_mut(&mut self) -> Option<&mut CodeFile> {
        self.code_files
            .iter_mut()
            .find(|c| c.name == self.active_code)
    }

    /// 切换当前激活的 `.code` 文件。
    pub fn set_active_code(&mut self, name: &str) -> Result<()> {
        if self.code_files.iter().any(|c| c.name == name) {
            self.active_code = name.to_string();
            Ok(())
        } else {
            Err(FlowError::Validation(format!(
                "No .code files in project: {}",
                name
            )))
        }
    }

    /// 将当前节点图与画布状态同步到激活的 `.code` 文件。
    pub fn sync_active_code(&mut self, graph: ContainerGraph, viewport: Viewport) -> Result<()> {
        if let Some(code_file) = self.active_code_file_mut() {
            code_file.update_container_graph(graph, viewport)?;
        }
        Ok(())
    }

    /// 在当前工程中新建一个 `.code` 文件。
    pub fn add_code_file(&mut self, name: &str) -> Result<()> {
        let sanitized = sanitize_name(name);
        if sanitized.is_empty() {
            return Err(FlowError::Validation("Code file name cannot be empty".to_string()));
        }
        if self.code_files.iter().any(|c| c.name == sanitized) {
            return Err(FlowError::Validation(format!(
                "Code file {} already exists",
                sanitized
            )));
        }

        let mut code_file = CodeFile {
            name: sanitized,
            graph_doc: default_graph_doc(),
            generated_code: String::new(),
            code_text: String::new(),
            code_text_dirty: false,
        };
        code_file.regenerate_code()?;
        let name = code_file.name.clone();
        self.code_files.push(code_file);
        self.code_files.sort_by(|a, b| a.name.cmp(&b.name));
        self.active_code = name;
        Ok(())
    }

    /// 重命名 `.code` 文件。
    ///
    /// 会同时重命名磁盘上的 `.code` 文件和内部 JSON 文件。
    pub fn rename_code_file(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        let new_name = sanitize_name(new_name);
        if new_name.is_empty() {
            return Err(FlowError::Validation("Code file name cannot be empty".to_string()));
        }
        if old_name == new_name {
            return Ok(());
        }
        if self.code_files.iter().any(|c| c.name == new_name) {
            return Err(FlowError::Validation(format!(
                "Code file {} already exists",
                new_name
            )));
        }

        if let Some(code_file) = self.code_files.iter_mut().find(|c| c.name == old_name) {
            // 重命名磁盘文件
            let old_code_path = code_file.code_path(&self.root);
            let new_code_path = self.root.join(format!("{}.code", new_name));
            if old_code_path.exists() {
                std::fs::rename(&old_code_path, &new_code_path)?;
            }
            let old_graph_path = code_file.graph_path(&self.root);
            let new_graph_path = self
                .root
                .join(EDITOR_DIR)
                .join(format!("{}.code.json", new_name));
            if old_graph_path.exists() {
                std::fs::rename(&old_graph_path, &new_graph_path)?;
            }
            code_file.name = new_name.clone();
        }

        if self.active_code == old_name {
            self.active_code = new_name;
        }
        self.code_files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }

    /// 删除 `.code` 文件及其内部节点图。
    pub fn remove_code_file(&mut self, name: &str) -> Result<()> {
        let index = self
            .code_files
            .iter()
            .position(|c| c.name == name)
            .ok_or_else(|| FlowError::Validation(format!("Code file {} does not exist", name)))?;
        let code_file = self.code_files.remove(index);

        let code_path = code_file.code_path(&self.root);
        if code_path.exists() {
            std::fs::remove_file(code_path)?;
        }
        let graph_path = code_file.graph_path(&self.root);
        if graph_path.exists() {
            std::fs::remove_file(graph_path)?;
        }

        if self.active_code == name {
            self.active_code = self
                .code_files
                .first()
                .map(|c| c.name.clone())
                .unwrap_or_default();
        }
        Ok(())
    }

    /// 更新 `meta_text` 并尝试解析为 `MissionMeta`。
    pub fn set_meta_text(&mut self, text: &str) {
        self.meta_text = text.to_string();
        match serde_json::from_str::<MissionMeta>(text) {
            Ok(meta) => {
                self.meta = meta;
                self.meta_text_invalid = false;
            }
            Err(_) => {
                self.meta_text_invalid = true;
            }
        }
    }

    /// 重新生成 `meta_text` 以反映当前 `meta` 的变化。
    pub fn refresh_meta_text(&mut self) -> Result<()> {
        self.meta_text = serde_json::to_string_pretty(&self.meta)?;
        self.meta_text_invalid = false;
        Ok(())
    }
}

/// 创建一个默认的容器化节点图，不包含任何线程或标签。
///
/// 新建 `.code` 文件时使用空图，因为不是每个文件都需要线程/标签。
pub(crate) fn default_graph_doc() -> GraphDocument {
    GraphDocument::from_graph(
        ContainerGraph::default_empty(),
        Value::Object(serde_json::Map::new()),
        Viewport::default(),
        Vec::new(),
    )
}

/// 从磁盘加载 `meta.json`。
fn load_meta(root: &Path) -> Result<MissionMeta> {
    let path = root.join("meta.json");
    if !path.exists() {
        return Ok(MissionMeta::with_title(
            root.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "New Project".to_string())
                .as_str(),
        ));
    }
    let json = std::fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(FlowError::from)
}

/// 收集项目根目录下所有 `.code` 文件的无扩展名名称。
fn collect_code_files(root: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("code") {
                        if let Some(stem) = path.file_stem() {
                            names.push(stem.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    names.sort();
    Ok(names)
}

/// 加载单个 `.code` 文件及其对应的节点图。
fn load_code_file(root: &Path, name: &str) -> Result<CodeFile> {
    let code_path = root.join(format!("{}.code", name));
    let graph_path = root.join(EDITOR_DIR).join(format!("{}.code.json", name));

    let code_text = if code_path.exists() {
        std::fs::read_to_string(&code_path)?
    } else {
        String::new()
    };

    let mut loaded_from_code = false;
    let graph_doc = if graph_path.exists() {
        let json = std::fs::read_to_string(&graph_path)?;
        GraphDocument::from_json(&json)?
    } else {
        loaded_from_code = true;
        GraphDocument::from_graph(
            ContainerGraph::default_empty(),
            Value::Object(serde_json::Map::new()),
            Viewport::default(),
            Vec::new(),
        )
    };

    let mut code_file = CodeFile {
        name: name.to_string(),
        graph_doc,
        generated_code: String::new(),
        code_text,
        code_text_dirty: loaded_from_code,
    };
    code_file.regenerate_code()?;
    Ok(code_file)
}

/// 复制工程资源目录（如 `Images/`）到目标文件夹。
fn copy_assets(root: &Path, target: &Path) -> Result<()> {
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().map(|n| n.to_string_lossy().to_string());
                if let Some(name) = name {
                    // 跳过编辑器内部目录
                    if name == EDITOR_DIR || name.starts_with('.') {
                        continue;
                    }
                    let dst = target.join(&name);
                    copy_dir_all(&path, &dst)?;
                }
            }
        }
    }
    Ok(())
}

/// 递归复制目录。
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let path = entry.path();
        let file_name = path
            .file_name()
            .ok_or_else(|| FlowError::Io(format!("Cannot get file name: {}", path.display())))?;
        let target = dst.join(file_name);
        if path.is_dir() {
            copy_dir_all(&path, &target)?;
        } else {
            std::fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

/// 去除代码文件名中的非法字符与扩展名。
fn sanitize_name(name: &str) -> String {
    let mut result = name.trim().to_string();
    if result.to_lowercase().ends_with(".code") {
        let len = result.len() - 5;
        result.truncate(len);
    }
    let sanitized: String = result
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == ' ' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_meta_roundtrip() -> Result<()> {
        let mut meta = MissionMeta::with_title("Test Mission");
        meta.description
            .insert("En".to_string(), "A test mission".to_string());
        meta.default_active = true;
        meta.settings.push(Setting::Boolean {
            name: "enabled".to_string(),
            title: "Enabled".to_string(),
            default: true,
        });
        meta.settings.push(Setting::Enum {
            name: "mode".to_string(),
            title: "Mode".to_string(),
            options: vec!["A".to_string(), "B".to_string()],
            default: 0,
        });

        let json = serde_json::to_string_pretty(&meta)?;
        let meta2: MissionMeta = serde_json::from_str(&json)?;
        assert_eq!(meta.title, meta2.title);
        assert_eq!(meta.description, meta2.description);
        assert_eq!(meta.default_active, meta2.default_active);
        assert_eq!(meta.settings.len(), meta2.settings.len());
        Ok(())
    }

    #[test]
    fn test_setting_label_deserialization() -> Result<()> {
        let json = r#"{"type":"Label","title":"Hello"}"#;
        let setting: Setting = serde_json::from_str(json)?;
        assert!(matches!(setting, Setting::Label { title } if title == "Hello"));
        Ok(())
    }

    #[test]
    fn test_project_create_and_open() -> Result<()> {
        let temp = std::env::temp_dir().join(format!("cm2_test_{}", uuid::Uuid::new_v4()));
        {
            let project = Project::create(&temp, "MyMission")?;
            assert!(project.root.join("meta.json").exists());
            assert!(project.root.join("main.code").exists());
            assert!(
                project
                    .root
                    .join(EDITOR_DIR)
                    .join("main.code.json")
                    .exists()
            );
        }
        let project = Project::open(temp.join("MyMission"))?;
        assert_eq!(project.code_files.len(), 1);
        assert_eq!(project.active_code, "main");
        assert!(project.meta.title.contains_key("En"));

        // 清理
        let _ = std::fs::remove_dir_all(&project.root);
        Ok(())
    }

    #[test]
    fn test_project_add_rename_remove_code() -> Result<()> {
        let temp = std::env::temp_dir().join(format!("cm2_test_{}", uuid::Uuid::new_v4()));
        let mut project = Project::create(&temp, "Mission")?;
        project.add_code_file("second")?;
        assert_eq!(project.code_files.len(), 2);
        project.rename_code_file("second", "renamed")?;
        assert!(project.code_files.iter().any(|c| c.name == "renamed"));
        project.remove_code_file("renamed")?;
        assert_eq!(project.code_files.len(), 1);
        let _ = std::fs::remove_dir_all(&project.root);
        Ok(())
    }

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("  hello.world  "), "hello_world");
        assert_eq!(sanitize_name("main.code"), "main");
        assert_eq!(sanitize_name("a/b"), "a_b");
    }

    #[test]
    fn test_project_save_and_export() -> Result<()> {
        use crate::graph::node::{Node, Vec2};
        use crate::graph::types::NodeType;

        let temp = std::env::temp_dir().join(format!("cm2_test_{}", uuid::Uuid::new_v4()));
        let mut project = Project::create(&temp, "Mission")?;

        if let Some(code_file) = project.active_code_file_mut() {
            let node = Node::new(NodeType::Log, Vec2::default());
            code_file
                .graph_doc
                .graph
                .threads[0]
                .labels[0]
                .nodes
                .insert(node.id.clone(), node);
        }

        project.save()?;
        let graph_path = project.root.join(EDITOR_DIR).join("main.code.json");
        assert!(graph_path.exists());
        let content = std::fs::read_to_string(&graph_path)?;
        assert!(content.contains("2.0"));
        assert!(content.contains("Log"));

        let export_dir = temp.join("export");
        project.export(&export_dir)?;
        let exported_project = export_dir.join("Mission");
        assert!(exported_project.join("meta.json").exists());
        assert!(exported_project.join("main.code").exists());

        let _ = std::fs::remove_dir_all(&temp);
        Ok(())
    }

    #[test]
    fn test_new_code_file_starts_empty() -> Result<()> {
        let temp = std::env::temp_dir().join(format!("cm2_test_{}", uuid::Uuid::new_v4()));
        let mut project = Project::create(&temp, "Mission")?;

        // 新建项目的主文件仍保留默认 main 线程，便于快速开始。
        assert_eq!(project.code_files[0].graph_doc.graph.threads.len(), 1);

        // 通过 add_code_file 创建的附加文件应为空图，不强制包含线程。
        project.add_code_file("empty")?;
        let empty_file = project
            .code_files
            .iter()
            .find(|c| c.name == "empty")
            .expect("empty file exists");
        assert_eq!(empty_file.graph_doc.graph.threads.len(), 0);

        let _ = std::fs::remove_dir_all(&project.root);
        Ok(())
    }
}

