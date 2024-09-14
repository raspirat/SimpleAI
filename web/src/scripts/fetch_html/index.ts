export function fetch_html(self: & HTMLElement, path: & string, after = () => {}) {
    fetch(path)
        .then(response => response.text())
        .then(data => {
                const SHADOW_ROOT = self.attachShadow({mode: 'open'});
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
