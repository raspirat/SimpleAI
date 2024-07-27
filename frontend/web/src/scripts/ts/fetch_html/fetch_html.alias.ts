export function fetch_html(self: & HTMLElement, path: & string) {
    fetch(path)
        .then(response => response.text())
        .then(data => {
                const TEMPLATE = document.createElement('div');
                const SHADOW_ROOT = self.attachShadow({mode: 'open'});
                TEMPLATE.innerHTML = data;
                try {
                    SHADOW_ROOT.appendChild(
                        TEMPLATE.firstChild.cloneNode(true)
                    );
                }
                catch (error)
                {
                    console.log("couldn't register custom html component.");
                }
            }
        )
}
