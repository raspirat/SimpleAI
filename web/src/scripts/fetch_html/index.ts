export function fetch_html(element: & HTMLElement, path: & string, after = () => {}, noshadow=false) {
    fetch(path)
        .then(response => response.text())
        .then(data => {
                let root: any = element;
                if (! noshadow) root = root.attachShadow({mode: 'open'});
                root.innerHTML = data;
                after();
            }
        )
}
