/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 24/6/24
 ******************************************************************************/


use crate::data::TermStructure;
use std::collections::BTreeMap;

/// Surface data.
pub struct Surface {
    /// Nodes of the surface.
    pub nodes: BTreeMap<f64, TermStructure>,
}
