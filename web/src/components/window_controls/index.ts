import { reg_css, fetch_html } from "scripts";
import {app} from "@tauri-apps/api";
reg_css("window_controls");

// @ts-ignore
const appWindow = window.__TAURI__.window.appWindow;

export class WindowControls extends HTMLElement {
  fullscreenIcon: string = '<i class="ri-tv-2-line"></i>'; // '<i class="ri-fullscreen-line"></i>'; // '<i class="ri-window-line"></i>'; //'<i class="ri-rectangle-line"></i>';
  maximizedIcon: string = '<i class="ri-window-line"></i>'; // '<i class="ri-fullscreen-line"></i>';
  unmaximizedIcon: string = '<i class="ri-checkbox-multiple-blank-line"></i>'; //<i class="ri-fullscreen-exit-line"></i>';
  constructor() {
    super();
    fetch_html(this, "components/window_controls/index.html", async () => {
      this.shadowRoot
        .querySelector("#window-minimize")
        .addEventListener("click", () => {
          appWindow.minimize();
        });

      let toggle_fs = this.shadowRoot.querySelector("#window-toggle-fs");
      if (await appWindow.isFullscreen()) {
        toggle_fs.innerHTML = this.maximizedIcon;
      }
      else if (await appWindow.isMaximized()) {
        toggle_fs.innerHTML = this.fullscreenIcon;
      }
      else {
        toggle_fs.innerHTML = this.maximizedIcon;
      }
      toggle_fs.addEventListener("click", async () => {
        // switch to maximized
        if (await appWindow.isFullscreen())
        {
          await appWindow.setFullscreen(false);
          await appWindow.maximize();
          toggle_fs.innerHTML = this.unmaximizedIcon;
        }
        // switch to unmaximized/fullscreen
        else if (await appWindow.isMaximized())
        {
          if (toggle_fs.innerHTML == this.unmaximizedIcon)
          {
            toggle_fs.innerHTML = this.maximizedIcon;
            await appWindow.unmaximize();
          } else {
            toggle_fs.innerHTML = this.maximizedIcon;
            await appWindow.setFullscreen(true);
          }
        }
        // switch to maximized
        else
        {
          toggle_fs.innerHTML = this.fullscreenIcon;
          await appWindow.maximize();
        }
      });

      this.shadowRoot
        .querySelector("#window-close")
        .addEventListener("click", () => {
          appWindow.close();
        });
    });
  }
}
customElements.define("window-controls", WindowControls);
