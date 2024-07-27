import { reg_css, fetch_html } from "@scripts";
reg_css("search-bar", "sb");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class SearchBar extends HTMLElement
{
    onresult = (results) => {};
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/search_bar/html/index.html',
            () => {
                let search_form: HTMLFormElement = this.shadowRoot.querySelector('#search_form');
                search_form.addEventListener('submit', (event: SubmitEvent) => {
                        event.preventDefault();
                        let search_input: HTMLInputElement = this.shadowRoot.querySelector('#search_bar_input');
                        console.log("searching: ", search_input.value);
                        // let results = invoke('search', {query: search_input.value);
                        // onresult(this.results);
                    }
                )
            });
    }
}

customElements.define('search-bar', SearchBar);
console.log("script loaded.")
