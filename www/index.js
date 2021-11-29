import init, * as wasm from "./wasm/mazes.js";

await init();

var canvas = document.getElementById("canvas")
let data = wasm.wasm_init(canvas, 15, 0);

var tilingElement = document.getElementById("select-tiling");
var runElement = document.getElementById("run");
var stopElement = document.getElementById("stop");
var stepElement = document.getElementById("step");
var speedElement = document.getElementById("speed");
var rotationElement = document.getElementById("rotation");
var scaleElement = document.getElementById("scale");
var interval = null;
speedElement.value = 50;
scaleElement.value = 15;
rotationElement.value = 0;
var running = false;
var finished = false;

for (var tiling of wasm.get_tilings()) {
    var option = document.createElement("option");
    option.text = tiling;
    option.value = tiling;
    tilingElement.add(option);
}

tilingElement.onchange = function() {
    set_running(false);
    wasm.set_tiling(this.value, data);
}

run.onclick = function() {
    set_running(true);
}

function set_running(bool) {
    if (bool && finished) {
        finished = false;
        wasm.reset(data);
    }

    tilingElement.disabled = bool;
    stepElement.disabled = bool;
    runElement.disabled = bool;
    scaleElement.disabled = bool;
    rotationElement.disabled = bool;
    stopElement.disabled = !bool;    
    running = bool;
    if (bool) {
        let timeout = 0;
        let iterations = 1;
        let speed = parseInt(speedElement.value);
        if (speed > 50) {
            iterations = speed - 49;
        }
        if (speed < 50) {
            timeout = (50 - speed) ** 1.5;
        }
        if (interval != null) {
            clearInterval(interval);
        }
        interval = setInterval(() => {
            if (wasm.step(data, iterations)){
                set_running(false);
                finished = true;
            }
        }, timeout)
    } else {
        if (interval != null) {
            clearInterval(interval);
        }
    }
}

stopElement.onclick = function() {
    set_running(false);
}

stepElement.onclick = function() {
    wasm.step(data, 1);
}

speedElement.oninput = function() {
    if (running) {
        set_running(true);
    }
}

scaleElement.oninput = function() {
    var scale = parseInt(this.value);
    wasm.set_scale(scale, data);
}

rotationElement.oninput = function() {
    var rotation = parseInt(this.value);
    wasm.set_rotation(rotation, data);
}