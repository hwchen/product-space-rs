mod rca;
pub use rca::{
    into_fair_share,
    into_rca,
};

mod proximity;
pub use proximity::into_proximity;

mod density;
pub use density::into_density;
