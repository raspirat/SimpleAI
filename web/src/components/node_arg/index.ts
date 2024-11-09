import { fetch_html, reg_css } from "scripts";

import { Node as SNode }         from "components/node"
import { Viewport as SViewport } from "components/viewport"


reg_css( "node_arg", "ndearg" );

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
    hasCanvas: boolean = true;

    constructor()
    {
        super();
        fetch_html( this, 'components/node_arg/index.html', () => {
            let pe: HTMLParagraphElement = this.shadowRoot.querySelector( 'p' );
            let name: Text = document.createTextNode( this.getAttribute(
                'arg_name' ) );
            this.canvas = this.shadowRoot.querySelector( 'canvas.draw' );
            this.outDiv = this.shadowRoot.querySelector( 'div.right' );
            this.inDiv = this.shadowRoot.querySelector( 'div.left' );
            this.container = this.shadowRoot.querySelector( 'section' );
            this.node = this.getNode();
            this.viewport = this.getViewport( "nodeViewport" );

            pe.appendChild( name );
            if ( this.hasAttribute( "left" ) )
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
        } );
    }

    get connected(): boolean
    {
        return this.connect != null;
    }

    coords = ( element: any ) => {
        let x = 0;
        let y = 0;

        while ( element )
        {
            x += element.offsetLeft;
            y += element.offsetTop;
            element = element.offsetParent;
        }

        return { x, y };
    }

    styleLeft = ( d, x ) => {
        this.canvas.style.left = d.offsetLeft + d.clientWidth / 2 - x + 'px';
    }

    styleTop = ( d, y ) => {
        this.canvas.style.top = d.offsetTop + d.clientHeight / 2 - y + 'px';
    }

    canvasContext = (): CanvasRenderingContext2D => {
        return this.canvas.getContext( '2d' );
    }

    divMousemove = ( e: MouseEvent ) => {
        this.move( new DOMPoint( e.offsetX, e.offsetY ) );
    }

    dAbsPageCoords( d: HTMLDivElement = this.activeDiv ): DOMPoint
    {
        const dBoundingRect: DOMRect = d.getBoundingClientRect();
        return new DOMPoint( dBoundingRect.x + dBoundingRect.width / 2,
                             dBoundingRect.y -
                             this.viewport.getBoundingClientRect().top +
                             dBoundingRect.height /
                             2 // + d.parentElement.getBoundingClientRect().height / 2 - dBoundingRect.height // for some reason the y is on the top of the container
        )
    } // add them to the height/width to get the middle absolute coords of the d div

    computeOffset( FROM: DOMPoint, TO: DOMPoint ): DOMPoint
    {
        return new DOMPoint( TO.x - FROM.x, FROM.y - TO.y )
    }

    scaledOffset( O: DOMPoint, WITH: number ): DOMPoint
    {
        return new DOMPoint( O.x / WITH, O.y / WITH )
    }

    absoluteOffset( O: DOMPoint ): DOMPoint
    {
        return new DOMPoint( Math.abs( O.x ), Math.abs( O.y ) )
    }

    dOffset( TO: DOMPoint ): DOMPoint
    {
        return this.computeOffset( this.dAbsPageCoords(), TO );
    }

    move = ( to: DOMPoint, d: HTMLDivElement = this.activeDiv ) => {
        let ctx: CanvasRenderingContext2D = this.canvasContext();
        ctx.reset();
        this.canvas.height = 0;
        this.canvas.width = 0;

        let offset: DOMPoint = this.dOffset( to ); // compute
        // the offset of
        // the cursor
        // relative to the div - the offset behaves like a std. coordinates-system

        // scale it to match it with the zoom capability of the parent node object
        offset = this.scaledOffset( offset, this.scale );

        // console.log(offset.x);

        // compute the absolute offset in order to get/set the dimensions of the canvas
        const absOffset: DOMPoint = this.absoluteOffset( offset );

        const element: DOMPoint = absOffset;

        const margin = new DOMPoint( d.offsetWidth / 2 + 0.5,
                                     d.offsetHeight / 2 + 0.5 );

        this.canvas.width = element.x + margin.x * 2;
        this.canvas.height = element.y + margin.y * 2;

        // now move the canvas
        let move: DOMPoint = new DOMPoint( 0,
                                           absOffset.y // default move up it's height (inverted because of top)
        );
        if ( offset.x < 0 )
        {
            move.x = absOffset.x;
        }
        if ( offset.y < 0 )
        {
            move.y = 0;
        }
        this.canvas.style.left = d.offsetLeft +
                                 d.offsetWidth /
                                 2 -
                                 move.x -
                                 margin.x +
                                 'px';
        this.canvas.style.top = d.offsetTop +
                                d.offsetHeight /
                                2 -
                                move.y -
                                margin.y +
                                'px';

        const start = new DOMPoint( move.x + margin.x, move.y + margin.x );

        const end = new DOMPoint( this.canvas.width - start.x,
                                  this.canvas.height - start.y )

        let cp1: DOMPoint = new DOMPoint( this.canvas.width / 2, start.y );
        let cp2: DOMPoint = new DOMPoint( this.canvas.width / 2, end.y );

        ctx.beginPath();
        ctx.moveTo( start.x, start.y );
        ctx.bezierCurveTo( cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y );
        ctx.lineWidth = this.lineWidth;
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo( start.x, start.y );
        ctx.arc( start.x, start.y, d.clientHeight / 2, 0, 2 * Math.PI );
        ctx.fillStyle = "dimgrey";
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo( start.x, start.y );
        ctx.arc( end.x, end.y, d.clientHeight / 2, 0, 2 * Math.PI );
        ctx.fillStyle = "dimgrey";
        ctx.fill();
    }

    connectionUpdate = () => {
        if (! this.connected ) return;
        const arg = this.getCanvasNodeArg();
        arg.move( arg.connect.dAbsPageCoords() );
    }

    unlink()
    {
        this.disconnect();
        this.clearCanvas();
    }

    regDiv = ( d = this.activeDiv ) => {
        d.addEventListener( 'mousedown', ( e: MouseEvent ) => {
            this.viewport.addEventListener( 'mousemove', this.divMousemove );
            this.viewport.dragStart( this );
            this.unlink();
        } );
        this.viewport.addEventListener( 'mouseup', ( e ) => {
            this.viewport.removeEventListener( 'mousemove', this.divMousemove );
            if ( this.viewport.connecting )
            {
                const P: DOMPoint = new DOMPoint( e.offsetX, e.offsetY );
                let O: DOMPoint = this.scaledOffset( this.absoluteOffset( this.dOffset(
                    P ) ), this.scale );
                const B: DOMRect = this.getBoundingClientRect();
                const T: number = B.height / 2;
                let connect: NodeArg = this.viewport.dragOrigin;
                if ( O.x <
                     (
                         T * 4
                     ) &&
                     O.y <
                     T )
                {
                    this.viewport.dragEnd();
                    if (this.tryConnect(connect)) { return; }
                    this.unlink();
                }
                connect.unlink();
            }
        } );
    }

    clearCanvas()
    {
        let c = this.canvasContext();
        c.clearRect( 0, 0, this.canvas.width, this.canvas.height );
        c.reset();
    }

    disconnect()
    {
        if (! this.connected) { return; }
        this.getCanvasNodeArg().clearCanvas();
        this.connect.hasCanvas = true;
        this.hasCanvas = true;
        this.connect.connect = null;
        this.connect = null;
    }

    isInput(): boolean { return this.activeDiv.id == "input"; }

    isOutput(): boolean { return this.activeDiv.id == "output"; }

    tryConnect(to: NodeArg): boolean
    {
        if ( (
                 to.isOutput() && this.isOutput()
             ) ||
             (
                 to.isInput() && this.isInput()
             ) || ( to.node == this.node ))
        {
            return false;
        }
        this.disconnect();
        this.connect = to
        this.connect.connect = this;
        this.connect.hasCanvas = true;
        this.hasCanvas = false;
        this.connectionUpdate()
        return true;
    }

    getCanvasNodeArg = (): NodeArg => {
        if ( this.hasCanvas ) return this;
        return this.connect;
    }

    getViewport = ( attrName: string = "nodeViewport" ): & SViewport => {
        let element: any = this.parentElement;
        while ( !element.hasAttribute( attrName ) )
        {
            let node: Node = element.parentNode
            if ( node instanceof ShadowRoot ) node = node.host;
            if ( node instanceof HTMLElement )
            {
                element = node;
            }
            else
            {
                console.error(
                    "Please specify the viewport with an attribute on your viewport named: ",
                    attrName );
            }
        }
        return element;
    }

    getNode = (): & SNode => {
        let element: any = this.parentElement;
        while ( !(
            element instanceof SNode
        ) )
        {
            let node: Node = element.parentNode
            if ( node instanceof ShadowRoot ) node = node.host;
            if ( node instanceof HTMLElement ) element = node;
        }
        return element;
    }
}



customElements.define( 'node-arg', NodeArg );
console.log( "script loaded." );