#[cfg(test)]
mod tests {
    use bluebell::testing::test_execution_path;

    #[test]
    fn test_panic() {
        /*
                test_execution_path(
                    "PanicExample::TriggerPanic",
                    "[42]",
                    r#"
        --| scilla_version 0
        --| library PanicContract
        --| contract PanicExample()
        --| transition TriggerPanic()
        -->   msg = "This is a panic message.";
        -->   panic msg
        --| end
        "#,
                    "",
                    "",
                );
                */
    }

    // -->   panic msg
}
