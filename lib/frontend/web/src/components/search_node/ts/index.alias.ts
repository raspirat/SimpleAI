import { reg_css, fetch_html } from "@scripts";
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
            '/components/main_page_selector/html/index.html',
            () => {
                let search = this.shadowRoot.querySelector("#search");
                console.log('search');
                search.addEventListener('click', () => {
                    invoke('search_page');
                });
            });
    }
}

customElements.define('main-page-selector', MainPageSelector);
console.log("script loaded.")
