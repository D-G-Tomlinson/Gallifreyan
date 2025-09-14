import init, {to_gallifreyan } from "./pkg/gallifreyan.js";
init().then();
function process_conversion() {	document.getElementById("svg_box").innerHTML = to_gallifreyan(document.getElementById("input_text").value);}
document.getElementById("input_button").addEventListener("click",process_conversion,false);
