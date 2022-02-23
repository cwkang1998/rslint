use crate::rule_prelude::*;
use ast::SwitchStmt;

declare_lint! {
    /**
    Disallow lexical declarations in case/default clauses.

    Lexical declarations such as `let`, `const`, `function` and `class` in `case`/`default`
    clauses is not allowed as the lexical declaration is visible in the entire switch block
    bit only gets initialized when its first assigned which happens only if the case where
    it is defined is reached.

    Wrapping it in blocks ensures the lexical declaration only applies to the current case clauses.

    ## Invalid Code Examples

    ```js
    switch (foo) {
        case 1:
            let x = 1;
            break;
        case 2:
            const y = 2;
            break;
        case 3:
            function f() {}
            break;
        default:
            class C {}
    }
    ```

    ## Valid code examples

    ```js
    // Declarations outside switch-statements are valid
    const a = 0;

    switch (foo) {
        // The following case clauses are wrapped into blocks using brackets
        case 1: {
            let x = 1;
            break;
        }
        case 2: {
            const y = 2;
            break;
        }
        case 3: {
            function f() {}
            break;
        }
        case 4:
            // Declarations using var without brackets are valid due to function-scope hoisting
            var z = 4;
            break;
        default: {
            class C {}
        }
    }
    ```
    */
    #[derive(Default)]
    NoCaseDeclaration,
    errors,
    tags(Recommended),
    "no-case-declarations"
}

#[typetag::serde]
impl CstRule for NoCaseDeclaration {
    fn check_node(&self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        None
    }
}

rule_tests! {
    NoCaseDeclaration::default(),
    err: {
        "
        switch (foo) {
            case 2:
                const y = 2;
                break;
        }
        ",
        "
        switch (foo) {
            case 3:
                function f() {};
                break;
        }
        ",
        "
        switch (foo) {
            default:
                class C {}
        }
        ",
        "
        switch (foo) {
            case 1:
                let x = 1;
                break;
        }
        ",
        "
        switch (foo) {
            case 1:
                let x = 1;
                break;
            default:
                break;
        }
        "
    },
    ok: {
        "
        switch (foo) {
            case 1: {
                let x = 1;
                break;
            }
        }
        ",
        "
        switch (foo) {
            case 2: {
                const y = 2;
                break;
            }
        }
        ",
        "
        switch (foo) {
            case 3: {
                function f() {}
                break;
            }
        }
        ",
        "
        switch (foo) {
            case 1: {
                class C {}
            }
        }
        ",
        "
        switch (foo) {
            case 1:
                var z = 4;
                break;
        }
        ",
        "
        switch (foo) {
            case 1: {
                let x = 1;
                break;
            }
            default:
                break;
        }
        ",
        "
        switch (foo) {
            case 1:
                var x = 3;
                break;
            default:
                break;
        }
        "
    }
}
