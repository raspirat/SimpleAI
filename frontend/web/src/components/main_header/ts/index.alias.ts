import { reg_css, fetch_html } from "@scripts";
reg_css("header", "hdr");

import { WindowControls } from "@window_controls";
console.log("Dependencies loaded: ", WindowControls.name)
export class MainHeader extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/main_header/html/index.html');
    }
}

customElements.define('main-header', MainHeader);


console.log("script loaded.")
