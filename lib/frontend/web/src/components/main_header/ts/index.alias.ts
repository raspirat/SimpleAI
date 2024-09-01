import { reg_css, fetch_html } from "@scripts";
reg_css("header", "hdr");
import { WindowControls } from "@window_controls";
console.log("Dependencies loaded: ", WindowControls.name)


// @ts-ignore
const invoke = window.__TAURI__.invoke;
// @ts-ignore
const appWindow = window.__TAURI__.window.appWindow;

export class MainHeader extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/main_header/html/index.html',
            () => {
                let rb: HTMLButtonElement = this.shadowRoot.querySelector('#run-button');
                let sb: HTMLButtonElement = this.shadowRoot.querySelector('#settings-button');
                let hb: HTMLButtonElement = this.shadowRoot.querySelector('#home-button');

                sb.addEventListener('click', () => {
                    invoke('settings_page');
                });
                hb.addEventListener('click', () => {
                    invoke('start_page');
                });

                if (! this.hasAttribute('run'))
                {
                    rb.style.display = "none";
                }
                if (! this.hasAttribute('settings'))
                {
                    sb.style.display = "none";
                }
                if (! this.hasAttribute('home'))
                {
                    hb.style.display = "none";
                }
                let listener = () => {appWindow.startDragging();};
                this.shadowRoot.querySelector('#logo')
                    .addEventListener('mousedown', listener);
                this.shadowRoot.querySelector('#drag-area')
                    .addEventListener('mousedown', listener);
            }
        );
    }
}

customElements.define('main-header', MainHeader);


console.log("script loaded.")
