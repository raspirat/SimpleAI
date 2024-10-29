import { fetch_html, reg_css } from "scripts";

import { Node as SNode } from "components/node"
import { Viewport as SViewport } from "components/viewport"

reg_css("node_arg", "ndearg");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class NodeArg extends HTMLElement
{
    canvas: HTMLCanvasElement;
    outDiv: HTMLDivElement;
    inDiv: HTMLDivElement;
    activeDiv: HTMLDivElement;
    container: HTMLElement;
    lineWidth: number = 3;
    scale: number = 1;
    connect: NodeArg;
    viewport: SViewport;
    node: SNode;
    hasCanvas: boolean = false;

    coords = (element: any) => {
        let x = 0;
        let y = 0;

        while(element) {
            x += element.offsetLeft;
            y += element.offsetTop;
            element = element.offsetParent;
        }

        return { x, y };
    }

    styleLeft = (d, x) => {
        this.canvas.style.left = d.offsetLeft + d.clientWidth / 2 - x + 'px';
    }

    styleTop = (d, y) => {
        this.canvas.style.top = d.offsetTop + d.clientHeight / 2 - y + 'px';
    }

    canvasContext = (): CanvasRenderingContext2D => { return this.canvas.getContext('2d'); }

    divMousemove = (e: MouseEvent) => {
        if (this.connect != null) return;
        this.move({x: e.offsetX, y: e.offsetY});
    }

    dAbsPageCoords = (d: HTMLDivElement = this.activeDiv): { x: number, y: number } => {
        const dBoundingRect: DOMRect = d.getBoundingClientRect();
        return {
            x: dBoundingRect.x + dBoundingRect.width / 2,
            y: dBoundingRect.y - this.viewport.getBoundingClientRect().top + dBoundingRect.height / 2 // + d.parentElement.getBoundingClientRect().height / 2 - dBoundingRect.height // for some reason the y is on the top of the container
        }
    } // add them to the height/width to get the middle absolute coords of the d div

    move = (to: {x: number, y: number}, d: HTMLDivElement = this.activeDiv) => {
        let ctx: CanvasRenderingContext2D = this.canvasContext();
        ctx.reset();
        this.canvas.height = 0;
        this.canvas.width = 0;

        const dAbsPageCoords = this.dAbsPageCoords();

        let offset = {
            x: to.x - dAbsPageCoords.x,
            y: dAbsPageCoords.y - to.y
        }; // compute the offset of the cursor relative to the div - the offset behaves like a std. coordinates-system

        // scale it to match it with the zoom capability of the parent node object
        offset.x /= this.scale;
        offset.y /= this.scale;

        // console.log(offset.x);

        // compute the absolute offset in order to get/set the dimensions of the canvas
        const absOffset = {
            x: Math.abs(offset.x),
            y: Math.abs(offset.y)
        }

        const element = {
            x: absOffset.x,
            y: absOffset.y
        }

        const margin = {
            x: d.offsetWidth / 2 + 0.5,
            y: d.offsetHeight / 2 + 0.5
        }

        this.canvas.width = element.x + margin.x * 2;
        this.canvas.height = element.y + margin.y * 2;

        // now move the canvas
        let move = {
            x: 0,
            y: absOffset.y // default move up it's height (inverted because of top)
        };
        if (offset.x < 0)
        {
            move.x = absOffset.x;
        }
        if (offset.y < 0)
        {
            move.y = 0;
        }
        this.canvas.style.left = d.offsetLeft + d.offsetWidth / 2 - move.x - margin.x + 'px';
        this.canvas.style.top = d.offsetTop + d.offsetHeight / 2 - move.y - margin.y + 'px';

        const start = {
            x: move.x + margin.x,
            y: move.y + margin.x
        }

        const end = {
            x: this.canvas.width - start.x,
            y: this.canvas.height - start.y
        }

        let cp1 = { x: this.canvas.width / 2, y: start.y };
        let cp2 = { x: this.canvas.width / 2, y: end.y };

        ctx.beginPath();
        ctx.moveTo(start.x, start.y);
        ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y);
        ctx.lineWidth = this.lineWidth;
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo(start.x, start.y);
        ctx.arc(start.x, start.y, d.clientHeight / 2, 0, 2 * Math.PI);
        ctx.fillStyle = "dimgrey";
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(start.x, start.y);
        ctx.arc(end.x, end.y, d.clientHeight / 2, 0, 2 * Math.PI);
        ctx.fillStyle = "dimgrey";
        ctx.fill();

        // const end = {
        //     x: Math.abs(offset.x),
        //     y: Math.abs(offset.y)
        // };
        //
        // let cs = {x: 0, y: 0};
        // let ce = {
        //     x: end.x,
        //     y: end.y
        // };
        //
        // if (offset.x > 0)
        //     cs.x = end.x; ce.x = 0;
        // this.styleLeft(d, ce.x);
        // this.canvas.width = end.x + d.clientHeight / 2;
        //
        // if (offset.y > 0)
        //     cs.y = end.y; ce.y = 0;
        // this.styleTop(ce.y, d);
        // this.canvas.height = end.y + d.clientHeight / 2;
        //
        // let cp1 = { x: end.x / 2, y: cs.y };
        // let cp2 = { x: end.x / 2, y: ce.y };
        //
        // ctx.beginPath();
        // ctx.moveTo(cs.x, cs.y);
        // ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, ce.x, ce.y);
        // ctx.lineWidth = this.lineWidth;
        // ctx.stroke();
        //
        // ctx.beginPath();
        // ctx.moveTo(ce.x, ce.y);
        // ctx.arc(cs.x, cs.y, d.clientHeight / 2, 0, 2 * Math.PI);
        // ctx.fillStyle = "dimgrey";
        // ctx.fill();

        // ctx.fillStyle = "blue";
        // ctx.beginPath();
        // ctx.arc(end.x, end.y, 5, 0, 2 * Math.PI); // Start point
        // ctx.arc(end.x, end.y, 5, 0, 2 * Math.PI); // End point
        // ctx.fill();
        //
        // ctx.fillStyle = "red";
        // ctx.beginPath();
        // ctx.arc(cp1.x, cp1.y, 5, 0, 2 * Math.PI); // Control point one
        // ctx.arc(cp2.x, cp2.y, 5, 0, 2 * Math.PI); // Control point two
        // ctx.fill();
        // ctx.stroke();
    }

    connectionUpdate = () => {
        const arg = this.getCanvasNodeArg();
        if (arg == null) return;
        arg.move(arg.connect.dAbsPageCoords());
    }

    regDiv = (d = this.activeDiv) => {
        d.addEventListener('mousedown', (e: MouseEvent) => {
            this.viewport.addEventListener('mousemove', this.divMousemove);
            this.viewport.dragStart(this);
        });
        document.addEventListener('mouseup', (e) => {
            this.viewport.removeEventListener('mousemove', this.divMousemove);
        });
        d.addEventListener('mouseup', () => {
            this.connect = this.viewport.dragEnd();
            this.connect.connect = this;
            this.connect.hasCanvas = true;
            this.connectionUpdate()
        });
    }

    getCanvasNodeArg = (): NodeArg  => {
        if (this.hasCanvas)
            return this;
        return this.connect;
    }

    getViewport = (attrName: string = "nodeViewport"): & SViewport => {
        let element: any = this.parentElement;
        while (! element.hasAttribute(attrName))
        {
            let node: Node = element.parentNode
            if (node instanceof ShadowRoot)
                node = node.host;
            if (node instanceof HTMLElement)
                element = node;
            else {
                console.error("Please specify the viewport with an attribute on your viewport named: ", attrName);
            }
        }
        return element;
    }

    getNode = (): & SNode => {
        let element: any = this.parentElement;
        while (!(element instanceof SNode))
        {
            let node: Node = element.parentNode
            if (node instanceof ShadowRoot)
                node = node.host;
            if (node instanceof HTMLElement)
                element = node;
        }
        return element;
    }

    constructor()
    {
        super();
        fetch_html(
            this,
            'components/node_arg/index.html',
            () => {
                let pe: HTMLParagraphElement = this.shadowRoot.querySelector('p');
                let name: Text = document.createTextNode(this.getAttribute('arg_name'));
                this.canvas = this.shadowRoot.querySelector('canvas.draw');
                this.outDiv = this.shadowRoot.querySelector('div.right');
                this.inDiv = this.shadowRoot.querySelector('div.left');
                this.container = this.shadowRoot.querySelector('section');
                this.node = this.getNode();
                this.viewport = this.getViewport("nodeViewport");

                pe.appendChild(name);
                if (this.hasAttribute("left"))
                {
                    this.outDiv.style.visibility = "hidden";
                    this.activeDiv = this.inDiv;
                }
                else
                {
                    this.inDiv.style.visibility = "hidden";
                    this.activeDiv = this.outDiv;
                }
                this.regDiv();
            });
    }
}

customElements.define('node-arg', NodeArg);
console.log("script loaded.");