export function fetch_html(element: & HTMLElement, path: & string, after = () => {
}, noshadow = false) {
    fetch(path)
        .then(response => response.text())
        .then(data => {
            let root: any = element;
            if (!noshadow) root = root.attachShadow({mode: 'open'});
            root.innerHTML = data;
            after();
        })
}

export class ElementFromHTML extends HTMLElement {
    protected onElementLoaded(): void {}

    constructor(path: string, noShadow = false) {
        super();
        fetch(path)
            .then(r => r.text())
            .then(data => {
                let root: any = this;
                if (!noShadow) root = root.attachShadow({mode: 'open'});
                root.innerHTML = data;
                this.onElementLoaded();
            });
    }
}
