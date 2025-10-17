/// Operators for the declarative DSL
///
/// This module contains all operator types that can be used in
/// pipeline configurations. All operators use a `$` prefix.

mod conditional;
mod data;
mod database;
mod collection;
mod utility;

pub use conditional::{IfOp, SwitchCase, SwitchOp};
pub use data::{GetOp, JsonPathOp};
pub use database::{DbDeleteOp, DbInsertOp, DbQueryOp, DbUpdateOp, SortOrder};
pub use collection::{FilterOp, MapOp, ReduceOp};
pub use utility::{ExistsOp, MergeOp, NowOp, RenderStringOp, ReturnOp, ValidateOp};

use serde::{Deserialize, Serialize};

/// A value that can be either an operator or a literal value
///
/// This type enables recursive nesting of operators while also
/// supporting literal values (strings, numbers, booleans, objects, arrays, null).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperatorValue {
    /// An operator expression
    Operator(Box<Operator>),
    /// A literal value (string, number, bool, null, object, array)
    Literal(serde_json::Value),
}

/// Operator enum representing all possible operators
///
/// Each operator is prefixed with `$` in JSON. Uses external tagging
/// where the operator name is the key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    // Data access
    #[serde(rename = "$get")]
    Get(GetOp),
    #[serde(rename = "$jsonPath")]
    JsonPath(JsonPathOp),

    // Conditionals
    #[serde(rename = "$if")]
    If(IfOp),
    #[serde(rename = "$switch")]
    Switch(SwitchOp),

    // Collection operations
    #[serde(rename = "$map")]
    Map(MapOp),
    #[serde(rename = "$filter")]
    Filter(FilterOp),
    #[serde(rename = "$reduce")]
    Reduce(ReduceOp),

    // Database operations
    #[serde(rename = "$dbQuery")]
    DbQuery(DbQueryOp),
    #[serde(rename = "$dbInsert")]
    DbInsert(DbInsertOp),
    #[serde(rename = "$dbUpdate")]
    DbUpdate(DbUpdateOp),
    #[serde(rename = "$dbDelete")]
    DbDelete(DbDeleteOp),

    // Utility operators
    #[serde(rename = "$merge")]
    Merge(MergeOp),
    #[serde(rename = "$exists")]
    Exists(ExistsOp),
    #[serde(rename = "$renderString")]
    RenderString(RenderStringOp),
    #[serde(rename = "$return")]
    Return(ReturnOp),
    #[serde(rename = "$validate")]
    Validate(ValidateOp),
    #[serde(rename = "$now")]
    Now(NowOp),

    // Comparison operators (used within conditionals and filters)
    #[serde(rename = "$eq")]
    Eq { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$ne")]
    Ne { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$gt")]
    Gt { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$gte")]
    Gte { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$lt")]
    Lt { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$lte")]
    Lte { left: OperatorValue, right: OperatorValue },

    // Logical operators
    #[serde(rename = "$and")]
    And { conditions: Vec<OperatorValue> },
    #[serde(rename = "$or")]
    Or { conditions: Vec<OperatorValue> },
    #[serde(rename = "$not")]
    Not { condition: OperatorValue },

    // Math operators
    #[serde(rename = "$add")]
    Add { operands: Vec<OperatorValue> },
    #[serde(rename = "$subtract")]
    Subtract { left: OperatorValue, right: OperatorValue },
    #[serde(rename = "$multiply")]
    Multiply { operands: Vec<OperatorValue> },
    #[serde(rename = "$divide")]
    Divide { left: OperatorValue, right: OperatorValue },
}
