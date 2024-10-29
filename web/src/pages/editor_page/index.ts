//
//
// // // @ts-ignore
// // const listen = window.__TAURI__.event.listen;
// // // @ts-ignore
// // const emit = window.__TAURI__.event.emit;
// //
// // let error_msg_listen = await listen('error_message', (event) => {
// //     let p = Object.values(event.payload)[0];
// //     console.log(p);
// //     document.querySelector('#type').innerHTML = p.error_type;
// //     document.querySelector('#code').innerHTML = p.code;
// //     document.querySelector('#note').innerHTML = p.note;
// // });
// //
// // let err_msg_emit = emit('get_error_message');
//
// let m: HTMLElement = document.querySelector('main');
// let sb: HTMLElement = document.querySelector('aside');
// let st: HTMLElement = document.querySelector('section');
//
//
// const rth = 450;
// let s;
// let sw;
// const sbwt = 4 // tolerance
// let sbw = parseInt(getComputedStyle(sb, '').borderWidth) + sbwt;
// let mw = () => {return parseInt(getComputedStyle(m, '').width);}
// let md = false;
//
// const listener = (me) => {
//     let e = me.x; // the end coord (now x)
//     let d = s - e; // the difference between start and end coords
//     let nsbwp = (sw + d);
//     if (nsbwp < rth) { return; }
//     let nsbw = nsbwp/mw()*100;
//     sb.style.width = nsbw + "%";
//     st.style.width = 100 - nsbw + "%";
// }
//
// sb.addEventListener('mousedown', (e) => {
//     md = true;
//     m.style.cursor = "col-resize";
//     if (e.offsetX < sbw)
//     {
//         s = e.x;
//         sw = parseInt(getComputedStyle(sb, '').width);
//         document.addEventListener('mousemove', listener);
//     }
// });
//
// sb.addEventListener('mousemove', (e) => {
//     if (! md)
//     {
//         if (e.offsetX < sbw)
//         {
//             sb.style.cursor = "col-resize";
//         }
//         else
//         {
//             sb.style.cursor = "default";
//         }
//     }
// });
//
// document.addEventListener('mouseup', () => {
//     md = false;
//     document.removeEventListener("mousemove", listener);
//     m.style.cursor = "default";
// });
//
// window.addEventListener('resize', () => {
//     if (sbw < rth)
//     {
//         sb.style.width = rth + "px"
//     }
//     st.style.width = (mw() - rth) + "px"
// });
import { VerticalDivider, HorizontalDivider } from "scripts";

const main: HTMLElement = document.querySelector('main');
const section: HTMLElement = document.querySelector('section');
const aside: HTMLElement = document.querySelector('aside');
let vd1 = new VerticalDivider(
    main,
    section,
    aside
);

const div: HTMLElement = document.querySelector('div');
const sc_project: HTMLElement = document.querySelector('search-component.project');
const sc_installed: HTMLElement = document.querySelector('search-component.installed');
let hd1 = new HorizontalDivider(
    div,
    sc_project,
    sc_installed
)


const sb: HTMLElement = document.querySelector('right-sidebar');
const sb_w: number = parseInt(getComputedStyle(sb).width);


type regableButton = {
    be: HTMLButtonElement,
    e: HTMLElement
};

const projectButton: regableButton = {
    be: document.querySelector('button.project'),
    e: sc_project
};

const installedButton = {
    be: document.querySelector('button.installed'),
    e: sc_installed
};

const asideButtonList = [
    projectButton,
    installedButton
];

const regButton = (b: regableButton) => {
    b.be.addEventListener('click', () => {
        if (b.e.classList.contains("hidden"))
        {
            b.be.style.backgroundColor = "var(--i-rsb-button-active-background-color)";
            b.be.classList.remove("hidden");
            b.e.classList.remove("hidden");
            vd1.unhideBorder();
        }
        else
        {
            b.be.style.backgroundColor = "var(--i-rsb-button-background-color)";
            b.be.classList.add("hidden");
            b.e.classList.add("hidden");
            let hideBorder = true;
            for (let i: number = 0; i < asideButtonList.length; ++i)
                if (! (asideButtonList[i].e.classList.contains("hidden"))) { hideBorder = false; break; }
            if (hideBorder) vd1.hideBorder();
        }
    });
}

asideButtonList.forEach(regButton);