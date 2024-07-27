import { fetch_html } from '@scripts';
export class SettingsButton extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(this, '/components/settings_button/html/index.html');
    }
}

customElements.define('settings-button', SettingsButton);