import { ElementFromHTML } from "scripts";


function addDims( ...DIMS: DOMPoint[] ): DOMPoint
{
    return DIMS.reduce( ( acc: DOMPoint, point: DOMPoint ): DOMPoint => {
        acc.x += point.x;
        acc.y += point.y;
        return acc;
    }, new DOMPoint( 0, 0 ) );
}

function NaNToZero( N: number ): number { return isNaN( N ) ? 0 : N; }

function NaNToInfinity( N: number ): number
{
    return isNaN( N ) ? Infinity : N;
}

function parseFloatForMin( S: string ): number { return NaNToZero( parseFloat( S ) ); }

function parseFloatForMax( S: string ): number
{
    return NaNToInfinity( parseFloat( S ) );
}

function normalizeDim( D: number, MIN: number, MAX: number ): number
{
    return Math.max( Math.min( D, MAX ), MIN );
}

function normalizeDimsWithStyle(
    STYLE: CSSStyleDeclaration,
    DIMS: DOMPoint ): DOMPoint
{
    const X_MIN: number = parseFloatForMin( STYLE.minWidth );
    const X_MAX: number = parseFloatForMax( STYLE.maxWidth );
    const Y_MIN: number = parseFloatForMin( STYLE.minHeight );
    const Y_MAX: number = parseFloatForMax( STYLE.maxHeight );
    return new DOMPoint( normalizeDim( DIMS.x, X_MIN, X_MAX ),
                         normalizeDim( DIMS.y, Y_MIN, Y_MAX ) );
}

function normalizeDimsFor( E: HTMLElement, DIMS: DOMPoint ): DOMPoint
{
    const STYLE: CSSStyleDeclaration = getComputedStyle( E );
    return normalizeDimsWithStyle( STYLE, DIMS );
}

function dimsOf( E: HTMLElement ): DOMPoint
{
    const BR: DOMRect = E.getBoundingClientRect();
    return new DOMPoint( BR.width, BR.height );
}



class DividerElement
{
    private readonly __element: HTMLElement
    private __savedDims: DOMPoint
    private __startDims: DOMPoint

    public constructor( ELEMENT: HTMLElement )
    {
        this.__element = ELEMENT;
        this.__savedDims = new DOMPoint( 0, 0 );
        this.__startDims = new DOMPoint( 0, 0 );
        this.saveDims();
    }

    public get element(): HTMLElement
    {
        return this.__element;
    }

    public get savedDims(): DOMPoint
    {
        return this.__savedDims;
    }

    public get startDims(): DOMPoint
    {
        return this.__startDims;
    }

    public get zIndex(): number
    {
        return NaNToZero( parseInt( getComputedStyle( this.__element ).zIndex ) );
    }

    public get dims(): DOMPoint
    {
        return dimsOf( this.__element );
    }

    public set dims( d: DOMPoint )
    {
        d = normalizeDimsFor(this.__element, d);
        const PR: DOMRect = this.element.parentElement.getBoundingClientRect();
        this.element.style.width = `${ d.x / PR.width * 100 }%`;
        this.element.style.height = `${ d.y / PR.height * 100 }%`;
    }

    public start(): void
    {
        this.__startDims = this.dims;
    }

    public saveDims(): void
    {
        this.__savedDims = this.dims;
    }

    public newDims( OFFSET: DOMPoint ): DOMPoint
    {
        const D: DOMPoint = this.startDims;
        return new DOMPoint( D.x - OFFSET.x, D.y - OFFSET.y );
    }

    public toSave(): void
    {
        this.dims = this.savedDims;
    }
}



class Divider extends ElementFromHTML
{
    private __startCoords: DOMPoint
    private readonly __elementBefore: DividerElement
    private readonly __elementAfter: DividerElement

    constructor( cursorStyle: string )
    {
        super( "components/divider/index.html", true );

        this.style.display = "flex";
        this.style.alignItems = "center";

        if ( this.previousElementSibling instanceof HTMLElement )
        {
            this.__elementBefore
                = new DividerElement( this.previousElementSibling );
        }
        else
        {
            console.error( "No Element before Divider!!" );
        }

        if ( this.nextElementSibling instanceof HTMLElement )
        {
            this.__elementAfter = new DividerElement( this.nextElementSibling );
        }
        else
        {
            console.error( "No Element after Divider!!" );
        }

        this.addEventListener( 'mouseenter', () => {
            this.style.cursor = cursorStyle;
        } );

        this.addEventListener( 'mouseleave', () => {
            this.style.cursor = "initial";
        } );

        this.addEventListener( 'mousedown', ( E: MouseEvent ) => {
            this.start( E );
            document.addEventListener( 'mousemove', this.mouseMove );
        } );

        document.addEventListener( 'mouseup', () => {
            document.removeEventListener( 'mousemove', this.mouseMove );
        } );
    }

