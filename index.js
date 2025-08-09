import init, * as wasm from "./wasm_interface/pkg/wasm_interface.js";
await init();
window.wasm = wasm;
const themes = wasm.themes();
const themesElement = document.getElementById("themes");
// place default at top
{
	const defaultIndex = themes.indexOf("default");
	const [defaultTheme] = themes.splice(defaultIndex, 1);
	themes.unshift(defaultTheme);
}
themes.forEach(theme => {
	const option = document.createElement("option");
	option.textContent = theme;
	themesElement.appendChild(option);
})
const htmloutElement = document.getElementById("htmlout");
const cssoutElement = document.getElementById("cssout");
document.getElementById("htmlcopy").addEventListener('click', _ => {
	htmloutElement.select();
	document.execCommand("copy");
});
document.getElementById("csscopy").addEventListener('click', _ => {
	cssoutElement.select();
	document.execCommand("copy");
});
const passcodeElement = document.getElementById("passcode");
const generateElement = document.getElementById("generate");
const errorElement = document.getElementById("error");
const assert = (cond, msg) => { if (!cond) throw new Error(msg); };
generateElement.addEventListener("click", _ => {
	assert(typeof passcodeElement.value === "string");
	assert(typeof themesElement.value === "string");
	try {
		const config = new wasm.KeypadConfig(passcodeElement.value, themesElement.value);
		const html = wasm.wasm_generate_keypad_html(config);
		const css = wasm.wasm_generate_keypad_css(config);
		htmloutElement.textContent = html;
		cssoutElement.textContent = css;
		config.free();
		errorElement.textContent = "";
	} catch(err) {
		errorElement.textContent = err.toString();
	}
});