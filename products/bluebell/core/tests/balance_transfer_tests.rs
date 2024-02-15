#[cfg(test)]
mod tests {

    /*
    TODO:
        #[test]
        fn test_visiting() {
            test_execution_path(
                "HelloWorld::setHello",
                "[42]",
                r#"
    --| scilla_version 0
    --| library BalanceChecker
    --| contract BalanceQuery()
    --> transition CheckBalance()
    -->   current_balance <- _balance;
    -->   e = { _eventname : "ContractBalance"; current_balance : current_balance };
    -->   event e
    --| end
    "#,
                "",
                "",
            );
        }
    */
}
