use std::fmt;

use itertools::Itertools;
use risingwave_common::catalog::{Field, Schema};

use super::{ColPrunable, IntoPlanRef, PlanRef, PlanTreeNodeUnary};
use crate::expr::{assert_input_ref, BoundExpr, BoundExprImpl};
use crate::optimizer::property::{WithDistribution, WithOrder, WithSchema};

#[derive(Debug, Clone)]
pub struct LogicalProject {
    exprs: Vec<BoundExprImpl>,
    expr_alias: Vec<Option<String>>,
    input: PlanRef,
    schema: Schema,
}
impl LogicalProject {
    fn new(input: PlanRef, exprs: Vec<BoundExprImpl>, expr_alias: Vec<Option<String>>) -> Self {
        let schema = Self::derive_schema(&exprs, &expr_alias);
        for expr in &exprs {
            assert_input_ref(expr, input.schema().fields().len());
        }
        LogicalProject {
            input,
            schema,
            exprs,
            expr_alias,
        }
    }
    pub fn create(
        input: PlanRef,
        exprs: Vec<BoundExprImpl>,
        expr_alias: Vec<Option<String>>,
    ) -> PlanRef {
        Self::new(input, exprs, expr_alias).into_plan_ref()
    }

    fn derive_schema(exprs: &[BoundExprImpl], expr_alias: &[Option<String>]) -> Schema {
        let fields = exprs
            .iter()
            .zip_eq(expr_alias.iter())
            .enumerate()
            .map(|(id, (expr, alias))| {
                let name = alias.clone().unwrap_or(format!("expr#{}", id));
                Field {
                    name,
                    data_type: expr.return_type(),
                }
            })
            .collect();
        Schema { fields }
    }
    pub fn exprs(&self) -> &Vec<BoundExprImpl> {
        &self.exprs
    }

    /// Get a reference to the logical project's expr alias.
    pub fn expr_alias(&self) -> &[Option<String>] {
        self.expr_alias.as_ref()
    }
}
impl PlanTreeNodeUnary for LogicalProject {
    fn input(&self) -> PlanRef {
        self.input.clone()
    }
    fn clone_with_input(&self, input: PlanRef) -> Self {
        Self::new(input, self.exprs.clone(), self.expr_alias().to_vec())
    }
}
impl_plan_tree_node_for_unary! {LogicalProject}
impl fmt::Display for LogicalProject {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
impl WithOrder for LogicalProject {}
impl WithDistribution for LogicalProject {}
impl WithSchema for LogicalProject {
    fn schema(&self) -> &Schema {
        &self.schema
    }
}
impl ColPrunable for LogicalProject {}
