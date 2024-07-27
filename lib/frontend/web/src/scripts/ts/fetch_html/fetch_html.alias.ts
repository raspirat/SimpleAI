import {Child} from "@tauri-apps/api/shell";

export function fetch_html(self: & HTMLElement, path: & string, after = () => {}) {
    fetch(path)
        .then(response => response.text())
        .then(data => {
                const TEMPLATE = document.createElement('div');
                const SHADOW_ROOT = self.attachShadow({mode: 'open'});
                // TEMPLATE.innerHTML = data;
                try {
                    SHADOW_ROOT.innerHTML = data;
                }
                catch (error)
                {
                    console.log("couldn't register custom html component.");
                    return;
                }
                after();
            }
        )
}
