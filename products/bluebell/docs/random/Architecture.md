# Architecture Overview of the Bluebell Compiler

The Bluebell compiler is designed specifically for compiling the Zilliqa Scilla
Language. The compiling process is divided into four structured stages, each
implementing a trait to convert or transform the code into a more low-level
representation.

```text
TODO: Expand on following:
- Bluebell is a the next generation scilla compiler
- It aims to target both WASM and EVM
```

## Compilation Stages

The compilation process in Bluebell involves several stages that transform
Scilla code from its initial high-level format into machine-executable bytecode.
Each stage refines and optimizes the code, focusing on specific aspects to
enhance execution, resource utilization, and maintainability. The stages of
compilation include:

1. Parsing
2. Converting the AST to a high-level IR
3. Lowering the high-level IR to a low-level IR
4. Emitting bytecode

### Parsing (Scilla Parser)

The parser function forms the initial phase of the compiler. Bluebell uses a
parser implemented with LALRPOP, with a custom lexer, to produce an Abstract
Syntax Tree (AST) representation of the Scilla code.

The parsing process begins as the source code is read from left to right and
tokenized by the lexer. These tokens represent the smallest meaningful units of
the code, such as literals, identifiers, operators, and assorted keyword.
Following the rules defined in the LALRPOP grammar, these tokens are then
grouped into expressions and statements that construct the AST.

One of the distinct challenges in parsing Scilla code is handling its functional
programming aspects and strict typing system. As Scilla has been designed with
formal verification in mind, its syntax differs substantially from regular
programming languages, necessitating a custom lexer and unique parsing
strategies.

Moreover, the error recovery during this phase needs special attention. When a
syntax error is encountered, the parsing should be able to recover and continue
with the next statements or declarations, in order to report multiple errors in
one run if necessary.

Additionally, as a part of the parsing process, some initial semantic checks can
also be performed such as checking for undeclared variables or incompatible data
types, depending on the complexity and runtime cost of these checks. Hence, the
efficiency of the parser is crucial to the overall function of Bluebell.

The result of this process is an AST, a tree-like data structure that simplifies
subsequent compiler phases like static checking and code optimization.

### AST Conversion to High-Level IR

Using the trait `AstConverting`, the AST is converted to a High-Level
Intermediate Representation (IR). It implements type deduction during the
generation of this high-level IR.

During this stage, the AST representation of the Scilla code is converted into a
high-level intermediate representation (IR) using the trait `AstConverting`.
This facilitates further optimization processes. The representation carries out
type deduction to ensure that all variables, expressions, and operations are
type-safe. The high-level IR stage reformulates the code into a format which is
easier for subsequent stages of the compiler to process, optimizing its
structure without altering its behavior. This format could be more flexible and
efficient to manipulate compared to the original syntax, particularly for
complex operations such as loop unrolling, constant folding, and strength
reduction.

### High-Level IR Lowering to Low-Level IR

The High-Level IR is then lowered to a Low-Level IR using the `IrLowering`
trait. The Low-Level IR used here is the LLVM infrastructural framework,
providing robust and extensive architecture support.

Once the high-level IR has been established, it's then further refined into a
low-level IR through the `IrLowering` trait. This low-level IR relies on the
LLVM infrastructural framework, which has remarkable architecture support. At
this level, the IR has lost most of its high-level structures (like loops) and
takes on a form much closer to assembly language. This form is more suitable for
machine-code translation and allows for machine-specific optimizations.

### Low-Level IR Lowering to Byte Code

Finally, the Low-Level IR is lowered into bytecode using the `BytecodeEmitting`
trait.

The final stage of the compilation process involves lowering the low-level IR
into bytecode with the `BytecodeEmitting` trait. This bytecode is the final
machine-executable format that can be interpreted by the host JVM. During this
stage, all the aggregated optimizations and transformations from previous stages
are finalized and applied to produce efficient and robust machine code.
Optimizations at this stage are typically related to register allocation,
instruction scheduling, and peephole optimizations, where the compiler looks at
a few lines of code (the 'peephole') to optimize its operations.

## Extending the Compiler

For adding more functionality like a code formatter or a new target, Bluebell
has designated spaces to add these:

### Code Formatter

To implement a code formatter, you can make use of the `AstConverting` trait.

_TODO: Provide specific instructions or details on how to use the
`AstConverting` trait to implement a code formatter._

### New Target Implementation

If you wish to add a new target, one would implement a `BytecodeEmitter` trait
for the struct representing that target.

_TODO: Provide specific instructions or details on how to implement the
`BytecodeEmitter` trait for a new target._
