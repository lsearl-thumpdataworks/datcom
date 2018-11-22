/// -----
/// File: /src/utlilty/mod.rs  
/// Project: datcon  
/// File Created: Wednesday, 14th Nov 182018 3:47:50 pm
/// Author: Leon S. Searl  
/// 
/// Last Modified: Wednesday, 14th November 2018 3:47:50 pm  
/// Modified By: Leon S. Searl
/// 
/// Copyright 2018 - 2018 Thump Data Works, Inc., Thump Data Works, Inc.
/// -----

pub mod tbfunx;
pub mod quad;

/// Enum to use where a type of flying service/planform is to be specfied.
pub enum FlySurface {
    /// Forward lifing surface
    Wing,
    /// Aft lifting surface.
    HorzTail,
    /// Vertical Tail
    VertTail,
    /// Ventral Fin.
    VentFin
}
