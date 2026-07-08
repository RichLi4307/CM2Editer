//! Phase 4.2 — 用 `docs/examples/` 全部 4 个示例完整验证序列化与代码生成
//!
//! 每个示例对应一个 `tests/fixtures/example_<name>.json` 图档（提取该示例中编辑器
//! 当前支持的功能子集），验证：
//! 1. JSON 反序列化 → 序列化 → 反序列化的结构保持（节点/边/标签/参数）
//! 2. 反序列化 → 代码生成，与 `example_<name>.code` 期望输出逐行一致
//! 3. 生成的代码包含示例的关键语义片段

use CM2Editer::code_gen::generator::generate_code;
use CM2Editer::error::{FlowError, Result};
use CM2Editer::serializer::json::deserialize_graph;
use std::fs;
use std::path::Path;

/// 读取 fixture 文件内容
fn read_fixture(name: &str) -> Result<String> {
    let path = format!("tests/fixtures/{name}");
    fs::read_to_string(&path).map_err(|e| FlowError::Io(e.to_string()))
}

/// 加载图档并执行序列化往返：原 JSON → GraphDocument → JSON → GraphDocument，
/// 断言两次反序列化得到的节点数、边数与标签一致。
fn assert_serialization_roundtrip(json: &str) -> Result<()> {
    let doc1 = deserialize_graph(json)?;
    let reparsed = doc1.to_json_pretty()?;
    let doc2 = deserialize_graph(&reparsed)?;

    assert_eq!(
        doc1.graph.nodes.len(),
        doc2.graph.nodes.len(),
        "节点数在序列化往返后不一致"
    );
    assert_eq!(
        doc1.graph.edges.len(),
        doc2.graph.edges.len(),
        "边数在序列化往返后不一致"
    );
    assert_eq!(
        doc1.graph.labels.len(),
        doc2.graph.labels.len(),
        "标签数在序列化往返后不一致"
    );

    // 逐节点比对类型与参数
    for (id, n1) in &doc1.graph.nodes {
        let n2 = doc2
            .graph
            .nodes
            .get(id)
            .ok_or_else(|| FlowError::Validation(format!("节点 {id} 在往返后丢失")))?;
        assert_eq!(n1.node_type, n2.node_type, "节点 {id} 类型改变");
        assert_eq!(n1.params, n2.params, "节点 {id} 参数改变");
    }
    Ok(())
}

/// 加载图档并生成代码，断言与期望 `.code` 文件内容一致（去除尾部空白）。
fn assert_code_matches(json_name: &str, code_name: &str) -> Result<String> {
    let json = read_fixture(json_name)?;
    let doc = deserialize_graph(&json)?;
    let code = generate_code(&doc.graph)?;
    let expected = read_fixture(code_name)?;
    assert_eq!(
        code.trim_end(),
        expected.trim_end(),
        "生成的代码与期望 {code_name} 不一致"
    );
    Ok(code)
}

// ── 示例 1：Test（多标签 + CreateThread + CreateListener） ──

#[test]
fn example_test_serialization_roundtrip() -> Result<()> {
    let json = read_fixture("example_test.json")?;
    assert_serialization_roundtrip(&json)
}

#[test]
fn example_test_code_generation_matches_expected() -> Result<()> {
    let code = assert_code_matches("example_test.json", "example_test.code")?;
    assert!(code.contains("main:"));
    assert!(code.contains("testmission:"));
    assert!(code.contains("CreateThread(labelName=\"testmission\")"));
    assert!(code.contains("CreateListener(labelName=\"reset\")"));
    Ok(())
}

// ── 示例 2：NPC_type（三线程并发启动） ──

#[test]
fn example_npc_type_serialization_roundtrip() -> Result<()> {
    let json = read_fixture("example_npc_type.json")?;
    assert_serialization_roundtrip(&json)
}

#[test]
fn example_npc_type_code_generation_matches_expected() -> Result<()> {
    let code = assert_code_matches("example_npc_type.json", "example_npc_type.code")?;
    assert!(code.contains("CreateThread(labelName=\"main\")"));
    assert!(code.contains("CreateThread(labelName=\"walk\")"));
    assert!(code.contains("CreateThread(labelName=\"blind\")"));
    Ok(())
}

// ── 示例 3：drop bra and panties（CreateCondition + If 分支汇合） ──

#[test]
fn example_drop_bra_serialization_roundtrip() -> Result<()> {
    let json = read_fixture("example_drop_bra.json")?;
    assert_serialization_roundtrip(&json)
}

#[test]
fn example_drop_bra_code_generation_matches_expected() -> Result<()> {
    let code = assert_code_matches("example_drop_bra.json", "example_drop_bra.code")?;
    assert!(code.contains("CreateCondition(condition=\"[Exposed_All]\", id=\"exposed\")"));
    assert!(code.contains("if true"));
    assert!(code.contains("Log(output=\"bra off\")"));
    assert!(code.contains("Log(output=\"bra on\")"));
    assert!(code.contains("Log(output=\"done\")"));
    Ok(())
}

// ── 示例 4：MessengerExample（多标签 + Goto + CreateMessengerChat） ──

#[test]
fn example_messenger_serialization_roundtrip() -> Result<()> {
    let json = read_fixture("example_messenger.json")?;
    assert_serialization_roundtrip(&json)
}

#[test]
fn example_messenger_code_generation_matches_expected() -> Result<()> {
    let code = assert_code_matches("example_messenger.json", "example_messenger.code")?;
    assert!(code.contains("main:"));
    assert!(code.contains("step1:"));
    assert!(code.contains("CreateMessengerChat(title=\"Messenger Example\")"));
    assert!(code.contains("Goto(\"step1\")"));
    Ok(())
}

// ── 端到端：生成代码写入临时文件并回读 ──

#[test]
fn example_test_generate_to_file_and_reload() -> Result<()> {
    let json = read_fixture("example_test.json")?;
    let doc = deserialize_graph(&json)?;

    let output_path = Path::new("tests/fixtures/example_test_output.code");
    CM2Editer::code_gen::generator::generate_code_to_file(&doc.graph, output_path)?;

    assert!(output_path.exists());
    let content = fs::read_to_string(output_path).map_err(|e| FlowError::Io(e.to_string()))?;
    let expected = read_fixture("example_test.code")?;
    assert_eq!(content.trim_end(), expected.trim_end());

    fs::remove_file(output_path).ok();
    Ok(())
}
