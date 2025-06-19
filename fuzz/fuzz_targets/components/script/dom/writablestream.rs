#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use std::rc::Rc;
use script_bindings::codegen::GenericBindings::QueuingStrategyBinding::QueuingStrategy;
use script_bindings::codegen::GenericBindings::WritableStreamBinding;
use script_bindings::codegen::InheritTypes::EventTargetTypeId::GlobalScope;
use script_bindings::codegen::PrototypeList::ID::WritableStream;
use script_bindings::DomTypes;
use script_bindings::error::Fallible;
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::codegen::DomTypes;

fuzz_target!(|data: &[u8]| {

    let cx = script_bindings::script_runtime::JSContext::from_ptr(Rc::new(runtime));
    let proto = None;
    let can_gc = CanGc::note();
    let underlying_sink = None;
    let strategy =QueuingStrategy::empty();

    let fallible : Fallible<DomRoot<DomTypes::WritableStream>> = WritableStreamBinding::WritableStreamMethods::Constructor(
        cx,
        &GlobalScope,
        proto,
        can_gc,
        underlying_sink,
        &strategy,
    );
    fallible.unwrap();

});
