mod attribute_type;

use crate::engine::Variadic;
pub use attribute_type::*;

pub struct Agent {
    ovxx: [Option<Variadic>; 100],
    attributes: AttributeType,
}
