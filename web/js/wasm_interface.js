
window.showPauseMenu = (show) => {
	const pauseMenu = document.getElementById("pause-menu");
	if (show) {
		pauseMenu.style.display = "flex";
	} else {
		pauseMenu.style.display = "none";
	}
};

window.showDebugText = (show) => {
	const debugTextBox = document.getElementById("debug-text")
	if (show) {
		debugTextBox.style.display = "block";
	} else {
		debugTextBox.style.display = "none";
	}
}

window.setDebugText = (text) => {
	const debugTextBox = document.getElementById("debug-text")
	debugTextBox.innerText = text;
}