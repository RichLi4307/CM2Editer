use crate::graph::graph::Graph;
use crate::serializer::json::{GraphDocument, Viewport};

/// JSON 预览面板。
pub struct JsonPreviewPanel;

impl JsonPreviewPanel {
    /// 显示当前图的 JSON 序列化预览。
    pub fn show(ui: &mut egui::Ui, graph: &Graph, viewport: &Viewport) {
        ui.horizontal(|ui| {
            ui.heading("JSON 预览");
        });

        let doc = GraphDocument::from_graph(
            graph.clone(),
            serde_json::Value::Object(serde_json::Map::new()),
            viewport.clone(),
            Vec::new(),
            Vec::new(),
        );
        let json = match doc.to_json_pretty() {
            Ok(s) => s,
            Err(e) => format!("序列化失败: {}", e),
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut json.clone())
                    .desired_rows(6)
                    .interactive(false)
                    .font(egui::TextStyle::Monospace),
            );
        });
    }
}
