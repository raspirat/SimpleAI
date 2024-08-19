import { reg_css } from "@scripts";
reg_css("page", "page");

// @ts-ignore
const listen = window.__TAURI__.event.listen;
// @ts-ignore
const emit = window.__TAURI__.event.emit;

let error_msg_listen = await listen('error_message', (event) => {
    let p = Object.values(event.payload)[0];
    console.log(p);
    // @ts-ignore
    document.querySelector('#type').innerHTML = p.error_type;
    // @ts-ignore
    document.querySelector('#code').innerHTML = p.code;
    // @ts-ignore
    document.querySelector('#note').innerHTML = p.note;
});

let err_msg_emit = emit('get_error_message');
