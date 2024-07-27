import { reg_css, fetch_html } from "@scripts";
reg_css("main_page_selector", "mps");

export class MainPageSelector extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/main_page_selector/html/index.html');
    }
}

customElements.define('main-page-selector', MainPageSelector);
console.log("script loaded.")
