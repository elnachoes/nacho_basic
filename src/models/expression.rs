use super::{Type, Variable};

pub enum Expression {
    Identity {
        identity : String
    },
    Literal {
        literal_type : Type,
        variable : Variable
    },
    Declar {
        identity : String,
        value_expression : Option<Box<Expression>>
    },
    Assign {
        assignee : String,
        value_expression : Box<Expression>
    }, 
    FuncInvoke {
        parameters : Vec<Expression>,
    }
}
