use worker::js_sys::Promise;
use worker::wasm_bindgen::prelude::*;
use worker::AbortSignal;

pub fn make(signal: AbortSignal, p: Promise) -> Promise {
    Promise::new(&mut |resolve, reject| {
        let msg = "Request has been aborted";

        if signal.aborted() {
            worker::console_log!("already aborted");
            reject
                .call1(&JsValue::undefined(), &msg.into())
                .unwrap_throw();
        }

        {
            let reject = reject.clone();
            let on_abort = Closure::<dyn FnMut(JsValue)>::new(move |_| {
                worker::console_log!("Request has been aborted");
                reject
                    .call1(&JsValue::undefined(), &msg.into())
                    .unwrap_throw();
            });
            signal.set_onabort(Some(&on_abort.as_ref().unchecked_ref()));
            on_abort.forget();
        }

        // Listen for the initial promise completion
        {
            let resolve2 = Closure::new(move |val| {
                worker::console_log!("initial resolved");
                resolve.call1(&JsValue::undefined(), &val).unwrap_throw();
            });
            let reject2 = Closure::new(move |val| {
                worker::console_log!("initial rejected");
                reject.call1(&JsValue::undefined(), &val).unwrap_throw();
            });
            p.then2(&resolve2, &reject2);
        }
        ()
    })
}
