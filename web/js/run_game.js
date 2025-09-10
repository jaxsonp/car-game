const canvas_id = "main-canvas";
let canvas = document.getElementById(canvas_id);

const updateCanvasDPR = () => {
	const dpr = window.devicePixelRatio;
	const maxPixelSize = 2048 / dpr;
	canvas.style.width = `min(100vw, ${maxPixelSize}px)`
	canvas.style.height = `min(100vh, ${maxPixelSize}px)`
};

updateCanvasDPR();
(function listenForDPRChange() {
	const onChange = () => {
		updateCanvasDPR();
		listenForDPRChange();
	}
	matchMedia(
		`(resolution: ${window.devicePixelRatio}dppx)`
	).addEventListener("change", onChange, { once: true });
})();

import init, { run_game } from "../pkg/car_game.js";

const runWasm = async () => {
	await init();
	run_game(canvas_id);
};
console.log("WASM module loaded")
runWasm();