    private get addedDims(): DOMPoint
    {
        const EC: Element = this.firstElementChild;
        let sd: DOMPoint = new DOMPoint( 0, 0 );
        if ( EC instanceof HTMLElement )
        {
            sd = dimsOf( EC );
        }
        return addDims( this.__elementBefore.dims,
                        this.__elementAfter.dims,
                        sd );
    }

    public start( E: MouseEvent ): void
    {
        this.__startCoords = new DOMPoint( E.x, E.y );
        this.__elementBefore.start();
        this.__elementAfter.start();
    }

    public hideBefore(): void
    {
        this.hideElement( this.__elementBefore, this.__elementAfter );
    }

    public hideAfter(): void
    {
        this.hideElement( this.__elementAfter, this.__elementBefore );
    }

    public revealBefore(): void
    {
        this.revealElement( this.__elementBefore );
    }

    public revealAfter(): void
    {
        this.revealElement( this.__elementAfter );
    }

    protected override onElementLoaded(): void
    {
        this.setZIndex();
        this.setMargin();
    }

    protected setZIndex(): void
    {
        this.style.zIndex = `${ Math.max( this.__elementAfter.zIndex,
                                          this.__elementBefore.zIndex ) + 1 }`;
    }

    protected setMargin(): void { }

    protected setDims( e: DividerElement, DA: DOMPoint ): void { }

    protected revealElement( toReveal: DividerElement ): void
    {
        toReveal.element.style.display = "inherit";
        if ( this.__elementAfter.element.style.display !=
             "none" &&
             this.__elementBefore.element.style.display !=
             "none" )
        {
            this.style.display = "inherit";
        }
        this.__elementAfter.toSave();
        this.__elementBefore.toSave();
    }

    private hideElement( toHide: DividerElement,
                         notToHide: DividerElement ): void
    {
        this.setDims( notToHide, this.addedDims );
        this.style.display = "none";
        toHide.element.style.display = "none";
    }

    private mouseMove = ( E: MouseEvent ): void => {
        const OA: DOMPoint = new DOMPoint( this.__startCoords.x - E.x,
                                           this.__startCoords.y - E.y )
        const OB: DOMPoint = new DOMPoint( -OA.x, -OA.y );
        const DA: DOMPoint = this.__elementAfter.newDims( OB );
        const DB: DOMPoint = this.__elementBefore.newDims( OA );
        this.__elementAfter.saveDims();
        this.__elementBefore.saveDims();
        this.setDims( this.__elementAfter, DA );
        this.setDims( this.__elementBefore, DB );
        this.setMargin();
    }
}



export class VerticalDivider extends Divider
{
    constructor()
    {
        super( "col-resize" );
        this.style.flexDirection = "column";
    }

    protected override setMargin()
    {
        const M: number = (
                              this.getBoundingClientRect().width -
                              this.firstElementChild.getBoundingClientRect().width
                          ) / 2;
        this.style.margin = `0 -${ M }px`;
    }

    protected override setDims( e: DividerElement, DA: DOMPoint )
    {
        e.dims = new DOMPoint( DA.x, e.dims.y );
    }
}



customElements.define( 'vertical-divider', VerticalDivider );



export class HorizontalDivider extends Divider
{
    constructor()
    {
        super( "row-resize" );
        this.style.flexDirection = "row";
    }

    protected override setMargin(): void
    {
        const M: number = (
                              this.getBoundingClientRect().height -
                              this.firstElementChild.getBoundingClientRect().height
                          ) / 2;
        this.style.margin = `-${ M }px 0`;
    }

    protected override setDims( e: DividerElement, DA: DOMPoint )
    {
        e.dims = new DOMPoint( e.dims.x, DA.y );
    }
}



customElements.define( 'horizontal-divider', HorizontalDivider );


console.log( "script loaded." );
