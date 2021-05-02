import * as wasm from "wasm-nes-emulator";

let nes = new wasm.WasmNes();
let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');

let romreader = document.getElementById("rom");

romreader.addEventListener("change", () => {
    var reader = new FileReader();
    reader.onload = (e) => {
        const arraybuffer = e.target.result;
        const bytearray = new Uint8Array(arraybuffer);
        nes.load(bytearray);
        nes.reset();
    }
    reader.readAsArrayBuffer(romreader.files[0]);
});

function render() {
    let tile = canvas.height / 256;

    for (let y = 0; y < 240; y++) {
        for (let x = 0; x < 256; x++) {
            let rgb = nes.screen_pixel(y, x);
            ctx.fillStyle = rgb == 0 ? "#000000" : "#ffffff";
            ctx.fillRect(x*tile, y*tile, tile, tile);    
        }
    }
}

function updateReg() {
    let regs = nes.get_cpu_reg();
    let reg_list = ["pc", "ac", "x", "y", "sp", "status"];
    for (let reg of reg_list) {
        document.getElementById(reg).innerHTML = regs[reg];
    }
}

setInterval(render, 1000 / 20);
setInterval(() => {
    nes.clock();
    updateReg();
}, 1000 / 60);

