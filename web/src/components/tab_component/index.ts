import { fetch_html, reg_css } from "scripts";

reg_css("node_pref", "tcpt");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class TabComponent extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            'components/tab_component/index.html',
            () => {
                let pe: HTMLParagraphElement = this.shadowRoot.querySelector('p');
                let name: Text = document.createTextNode(this.getAttribute('node_name'));
                pe.appendChild(name);
            });
    }
}

customElements.define('tab-component', TabComponent);
console.log("script loaded.");