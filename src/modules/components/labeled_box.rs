#[sai_macros::element("component")]
pub fn LabeledBox(children: Element) -> Element {
    let script = r#####"
((c) =>{
	console.log("hello");
	let l = c.parentElement;
	console.log(l);
	let middleIndex = Math.floor(l.children.length / 2);
	let div = document.createElement("div");
	div.className = "divider";
	l.insertBefore(div, l.children[middleIndex + 1]);
})(document.currentScript);
"#####;
    rsx! {
        div {
            class: "LabeledBox",
            script { { script } }
            { children }
        }
    }
}
