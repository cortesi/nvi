//! Utilities for working with Lua code.
//!
//! Contains macros and utilities for executing Lua code through Neovim's RPC interface, and helper
//! functions for string escaping.

/// Execute Lua code with optional arguments
#[macro_export]
macro_rules! lua {
    ($client:expr, $code:expr) => {
        $client.nvim.exec_lua($code, vec![])
    };
    ($client:expr, $code:expr, $($arg:expr),* $(,)?) => {
        $client.nvim.exec_lua(
            $code,
            vec![
                $(serde_rmpv::to_value(&$arg)?),*
            ],
        )
    };
}

/// Execute Lua code with optional arguments, always return a Value
#[macro_export]
macro_rules! lua_exec {
    ($client:expr, $code:expr) => {
        $client.nvim.exec_lua::<$crate::Value>($code, vec![])
    };
    ($client:expr, $code:expr, $($arg:expr),* $(,)?) => {
        $client.nvim.exec_lua::<$crate::Value>(
            $code,
            vec![
                $(serde_rmpv::to_value(&$arg)?),*
            ],
        )
    };
}

/// Escapes a string according to Lua string literal conventions.
/// This escapes special characters like newlines (\n), quotes ('),
/// double quotes ("), and backslashes (\).
pub fn escape_str(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\\' => result.push_str("\\\\"),
            '\'' => result.push_str("\\'"),
            '\"' => result.push_str("\\\""),
            '\0' => result.push_str("\\0"),
            c if c.is_ascii_control() => result.push_str(&format!("\\{}", c as u8)),
            c => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_str() {
        let cases = [
            ("hello", "hello"),
            ("hello\nworld", "hello\\nworld"),
            ("hello\rworld", "hello\\rworld"),
            ("hello\tworld", "hello\\tworld"),
            ("hello\\world", "hello\\\\world"),
            ("hello'world", "hello\\'world"),
            ("hello\"world", "hello\\\"world"),
            ("hello\0world", "hello\\0world"),
            ("hello\x01world", "hello\\1world"),
        ];

        for (input, expected) in cases {
            assert_eq!(escape_str(input), expected);
        }
    }
}
