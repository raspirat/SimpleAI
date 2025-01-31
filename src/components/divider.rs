use dioxus::prelude::*;
use dioxus::html::{ label, input };

static STYLE: Asset = asset!("/assets/theme/components/divider/index.css");

#[component]
pub fn Divider(
	#[props(extends = GlobalAttributes)]
	attributes: Vec<Attribute>,
	children: Element
) -> Element
{
	let script =
		r#####"
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
		document::Stylesheet { href: STYLE }
		div {
			class: "Divider",
			script { { script } }
			{ children }
		}
	}
}
