use crate::memcxt::palloc0_struct;
use crate::pg_sys;

/// Return a Postgres-allocated pointer to a `pg_sys::NodeTag` struct.  
///
/// See `#include "nodes/nodes.h"
///
/// ## Examples
///
/// ```rust
/// use pg_bridge::pg_sys;
/// let create_role_stmt = pg_sys::create_role::<pg_sys::CreateRoleStmt>(pg_sys::NodeTag_T_CreateRoleStmt);
/// ```
///
/// ## Safety
///
/// This function is unsafe not because of the allication it performs, but because it's possible
/// to specify the wrong `NodeTag` for the specified type `T`.  The caller needs to be sure
/// these things match
#[inline]
pub unsafe fn make_node<T>(tag: pg_sys::NodeTag) -> *mut T {
    // TODO:  we can convert pg_sys::NodeTag to a rust enum using bindgen
    // TODO:  and make this a gigantic match arm where we hardcode the struct name
    // TODO:  not sure that's a better idea, but it would be one less thing the caller
    // TODO:  would need to specify, reducing compilation problems
    let node = palloc0_struct::<T>() as *mut pg_sys::Node;
    (*node).type_ = tag;
    node as *mut T
}
