/* tslint:disable */
import * as wasm from "./wasm_webgl_ecs_bg";

const TextDecoder =
  typeof window === "object" && window.TextDecoder
    ? window.TextDecoder
    : require("util").TextDecoder;

let cachedDecoder = null;
function textDecoder() {
  if (cachedDecoder) return cachedDecoder;
  cachedDecoder = new TextDecoder("utf-8");
  return cachedDecoder;
}

let cachedUint8Memory = null;
function getUint8Memory() {
  if (
    cachedUint8Memory === null ||
    cachedUint8Memory.buffer !== wasm.memory.buffer
  )
    cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
  return cachedUint8Memory;
}

function getStringFromWasm(ptr, len) {
  return textDecoder().decode(getUint8Memory().slice(ptr, ptr + len));
}

let cachedUint32Memory = null;
function getUint32Memory() {
  if (
    cachedUint32Memory === null ||
    cachedUint32Memory.buffer !== wasm.memory.buffer
  )
    cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
  return cachedUint32Memory;
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
  if (cachedGlobalArgumentPtr === null)
    cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
  return cachedGlobalArgumentPtr;
}

function getGlobalArgument(arg) {
  const idx = globalArgumentPtr() / 4 + arg;
  return getUint32Memory()[idx];
}

const __wbg_f_log_log_n_target = console.log;
export function __wbg_f_log_log_n(arg0) {
  let len0 = getGlobalArgument(0);
  let v0 = getStringFromWasm(arg0, len0);
  __wbg_f_log_log_n_target(v0);
}
export class Engine {
  constructor(ptr) {
    this.ptr = ptr;
  }

  free() {
    const ptr = this.ptr;
    this.ptr = 0;
    wasm.__wbg_engine_free(ptr);
  }
  static new() {
    const ret = wasm.engine_new();
    return new Engine(ret);
  }
  play(arg0) {
    const ret = wasm.engine_play(this.ptr, arg0);
    return ret;
  }
  update(arg0) {
    const ret = wasm.engine_update(this.ptr, arg0);
    return ret;
  }
  pause() {
    const ret = wasm.engine_pause(this.ptr);
    return ret;
  }
}
export function __wbindgen_throw(ptr, len) {
  throw new Error(getStringFromWasm(ptr, len));
}
