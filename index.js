const twgl = require("twgl.js");

const canvas = document.getElementById("canvas");


class CanvasRenderer {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");

    this.setFillColor = this.setFillColor.bind(this);
    this.fillRect = this.fillRect.bind(this);
    this.clearCanvas = this.clearCanvas.bind(this);
    this.resizeCanvas = this.resizeCanvas.bind(this);

    this.resizeCanvas();

    window.addEventListener("resize", this.resizeCanvas);
  }

  resizeCanvas() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  setFillColor(r, g, b, a) {
    this.ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
  }

  fillRect(x, y, width, height) {
    this.ctx.fillRect(x, y, width, height);
  }

  clearCanvas() {
    this.ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
}

window.canvasRenderer = new CanvasRenderer(canvas);

const rust = import("./wasm_webgl_ecs");

rust.then(({ Engine }) => {
  const engine = Engine.new();

  engine.play(performance.now());

  function update(time) {
    engine.update(time);
    requestAnimationFrame(update);
  }

  requestAnimationFrame(update);
});