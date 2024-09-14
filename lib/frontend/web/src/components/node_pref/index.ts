import { reg_css, fetch_html } from "scripts";
reg_css("node_pref", "ndpf");

// @ts-ignore
const invoke = window.__TAURI__.invoke; // uncomment
export class NodePref extends HTMLElement
{
    fail(elm, err)
    {
        elm.setCustomValidity(err.args.msg);
        elm.reportValidity();
    }
    constructor()
    {
        super();
        fetch_html(
            this,
            'components/node_pref/index.html',
            () => {
                let button: HTMLButtonElement = this.shadowRoot.querySelector('button');
                button.innerHTML = this.getAttribute('button-name');
                let form: HTMLFormElement = this.shadowRoot.querySelector('form');
                form.addEventListener('submit', (event: SubmitEvent) => {
                    event.preventDefault();
                    let name_input: HTMLInputElement = this.shadowRoot.querySelector('#name');
                    let desc_input: HTMLTextAreaElement = this.shadowRoot.querySelector('#desc');
                    let keyword_input: HTMLTextAreaElement = this.shadowRoot.querySelector('#keywords');
                    let version_input: HTMLInputElement = this.shadowRoot.querySelector('#version');
                    let author_input: HTMLInputElement = this.shadowRoot.querySelector('#author');
                    let website_input: HTMLInputElement = this.shadowRoot.querySelector('#website');
                    let request = invoke(
                        'create_node',
                        {
                            name: name_input.value,
                            desc: desc_input.value,
                            keywords: keyword_input.value,
                            version: version_input.value,
                            author: author_input.value,
                            website: website_input.value
                        }
                    );
                    console.log(request);
                    // if (request.failed())
                    // {
                    //     let errors = request.payload.errors;
                    //     // @ts-ignore
                    //     for (let error: any in errors)
                    //     {
                    //         switch (error.args.on)
                    //         {
                    //             case "name": this.fail(name_input, error); break;
                    //             case "desc": this.fail(desc_input, error); break;
                    //             case "keyword": this.fail(keyword_input, error); break;
                    //             case "version": this.fail(version_input, error); break;
                    //             case "author": this.fail(author_input, error); break;
                    //             case "website": this.fail(website_input, error); break;
                    //         }
                    //     }
                    // }
                    // else
                    // {
                    //     invoke("editor_page", request.payload.node.get_hash());
                    // }
                });
            });
    }
}

customElements.define('node-pref', NodePref);
console.log("script loaded.");
