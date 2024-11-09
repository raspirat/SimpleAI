import {HorizontalDivider, VerticalDivider} from "components/divider";

const vd: VerticalDivider = document.querySelector('vertical-divider');
const hd1: HorizontalDivider = document.querySelector('horizontal-divider');

const sc_project: HTMLElement = document.querySelector('search-component.project');
const sc_installed: HTMLElement = document.querySelector('search-component.installed');

let regButtonList = [];

const regButton = (b: HTMLButtonElement, afterReveal = () => {}, afterHide = () => {}) => {
    regButtonList.push(b);
    b.addEventListener('click', () => {
        if (b.classList.contains("hidden")) {
            b.classList.remove("hidden");
            vd.revealAfter()
            afterReveal();
        } else {
            b.classList.add("hidden");
            let hideDivider = true;
            for (let i: number = 0; i < regButtonList.length; ++i)
            {
                if (!(regButtonList[i].classList.contains("hidden"))) {
                    hideDivider = false;
                    break;
                }
            }
            if (hideDivider) {
                vd.hideAfter();
            }
            afterHide();
        }
    });
}


regButton(
    document.querySelector('button.project'),
    () => {
        hd1.revealBefore();
    },
    () => {
        hd1.hideBefore();
    }
)

regButton(
    document.querySelector('button.installed'),
    () => {
        hd1.revealAfter();
    },
    () => {
        hd1.hideAfter();
    }
)