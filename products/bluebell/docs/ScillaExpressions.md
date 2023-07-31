# Scilla expressions used for development

Here is a list of Scilla expressions that can be used for testing type
inference. Each expression consists of a combination of different types,
functions, and Scilla specific constructs to help you cover various cases during
type inference testing.

1. Add an integer and a boolean:

   ```scilla
   let x = 2 in
   let y = True in
   x + y
   ```

   ```rust
   NodeFullExpression::LocalVariableDeclaration {
       identifier_name: "x".to_string(),
       expression: Box::new(NodeFullExpression::ExpressionAtomic(Box::new(NodeAtomicExpression::AtomicLit(
           NodeValueLiteral::LiteralInt(NodeTypeNameIdentifier::ByteStringType(NodeByteStr::Constant("Int32".to_string())), "2".to_string())
       ))),
       type_annotation: None,
       containing_expression: Box::new(NodeFullExpression::LocalVariableDeclaration {
           identifier_name: "y".to_string(),
           expression: Box::new(NodeFullExpression::ExpressionAtomic(Box::new(NodeAtomicExpression::AtomicLit(
               NodeValueLiteral::LiteralInt(NodeTypeNameIdentifier::ByteStringType(NodeByteStr::Constant("Bool".to_string())), "True".to_string())
           ))),
           type_annotation: None,
           containing_expression: Box::new(NodeFullExpression::ExpressionBuiltin {
               b: "_add_".to_string(),
               targs: None,
               xs: NodeBuiltinArguments {
                   arguments: vec![
                       NodeVariableIdentifier::VariableName("x".to_string()),
                       NodeVariableIdentifier::VariableName("y".to_string()),
                   ],
                   type_annotation: None,
               },
           }),
       }),
   }
   ```

2. Define a custom function and use it:

   ```scilla
   let foo = tfun ('T) => fun (x : 'T) => x in
   let bar = @foo Uint32 42 in
   bar
   ```

3. Create a custom List ADT and define functions to manipulate it (similar to a
   simple implementation for List.length):

   ```scilla
   type List = | Nil | Cons of (Uint32, List);
   fun length : List -> Uint32 =
     tfun l =>
       match l with
       | Nil => Uint32 0
       | Cons h t => let tmp_length = @length t in
                     1 + tmp_length
       end;
   ```

4. Work with Scilla's transactions, messages, and events:

   ```scilla
   type Payment = (ByStr20, Uint128);
   type Event = | ReceivePayment of Payment;
   transition OnPayment(sender: ByStr20, amt: Uint128)
     is_sender = builtin eq _sender sender;
     match is_sender with
     | False =>
       msg = {_tag : ""; _recipient : sender; _amount_QTZ : amt};
       e = ReceivePayment (sender, amt);
       event e;
       msgs = one_msg msg;
       send msgs
     | True => skip
     end
   end
   ```

5. Use Map and Functors:

   ```scilla
   type Storage = Map ByStr20 Uint128;
   fun get_balance: Storage -> ByStr20 -> Uint128 =
     fun (s : Storage) =>
     fun (addr : ByStr20) =>
       match (builtin get s addr) with
       | Some bal => bal
       | None => Uint128 0
       end;
   ```

6. Recursion with Fibonacci sequence:

   ```scilla
   fun fibonacci : Uint32 -> Uint32 =
     fun (n : Uint32) =>
       let eq1 = uint32_eq n (Uint32 0) in
       let eq2 = uint32_eq n (Uint32 1) in
       match eq1 with
       | True => Uint32 0
       | False =>
         match eq2 with
         | True => Uint32 1
         | False =>
           let n_minus1 = builtin sub n (Uint32 1) in
           let fib_n_minus1 = fibonacci n_minus1 in
           let n_minus2 = builtin sub n (Uint32 2) in
           let fib_n_minus2 = fibonacci n_minus2 in
           builtin add fib_n_minus1 fib_n_minus2
         end
       end;
   ```

7. Custom User ADT with type parameters:

   ```Scilla
   type User (ByStr, StdLib.Option Uint32)  =
     | Unknown
     | UserDetails of (ByStr, StdLib.Option Uint32);

   let user = UserDetails (@0x123, (Some (Uint32 28)));
   ```

8. Higher-order functions and mapping a function over a List:

   ```Scilla
   type List a = | Nil | Cons of (a, List a);
   free function map_for_List : ((List a) -> b) -> List a -> List b =
     fun (f: (a -> b)) =>
     fun (l: List a) =>
       match l with
       | Nil => Nil
       | Cons h t => (@Cons b) (f h) (map_for_List f t)
       end;

   let square = fun (x: Uint32) => builtin mul x x;
   let numbers = Cons {a = Uint32; b = List Uint32} (Uint32 1) (Cons {a = Uint32; b = List Uint32} (Uint32 2) (Cons {a = Uint32; b = List Uint32} (Uint32 3) Nil));
   let squares = map_for_List square numbers;
   ```

9. Custom Result type and safe division function:

   ```Scilla
   type Result a b = | Ok of a | Error of b;
   let safe_div : Uint32 -> Uint32 -> Result Uint32 String =
     fun (x : Uint32) =>
     fun (y : Uint32) =>
       let eq_zero = uint32_eq y (Uint32 0) in
       match eq_zero with
       | True  => Error ("Cannot divide by zero")
       | False => let quotient = builtin div x y in
                  Ok (quotient)
       end;
   ```

10. Custom Tree ADT and Sum of elements:

    ```Scilla
    type Tree a = | Empty | Node of (a, Tree a, Tree a);
    fun tree_sum : Tree Uint32 -> Uint32 =
      fun (t : Tree Uint32) =>
        match t with
        | Empty => Uint32 0
        | Node (val, left, right) =>
          let left_sum = tree_sum left in
          let right_sum = tree_sum right in
          let subtrees_sum = builtin add left_sum right_sum in
          builtin add subtrees_sum val
        end;
    ```
