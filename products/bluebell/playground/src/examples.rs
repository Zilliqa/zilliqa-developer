pub static EXAMPLES: [(&str, &str); 5] = [
    (
        "Hello Builtin",
        r#"scilla_version 0

library HelloWorld
contract HelloWorld()

transition setHello ()
  x = Uint64 1;
  print x
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
  welcome_msg := x;
  y <- welcome_msg
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
    print msg
  | False =>
    print msg;
    print msg
  end
end
"#,
    ),
    (
        "Simple Logic",
        r#"scilla_version 0

      library BasicLogic
      
      contract BasicLogic()
      
      transition testValue (msg: Uint64)
        reference = Uint64 11;
        is_owner = builtin eq msg reference;
        logic_reference = False;
        is_false = builtin eq logic_reference is_owner;
        match is_false with
        | True =>
          msg = "The values were different";
          print msg
        | False =>
          msg = "The values were equal";
          print msg
        end
      end      
      "#,
    ),
    (
        "Special variables",
        r#"
scilla_version 0
library HelloWorldContract
contract HelloWorldExample()
transition TriggerHelloWorld()
  msg = _sender;
  print msg
end
"#,
    ),
    /*
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
        */
];
