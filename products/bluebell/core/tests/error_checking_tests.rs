#[cfg(test)]
mod tests {
    use bluebell::testing::test_execution_path;

    #[test]
    fn test_hello_world() {
        test_execution_path(
            "HelloWorldExample::TriggerHelloWorld",
            "[42]",
            r#"
--| scilla_version 0
--| library HelloWorldContract
--| contract HelloWorldExample()
--> transition TriggerHelloWorld()
-->   msg = "Hello world.";
-->   print msg
--| end
        "#,
            "",
            "",
        );
    }

    // TODO: Add test for stdout.
    // TODO: Work out how to test panic
    // -->   panic msg
}
