import { fetch_html, reg_css } from "scripts";

reg_css("node_pref", "ndpf");

// @ts-ignore
const invoke = window.__TAURI__.invoke; // uncomment
export class RightSidebar extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            'components/right_sidebar/index.html',
            () => {
                let buttons: NodeListOf<HTMLButtonElement> = this.querySelectorAll('button');
                let div: HTMLDivElement = this.shadowRoot.querySelector('div');
                buttons.forEach((button) => {
                    div.appendChild(button);
                })
            });
    }
}

customElements.define('right-sidebar', RightSidebar);
console.log("script loaded.");
