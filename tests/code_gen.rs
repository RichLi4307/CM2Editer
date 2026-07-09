use CM2Editer::code_gen::generator::{generate_code, generate_code_to_file};
use CM2Editer::error::Result;
use CM2Editer::serializer::json::deserialize_graph;
use std::fs;
use std::path::Path;

#[test]
fn generate_code_from_json_fixture_matches_expected() -> Result<()> {
    let json = fs::read_to_string("tests/fixtures/simple_mission.json")
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    let doc = deserialize_graph(&json)?;
    let code = generate_code(&doc.graph)?;
    let expected = fs::read_to_string("tests/fixtures/simple_mission.code")
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    assert_eq!(code.trim_end(), expected.trim_end());
    Ok(())
}

#[test]
fn generate_code_to_file_creates_file() -> Result<()> {
    let json = fs::read_to_string("tests/fixtures/simple_mission.json")
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    let doc = deserialize_graph(&json)?;

    let output_path = Path::new("tests/fixtures/simple_mission_output.code");
    generate_code_to_file(&doc.graph, output_path)?;

    assert!(output_path.exists());
    let content = fs::read_to_string(output_path)
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    assert!(content.contains("main:"));
    assert!(content.contains("if true"));
    assert!(content.contains("_result = null"));

    fs::remove_file(output_path).ok();
    Ok(())
}

#[test]
fn generated_code_preserves_semantic_elements() -> Result<()> {
    let json = fs::read_to_string("tests/fixtures/simple_mission.json")
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    let doc = deserialize_graph(&json)?;
    let code = generate_code(&doc.graph)?;

    assert!(code.contains("main:"));
    assert!(code.contains("Log(output=\"init\")"));
    assert!(code.contains("if true"));
    assert!(code.contains("Log(output=\"true branch\")"));
    assert!(code.contains("Log(output=\"false branch\")"));
    assert!(code.contains("Log(output=\"end\")"));
    assert!(code.contains("_result = null"));
    Ok(())
}

#[test]
fn audit_auto_ecstasy_json() -> Result<()> {
    let json = fs::read_to_string("tests/fixtures/auto_ecstasy.json")
        .map_err(|e| CM2Editer::error::FlowError::Io(e.to_string()))?;
    let doc = match deserialize_graph(&json) {
        Ok(d) => d,
        Err(e) => {
            println!("Deserialize error: {e:?}");
            return Err(e);
        }
    };
    let code = generate_code(&doc.graph)?;
    println!("=== Generated .code ===\n{}", code);

    assert!(code.contains("var_main_thread"));
    assert!(code.contains("var_check_loop_thread"));
    assert!(code.contains("main:"));
    assert!(code.contains("check_loop:"));
    assert!(code.contains("CreateListener(labelName=\"check_loop\")"));
    assert!(code.contains("_state.Ecstasy"));
    assert!(code.contains("_result = null"));
    Ok(())
}
