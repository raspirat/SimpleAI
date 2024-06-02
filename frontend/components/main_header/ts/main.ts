import { fetch_html } from '../../scripts/fetch_html.js';
class MainHeader extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/main_header/html/index.html');
    }
}

customElements.define('main-header', MainHeader);