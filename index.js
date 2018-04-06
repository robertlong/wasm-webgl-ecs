const twgl = require("twgl.js");
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