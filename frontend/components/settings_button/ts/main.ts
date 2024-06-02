import { fetch_html } from '../../scripts/fetch_html.js';
class SettingsButton extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/settings_button/html/index.html');
    }
}

customElements.define('settings-button', SettingsButton);