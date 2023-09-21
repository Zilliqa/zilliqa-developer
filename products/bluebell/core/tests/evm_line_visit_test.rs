#[cfg(test)]
mod tests {
    use bluebell::testing::test_execution_path;

    #[test]
    fn test_visiting() {
        test_execution_path(
            "HelloWorld::setHello",
            "[42]",
            r#"
--| scilla_version 0
--|
--| contract HelloWorld()
--|
--> transition setHello ()
-->   msg = Uint64 12;
-->   x = builtin print__impl msg;
-->   y = builtin print__impl msg
--|  end
"#,
            "",
        );

        test_execution_path(
            "HelloWorld::setHello",
            "[42]",
            r#"
--| scilla_version 0
--| 
--| library HelloWorld
--| contract HelloWorld()
--| field welcome_msg : Uint64 = Uint64 0
--| 
--> transition setHello (x: Uint64)
-->   welcome_msg := x;
--|   y <- welcome_msg (* TODO: Source map not correctly generated here *)
--| end

"#,
            "0x00...1337:0x00...2a",
        );
    }
}
