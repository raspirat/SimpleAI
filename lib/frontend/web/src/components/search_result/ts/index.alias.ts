import { reg_css, fetch_html } from "@scripts";
reg_css("search-result", "sr");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class SearchResult extends HTMLElement
{
    nodeId: bigint;
    constructor(node)
    {
        super();
        this.nodeId = node.id;
        fetch_html(
            this,
            '/components/search_result/html/index.html',
            () => {
                this.shadowRoot.querySelector('#name').innerHTML = node.name;
                this.shadowRoot.querySelector('#date').innerHTML = node.date;
                this.shadowRoot.querySelector('#website').innerHTML = node.website;
                this.shadowRoot.querySelector('#description').innerHTML = node.description;
                this.shadowRoot.querySelector('#author').innerHTML = node.author;
                let favourite: Element = this.shadowRoot.querySelector('#favorite');
                favourite.addEventListener('click', () => {
                    // invoke('toggle_favourite', {id: this.nodeId});
                    // if (invoke('is_favourite') {
                    favourite.className = "ri-star-fill";
                    // }
                    // else
                    // {
                    //      favourite.className = "ri-star-line";
                    // }
                });
                let open: Element = this.shadowRoot.querySelector('#open');
                open.addEventListener('click', () => {
                    // invoke('editor', {id: this.nodeId});
                });
            });
    }
}

customElements.define('search-result', SearchResult);
console.log("script loaded.")
