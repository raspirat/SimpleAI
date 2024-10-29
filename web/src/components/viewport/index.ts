import { fetch_html, reg_css } from "scripts";
import { Node } from "components/node";
import { NodeArg } from "components/node_arg"

console.log("Dependencies loaded: ", Node.name);
reg_css("node_pref", "tcpt");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class Viewport extends HTMLElement
{
    main: HTMLElement;
    dragOrigin: NodeArg;

    dragStart(e: NodeArg){ this.dragOrigin = e; }
    dragEnd(): NodeArg {
        const r = this.dragOrigin;
        this.dragOrigin = null;
        return r;
    }

    cs = (child) => getComputedStyle(child);
    scale = (cs) => parseFloat(cs.scale);
    posX = (cs): number => parseFloat(cs.left);
    posY = (cs): number => parseFloat(cs.top);

    mainChildren = (): NodeListOf<HTMLElement> => { return this.shadowRoot.querySelectorAll('main>*'); }
    scrollFactor = (d: number, s: number): number => {
        let a = (Math.abs(Math.min(Math.max(s, 0), 2) - 1) - 1)
        return d * (a ^ 20) / 100;
    }

    minValue = (v: number): number => { return Math.max(v, 0.1); }

    forNode = (f) => {
        this.mainChildren().forEach((child: HTMLElement) => {
            if (child instanceof Node)
                f(child);
        })
    }

    scrollListener = (e: WheelEvent) => {
        this.forNode((c) => {
            c.wheel(e);
        })
    }

    mouseDownListener = (e: MouseEvent) => {
        if (e.target != this.main) return;
        this.style.cursor = 'all-scroll';
        this.forNode((c) => {
            c.dragStart(e);
        })
    }

    mouseUpListener = () => {
        this.style.cursor = 'default';
    }

    addNode(child)
    {
        let cs: CSSStyleDeclaration = this.cs(child);
        let scale: number = this.scale(child);
        let posX:  number = this.posX(cs);
        let posY: number = this.posY(cs);
        let newPosX: string = (posX + 50) + "px";
        let newPosY: string = (posY + 50) + "px";
        if (isNaN(scale)) child.style.scale = "1";
        // if (isNaN(posX)) child.style.left = newPosX;
        // if (isNaN(posY)) child.style.top = newPosY;
    }

    constructor()
    {
        super();
        fetch_html(
            this,
            'components/viewport/index.html',
            () => {
                this.main = this.shadowRoot.querySelector('main');

                this.mainChildren().forEach((child: HTMLElement) => {
                    this.addNode(child);
                });

                this.addEventListener('wheel', this.scrollListener);
                this.main.addEventListener('mousedown', this.mouseDownListener);
                window.addEventListener('mouseup', this.mouseUpListener);
            });
    }
}

customElements.define('view-port', Viewport);
console.log("script loaded.");
