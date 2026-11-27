// Compiles a dart2wasm-generated main module from `source` which can then
// be instantiated via the `instantiate` method.
//
// `source` needs to be a `Response` object (or promise thereof) e.g. created
// via the `fetch()` JS API.
export async function compileStreaming(source) {
  const builtins = {builtins: ['js-string'], importedStringConstants: ''};
  return new CompiledApp(
      await _compileStreaming(source, builtins), builtins);
}

// Compiles a dart2wasm-generated wasm module from `bytes` which is then
// instantiable via the `instantiate` method.
export async function compile(bytes) {
  const builtins = {builtins: ['js-string'], importedStringConstants: ''};
  return new CompiledApp(await WebAssembly.compile(bytes, builtins), builtins);
}

let _isCompileStreamingSupported;
async function _compileStreaming(source, builtins) {
  _isCompileStreamingSupported ??= WebAssembly.compileStreaming(
    new Response(
      new Uint8Array([0,97,115,109,1,0,0,0,1,4,1,96,0,0,2,23,1,14,119,97,115,109,58,106,115,45,115,116,114,105,110,103,4,99,97,115,116,0,0]),
      {headers: {'Content-Type': 'application/wasm'}},
    ),
    builtins,
  ).then(() => false, (e) => e instanceof WebAssembly.CompileError);
  if (await _isCompileStreamingSupported) {
    return WebAssembly.compileStreaming(source, builtins);
  }
  return WebAssembly.compile(await (await source).arrayBuffer(), builtins);
}

class CompiledApp {
  constructor(module, builtins) {
    this.module = module;
    this.builtins = builtins;
  }

