import init, * as wasm from "./wasm/mazes.js";

await init();

var canvas = document.getElementById("canvas")
let data = wasm.wasm_init(canvas);

var tilingElement = document.getElementById("select-tiling");
var runElement = document.getElementById("run");
var stopElement = document.getElementById("stop");
var stepElement = document.getElementById("step");
var speedElement = document.getElementById("speed");

var interval = null;
var speed = 50;
speedElement.value = speed;
var running = false;

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
    tilingElement.disabled = bool;
    stepElement.disabled = bool;
    runElement.disabled = bool;
    stopElement.disabled = !bool;
    running = bool;
    if (bool) {
        let timeout = 0;
        let iterations = 1;
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
    speed = parseInt(this.value);
    if (running) {
        set_running(true);
    }
}