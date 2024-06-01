export function fetch_html(self: & HTMLElement, path: & string) {
    const PATH_URL = new URL(path);
    fetch(PATH_URL)
        .then(response => response.text())
        .then(data => {
                const TEMPLATE = document.createElement('template');
                const SHADOW_ROOT = self.attachShadow({mode: 'open'});
                TEMPLATE.innerHTML = data;
                SHADOW_ROOT.appendChild(
                    TEMPLATE.content.cloneNode(true)
                );
            }
        )
}
