import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();

function process_conversion() {
	var text = document.getElementById("input_text").value;
	var result = to_gallifreyan(text);
	console.log("result: "+result);
	var container = document.getElementById("svg_container");
	container.innerHTML = result;
}
console.log("load");
document.getElementById("input_button").addEventListener("click",process_conversion,false);
