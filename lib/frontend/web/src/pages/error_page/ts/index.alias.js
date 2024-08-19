"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var _scripts_1 = require("@scripts");
(0, _scripts_1.reg_css)("page", "page");
// @ts-ignore
var listen = window.__TAURI__.event.listen;
// @ts-ignore
var emit = window.__TAURI__.event.emit;
var error_msg_listen = await listen('error_message', function (event) {
    var p = Object.values(event.payload)[0];
    console.log(p);
    // @ts-ignore
    document.querySelector('#type').innerHTML = p.error_type;
    // @ts-ignore
    document.querySelector('#code').innerHTML = p.code;
    // @ts-ignore
    document.querySelector('#note').innerHTML = p.note;
});
var err_msg_emit = emit('get_error_message');
