import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();

function process_conversion() {
	var text = document.getElementById("input_text").value;
	var result = to_gallifreyan(text);
	console.log("result: "+result);
	var container = document.getElementById("svg_box");
	container.innerHTML = result;
}
console.log("load");
document.getElementById("input_text").addEventListener("input",process_conversion,false);
