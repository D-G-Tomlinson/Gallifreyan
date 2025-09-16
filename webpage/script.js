import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();
function process_conversion() {
	var result = to_gallifreyan(document.getElementById("input_text").value);
	document.getElementById("svg_box").innerHTML = result;
	document.getElementById("download_link").setAttribute("href","data:image/svg+xml;base64," + btoa(result));
}
document.getElementById("input_button").addEventListener("click",process_conversion,false);
