//! ```elixir
//! # label 4
//! # pushed to stack: (parent. old_child)
//! # returned form call: :ok
//! # full stack: (:ok, parent, old_child)
//! # returns: {:error, :hierarchy_request}
//! {:error, :hierarchy_request} = Lumen.Web.replace_child(parent, old_child, parent)
//! ```

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::label]
fn result(process: &Process, ok: Term, parent: Term, old_child: Term) -> Term {
    assert_eq!(ok, Atom::str_to_term("ok"));
    assert!(parent.is_boxed_resource_reference());
    assert!(old_child.is_boxed_resource_reference());

    process.queue_frame_with_arguments(
        liblumen_web::node::replace_child_3::frame()
            .with_arguments(false, &[parent, parent, old_child]),
    );

    Term::NONE
}
