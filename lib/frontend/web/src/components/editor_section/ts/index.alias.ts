import { fetch_html, reg_css } from "@scripts";
import { TabComponent } from "@tab_component";
import { Viewport } from "@viewport";

console.log("Dependencies loaded: ", TabComponent.name, Viewport.name);

reg_css("node_pref", "esc");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class EditorSection extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/editor_section/html/index.html',
            () => {

            });
    }
}

customElements.define('editor-section', EditorSection);
console.log("script loaded.");
