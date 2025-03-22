/// Represents an entire program consisting of a single function definition.
struct Program {
    function: Function,
}

/// Represents a function definition with a name and a body consisting of a statement.
struct Function {
    name: String,
    body: Statement,
}

/// Represents different kinds of statements.
enum Statement {
    Return(Expr),
}

/// Represents different kinds of expressions.
enum Expr {
    Constant(i32),
}

