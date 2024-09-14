export class Devider
{
    parent: HTMLElement;
    noBorder: HTMLElement;
    border: HTMLElement;
    isMouseDown: boolean = false;
    startPos: number;
    borderStartWidth: number;
    mouseMoveListener = (e) => {this.mouseMove(e)}

    borderWidth: () => number;
    pos: (e: MouseEvent) => number;
    offset: (e: MouseEvent) => number;
    borderStyle: (s: string) => void;
    noBorderStyle: (s: string) => void;
    borderElementSize: () => number;
    parentElementSize: () => number;
    cursor: () => string;

    onBorder(e: MouseEvent) : boolean
    {
        return e.target == this.border && this.offset(e) < this.borderWidth();
    }
    mouseMove(e: MouseEvent)
    {
        let diff: number = this.startPos - this.pos(e);
        let newBorderPos: number = this.borderStartWidth + diff;
        let newBorderPercent: number = newBorderPos / this.parentElementSize() * 100;
        this.borderStyle( newBorderPercent + "%");
        this.noBorderStyle(100 - newBorderPercent + "%");
    }
    mouseDown(e: MouseEvent)
    {
        this.isMouseDown = true;
        if (this.onBorder(e))
        {
            this.startPos = this.pos(e);
            this.borderStartWidth = this.borderElementSize();
            this.parent.style.cursor = this.cursor();
            document.addEventListener('mousemove', this.mouseMoveListener);
        }
    }
    mouseUp(e: MouseEvent)
    {
        this.isMouseDown = false;
        this.parent.style.cursor = "default";
        document.removeEventListener('mousemove', this.mouseMoveListener);
    }
    mouseHover(e: MouseEvent)
    {
        if (this.onBorder(e))
        {
            this.border.style.cursor = this.cursor();
        }
        else
        {
            this.border.style.cursor = "default";
        }
    }
    constructor
    (
        parent: HTMLElement,
        noBorder: HTMLElement,
        border: HTMLElement
    )
    {
        this.parent = parent;
        this.noBorder = noBorder;
        this.border = border;

        let bzi: number = parseFloat(getComputedStyle(this.noBorder).zIndex);
        this.border.style.zIndex = (bzi - 1).toString() ;

        this.border.addEventListener('mousedown', (e) => {this.mouseDown(e)});
        document.addEventListener('mouseup', (e) => {this.mouseUp(e)});
        this.border.addEventListener('mousemove', (e) => {this.mouseHover(e)});
    }
}

export class VerticalDevider extends Devider
{
    borderWidth = () => { return parseFloat(getComputedStyle(this.border).borderLeftWidth); }
    pos = (e) => { return e.x; }
    offset = (e) => { return e.offsetX; }

    borderStyle = (s) => { this.border.style.width = s; }
    noBorderStyle = (s) => { this.noBorder.style.width = s; }

    borderElementSize = () =>
    {
        return parseFloat(getComputedStyle(this.border).width);
    }
    parentElementSize = () =>
    {
        return parseFloat(getComputedStyle(this.parent).width);
    }

    cursor = () => {
        return "col-resize";
    }

    constructor(
        parent: HTMLElement,
        noBorder: HTMLElement,
        border: HTMLElement,
    )
    {
        super(
            parent,
            noBorder,
            border
        )
    }
}

export class HorizontalDevider extends Devider
{
    borderWidth = () => { return parseFloat(getComputedStyle(this.border).borderTopWidth); }
    pos = (e) => { return e.y; }
    offset = (e) => { return e.offsetY; }

    borderStyle = (s) => { this.border.style.height = s; }
    noBorderStyle = (s) => { this.noBorder.style.height = s; }

    borderElementSize = () =>
    {
        return parseFloat(getComputedStyle(this.border).height);
    }
    parentElementSize = () =>
    {
        return parseFloat(getComputedStyle(this.parent).height);
    }

    cursor = () => {
        return "row-resize";
    }

    constructor(
        parent: HTMLElement,
        noBorder: HTMLElement,
        border: HTMLElement,
    )
    {
        super(
            parent,
            noBorder,
            border
        )
    }
}

