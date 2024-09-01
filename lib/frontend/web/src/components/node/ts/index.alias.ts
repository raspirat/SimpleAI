import { fetch_html, reg_css } from "@scripts";
import { NodeArg } from "@node_arg";

console.log("Dependencies loaded: ", NodeArg.name);
reg_css("node", "nde");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class Node extends HTMLElement
{
    posX: number;
    posY: number;
    xM: number;
    yM: number;
    dragStartX: number;
    dragStartY: number;
    dragStartLeft: number;
    dragStartTop: number;
    he: HTMLElement;

    set sXM(p) {
        let d: number = this.xM - p;
        let e: number = d * (1 - this.scale);
        let n: number = this.cPX + e;
        if (e > 0) console.log(e);
        this.regXPos( n + 'px');
        this.xM = p;
    }
    set sYM(p) {
        let d: number = this.yM - p;
        let n: number = this.cPY + d * (1 - this.scale);
        this.regYPos(n + 'px');
        this.yM = p;
    }

    get width() { return this.shadowRoot.firstElementChild.clientWidth; }
    get height() { return this.shadowRoot.firstElementChild.clientHeight; }

    get hw() { return this.width / 2; }
    get hh() { return this.height / 2; }

    get dXM(): number { return this.posX - this.xM + this.hw; }
    get dYM(): number { return this.posY - this.yM + this.hh }
    get sDXM(): number { return this.dXM * this.scale; }
    get sDYM(): number { return this.dYM * this.scale; }
    get vPX(): number { return this.sDXM + this.xM - this.hw; }
    get vPY(): number { return this.sDYM + this.yM - this.hh; }


    get vDXM(): number { return this.cPX - this.xM + this.hw; }
    get vDYM(): number { return this.cPY - this.yM + this.hh; }
    get vSDXM(): number { return this.vDXM / this.scale; }
    get vSDYM(): number { return this.vDYM / this.scale; }
    get pX(): number { return this.vSDXM + this.xM - this.hw; }
    get pY(): number { return this.vSDYM + this.yM - this.hh; }

    set ssc(s: number) { this.style.scale = s.toString(); }
    set ssx(px: number) { this.style.left = px + 'px'; }
    set ssy(px: number) { this.style.top = px + 'px'; }

    cs = (): CSSStyleDeclaration => {
        return getComputedStyle(this);
    }

    get cPX(): number {
        return parseFloat(this.cs().left);
    }

    get cPY(): number {
        return parseFloat(this.cs().top);
    }

    get scale(): number {
        return parseFloat(this.style.scale);
    }

    regXPos = (newPos: string) => {
        this.style.left = newPos;
        this.posX = this.pX;
    }

    regYPos = (newPos: string) => {
        this.style.top = newPos;
        this.posY = this.pY;
    }

    wheel(e: WheelEvent) {
        this.sXM = e.x; // parent.innerWidth / 2;
        this.sYM = e.y; // parent.innerHeight / 2;

        let d: number = e.deltaY/1000;
        this.ssc = Math.max(Math.min(this.scale + d, 1), 0.01);
        this.ssx = this.vPX;
        this.ssy = this.vPY;
    }

    dragListener = (e: MouseEvent) => {
        let movementX: number = e.x - this.dragStartX;
        let movementY: number = e.y - this.dragStartY;
        this.regXPos((this.dragStartLeft + movementX) + 'px');
        this.regYPos((this.dragStartTop + movementY) + 'px');
    }

    dragStart = (e) => {
        this.dragStartX = e.x;
        this.dragStartY = e.y;
        this.dragStartLeft = this.cPX;
        this.dragStartTop = this.cPY;
        this.he.style.cursor = "grabbing";
        window.addEventListener('mousemove', this.dragListener);
    }

    dragStop = (e) => {
        this.he.style.cursor = "grab";
        window.removeEventListener('mousemove', this.dragListener);
    }

    constructor()
    {
        super();
        fetch_html(
            this,
            '/components/node/html/index.html',
            () => {
                // this is later obsolete (give hash/node obj) and figure itself out (here)
                let pe: HTMLHeadingElement = this.shadowRoot.querySelector('h2');
                let name: Text = document.createTextNode(this.getAttribute('node_name'));
                pe.appendChild(name);

                this.he = this.shadowRoot.querySelector('header');

                this.style.scale = '1';
                this.regXPos('25cqw');
                this.regYPos('50cqh');

                this.he.addEventListener('mousedown', this.dragStart);
                window.addEventListener('mouseup', this.dragStop);
            });
    }
}

customElements.define('node-elm', Node);
console.log("script loaded.");
