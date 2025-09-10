
const updateFPS = (fpsValue) => {
	const fpsText = document.getElementById("fps-value");
	if (fpsText) {
		fpsText.innerText = Math.round(fpsValue);
	}
};
window.updateFPS = updateFPS;


const setDebugText = (text) => {
	const debugTextBox = document.getElementById("debug-text")
	if (debugTextBox) {
		debugTextBox.innerText = text;
	}
}
window.setDebugText = setDebugText;