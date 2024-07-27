import { reg_css, fetch_html } from "@scripts";
reg_css("header", "hdr");
import { WindowControls } from "@window_controls";
console.log("Dependencies loaded: ", WindowControls.name)
import { appWindow } from "@node_modules/@tauri-apps/api/window.js";
export class MainHeader extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/main_header/html/index.html',
            () => {
                let listener = () => {appWindow.startDragging();};
                this.shadowRoot.querySelector('#logo')
                    .addEventListener('mousedown', listener);
                this.shadowRoot.querySelector('#drag-area')
                    .addEventListener('mousedown', listener);
            }
        );
    }
}

customElements.define('main-header', MainHeader);


console.log("script loaded.")
