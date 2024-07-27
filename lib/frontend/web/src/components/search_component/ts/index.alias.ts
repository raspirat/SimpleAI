import { reg_css, fetch_html } from "@scripts";
reg_css("search-component", "scp");
import { SearchBar } from "@search_bar";
import { SearchResult} from "@search_result";

console.log("Dependencies loaded: ", SearchBar.name, SearchResult.name);
// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class SearchComponent extends HTMLElement
{
    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/search_component/html/index.html',
            () => {
                let search_results: HTMLUListElement = this.shadowRoot.querySelector('#search-results')
                let search_bar: SearchBar = this.shadowRoot.querySelector('search-bar');
                search_bar.onresult = (results) => {
                    results.forEach(
                        (result) =>
                        {
                            search_results.appendChild(
                                new SearchResult(result)
                            )
                        }
                    )
                };
            });
    }
}

customElements.define('search-component', SearchComponent);
console.log("script loaded.");
