export {};

declare global {
  interface Window {
    showPauseMenu: (show: boolean) => void;
    showDebugText: (show: boolean) => void;
    setDebugText: (text: string) => void;
    setSpeed: (speed: number) => void;
    showFlipMessage: (show: boolean) => void;
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

window.setSpeed = (speed: number): void => {
	const gauge = document.getElementById("gauge-container");
	if (gauge) {
		gauge.style.setProperty('--speed', speed.toString());
	} else {
		console.warn('Failed to find gauge container element');
	}
}

window.showFlipMessage = (show: boolean): void => {
	const msg = document.getElementById("flip-msg");
	if (msg) {
		show ?  msg.classList.add("visible") : msg.classList.remove("visible");
	} else {
		console.warn('Failed to find flip message element');
	}
}