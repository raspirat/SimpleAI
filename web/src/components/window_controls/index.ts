import { reg_css, fetch_html } from "scripts";
reg_css("window_controls");

// @ts-ignore
const appWindow = window.__TAURI__.window.appWindow;

export class WindowControls extends HTMLElement {
  constructor() {
    super();
    fetch_html(this, "components/window_controls/index.html", () => {
      this.shadowRoot
        .querySelector("#window-minimize")
        .addEventListener("click", () => {
          appWindow.minimize();
          console.log("minimizing");
        });

      let toggle_fs = this.shadowRoot.querySelector("#window-toggle-fs");
      toggle_fs.addEventListener("click", async () => {
        console.log("toggling");
        appWindow.toggleMaximize();
        let max: boolean = await appWindow.isMaximized();
        if (max) {
          console.log("maximized");
          toggle_fs.innerHTML = '<i class="ri-fullscreen-line"></i>';
        } else {
          console.log("maximizing");
          toggle_fs.innerHTML = '<i class="ri-fullscreen-exit-line"></i>';
        }
      });

      this.shadowRoot
        .querySelector("#window-close")
        .addEventListener("click", () => {
          appWindow.close();
          console.log("closing");
        });
    });
  }
}
customElements.define("window-controls", WindowControls);
