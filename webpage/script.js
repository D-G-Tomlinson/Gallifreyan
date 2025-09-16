import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();

const downloadhtml = '<a class="gall_button" id="download_link" title="Download" download="gallifreyan.svg"><button class="gall_button"><img id="download_img" class="pulsing" src="webpage/Images/download.svg" alt="download"></button></a>';

function get_el(html) {
	var templ = document.createElement("template");
	templ.innerHTML = html.trim();
	return templ.content.firstChild;
}

function add_download(text) {
	let svg = get_el(text);

	let down_butt = document.getElementById("download_link");
	let not_empty = document.getElementById("all_gall");

	if (!not_empty) {
		if (down_butt) {
			down_butt.remove();
		}
		return;
	}
	if (!down_butt) {
		let new_download = get_el(downloadhtml);
		document.getElementById("buttons").appendChild(new_download);
	}
	let style = window.getComputedStyle(document.getElementById("generated_svg"),null);
	svg.setAttribute("stroke",style.stroke);
	svg.setAttribute("fill", style.fill);
	let address = "data:image/svg+xml;base64," + btoa(svg.outerHTML);
	console.log(address);
	document.getElementById("download_link").setAttribute("href",address);
}
function process_conversion() {
	let result = to_gallifreyan(document.getElementById("input_text").value);
	document.getElementById("svg_box").innerHTML = result;
	add_download(result);
}
document.getElementById("input_button").addEventListener("click",process_conversion,false);
