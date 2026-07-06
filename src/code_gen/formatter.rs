/// 缩进管理器，用于生成 `.code` 文件的文本内容
///
/// 使用 Tab 字符缩进，提供 `indent`/`dedent` 控制层级。
#[derive(Debug, Clone)]
pub struct CodeFormatter {
    content: String,
    indent_level: usize,
    indent: String,
}

impl Default for CodeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeFormatter {
    /// 创建一个默认使用 Tab 缩进的格式化器
    pub fn new() -> Self {
        Self {
            content: String::new(),
            indent_level: 0,
            indent: "\t".to_string(),
        }
    }

    /// 使用指定缩进字符串创建格式化器
    pub fn with_indent(indent: &str) -> Self {
        Self {
            content: String::new(),
            indent_level: 0,
            indent: indent.to_string(),
        }
    }

    /// 增加一级缩进
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// 减少一级缩进
    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// 在当前缩进层级写入一行文本
    pub fn write_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.content.push_str(&self.indent);
        }
        self.content.push_str(line);
        self.content.push('\n');
    }

    /// 在当前缩进层级写入一行原始文本，不追加换行
    pub fn write_raw(&mut self, text: &str) {
        for _ in 0..self.indent_level {
            self.content.push_str(&self.indent);
        }
        self.content.push_str(text);
    }

    /// 返回当前累积的文本内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 消费格式化器，返回完整文本
    pub fn into_content(self) -> String {
        self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_formatter() {
        let f = CodeFormatter::new();
        assert_eq!(f.content(), "");
    }

    #[test]
    fn test_single_line() {
        let mut f = CodeFormatter::new();
        f.write_line("Log(\"hello\")");
        assert_eq!(f.content(), "Log(\"hello\")\n");
    }

    #[test]
    fn test_indent_and_dedent() {
        let mut f = CodeFormatter::new();
        f.write_line("If(true) [");
        f.indent();
        f.write_line("Log(\"yes\")");
        f.dedent();
        f.write_line("]");
        assert_eq!(f.content(), "If(true) [\n\tLog(\"yes\")\n]\n");
    }

    #[test]
    fn test_dedent_does_not_go_negative() {
        let mut f = CodeFormatter::new();
        f.dedent();
        f.dedent();
        f.write_line("x");
        assert_eq!(f.content(), "x\n");
    }

    #[test]
    fn test_custom_indent() {
        let mut f = CodeFormatter::with_indent("    ");
        f.indent();
        f.write_line("x");
        assert_eq!(f.content(), "    x\n");
    }
}
