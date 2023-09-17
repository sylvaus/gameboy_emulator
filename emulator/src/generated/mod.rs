// TODO: optimize code generation to be able to remove identity_op, eq_op, and self_assignment
#[allow(
    clippy::assign_op_pattern,
    clippy::needless_return,
    clippy::identity_op,
    clippy::eq_op,
    clippy::self_assignment,
    dead_code
)]
pub(crate) mod instructions;
