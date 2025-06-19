#![no_main]
#[macro_use]

extern crate libfuzzer_sys;
extern crate js;

use js::rust::Runtime;
use script_bindings::codegen::GenericBindings::QueuingStrategyBinding::QueuingStrategy;
use script_bindings::codegen::GenericBindings::WritableStreamBinding;
use script_bindings::codegen::InheritTypes::EventTargetTypeId::GlobalScope;
use script_bindings::DomTypes;
use script_bindings::error::Fallible;
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;

fuzz_target!(|data: &[u8]| unsafe{

    let engine = js::rust::JSEngine::init().unwrap();
    let rt = Runtime::new(engine.handle());
    let cx = script_bindings::script_runtime::JSContext::from_ptr(rt.cx());
    
    let proto = None;
    let can_gc = CanGc::note();
    let underlying_sink = None;
    let strategy: QueuingStrategy<WritableStream> = QueuingStrategy::empty();

    let fallible: Fallible<DomRoot<WritableStream>> = WritableStreamBinding::WritableStreamMethods::Constructor(
        cx,
        &GlobalScope,
        proto,
        can_gc,
        underlying_sink,
        &strategy,
    );
    fallible.unwrap();

});
