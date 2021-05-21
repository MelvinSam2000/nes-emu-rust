import * as wasm from "wasm-nes-emulator";
import AudioEngine from "./audio-engine";

let nes = new wasm.WasmNes();
let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');
let paused = true;

let romreader = document.getElementById("rom");

let audioEngine = new AudioEngine();

romreader.addEventListener("change", () => {
    var reader = new FileReader();
    reader.onload = (e) => {
        const arraybuffer = e.target.result;
        const bytearray = new Uint8Array(arraybuffer);
        paused = true;
        nes.load(bytearray);
        nes.reset();
        paused = false;
        audioEngine.start();
    }
    reader.readAsArrayBuffer(romreader.files[0]);
});

function render() {

    let image_data = ctx.getImageData(0, 0, canvas.width, canvas.height);
    let pixels = image_data.data;

    for (let x = 0; x < 256; x++) {
        for (let y = 0; y < 240; y++) {

            let r = nes.get_pixel_r(x, y);
            let g = nes.get_pixel_g(x, y);
            let b = nes.get_pixel_b(x, y);
            let off = (y * image_data.width + x) * 4;
            
            pixels[off] = r;
            pixels[off + 1] = g;
            pixels[off + 2] = b;
            pixels[off + 3] = 255;
        }
    }
    ctx.putImageData(image_data, 0, 0);
}

window.addEventListener("keydown", e => {
    if (e.key == "a" || e.key == "A")   nes.button_pressed(0, true);
    if (e.key == "s" || e.key == "S")   nes.button_pressed(1, true);
    if (e.key == "z" || e.key == "Z")   nes.button_pressed(2, true);
    if (e.key == "x" || e.key == "X")   nes.button_pressed(3, true);
    if (e.key == "ArrowUp")             nes.button_pressed(4, true);
    if (e.key == "ArrowDown")           nes.button_pressed(5, true);
    if (e.key == "ArrowLeft")           nes.button_pressed(6, true);
    if (e.key == "ArrowRight")          nes.button_pressed(7, true);
});

window.addEventListener("keyup", e => {
    if (e.key == "a" || e.key == "A")   nes.button_pressed(0, false);
    if (e.key == "s" || e.key == "S")   nes.button_pressed(1, false);
    if (e.key == "z" || e.key == "Z")   nes.button_pressed(2, false);
    if (e.key == "x" || e.key == "X")   nes.button_pressed(3, false);
    if (e.key == "ArrowUp")             nes.button_pressed(4, false);
    if (e.key == "ArrowDown")           nes.button_pressed(5, false);
    if (e.key == "ArrowLeft")           nes.button_pressed(6, false);
    if (e.key == "ArrowRight")          nes.button_pressed(7, false);
});

setInterval(() => {
    if (!paused) {
        render();
    }
    
}, 1000 / 30);

function update() {
    if (!paused) {
        try {
            for (let i = 0; i < 100000; i++) {
                nes.clock();
                let apu_code = nes.apu_check_updated();
                if (apu_code != 0) {
                    audioEngine.processApuCode(apu_code, nes.get_apu_config())
                }
            }  
        } catch(e) {
            paused = true;
            console.log(e);
        }
    }
    requestAnimationFrame(update);
}

update();
