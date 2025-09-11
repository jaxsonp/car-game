
const showPauseMenu = (show) => {
	console.log("(TODO) Pause menu shown: ", show);
};
window.showPauseMenu = showPauseMenu;

const setDebugText = (text) => {
	const debugTextBox = document.getElementById("debug-text")
	if (debugTextBox) {
		debugTextBox.innerText = text;
	}
}
window.setDebugText = setDebugText;