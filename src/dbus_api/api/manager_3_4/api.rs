// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dbus_tree::{Factory, MTSync, Method};

use crate::{
    dbus_api::{api::manager_3_4::methods::start_pool, types::TData},
    engine::Engine,
};

pub fn start_pool_method<E>(
    f: &Factory<MTSync<TData<E>>, TData<E>>,
) -> Method<MTSync<TData<E>>, TData<E>>
where
    E: 'static + Engine,
{
    f.method("StartPool", (), start_pool)
        .in_arg(("id", "s"))
        .in_arg(("id_type", "s"))
        .in_arg(("unlock_method", "(bs)"))
        // In order from left to right:
        // b: true if the pool was newly started
        // o: pool path
        // oa: block device paths
        // oa: filesystem paths
        //
        // Rust representation: bool
        .out_arg(("result", "(b(oaoao))"))
        .out_arg(("return_code", "q"))
        .out_arg(("return_string", "s"))
}
