#![recursion_limit = "3"]
use glib::{IsA, ObjectType};
use gst::prelude::*;
use gstreamer as gst;

fn bar<S, T>(a: &S) -> &T
where
    S: IsA<T>,
    T: ObjectType,
{
    unimplemented!()
}

fn foo(x: gst::Object) {
    x.re
    let y: gst::Object = bar(x);
}
