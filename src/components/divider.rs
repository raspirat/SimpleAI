#[sai_macros::element("component")]
pub fn Divider(style: String, children: Element) -> Element {
    let script = r#####"
((c) =>{
	console.log("hello");
	let l = c.parentElement;
	console.log(l);
	let middleIndex = Math.floor(l.children.length / 2);
	let wrapper = document.createElement("div");
	wrapper.className = "wrapper";
	let div = document.createElement("div");
	div.className = "inner";
	wrapper.appendChild(div);
	l.insertBefore(wrapper, l.children[middleIndex + 1]);
})(document.currentScript);
"#####;
    rsx! {
        style { { style } }
        div {
            class: "Divider",
            script { { script } }
            { children }
        }
    }
}
