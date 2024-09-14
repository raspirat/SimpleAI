import { reg_css, fetch_html } from "scripts";
reg_css("main_page_selector", "mps");

// @ts-ignore
const invoke = window.__TAURI__.invoke;
export class MainPageSelector extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            'components/main_page_selector/index.html',
            () => {
                let search: HTMLButtonElement = this.shadowRoot.querySelector("#search");
                search.addEventListener('click', () => {
                    invoke('search_page');
                });
                let node: HTMLButtonElement = this.shadowRoot.querySelector("#new");
                node.addEventListener('click', () => {
                    invoke('new_node_page');
                });
                let market: HTMLButtonElement = this.shadowRoot.querySelector("#market");
                let ml: HTMLLIElement = this.shadowRoot.querySelector("li.nav.selector.marketplace");
                ml.style.display = "none";
            });
    }
}

customElements.define('main-page-selector', MainPageSelector);
console.log("script loaded.")
