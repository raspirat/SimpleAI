import { reg_css, fetch_html } from "@scripts";
reg_css("window_controls");
export class WindowControls extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/window_controls/html/index.html');
    }
}

customElements.define('window-controls', WindowControls);