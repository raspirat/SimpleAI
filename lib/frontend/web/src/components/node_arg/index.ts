import { fetch_html, reg_css } from "scripts";

reg_css("node_arg", "ndearg");

// @ts-ignore
// const invoke = window.__TAURI__.invoke; // uncomment
export class NodeArg extends HTMLElement
{
    canvas: HTMLCanvasElement;
    outDiv: HTMLDivElement;
    inDiv: HTMLDivElement;
    lineWidth: number = 3;
    scale: number = 1;

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
    divMousemove = (e: MouseEvent, d) => {
        let ctx: CanvasRenderingContext2D = this.canvasContext();
        ctx.reset();

        const coords = this.coords(d);
        const offset = {
            x: e.pageX - (coords.x + d.clientWidth / 2),
            y: e.pageY - (coords.y + d.clientHeight / 2)
        };

        offset.x /= this.scale;
        offset.y /= this.scale;

        //console.log(e.pageX, e.pageY, coords, offset);

        const end = {
            x: Math.abs(offset.x),
            y: Math.abs(offset.y)
        };

        let cs = {x: 0, y: 0};
        let ce = {
            x: end.x,
            y: end.y
        };

        if (offset.x > 0) {
            cs.x = end.x; ce.x = 0;
        } else
        {
            this.canvas.style.left = '0px';
        }
        this.styleLeft(d, ce.x);
        this.canvas.width = end.x;

        const lwh = this.lineWidth / 2;
        let cha = this.lineWidth * 1.5;
        if (offset.y > 0) {
            cs.y = end.y; ce.y = 0;
            this.styleTop(d, ce.y += this.lineWidth);
            cs.y += lwh;
        } else
        {
            this.canvas.style.top = '0px';
            this.styleTop(d, ce.y += this.lineWidth);
            cs.y += lwh;
            cha += this.lineWidth;
        }
        this.canvas.height = end.y + cha;

        let cp1 = { x: end.x / 2, y: cs.y };
        let cp2 = { x: end.x / 2, y: ce.y };

        ctx.beginPath();
        ctx.moveTo(cs.x, cs.y);
        ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, ce.x, ce.y);
        ctx.lineWidth = this.lineWidth;
        ctx.stroke();

        // ctx.fillStyle = "blue";
        // ctx.beginPath();
        // ctx.arc(ce.x, ce.y, 5, 0, 2 * Math.PI); // Start point
        // ctx.arc(ce.x, ce.y, 5, 0, 2 * Math.PI); // End point
        // ctx.fill();
        //
        // ctx.fillStyle = "red";
        // ctx.beginPath();
        // ctx.arc(cp1.x, cp1.y, 5, 0, 2 * Math.PI); // Control point one
        // ctx.arc(cp2.x, cp2.y, 5, 0, 2 * Math.PI); // Control point two
        // ctx.fill();
        // ctx.stroke();
    }
    regDiv = (d, mm) => {
        d.addEventListener('mousedown', (e: MouseEvent) => {
            document.addEventListener('mousemove', mm);
        });
        document.addEventListener('mouseup', () => {
            document.removeEventListener('mousemove', mm);
            this.canvasContext().reset();
        });
    }
    outDivMousemove = (e) => {
        this.divMousemove(e, this.outDiv);
    }
    regOutDiv = () => {
        this.regDiv(this.outDiv, this.outDivMousemove);
    }

    inDivMousemove = (e) => {
        this.divMousemove(e, this.inDiv);
    }
    regInDiv = () => {
        this.regDiv(this.inDiv, this.inDivMousemove);
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

                pe.appendChild(name);
                if (this.hasAttribute("left"))
                {
                    this.outDiv.style.visibility = "hidden";
                    this.regInDiv();
                }
                else
                {
                    this.inDiv.style.visibility = "hidden";
                    this.regOutDiv();
                }
            });
    }
}

customElements.define('node-arg', NodeArg);
console.log("script loaded.");