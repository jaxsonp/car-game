const canvasId: string = "main-canvas";
const canvas = document.getElementById(canvasId) as HTMLCanvasElement;

if (!canvas) {
    throw new Error(`Canvas element with ID "${canvasId}" not found.`);
}

// update the canvas size based on the device pixel ratio
const updateCanvasDPR = (): void => {
    const dpr: number = window.devicePixelRatio;
    const maxPixelSize: number = 2048 / dpr;
    canvas.style.width = `min(100vw, ${maxPixelSize}px)`;
    canvas.style.height = `min(100vh, ${maxPixelSize}px)`;
};
updateCanvasDPR();

// listen and handle changes in the device's resolution.
(function listenForDPRChange(): void {
    const onChange = (): void => {
        updateCanvasDPR();
        listenForDPRChange();
    };

    matchMedia(`(resolution: ${window.devicePixelRatio}dppx)`).addEventListener(
        "change",
        onChange,
        { once: true }
    );
})();

// running the wasm
import init, { run_game } from "../pkg/car_game.js";

const runWasm = async (): Promise<void> => {
    await init();
    run_game(canvasId);
};

console.log("WASM module loaded");
runWasm();