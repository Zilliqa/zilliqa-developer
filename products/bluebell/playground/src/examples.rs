pub static EXAMPLES: [(&str, &str); 4] = [
    (
        "Hello Builtin",
        r#"scilla_version 0

library HelloWorld
contract HelloWorld()

transition setHello ()
  x = Uint64 1;
  y = builtin print__impl x
end
"#,
    ),
    (
        "State store",
        r#"scilla_version 0

library HelloWorld
contract HelloWorld()
field welcome_msg : Uint64 = Uint64 0

transition setHello ()
  x = Uint64 1;
  welcome_msg := x
end
"#,
    ),
    (
        "Branching",
        r#"scilla_version 0

library HelloWorld
type Bool = 
  | True
  | False

contract HelloWorld()

transition setHello ()
  msg = Uint64 1;
  is_owner = False;
  match is_owner with
  | True =>
    x = builtin print__impl msg
  | False =>
    x = builtin print__impl msg;
    y = builtin print__impl msg
  end
end
"#,
    ),
    (
        "Not working 1",
        r#"scilla_version 0

library HelloWorld
contract HelloWorld()

field welcome_msg : Uint64 = Uint64 0
transition setHelloImpl(msg: Uint64)
  welcome_msg := msg
end

transition setHello (msg : Uint64)
  setHelloImpl msg;
  msg <- welcome_msg;
  x = builtin print msg
end


transition printState()
  msg <- welcome_msg;
  x = builtin print msg
end
"#,
    ),
];