  // The second argument is an options object containing:
  // `loadDeferredModules` is a JS function that takes an array of module names
  //   matching wasm files produced by the dart2wasm compiler. It also takes a
  //   callback that should be invoked for each loaded module with 2 arguments:
  //   (1) the module name, (2) the loaded module in a format supported by
  //   `WebAssembly.compile` or `WebAssembly.compileStreaming`. The callback
  //   returns a Promise that resolves when the module is instantiated.
  //   loadDeferredModules should return a Promise that resolves when all the
  //   modules have been loaded and the callback promises have resolved.
  // `loadDeferredId` is a JS function that takes load ID produced by the
  //   compiler when the `use-load-ids` option is passed. Each load ID maps to
  //   one or more wasm files as specified in the emitted JSON file. It also
  //   takes a callback that should be invoked for each loaded module with 2
  //   arguments: (1) the module name, (2) the loaded module in a format
  //   supported by `WebAssembly.compile` or `WebAssembly.compileStreaming`.
  //   The callback returns a Promise that resolves when the module is
  //   instantiated.
  //   loadDeferredId should return a Promise that resolves when all the
  //   modules have been loaded and the callback promises have resolved.
  async instantiate(additionalImports, {loadDeferredModules, loadDeferredId} = {}) {
    let dartInstance;

    // Prints to the console
    function printToConsole(value) {
      if (typeof dartPrint == "function") {
        dartPrint(value);
        return;
      }
      if (typeof console == "object" && typeof console.log != "undefined") {
        console.log(value);
        return;
      }
      if (typeof print == "function") {
        print(value);
        return;
      }

      throw "Unable to print message: " + value;
    }

    // A special symbol attached to functions that wrap Dart functions.
    const jsWrappedDartFunctionSymbol = Symbol("JSWrappedDartFunction");

    function finalizeWrapper(dartFunction, wrapped) {
      wrapped.dartFunction = dartFunction;
      wrapped[jsWrappedDartFunctionSymbol] = true;
      return wrapped;
    }

    // Imports
    const dart2wasm = {
            AB: () => globalThis.Math,
      AC: Function.prototype.call.bind(DataView.prototype.setUint32),
      AD: x0 => x0.height,
      AE: (x0,x1) => { x0.textContent = x1 },
      AF: x0 => x0.identifier,
      AG: (x0,x1) => x0.querySelectorAll(x1),
      AH: (x0,x1) => x0.lock(x1),
      AI: x0 => x0.disabled,
      AJ: (x0,x1) => x0.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustDrill(x1),
      AK: x0 => x0.done,
      B: s => printToConsole(s),
      BB: (x0,x1,x2,x3) => x0.addEventListener(x1,x2,x3),
      BC: Function.prototype.call.bind(DataView.prototype.setInt16),
      BD: x0 => x0.width,
      BE: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      BF: x0 => x0.touches,
      BG: (x0,x1) => x0.requestAnimationFrame(x1),
      BH: x0 => x0.orientation,
      BI: (x0,x1) => { x0.min = x1 },
      BJ: (x0,x1) => x0.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustDrill(x1),
      BK: x0 => x0.read(),
      C: Function.prototype.call.bind(Number.prototype.toString),
      CB: b => !!b,
      CC: Function.prototype.call.bind(DataView.prototype.setUint16),
      CD: x0 => x0.screen,
      CE: x0 => x0.matches,
      CF: x0 => x0.pressure,
      CG: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      CH: (x0,x1) => x0.querySelector(x1),
      CI: (x0,x1) => { x0.max = x1 },
      CJ: x0 => x0.frb_get_rust_content_hash(),
      CK: x0 => x0.body,
      D: Function.prototype.call.bind(BigInt.prototype.toString),
      DB: () => globalThis.document,
      DC: Function.prototype.call.bind(DataView.prototype.setUint8),
      DD: o => {
        if (o === null || o === undefined) return 0;
        if (typeof(o) === 'string') return 1;
        return 2;
      },
      DE: (x0,x1) => x0.matchMedia(x1),
      DF: x0 => x0.tiltY,
      DG: x0 => x0.now(),
      DH: (x0,x1) => { x0.title = x1 },
      DI: (x0,x1) => { x0.disabled = x1 },
      DJ: (x0,x1) => x0.append(x1),
      DK: (x0,x1) => new OffscreenCanvas(x0,x1),
      E: (exn) => {
        let stackString = exn.toString();
        let frames = stackString.split('\n');
        let drop = 4;
        if (frames[0].startsWith('Error')) {
            drop += 1;
        }
        return frames.slice(drop).join('\n');
      },
      EB: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      EC: Function.prototype.call.bind(DataView.prototype.setInt8),
      ED: x0 => x0.tabIndex,
      EE: x0 => x0.matches,
      EF: x0 => x0.tiltX,
      EG: x0 => x0.performance,
      EH: (x0,x1) => x0.vibrate(x1),
      EI: (x0,x1) => { x0.scrollLeft = x1 },
      EJ: (x0,x1) => x0.call(x1),
      EK: x0 => x0.assetBase,
      F: o => String(o),
      FB: (x0,x1) => x0.focus(x1),
      FC: Function.prototype.call.bind(DataView.prototype.getInt8),
      FD: (x0,x1) => x0.contains(x1),
      FE: f => f.dartFunction,
      FF: x0 => x0.pointerType,
      FG: (d, digits) => d.toFixed(digits),
      FH: x0 => x0.arrayBuffer(),
      FI: (x0,x1) => { x0.spellcheck = x1 },
      FJ: x0 => x0.call(),
      FK: x0 => x0.loader,
      G: () => new Error().stack,
      GB: () => ({}),
      GC: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Int8Array) return 1;
        return 2;
      },
      GD: x0 => x0.activeElement,
      GE: o => typeof o === 'function' && o[jsWrappedDartFunctionSymbol] === true,
      GF: x0 => x0.pointerId,
      GG: x0 => x0.maxHeight,
      GH: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof ArrayBuffer) return 1;
        if (globalThis.SharedArrayBuffer !== undefined &&
            o instanceof SharedArrayBuffer) {
          return 2;
        }
        return 3;
      },
      GI: (x0,x1) => { x0.disabled = x1 },
      GJ: x0 => new Function(x0),
      GK: () => globalThis._flutter,
      H: s => JSON.stringify(s),
      HB: (o, p, v) => o[p] = v,
      HC: (o, start, length) => new Float64Array(o.buffer, o.byteOffset + start, length),
      HD: x0 => x0.parentNode,
      HE: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      HF: x0 => x0.getCoalescedEvents(),
      HG: x0 => x0.maxWidth,
      HH: x0 => x0.status,
      HI: (a, i) => a.splice(i, 1),
      HJ: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      I: Function.prototype.call.bind(Number.prototype.toString),
      IB: () => [],
      IC: (o, start, length) => new Float32Array(o.buffer, o.byteOffset + start, length),
      ID: x0 => x0.tagName,
      IE: (wasmFunction,f) => finalizeWrapper(f, function(x0,x1) { return wasmFunction(f,arguments.length,x0,x1) }),
      IF: s => s.trimLeft(),
      IG: x0 => x0.minHeight,
      IH: (x0,x1) => x0.fetch(x1),
      II: a => a.pop(),
      IJ: (x0,x1,x2,x3) => x0.addEventListener(x1,x2,x3),
      J: Function.prototype.call.bind(String.prototype.indexOf),
      JB: (a, i) => a.push(i),
      JC: (o, start, length) => new Uint32Array(o.buffer, o.byteOffset + start, length),
      JD: x0 => x0.target,
      JE: (p, s, f) => p.then(s, (e) => f(e, e === undefined)),
      JF: s => s.toUpperCase(),
      JG: x0 => x0.minWidth,
      JH: x0 => x0.content,
      JI: x0 => new WeakRef(x0),
      JJ: (x0,x1,x2,x3) => x0.removeEventListener(x1,x2,x3),
      K: (s, p, i) => s.lastIndexOf(p, i),
      KB: x0 => new Int8Array(x0),
      KC: (o, start, length) => new Int32Array(o.buffer, o.byteOffset + start, length),
      KD: x0 => x0.clientY,
      KE: (o, i) => o[i],
      KF: (x0,x1) => x0.test(x1),
      KG: (x0,x1) => x0.removeProperty(x1),
      KH: x0 => x0.document,
      KI: x0 => x0.deref(),
      KJ: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      L: o => o,
      LB: (jsArray, jsArrayOffset, wasmArray, wasmArrayOffset, length) => {
        const getValue = dartInstance.exports.$wasmI8ArrayGet;
        for (let i = 0; i < length; i++) {
          jsArray[jsArrayOffset + i] = getValue(wasmArray, wasmArrayOffset + i);
        }
      },
      LC: (o, start, length) => new Uint16Array(o.buffer, o.byteOffset + start, length),
      LD: x0 => x0.clientX,
      LE: o => o.length,
      LF: x0 => x0.pop(),
      LG: (x0,x1) => x0.add(x1),
      LH: () => typeof dartUseDateNowForTicks !== "undefined",
      LI: () => globalThis.WeakRef,
      LJ: x0 => x0.head,
      M: o => {
        if (o === undefined || o === null) return 0;
        if (typeof o === 'number') return 1;
        return 2;
      },
      MB: x0 => new Uint8Array(x0),
      MC: (o, start, length) => new Int16Array(o.buffer, o.byteOffset + start, length),
      MD: (x0,x1,x2) => x0.addEventListener(x1,x2),
      ME: o => {
        if (o === undefined) return 1;
        var type = typeof o;
        if (type === 'boolean') return 2;
        if (type === 'number') return 3;
        if (type === 'string') return 4;
        if (o instanceof Array) return 5;
        if (ArrayBuffer.isView(o)) {
          if (o instanceof Int8Array) return 6;
          if (o instanceof Uint8Array) return 7;
          if (o instanceof Uint8ClampedArray) return 8;
          if (o instanceof Int16Array) return 9;
          if (o instanceof Uint16Array) return 10;
          if (o instanceof Int32Array) return 11;
          if (o instanceof Uint32Array) return 12;
          if (o instanceof Float32Array) return 13;
          if (o instanceof Float64Array) return 14;
          if (o instanceof DataView) return 15;
        }
        if (o instanceof ArrayBuffer) return 16;
        // Feature check for `SharedArrayBuffer` before doing a type-check.
        if (globalThis.SharedArrayBuffer !== undefined &&
            o instanceof SharedArrayBuffer) {
            return 17;
        }
        if (o instanceof Promise) return 18;
        return 19;
      },
      MF: x0 => x0.flags,
      MG: x0 => x0.data,
      MH: () => Date.now(),
      MI: (o, offsetInBytes, lengthInBytes) => {
        var dst = new ArrayBuffer(lengthInBytes);
        new Uint8Array(dst).set(new Uint8Array(o, offsetInBytes, lengthInBytes));
        return new DataView(dst);
      },
      MJ: () => globalThis.document,
      N: x0 => x0.index,
      NB: x0 => new Uint8ClampedArray(x0),
      NC: (o, start, length) => new Uint8ClampedArray(o.buffer, o.byteOffset + start, length),
      ND: (x0,x1,x2) => x0.setAttribute(x1,x2),
      NE: x0 => x0.language,
      NF: (x0,x1) => x0.error(x1),
      NG: (x0,x1) => x0.removeAttribute(x1),
      NH: () => 1000 * performance.now(),
      NI: (a, s, e) => a.slice(s, e),
      NJ: (x0,x1) => { x0.src = x1 },
      O: o => o === undefined,
      OB: x0 => new Int16Array(x0),
      OC: (o, start, length) => new Uint8Array(o.buffer, o.byteOffset + start, length),
      OD: (x0,x1) => x0.closest(x1),
      OE: (x0,x1,x2,x3) => x0.register(x1,x2,x3),
      OF: () => globalThis.console,
      OG: (x0,x1) => { x0.value = x1 },
      OH: (x0,x1) => x0.decode(x1),
      OI: (x0,x1) => x0.error(x1),
      OJ: (x0,x1) => x0.createElement(x1),
      P: (x0,x1) => x0.exec(x1),
      PB: x0 => new Uint16Array(x0),
      PC: (o, start, length) => new Int8Array(o.buffer, o.byteOffset + start, length),
      PD: (ms, c) =>
      setTimeout(() => dartInstance.exports.$invokeCallback(c),ms),
      PE: () => globalThis.window.FinalizationRegistry,
      PF: s => s.trimRight(),
      PG: (x0,x1) => { x0.value = x1 },
      PH: (x0,x1,x2) => x0.slice(x1,x2),
      PI: () => globalThis.console,
      PJ: (x0,x1) => x0.warn(x1),
      Q: (x0,x1) => { x0.lastIndex = x1 },
      QB: x0 => new Int32Array(x0),
      QC: (x0,x1) => x0.querySelector(x1),
      QD: (handle) => clearTimeout(handle),
      QE: x0 => new window.FinalizationRegistry(x0),
      QF: x0 => x0.blur(),
      QG: x0 => x0.value,
      QH: x0 => new Uint8Array(x0),
      QI: () => new MessageChannel(),
      QJ: () => globalThis.crossOriginIsolated,
      R: o => o,
      RB: (jsArray, jsArrayOffset, wasmArray, wasmArrayOffset, length) => {
        const getValue = dartInstance.exports.$wasmI32ArrayGet;
        for (let i = 0; i < length; i++) {
          jsArray[jsArrayOffset + i] = getValue(wasmArray, wasmArrayOffset + i);
        }
      },
      RC: (x0,x1) => x0.item(x1),
      RD: x0 => x0.maxTouchPoints,
      RE: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      RF: (x0,x1) => x0.setPointerCapture(x1),
      RG: x0 => x0.selectionDirection,
      RH: (x0,x1) => x0.adoptText(x1),
      RI: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      RJ: (x0,x1,x2,x3,x4,x5,x6,x7,x8,x9) => x0.drawImage(x1,x2,x3,x4,x5,x6,x7,x8,x9),
      S: (s, m) => {
        try {
          return new RegExp(s, m);
        } catch (e) {
          return String(e);
        }
      },
      SB: x0 => new Uint32Array(x0),
      SC: x0 => x0.length,
      SD: x0 => x0.platform,
      SE: (x0,x1) => x0.unregister(x1),
      SF: x0 => x0.button,
      SG: x0 => x0.selectionStart,
      SH: x0 => x0.first(),
      SI: x0 => x0.port2,
      SJ: (x0,x1) => x0.getContext(x1),
      T: (string, times) => string.repeat(times),
      TB: x0 => new Float32Array(x0),
      TC: (x0,x1) => x0.querySelectorAll(x1),
      TD: s => new Date(s * 1000).getTimezoneOffset() * 60,
      TE: x0 => x0.preventDefault(),
      TF: (x0,x1) => x0.prepend(x1),
      TG: x0 => x0.selectionEnd,
      TH: x0 => x0.next(),
      TI: (x0,x1) => { x0.onmessage = x1 },
      TJ: (x0,x1) => { x0.height = x1 },
      U: o => o instanceof RegExp,
      UB: (jsArray, jsArrayOffset, wasmArray, wasmArrayOffset, length) => {
        const getValue = dartInstance.exports.$wasmF32ArrayGet;
        for (let i = 0; i < length; i++) {
          jsArray[jsArrayOffset + i] = getValue(wasmArray, wasmArrayOffset + i);
        }
      },
      UC: (x0,x1) => x0.getAttribute(x1),
      UD: Date.now,
      UE: x0 => x0.parent,
      UF: x0 => x0.parentElement,
      UG: x0 => x0.value,
      UH: x0 => x0.current(),
      UI: x0 => globalThis.Object.keys(x0),
      UJ: (x0,x1) => { x0.width = x1 },
      V: o => o,
      VB: x0 => new Float64Array(x0),
      VC: x0 => x0.remove(),
      VD: (x0,x1) => x0.contains(x1),
      VE: x0 => x0.timeStamp,
      VF: x0 => x0.innerHeight,
      VG: x0 => x0.selectionDirection,
      VH: (x0,x1) => new Intl.v8BreakIterator(x0,x1),
      VI: x0 => x0.length,
      VJ: (x0,x1) => x0.transferFromImageBitmap(x1),
      W: o => {
        if (o === undefined || o === null) return 0;
        if (typeof o === 'boolean') return 1;
        return 2;
      },
      WB: (jsArray, jsArrayOffset, wasmArray, wasmArrayOffset, length) => {
        const getValue = dartInstance.exports.$wasmF64ArrayGet;
        for (let i = 0; i < length; i++) {
          jsArray[jsArrayOffset + i] = getValue(wasmArray, wasmArrayOffset + i);
        }
      },
      WC: (x0,x1) => x0.appendChild(x1),
      WD: (decoder, codeUnits) => decoder.decode(codeUnits),
      WE: (x0,x1) => x0.hasAttribute(x1),
      WF: x0 => x0.innerWidth,
      WG: x0 => x0.selectionStart,
      WH: x0 => x0.v8BreakIterator,
      WI: (o) => Array.isArray(o) || o instanceof Array,
      WJ: x0 => x0.height,
      X: x0 => x0.dotAll,
      XB: x0 => new ArrayBuffer(x0),
      XC: (x0,x1) => x0.append(x1),
      XD: () => new TextDecoder("utf-8", {fatal: true}),
      XE: x0 => x0.type,
      XF: x0 => x0.height,
      XG: x0 => x0.selectionEnd,
      XH: () => globalThis.Intl,
      XI: (o, t) => typeof o === t,
      XJ: x0 => x0.width,
      Y: x0 => x0.unicode,
      YB: (x0,x1,x2) => new Uint8Array(x0,x1,x2),
      YC: (x0,x1,x2,x3) => x0.setProperty(x1,x2,x3),
      YD: () => new TextDecoder("utf-8", {fatal: false}),
      YE: (x0,x1) => x0.getModifierState(x1),
      YF: x0 => x0.width,
      YG: (x0,x1) => { x0.name = x1 },
      YH: (x0,x1) => x0.segment(x1),
      YI: x0 => x0.data,
      YJ: x0 => x0.rasterEndMilliseconds,
      Z: x0 => x0.ignoreCase,
      ZB: (x0,x1,x2) => new DataView(x0,x1,x2),
      ZC: x0 => x0.style,
      ZD: (jsArray, jsArrayOffset, wasmArray, wasmArrayOffset, length) => {
        const setValue = dartInstance.exports.$wasmI8ArraySet;
        for (let i = 0; i < length; i++) {
          setValue(wasmArray, wasmArrayOffset + i, jsArray[jsArrayOffset + i]);
        }
      },
      ZE: x0 => x0.buttons,
      ZF: x0 => x0.clientHeight,
      ZG: (x0,x1) => { x0.placeholder = x1 },
      ZH: x0 => x0.index,
      ZI: x0 => x0.port1,
      ZJ: x0 => x0.rasterStartMilliseconds,
      a: x0 => x0.multiline,
      aB: (o, p) => o[p],
      aC: x0 => x0.debugShowSemanticsNodes,
      aD: (s) => +s,
      aE: x0 => x0.ctrlKey,
      aF: x0 => x0.clientWidth,
      aG: (x0,x1) => { x0.autocomplete = x1 },
      aH: x0 => x0.next(),
      aI: (x0,x1) => new SharedWorker(x0,x1),
      aJ: x0 => x0.imageBitmaps,
      b: (exn) => {
        if (exn instanceof Error) {
          return exn.stack;
        } else {
          return null;
        }
      },
      bB: (o) => new DataView(o.buffer, o.byteOffset, o.byteLength),
      bC: (x0,x1) => x0.warn(x1),
      bD: s => {
        if (!/^\s*[+-]?(?:Infinity|NaN|(?:\.\d+|\d+(?:\.\d*)?)(?:[eE][+-]?\d+)?)\s*$/.test(s)) {
          return NaN;
        }
        return parseFloat(s);
      },
      bE: x0 => x0.getBoundingClientRect(),
      bF: x0 => x0.isConnected,
      bG: (x0,x1) => { x0.type = x1 },
      bH: x0 => x0.value,
      bI: x0 => new Worker(x0),
      bJ: x0 => x0.canvasKitMaximumSurfaces,
      c: (x0,x1) => x0.then(x1),
      cB: Function.prototype.call.bind(Object.getOwnPropertyDescriptor(DataView.prototype, 'byteLength').get),
      cC: x0 => x0.console,
      cD: s => s.trim(),
      cE: x0 => x0.y,
      cF: x0 => x0.head,
      cG: (x0,x1) => { x0.name = x1 },
      cH: x0 => x0.done,
      cI: (x0,x1,x2) => x0.postMessage(x1,x2),
      cJ: x0 => x0.hostElement,
      d: () => globalThis.Promise.resolve(),
      dB: o => o.byteOffset,
      dC: () => globalThis.window,
      dD: x0 => x0.classList,
      dE: x0 => x0.x,
      dF: (x0,x1) => { x0.content = x1 },
      dG: (x0,x1) => { x0.placeholder = x1 },
      dH: (o, m, a) => o[m].apply(o, a),
      dI: (o, p, v) => o[p] = v,
      dJ: x0 => x0.location,
      e: (c) =>
      queueMicrotask(() => dartInstance.exports.$invokeCallback(c)),
      eB: o => o.buffer,
      eC: (o, c) => o instanceof c,
      eD: x0 => x0.relatedTarget,
      eE: x0 => x0.top,
      eF: (x0,x1) => { x0.name = x1 },
      eG: (x0,x1) => { x0.scrollTop = x1 },
      eH: x0 => x0.iterator,
      eI: (x0,x1) => { x0.onerror = x1 },
      eJ: (x0,x1) => x0.getModifierState(x1),
      f: (x0,x1) => x0.didCreateEngineInitializer(x1),
      fB: Function.prototype.call.bind(DataView.prototype.getUint8),
      fC: (x0,x1) => x0[x1],
      fD: x0 => x0.hasFocus(),
      fE: x0 => x0.left,
      fF: (x0,x1) => x0.removeChild(x1),
      fG: (x0,x1,x2) => x0.setSelectionRange(x1,x2),
      fH: () => globalThis.Symbol,
      fI: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      fJ: x0 => x0.metaKey,
      g: (x0,x1) => ({initializeEngine: x0,autoStart: x1}),
      gB: (b, o) => new DataView(b, o),
      gC: x0 => x0.length,
      gD: x0 => x0.shiftKey,
      gE: x0 => x0.scrollTop,
      gF: x0 => x0.firstChild,
      gG: (x0,x1,x2) => x0.setSelectionRange(x1,x2),
      gH: (x0,x1) => new Intl.Segmenter(x0,x1),
      gI: (x0,x1,x2) => x0.postMessage(x1,x2),
      gJ: x0 => x0.altKey,
      h: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      hB: (b, o, l) => new DataView(b, o, l),
      hC: (string, token) => string.split(token),
      hD: x0 => x0.body,
      hE: x0 => x0.offsetTop,
      hF: x0 => x0.viewConstraints,
      hG: s => {
        if (/[[\]{}()*+?.\\^$|]/.test(s)) {
            s = s.replace(/[[\]{}()*+?.\\^$|]/g, '\\$&');
        }
        return s;
      },
      hH: x0 => x0.Segmenter,
      hI: x0 => x0.port,
      hJ: x0 => x0.ctrlKey,
      i: (wasmFunction,f) => finalizeWrapper(f, function() { return wasmFunction(f,arguments.length) }),
      iB: Function.prototype.call.bind(DataView.prototype.getFloat64),
      iC: (a, i) => a[i],
      iD: x0 => x0.visibilityState,
      iE: x0 => x0.scrollLeft,
      iF: x0 => x0.hostElement,
      iG: x0 => x0.keyCode,
      iH: x0 => x0.buffer,
      iI: (x0,x1) => { x0.onerror = x1 },
      iJ: x0 => x0.isComposing,
      j: x0 => new Promise(x0),
      jB: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Float64Array) return 1;
        return 2;
      },
      jC: a => a.length,
      jD: (x0,x1,x2) => x0.removeEventListener(x1,x2),
      jE: x0 => x0.offsetLeft,
      jF: x0 => ({runApp: x0}),
      jG: (x0,x1) => x0.scrollIntoView(x1),
      jH: x0 => x0.wasmMemory,
      jI: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      jJ: x0 => x0.code,
      k: (wasmFunction,f) => finalizeWrapper(f, function(x0,x1) { return wasmFunction(f,arguments.length,x0,x1) }),
      kB: Function.prototype.call.bind(DataView.prototype.setFloat64),
      kC: o => o instanceof Array,
      kD: x0 => x0.disconnect(),
      kE: x0 => x0.offsetParent,
      kF: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      kG: x0 => x0.multiViewEnabled,
      kH: () => globalThis.window._flutter_skwasmInstance,
      kI: (x0,x1,x2,x3,x4) => x0.frb_pde_ffi_dispatcher_sync(x1,x2,x3,x4),
      kJ: x0 => x0.repeat,
      l: (x0,x1,x2) => x0.call(x1,x2),
      lB: (t, s) => t.set(s),
      lC: x0 => x0.userAgent,
      lD: x0 => new Intl.Locale(x0),
      lE: x0 => x0.offsetY,
      lF: (o, start, length) => new BigInt64Array(o.buffer, o.byteOffset + start, length),
      lG: (x0,x1) => x0.replaceWith(x1),
      lH: () => new TextDecoder(),
      lI: () => globalThis.window,
      lJ: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      m: (constructor, args) => {
        const factoryFunction = constructor.bind.apply(
            constructor, [null, ...args]);
        return new factoryFunction();
      },
      mB: Function.prototype.call.bind(DataView.prototype.setFloat32),
      mC: x0 => x0.navigator,
      mD: x0 => x0.region,
      mE: x0 => x0.offsetX,
      mF: Function.prototype.call.bind(DataView.prototype.getBigInt64),
      mG: (x0,x1) => { x0.className = x1 },
      mH: (map, o, v) => map.set(o, v),
      mI: (x0,x1) => x0.unregister(x1),
      mJ: x0 => x0.userAgent,
      n: x0 => new Array(x0),
      nB: Function.prototype.call.bind(DataView.prototype.getFloat32),
      nC: Function.prototype.call.bind(String.prototype.toLowerCase),
      nD: x0 => x0.script,
      nE: (o, p, r) => o.replace(p, () => r),
      nF: o => o.byteLength,
      nG: (x0,x1) => { x0.tabIndex = x1 },
      nH: (map, o) => map.get(o),
      nI: x0 => new FinalizationRegistry(x0),
      nJ: x0 => x0.navigator,
      o: o => [o],
      oB: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Float32Array) return 1;
        return 2;
      },
      oC: Object.is,
      oD: x0 => x0.language,
      oE: (o, p, r) => o.replaceAll(p, () => r),
      oF: (x0,x1,x2,x3) => x0.pushState(x1,x2,x3),
      oG: (x0,x1) => { x0.action = x1 },
      oH: () => new WeakMap(),
      oI: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      oJ: (x0,x1) => x0.getItem(x1),
      p: (o0, o1) => [o0, o1],
      pB: Function.prototype.call.bind(DataView.prototype.getUint32),
      pC: x0 => x0.vendor,
      pD: x0 => x0.languages,
      pE: x0 => x0.deltaMode,
      pF: x0 => x0.history,
      pG: (x0,x1) => { x0.method = x1 },
      pH: x0 => x0.debugSkipFontRetryDelay,
      pI: () => globalThis.FinalizationRegistry,
      pJ: x0 => x0.localStorage,
      q: (o0, o1, o2) => [o0, o1, o2],
      qB: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Uint32Array) return 1;
        return 2;
      },
      qC: (x0,x1) => x0.createTextNode(x1),
      qD: (x0,x1) => x0.observe(x1),
      qE: x0 => x0.deltaY,
      qF: x0 => x0.search,
      qG: (x0,x1) => { x0.noValidate = x1 },
      qH: (x0,x1,x2) => x0.set(x1,x2),
      qI: (x0,x1,x2,x3) => x0.register(x1,x2,x3),
      qJ: (x0,x1) => x0.key(x1),
      r: (o0, o1, o2, o3) => [o0, o1, o2, o3],
      rB: Function.prototype.call.bind(DataView.prototype.getInt32),
      rC: (x0,x1) => { x0.id = x1 },
      rD: x0 => new ResizeObserver(x0),
      rE: x0 => x0.deltaX,
      rF: x0 => x0.location,
      rG: x0 => x0.click(),
      rH: x0 => x0.fontFallbackBaseUrl,
      rI: (x0,x1) => x0.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerDrillCollection(x1),
      rJ: x0 => x0.length,
      s: (x0,x1,x2) => { x0[x1] = x2 },
      sB: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Int32Array) return 1;
        return 2;
      },
      sC: (x0,x1) => { x0.nonce = x1 },
      sD: (wasmFunction,f) => finalizeWrapper(f, function(x0,x1) { return wasmFunction(f,arguments.length,x0,x1) }),
      sE: x0 => x0.wheelDeltaY,
      sF: x0 => x0.pathname,
      sG: (x0,x1) => x0.getElementsByClassName(x1),
      sH: (handle) => clearInterval(handle),
      sI: () => globalThis.wasm_bindgen,
      sJ: (x0,x1,x2) => x0.setItem(x1,x2),
      t: (o, p) => o[p],
      tB: Function.prototype.call.bind(DataView.prototype.getUint16),
      tC: x0 => x0.nonce,
      tD: (x0,x1) => x0.getPropertyValue(x1),
      tE: x0 => x0.wheelDeltaX,
      tF: (x0,x1,x2,x3) => x0.replaceState(x1,x2,x3),
      tG: (x0,x1) => x0.dispatchEvent(x1),
      tH: (ms, c) =>
      setInterval(() => dartInstance.exports.$invokeCallback(c), ms),
      tI: (x0,x1) => x0.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerDrillCollection(x1),
      tJ: x0 => x0.extraAssets,
      u: () => globalThis,
      uB: o => o instanceof Uint16Array,
      uC: () => globalThis.window.flutterConfiguration,
      uD: x0 => globalThis.parseFloat(x0),
      uE: x0 => x0.bottom,
      uF: o => Object.keys(o),
      uG: (x0,x1) => x0.createEvent(x1),
      uH: () => Date.now(),
      uI: (x0,x1) => x0.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerDiagnostics(x1),
      uJ: x0 => x0.fontManifest,
      v: (x0,x1) => ({addView: x0,removeView: x1}),
      vB: Function.prototype.call.bind(DataView.prototype.getInt16),
      vC: (x0,x1) => x0.attachShadow(x1),
      vD: (x0,x1) => x0.getComputedStyle(x1),
      vE: x0 => x0.right,
      vF: o => {
        const proto = Object.getPrototypeOf(o);
        return proto === Object.prototype || proto === null;
      },
      vG: (x0,x1,x2,x3) => x0.initEvent(x1,x2,x3),
      vH: (x0,x1,x2) => x0.insertBefore(x1,x2),
      vI: (x0,x1) => x0.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerDiagnostics(x1),
      vJ: x0 => x0.assetManifest,
      w: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      wB: o => o instanceof Int16Array,
      wC: (x0,x1) => x0.createElement(x1),
      wD: x0 => x0.documentElement,
      wE: x0 => x0.clientY,
      wF: x0 => x0.state,
      wG: x0 => x0.readText(),
      wH: x0 => x0.id,
      wI: (x0,x1) => x0.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSqlDrill(x1),
      wJ: x0 => x0.buildConfig,
      x: (wasmFunction,f) => finalizeWrapper(f, function(x0) { return wasmFunction(f,arguments.length,x0) }),
      xB: o => o instanceof Uint8ClampedArray,
      xC: x0 => x0.scale,
      xD: (x0,x1) => x0.get(x1),
      xE: x0 => x0.clientX,
      xF: x0 => x0.hash,
      xG: x0 => x0.clipboard,
      xH: x0 => x0.offsetHeight,
      xI: (x0,x1) => x0.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerSqlDrill(x1),
      xJ: x0 => x0.length,
      y: (l, r) => l === r,
      yB: o => {
        if (o === null || o === undefined) return 0;
        if (o instanceof Uint8Array) return 1;
        return 2;
      },
      yC: x0 => x0.visualViewport,
      yD: x0 => x0.computedStyleMap(),
      yE: x0 => x0.changedTouches,
      yF: x0 => x0.state,
      yG: (x0,x1) => x0.writeText(x1),
      yH: x0 => x0.offsetWidth,
      yI: (x0,x1) => x0.rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerJavaDrill(x1),
      yJ: x0 => x0.getReader(),
      z: x0 => x0.random(),
      zB: Function.prototype.call.bind(DataView.prototype.setInt32),
      zC: x0 => x0.devicePixelRatio,
      zD: (o, p) => p in o,
      zE: x0 => x0.key,
      zF: (x0,x1) => x0.go(x1),
      zG: x0 => x0.unlock(),
      zH: x0 => x0.stopPropagation(),
      zI: (x0,x1) => x0.rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerJavaDrill(x1),
      zJ: x0 => x0.value,

    };

    const baseImports = {
      _: dart2wasm,
      Math: Math,
      Date: Date,
      Object: Object,
      Array: Array,
      Reflect: Reflect,
      WebAssembly: {
        JSTag: WebAssembly.JSTag,
      },
      "": new Proxy({}, { get(_, prop) { return prop; } }),

    };

    

    dartInstance = await WebAssembly.instantiate(this.module, {
      ...baseImports,
      ...additionalImports,
      
    });

    return new InstantiatedApp(this, dartInstance);
  }
}

class InstantiatedApp {
  constructor(compiledApp, instantiatedModule) {
    this.compiledApp = compiledApp;
    this.instantiatedModule = instantiatedModule;
  }

  // Call the main function with the given arguments.
  invokeMain(...args) {
    this.instantiatedModule.exports.$invokeMain(args);
  }
}
