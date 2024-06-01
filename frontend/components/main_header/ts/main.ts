import { fetch_html } from '../../../scripts/ts/fetch_html';
class MainHeader extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '../html/index.html')
    }
}

customElements.define('main_header', MainHeader);