import { fetch_html, reg_css } from "@scripts";

reg_css("node_arg", "ndearg");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class NodeArg extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/node_arg/html/index.html',
            () => {
                let pe: HTMLParagraphElement = this.shadowRoot.querySelector('p');
                let name: Text = document.createTextNode(this.getAttribute('arg_name'));
                pe.appendChild(name);
                if (this.hasAttribute("left"))
                {
                    let r: HTMLDivElement = this.shadowRoot.querySelector('div.right')
                    r.style.visibility = "hidden";
                }
                else
                {
                    let l: HTMLDivElement = this.shadowRoot.querySelector('div.left')
                    l.style.visibility = "hidden";
                }
            });
    }
}

customElements.define('node-arg', NodeArg);
console.log("script loaded.");
