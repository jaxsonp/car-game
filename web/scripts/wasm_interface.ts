export {};

declare global {
  interface Window {
    showPauseMenu: (show: boolean) => void;
    showDebugText: (show: boolean) => void;
    setDebugText: (text: string) => void;
  }
}

window.showPauseMenu = (show: boolean): void => {
	const pauseMenu = document.getElementById("pause-menu");
	if (pauseMenu) {
		pauseMenu.style.display = show ? "flex" : "none";
	} else {
		console.warn('Failed to find pause menu element');
	}
};

window.showDebugText = (show: boolean): void => {
	const debugTextBox = document.getElementById("debug-text")
	if (debugTextBox) {
		debugTextBox.style.display = show ? "block" : "none";
	} else {
		console.warn('Failed to find debug text element');
	}
}

window.setDebugText = (text: string): void => {
	const debugTextBox = document.getElementById("debug-text")
	if (debugTextBox) {
		debugTextBox.innerText = text;
	} else {
		console.warn('Failed to find debug text element');
	}
}