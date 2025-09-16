import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();
function add_download(text) {
	var templ = document.createElement("template");
	templ.innerHTML = text.trim();
	let svg = templ.content.firstChild;

	let style = window.getComputedStyle(document.getElementById("generated_svg"),null);
	svg.setAttribute("stroke",style.stroke);
	svg.setAttribute("fill", style.fill);
	document.getElementById("download_link").setAttribute("href","data:image/svg+xml;base64," + btoa(svg.outerHTML));
}
function process_conversion() {
	let result = to_gallifreyan(document.getElementById("input_text").value);
	document.getElementById("svg_box").innerHTML = result;
	add_download(result);
}
document.getElementById("input_button").addEventListener("click",process_conversion,false);
