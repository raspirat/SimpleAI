import { fetch_html } from '../../scripts/fetch_html.js';
class WindowControls extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/window_controls/html/index.html');
    }
}

customElements.define('window_controls', WindowControls);