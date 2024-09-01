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
import { VerticalDevider, HorizontalDevider } from "@scripts";

let vd = new VerticalDevider(
    document.querySelector('main'),
    document.querySelector('section'),
    document.querySelector('aside')
);

let hd1 = new HorizontalDevider(
    document.querySelector('div'),
    document.querySelector('search-component.project'),
    document.querySelector('search-component.installed')
)


let as: HTMLElement = document.querySelector('aside');
let sb: HTMLElement = document.querySelector('right-sidebar');
let sb_w: number = parseInt(getComputedStyle(sb).width);
function regButton(be: HTMLButtonElement, e: HTMLElement)
{
    be.addEventListener('click', () => {
        if (e.style.display == "none")
        {
            be.style.backgroundColor = "var(--i-rsb-button-active-background-color)"
            e.style.display = "initial";
        }
        else
        {
            be.style.backgroundColor = "var(--i-rsb-button-background-color)"
            e.style.display = "none";
            as.style.width = sb_w + "px";
        }
    })
}

// the following works because this script is run before the right-sidebar script
let pb: HTMLButtonElement = document.querySelector('button.project');
let psc: HTMLElement = document.querySelector('search-component.project');
regButton(pb, psc);

let ib: HTMLButtonElement = document.querySelector('button.installed');
let isc: HTMLElement = document.querySelector('search-component.installed');
regButton(ib, isc);

