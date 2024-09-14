import { fetch_html, reg_css } from "scripts";

reg_css("node_pref", "tcpt");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class Viewport extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/viewport/html/index.html',
            () => {
            });
    }
}

customElements.define('tab-component', Viewport);
console.log("script loaded.");
