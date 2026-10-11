// WebGL2 globe renderer: hex water planet, stars, atmosphere, camera, picking.

const GLOBE_VS = `#version 300 es
layout(location=0) in vec3 aPos;
layout(location=1) in vec3 aCol;
layout(location=2) in float aEdge;
uniform mat4 uProj; uniform mat4 uRot; uniform vec3 uEye; uniform float uShift;
out vec3 vPos; out float vEdge;
flat out vec3 vCol; flat out int vCell;
void main(){
  vPos = aPos; vEdge = aEdge; vCol = aCol; vCell = gl_VertexID;
  gl_Position = uProj * (uRot * vec4(aPos - uEye, 1.0));
  gl_Position.x += uShift * gl_Position.w;
}`;

const GLOBE_FS = `#version 300 es
precision highp float; precision highp int;
in vec3 vPos; in float vEdge;
flat in vec3 vCol; flat in int vCell;
uniform vec3 uEye; uniform vec3 uLight;
uniform int uHover; uniform int uSel;
uniform float uBorders; uniform float uGrat;
out vec4 o;
const float PI = 3.14159265;
void main(){
  vec3 n = normalize(vPos);
  vec3 v = normalize(uEye - vPos);
  float nl = dot(n, uLight);
  float lam = 0.34 + 0.86 * smoothstep(-0.45, 1.0, nl);
  vec3 h = normalize(uLight + v);
  float wet = smoothstep(0.08, 0.2, vCol.b - vCol.r);
  float spec = pow(max(dot(n, h), 0.0), 90.0) * 0.32 * smoothstep(0.0, 0.3, nl) * wet;
  float fres = pow(1.0 - max(dot(n, v), 0.0), 3.0);
  vec3 c = vCol * lam;
  c += vec3(0.62, 0.7, 0.95) * spec;
  c += vec3(0.16, 0.24, 0.55) * fres * 0.7;

  // Hex borders: vEdge is 1 on the outer rim of each cell.
  float fw = fwidth(vEdge);
  float line = 1.0 - smoothstep(0.0, fw * 1.4, 1.0 - vEdge);
  float vis = 1.0 - smoothstep(0.10, 0.32, fw);
  vec3 edgeCol = vec3(0.16, 0.22, 0.42) * (0.55 + 0.6 * lam);
  c = mix(c, edgeCol, line * vis * uBorders * 0.75);

  // Graticule every 15 degrees.
  if (uGrat > 0.5) {
    float lat = asin(clamp(n.y, -1.0, 1.0)) * 180.0 / PI;
    float lon = atan(-n.z, n.x) * 180.0 / PI;
    float flat_ = fwidth(lat);
    float gl = abs(fract(lat / 15.0 + 0.5) - 0.5) * 15.0;
    float gn = abs(fract(lon / 15.0 + 0.5) - 0.5) * 15.0 * cos(radians(lat));
    float w = flat_ * 1.2;
    float g = max(1.0 - smoothstep(0.0, w, gl), 1.0 - smoothstep(0.0, w, gn));
    float eq = 1.0 - smoothstep(0.0, w * 1.6, abs(lat));
    c = mix(c, vec3(0.55, 0.6, 0.78), max(g * 0.22, eq * 0.35));
  }

  if (vCell == uSel) {
    c = mix(c, vec3(0.85, 0.78, 0.55), 0.10);
    c = mix(c, vec3(1.0, 0.9, 0.62), max(line, 0.0) * 0.95);
  } else if (vCell == uHover) {
    c = mix(c, vec3(0.6, 0.72, 1.0), 0.07);
    c = mix(c, vec3(0.75, 0.82, 1.0), line * 0.45);
  }
  o = vec4(c, 1.0);
}`;

const SKY_VS = `#version 300 es
out vec2 vNdc;
void main(){
  vec2 p = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2) * 2.0 - 1.0;
  vNdc = p; gl_Position = vec4(p, 0.0, 1.0);
}`;

const SKY_FS = `#version 300 es
precision highp float;
in vec2 vNdc;
uniform vec3 uEye, uRight, uUp, uFwd, uLight;
uniform float uTanX, uTanY, uShift, uAtmo;
out vec4 o;
void main(){
  vec2 ndc = vec2(vNdc.x - uShift, vNdc.y);
  vec3 d = normalize(uFwd + ndc.x * uTanX * uRight + ndc.y * uTanY * uUp);
  vec3 bg = mix(vec3(0.010, 0.012, 0.026), vec3(0.030, 0.026, 0.060), smoothstep(-0.6, 0.9, d.y));
  bg += vec3(0.05, 0.04, 0.09) * pow(max(dot(d, normalize(vec3(-0.4, 0.3, 0.8))), 0.0), 6.0) * 0.6;
  float b = dot(uEye, d);
  vec3 col = bg;
  if (b < 0.0 && uAtmo > 0.5) {
    vec3 cp = uEye - d * b;
    float r = length(cp);
    float alt = length(uEye) - 1.0;
    float k = mix(26.0, 60.0, clamp(1.0 / max(alt, 0.05) * 0.05, 0.0, 1.0));
    float g = r > 1.0 ? exp(-(r - 1.0) * k) : 1.0;
    float lit = 0.3 + 0.7 * smoothstep(-0.5, 0.8, dot(normalize(cp), uLight));
    col += vec3(0.28, 0.42, 0.95) * g * lit * 0.6;
  }
  o = vec4(col, 1.0);
}`;

const STAR_VS = `#version 300 es
layout(location=0) in vec3 aDir;
layout(location=1) in vec2 aInfo;
layout(location=2) in vec3 aCol;
uniform mat4 uProj; uniform mat4 uRot; uniform float uDpr; uniform float uShift;
out float vB; out float vT; out vec3 vC;
void main(){
  vec4 p = uProj * vec4(mat3(uRot) * aDir, 1.0);
  p.x += uShift * p.w * 0.35;
  gl_Position = p.xyww;
  gl_PointSize = aInfo.x * uDpr;
  vB = aInfo.y; vT = fract(dot(aDir, vec3(91.7, 47.3, 13.1))); vC = aCol;
}`;

const STAR_FS = `#version 300 es
precision mediump float;
in float vB; in float vT; in vec3 vC;
uniform float uTime;
out vec4 o;
void main(){
  vec2 q = gl_PointCoord * 2.0 - 1.0;
  float r = dot(q, q);
  float a = exp(-r * 4.0);
  float tw = 0.88 + 0.12 * sin(uTime * (0.6 + vT * 1.8) + vT * 40.0);
  o = vec4(vC * a * vB * tw, 1.0);
}`;

// River ribbons: each segment is a quad extruded in screen space.
const RIVER_VS = `#version 300 es
layout(location=0) in vec3 aA;
layout(location=1) in vec3 aB;
layout(location=2) in vec3 aInfo;   // side (-1/1), end (0/1), size (0..10)
uniform mat4 uProj; uniform mat4 uRot; uniform vec3 uEye; uniform float uShift;
uniform vec2 uView; uniform float uWidth;
out float vSide; out float vSize; out vec3 vN;
vec4 P(vec3 p){ vec4 c = uProj * (uRot * vec4(p * 1.0004 - uEye, 1.0)); c.x += uShift * c.w; return c; }
void main(){
  vec4 a = P(aA), b = P(aB);
  vec2 sa = a.xy / a.w * uView * 0.5, sb = b.xy / b.w * uView * 0.5;
  vec2 d = sb - sa; float dl = length(d);
  vec2 dir = dl > 1e-4 ? d / dl : vec2(1.0, 0.0);
  vec2 nrm = vec2(-dir.y, dir.x);
  float w = (0.55 + aInfo.z * 0.2) * uWidth;
  vec4 c = aInfo.y < 0.5 ? a : b;
  vec2 off = nrm * aInfo.x * w + dir * (aInfo.y < 0.5 ? -0.5 : 0.5) * w;
  c.xy += off / (uView * 0.5) * c.w;
  gl_Position = c;
  vSide = aInfo.x; vSize = aInfo.z; vN = aInfo.y < 0.5 ? aA : aB;
}`;

const RIVER_FS = `#version 300 es
precision highp float;
in float vSide; in float vSize; in vec3 vN;
uniform float uMinSize; uniform vec3 uLight;
out vec4 o;
void main(){
  float a = 1.0 - smoothstep(0.55, 1.0, abs(vSide));
  a *= smoothstep(uMinSize - 1.0, uMinSize + 0.5, vSize);
  float lit = 0.6 + 0.55 * smoothstep(-0.35, 1.0, dot(normalize(vN), uLight));
  vec3 col = mix(vec3(0.36, 0.55, 0.84), vec3(0.5, 0.68, 0.95), clamp(vSize / 10.0, 0.0, 1.0));
  o = vec4(col * lit * a, a);
}`;

// Live marks over the globe: storm and tornado tracks as coloured ribbons, storm centres as
// round dots. Each item is a quad extruded in screen space, like the river ribbons.
const MARK_VS = `#version 300 es
layout(location=0) in vec3 aA;
layout(location=1) in vec3 aB;
layout(location=2) in vec4 aInfo;   // side (-1/1), end (0/1), width (px), dot (0/1)
layout(location=3) in vec4 aCol;
uniform mat4 uProj; uniform mat4 uRot; uniform vec3 uEye; uniform float uShift; uniform vec2 uView;
out vec4 vCol; out vec2 vQ; out float vDot;
vec4 P(vec3 p){ vec4 c = uProj * (uRot * vec4(p * 1.0012 - uEye, 1.0)); c.x += uShift * c.w; return c; }
void main(){
  vec4 a = P(aA), b = P(aB);
  vec2 sa = a.xy / a.w * uView * 0.5, sb = b.xy / b.w * uView * 0.5;
  vec2 d = sb - sa; float dl = length(d);
  vec2 dir = dl > 1e-4 ? d / dl : vec2(1.0, 0.0);
  vec2 nrm = vec2(-dir.y, dir.x);
  float w = aInfo.z;
  vec4 c = aInfo.y < 0.5 ? a : b;
  vec2 off = nrm * aInfo.x * w * 0.5 + dir * (aInfo.y < 0.5 ? -0.5 : 0.5) * w;
  c.xy += off / (uView * 0.5) * c.w;
  gl_Position = c;
  vCol = aCol; vQ = vec2(aInfo.x, aInfo.y * 2.0 - 1.0); vDot = aInfo.w;
}`;
const MARK_FS = `#version 300 es
precision highp float;
in vec4 vCol; in vec2 vQ; in float vDot;
out vec4 o;
void main(){
  float r = vDot > 0.5 ? length(vQ) : abs(vQ.x);
  float a = 1.0 - smoothstep(0.6, 1.0, r);
  if (vDot > 0.5) a = max((1.0 - smoothstep(0.45, 0.55, r)) * 0.45, 1.0 - smoothstep(0.08, 0.16, abs(r - 0.75)));
  o = vec4(vCol.rgb * vCol.a * a, vCol.a * a);
}`;

// Sea channels (straits between land tiles): flat water-coloured quads.
const CHAN_VS = `#version 300 es
layout(location=0) in vec3 aPos;
uniform mat4 uProj; uniform mat4 uRot; uniform vec3 uEye; uniform float uShift;
out vec3 vN;
void main(){
  vN = aPos;
  gl_Position = uProj * (uRot * vec4(aPos * 1.00015 - uEye, 1.0));
  gl_Position.x += uShift * gl_Position.w;
}`;

const CHAN_FS = `#version 300 es
precision highp float;
in vec3 vN;
uniform vec3 uColor; uniform vec3 uLight;
out vec4 o;
void main(){
  float lam = 0.34 + 0.86 * smoothstep(-0.45, 1.0, dot(normalize(vN), uLight));
  o = vec4(uColor * lam, 1.0);
}`;

function compile(gl, vs, fs) {
  const mk = (type, src) => {
    const s = gl.createShader(type);
    gl.shaderSource(s, src); gl.compileShader(s);
    if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s) + '\n' + src);
    return s;
  };
  const p = gl.createProgram();
  gl.attachShader(p, mk(gl.VERTEX_SHADER, vs));
  gl.attachShader(p, mk(gl.FRAGMENT_SHADER, fs));
  gl.linkProgram(p);
  if (!gl.getProgramParameter(p, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(p));
  const u = {};
  const count = gl.getProgramParameter(p, gl.ACTIVE_UNIFORMS);
  for (let i = 0; i < count; i++) {
    const info = gl.getActiveUniform(p, i);
    u[info.name] = gl.getUniformLocation(p, info.name);
  }
  return { p, u };
}

function perspective(fovy, aspect, near, far) {
  const f = 1 / Math.tan(fovy / 2), nf = 1 / (near - far);
  return new Float32Array([f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0]);
}

let gridWorker = null;
let gridJob = 0;
function getGridWorker() {
  if (!gridWorker) {
    const src = '(' + gridWorkerMain.toString() + ')()';
    gridWorker = new Worker(URL.createObjectURL(new Blob([src], { type: 'text/javascript' })));
  }
  return gridWorker;
}

class Globe {
  constructor(canvas) {
    this.canvas = canvas;
    const gl = canvas.getContext('webgl2', { antialias: true, powerPreference: 'high-performance', preserveDrawingBuffer: false });
    if (!gl) throw new Error('WebGL2 is not available on this machine.');
    this.gl = gl;
    this.initGL();
    this.chanColor = [38 / 255, 84 / 255, 122 / 255];
    // The GPU can drop the context (a driver reset, or memory running short on the 16 km map). Keep
    // going without drawing, and when it comes back rebuild everything the GPU held.
    this.lost = false;
    canvas.addEventListener('webglcontextlost', (e) => { e.preventDefault(); this.lost = true; });
    canvas.addEventListener('webglcontextrestored', () => this.restoreGL());

    this.grid = null;
    this.vao = null;
    this.hover = -1;
    this.sel = -1;
    this.opts = { borders: true, graticule: false, stars: true, atmosphere: true, rivers: true, sensitivity: 1 };

    // Camera: yaw = longitude, pitch = latitude, dist from center (radius 1).
    this.cam = { yaw: 0.3, pitch: 0.35, dist: 2.8, tDist: 2.8, vYaw: 0, vPitch: 0 };
    this.minDist = 1.004; this.maxDist = 6;
    this.shift = 0; this.tShift = 0;
    this.autoSpin = 0;
    this.fov = 38 * Math.PI / 180;
    this.dirty = true;
    this.time = 0;
    this.onHover = null;
    this.onSelect = null;
    this.interactive = false;

    this.bindInput();
    this.resize();
    window.addEventListener('resize', () => { this.resize(); this.dirty = true; });
    this.last = performance.now();
    requestAnimationFrame((t) => this.frame(t));
  }

  // Programs, stars and empty overlays: everything that lives on the GPU apart from the grid.
  initGL() {
    const gl = this.gl;
    this.pv = gl.getExtension('WEBGL_provoking_vertex');
    if (this.pv) this.pv.provokingVertexWEBGL(this.pv.FIRST_VERTEX_CONVENTION_WEBGL);
    this.centerFirst = !!this.pv;
    this.progGlobe = compile(gl, GLOBE_VS, GLOBE_FS);
    this.progSky = compile(gl, SKY_VS, SKY_FS);
    this.progStar = compile(gl, STAR_VS, STAR_FS);
    this.progRiver = compile(gl, RIVER_VS, RIVER_FS);
    this.progChan = compile(gl, CHAN_VS, CHAN_FS);
    this.progMark = compile(gl, MARK_VS, MARK_FS);
    this.markVao = null; this.markCount = 0; this.markBufs = []; this.markCap = 0;
    this.riverVao = null; this.riverCount = 0; this.riverBufs = [];
    this.chanVao = null; this.chanCount = 0; this.chanBufs = [];
    this.emptyVao = gl.createVertexArray();
    this.starVao = null; this.starBuf = null;   // (the old ones died with the lost context)
    this.buildStars(this.starBytes);
  }

  // The context is back: new programs and buffers, then the grid again (its mesh is not kept on
  // the CPU, so it is rebuilt in the worker: about 2 s on the 16 km map), then the app repaints (onRestored).
  restoreGL() {
    this.initGL();
    this.vao = null; this.buffers = []; this.colBuffer = null; this.indexCount = 0;
    this.lost = false; this.dirty = true;
    const args = this.gridArgs, sel = this.sel;
    if (!args) return;
    this.buildGrid(args.n, args.seed, null, true).then((g) => {
      // (another grid was set meanwhile: that one stands)
      if (this.lost || !this.gridArgs || this.gridArgs.n !== args.n || this.gridArgs.seed !== args.seed || this.vao) return;
      this.setGrid(g); this.sel = sel;
      this.onRestored && this.onRestored();
    }).catch((e) => console.error('rebuilding the grid after a lost context', e));
  }

  // The night sky: our own stars (Yale Bright Star Catalogue) and the Milky Way where it really lies, baked by
  // tools/bake_stars.py (8 floats a point: direction, size, brightness, colour). Until the file is read (or if it
  // is missing) the sky is empty.
  buildStars(bytes) {
    const gl = this.gl;
    let data = new Float32Array(0), N = 0;
    if (bytes && bytes.byteLength > 4) {
      N = new DataView(bytes.buffer, bytes.byteOffset, 4).getUint32(0, true);
      data = new Float32Array(bytes.buffer.slice(bytes.byteOffset + 4, bytes.byteOffset + 4 + N * 32));
    }
    this.starBytes = bytes || null;
    if (this.starVao) gl.deleteVertexArray(this.starVao);
    if (this.starBuf) gl.deleteBuffer(this.starBuf);
    this.starVao = gl.createVertexArray();
    gl.bindVertexArray(this.starVao);
    const b = gl.createBuffer(); this.starBuf = b;
    gl.bindBuffer(gl.ARRAY_BUFFER, b);
    gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 32, 0);
    gl.enableVertexAttribArray(1); gl.vertexAttribPointer(1, 2, gl.FLOAT, false, 32, 12);
    gl.enableVertexAttribArray(2); gl.vertexAttribPointer(2, 3, gl.FLOAT, false, 32, 20);
    gl.bindVertexArray(null);
    this.starCount = N;
    this.dirty = true;
  }

  // Build a grid in the worker. Returns a promise of the grid.
  buildGrid(n, seed, onProgress, plain) {
    const w = getGridWorker();
    const id = ++gridJob;
    return new Promise((resolve, reject) => {
      const handler = (e) => {
        const m = e.data;
        if (m.id !== id) return;
        if (m.type === 'progress') onProgress && onProgress(m.p, m.label);
        else if (m.type === 'done') { w.removeEventListener('message', handler); m.grid.seed = seed; resolve(m.grid); }
        else if (m.type === 'error') { w.removeEventListener('message', handler); reject(new Error(m.message)); }
      };
      w.addEventListener('message', handler);
      w.postMessage({ id, n, seed, centerFirst: this.centerFirst, plain: !!plain });
    });
  }

  setGrid(grid) {
    const gl = this.gl;
    if (this.vao) {
      gl.deleteVertexArray(this.vao);
      for (const b of this.buffers) gl.deleteBuffer(b);
    }
    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    const mk = (target, data) => { const b = gl.createBuffer(); gl.bindBuffer(target, b); gl.bufferData(target, data, gl.STATIC_DRAW); return b; };
    const bPos = mk(gl.ARRAY_BUFFER, grid.mpos);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0);
    const bCol = mk(gl.ARRAY_BUFFER, grid.col);
    gl.enableVertexAttribArray(1); gl.vertexAttribPointer(1, 3, gl.UNSIGNED_BYTE, true, 0, 0);
    const bEdge = mk(gl.ARRAY_BUFFER, grid.edge);
    gl.enableVertexAttribArray(2); gl.vertexAttribPointer(2, 1, gl.UNSIGNED_BYTE, true, 0, 0);
    const bIdx = mk(gl.ELEMENT_ARRAY_BUFFER, grid.idx);
    gl.bindVertexArray(null);
    this.vao = vao;
    this.buffers = [bPos, bCol, bEdge, bIdx];
    this.colBuffer = bCol;
    this.indexCount = grid.idx.length;
    // Keep only what picking and the sim need. Drop big mesh arrays.
    this.gridArgs = { n: grid.n, seed: grid.seed };
    this.grid = { n: grid.n, V: grid.V, centers: grid.centers, binStart: grid.binStart, binCells: grid.binCells, neighbors: grid.neighbors, BIN_LAT: grid.BIN_LAT, BIN_LON: grid.BIN_LON };
    this.hover = -1; this.sel = -1;
    this.overlayFor = null; this.riverCount = 0; this.chanCount = 0;
    this.dirty = true;
  }

  // Replace per-tile colours (Uint8Array of V*3). Only tile centers carry colour.
  setColors(rgb) {
    const gl = this.gl;
    // (a colour pack for another grid, arriving after a world was closed or a new one built, is dropped)
    if (!this.grid || !rgb || rgb.length !== this.grid.V * 3) return;
    gl.bindBuffer(gl.ARRAY_BUFFER, this.colBuffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, rgb);
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
    this.dirty = true;
  }

  // Rivers: list of polylines [{p:[x,y,z], s:size}], smoothed and turned into ribbons.
  setRiverPaths(paths) {
    const gl = this.gl;
    for (const b of this.riverBufs) gl.deleteBuffer(b);
    if (this.riverVao) gl.deleteVertexArray(this.riverVao);
    const smooth = (pts) => {
      let cur = pts;
      for (let it = 0; it < 2; it++) {
        const nxt = [cur[0]];
        for (let k = 0; k < cur.length - 1; k++) {
          const a = cur[k], b = cur[k + 1];
          const q = (t) => ({ p: [a.p[0] + (b.p[0] - a.p[0]) * t, a.p[1] + (b.p[1] - a.p[1]) * t, a.p[2] + (b.p[2] - a.p[2]) * t], s: a.s + (b.s - a.s) * t });
          nxt.push(q(0.25), q(0.75));
        }
        nxt.push(cur[cur.length - 1]);
        cur = nxt;
      }
      for (const v of cur) { const l = Math.hypot(...v.p); v.p = v.p.map(x => x / l); }
      return cur;
    };
    let segs = 0;
    const sm = paths.map(p => { const q = smooth(p); segs += q.length - 1; return q; });
    const A = new Float32Array(segs * 4 * 3), B = new Float32Array(segs * 4 * 3), I = new Float32Array(segs * 4 * 3);
    const idx = new Uint32Array(segs * 6);
    let v = 0, e = 0;
    for (const pts of sm) {
      for (let k = 0; k < pts.length - 1; k++) {
        const a = pts[k], b = pts[k + 1];
        const corners = [[-1, 0, a.s], [1, 0, a.s], [-1, 1, b.s], [1, 1, b.s]];
        for (let c = 0; c < 4; c++) {
          A.set(a.p, (v + c) * 3); B.set(b.p, (v + c) * 3); I.set(corners[c], (v + c) * 3);
        }
        idx.set([v, v + 1, v + 2, v + 1, v + 3, v + 2], e);
        v += 4; e += 6;
      }
    }
    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    const mk = (data, loc) => { const b = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, b); gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW); gl.enableVertexAttribArray(loc); gl.vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0); return b; };
    const bufs = [mk(A, 0), mk(B, 1), mk(I, 2)];
    const ib = gl.createBuffer(); gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ib); gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, idx, gl.STATIC_DRAW);
    bufs.push(ib);
    gl.bindVertexArray(null);
    this.riverVao = vao; this.riverCount = e; this.riverBufs = bufs;
    this.dirty = true;
  }

  // Live marks: [{a:[x,y,z], b:[x,y,z], w: px, c:[r,g,b,a], dot}] (a dot has a == b).
  // (v0.43: the realm borders share the layer: setRealm keeps them under the storm marks)
  setMarks(items) { this.liveMarks = items; this.uploadMarks(this.realmMarks && this.realmMarks.length ? this.realmMarks.concat(items) : items); }
  setRealm(items) { this.realmMarks = items; this.uploadMarks(items.concat(this.liveMarks || [])); }
  uploadMarks(items) {
    const gl = this.gl;
    const n = items.length;
    this.markCount = n * 6;
    this.dirty = true;
    if (!n) return;
    // (the buffers are kept and refilled: the layer changes several times a second while storms move)
    if (!this.markVao) {
      this.markVao = gl.createVertexArray(); gl.bindVertexArray(this.markVao);
      const mk = (loc, size) => { const b = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, b); gl.enableVertexAttribArray(loc); gl.vertexAttribPointer(loc, size, gl.FLOAT, false, 0, 0); return b; };
      this.markBufs = [mk(0, 3), mk(1, 3), mk(2, 4), mk(3, 4)];
      this.markIdx = gl.createBuffer(); gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.markIdx);
      gl.bindVertexArray(null);
      this.markCap = 0;
    }
    if (n > this.markCap) {
      this.markCap = Math.max(n, this.markCap * 2, 256);
      const c = this.markCap;
      this.markA = new Float32Array(c * 12); this.markB = new Float32Array(c * 12); this.markI = new Float32Array(c * 16); this.markC = new Float32Array(c * 16);
      const idx = new Uint32Array(c * 6);
      for (let k = 0; k < c; k++) { const v = k * 4; idx.set([v, v + 1, v + 2, v + 1, v + 3, v + 2], k * 6); }
      gl.bindVertexArray(this.markVao);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.markIdx); gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, idx, gl.STATIC_DRAW);
      gl.bindVertexArray(null);
    }
    const A = this.markA, B = this.markB, I = this.markI, C = this.markC, dpr = this.dpr;
    for (let k = 0; k < n; k++) {
      const it = items[k], w = it.w * dpr, dt = it.dot ? 1 : 0;
      for (let q = 0; q < 4; q++) {
        const v3 = (k * 4 + q) * 3, v4 = (k * 4 + q) * 4;
        A[v3] = it.a[0]; A[v3 + 1] = it.a[1]; A[v3 + 2] = it.a[2];
        B[v3] = it.b[0]; B[v3 + 1] = it.b[1]; B[v3 + 2] = it.b[2];
        I[v4] = q & 1 ? 1 : -1; I[v4 + 1] = q >> 1; I[v4 + 2] = w; I[v4 + 3] = dt;
        C[v4] = it.c[0]; C[v4 + 1] = it.c[1]; C[v4 + 2] = it.c[2]; C[v4 + 3] = it.c[3];
      }
    }
    const up = (b, data, len) => { gl.bindBuffer(gl.ARRAY_BUFFER, b); gl.bufferData(gl.ARRAY_BUFFER, data.subarray(0, len), gl.DYNAMIC_DRAW); };
    up(this.markBufs[0], A, n * 12); up(this.markBufs[1], B, n * 12); up(this.markBufs[2], I, n * 16); up(this.markBufs[3], C, n * 16);
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
  }

  // Strait channel quads: [[a,b,c,d], ...] as unit vectors.
  setChannels(quads) {
    const gl = this.gl;
    for (const b of this.chanBufs) gl.deleteBuffer(b);
    if (this.chanVao) gl.deleteVertexArray(this.chanVao);
    const pos = new Float32Array(quads.length * 6 * 3);
    let o = 0;
    for (const [a, b, c, d] of quads) for (const p of [a, b, c, a, c, d]) { pos.set(p, o); o += 3; }
    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    const b = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, b); gl.bufferData(gl.ARRAY_BUFFER, pos, gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0);
    gl.bindVertexArray(null);
    this.chanVao = vao; this.chanCount = quads.length * 6; this.chanBufs = [b];
    this.dirty = true;
  }

  resize() {
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    this.dpr = dpr;
    const w = Math.floor(this.canvas.clientWidth * dpr), h = Math.floor(this.canvas.clientHeight * dpr);
    if (this.canvas.width !== w || this.canvas.height !== h) { this.canvas.width = w; this.canvas.height = h; }
  }

  basis() {
    const c = this.cam;
    const cp = Math.cos(c.pitch), sp = Math.sin(c.pitch), cy = Math.cos(c.yaw), sy = Math.sin(c.yaw);
    const dir = [cp * cy, sp, -cp * sy];
    const eye = [dir[0] * c.dist, dir[1] * c.dist, dir[2] * c.dist];
    const fwd = [-dir[0], -dir[1], -dir[2]];
    // right = fwd x worldUp
    let right = [fwd[1] * 0 - fwd[2] * 1, fwd[2] * 0 - fwd[0] * 0, fwd[0] * 1 - fwd[1] * 0];
    let rl = Math.hypot(...right) || 1; right = right.map(x => x / rl);
    const up = [right[1] * fwd[2] - right[2] * fwd[1], right[2] * fwd[0] - right[0] * fwd[2], right[0] * fwd[1] - right[1] * fwd[0]];
    return { eye, fwd, right, up };
  }

  bindInput() {
    const cv = this.canvas;
    let dragging = false, moved = 0, lx = 0, ly = 0, lastT = 0;
    this.mouse = { x: -1, y: -1, inside: false };
    cv.addEventListener('pointerdown', (e) => {
      if (!this.interactive || e.button !== 0 && e.button !== 2) return;
      dragging = true; moved = 0; lx = e.clientX; ly = e.clientY; lastT = performance.now();
      this.cam.vYaw = 0; this.cam.vPitch = 0;
      cv.setPointerCapture(e.pointerId);
      cv.classList.add('grabbing');
    });
    cv.addEventListener('pointermove', (e) => {
      this.mouse.x = e.clientX; this.mouse.y = e.clientY; this.mouse.inside = true;
      if (dragging) {
        const dx = e.clientX - lx, dy = e.clientY - ly;
        lx = e.clientX; ly = e.clientY;
        moved += Math.abs(dx) + Math.abs(dy);
        const k = this.radPerPixel() * this.opts.sensitivity;
        this.cam.yaw -= dx * k; this.cam.pitch += dy * k;
        this.clampPitch();
        const now = performance.now(), dt = Math.max(1, now - lastT); lastT = now;
        this.cam.vYaw = -dx * k / dt * 16; this.cam.vPitch = dy * k / dt * 16;
        this.dirty = true;
      }
      this.pickPending = true;
    });
    const end = (e) => {
      if (!dragging) return;
      dragging = false;
      cv.classList.remove('grabbing');
      if (performance.now() - lastT > 80) { this.cam.vYaw = 0; this.cam.vPitch = 0; }
      if (moved < 5 && e.button === 0) {
        const id = this.pick(e.clientX, e.clientY);
        this.sel = id; this.dirty = true;
        this.onSelect && this.onSelect(id);
      }
    };
    cv.addEventListener('pointerup', end);
    cv.addEventListener('pointercancel', end);
    cv.addEventListener('pointerleave', () => { this.mouse.inside = false; this.pickPending = true; });
    cv.addEventListener('contextmenu', (e) => e.preventDefault());
    cv.addEventListener('wheel', (e) => {
      if (!this.interactive) return;
      e.preventDefault();
      const d = e.deltaMode === 1 ? e.deltaY * 33 : e.deltaY;
      this.zoomBy(Math.exp(d * 0.0016));
    }, { passive: false });
  }

  zoomBy(f) {
    const alt = (this.cam.tDist - 1) * f;
    this.cam.tDist = Math.min(this.maxDist, Math.max(this.minDist, 1 + alt));
    this.dirty = true;
  }

  radPerPixel() {
    const alt = Math.max(this.cam.dist - 1, 0.0005);
    return (2 * Math.tan(this.fov / 2) * alt) / Math.max(1, this.canvas.clientHeight) * 1.05;
  }

  clampPitch() { this.cam.pitch = Math.max(-1.52, Math.min(1.52, this.cam.pitch)); }

  // Returns cell id under a screen point, or -1.
  pick(sx, sy) {
    if (!this.grid) return -1;
    const r = this.canvas.getBoundingClientRect();
    const nx = ((sx - r.left) / r.width) * 2 - 1 - this.shift;
    const ny = 1 - ((sy - r.top) / r.height) * 2;
    const { eye, fwd, right, up } = this.basis();
    const ty = Math.tan(this.fov / 2), tx = ty * (r.width / r.height);
    let d = [0, 1, 2].map(i => fwd[i] + nx * tx * right[i] + ny * ty * up[i]);
    const dl = Math.hypot(...d); d = d.map(x => x / dl);
    const b = eye[0] * d[0] + eye[1] * d[1] + eye[2] * d[2];
    const c = eye[0] ** 2 + eye[1] ** 2 + eye[2] ** 2 - 1;
    const disc = b * b - c;
    if (disc < 0) return -1;
    const t = -b - Math.sqrt(disc);
    if (t < 0) return -1;
    return this.nearestCell(eye[0] + d[0] * t, eye[1] + d[1] * t, eye[2] + d[2] * t);
  }

  nearestCell(x, y, z) {
    const g = this.grid;
    const L = g.BIN_LAT, M = g.BIN_LON, P = g.centers;
    const lat = Math.asin(Math.max(-1, Math.min(1, y)));
    const lon = Math.atan2(-z, x);
    const bi = Math.min(L - 1, Math.floor((lat + Math.PI / 2) / Math.PI * L));
    const bj = Math.min(M - 1, Math.floor((lon + Math.PI) / (2 * Math.PI) * M));
    const binDeg = Math.PI / L;
    const cellRad = 1.3 * Math.sqrt(4 * Math.PI / g.V);
    const rows = Math.ceil(cellRad / binDeg) + 1;
    let best = -1, bd = -2;
    for (let i = Math.max(0, bi - rows); i <= Math.min(L - 1, bi + rows); i++) {
      const rowLat = Math.abs((i + 0.5) / L * Math.PI - Math.PI / 2) + binDeg;
      const cosl = Math.cos(Math.min(Math.PI / 2, rowLat));
      const span = cosl < 1e-3 ? M : Math.min(M, Math.ceil(cellRad / (2 * Math.PI / M * Math.max(cosl, 1e-3))) + 1);
      for (let jj = -span; jj <= span; jj++) {
        if (span >= M / 2 && (jj < -M / 2 || jj >= M / 2)) continue;
        const j = ((bj + jj) % M + M) % M;
        const b = i * M + j;
        for (let k = g.binStart[b]; k < g.binStart[b + 1]; k++) {
          const c = g.binCells[k];
          const dd = P[c * 3] * x + P[c * 3 + 1] * y + P[c * 3 + 2] * z;
          if (dd > bd) { bd = dd; best = c; }
        }
      }
    }
    return best;
  }

  cellLatLon(id) {
    const P = this.grid.centers;
    const x = P[id * 3], y = P[id * 3 + 1], z = P[id * 3 + 2];
    return { lat: Math.asin(y) * 180 / Math.PI, lon: Math.atan2(-z, x) * 180 / Math.PI };
  }

  focusCell(id) {
    const { lat, lon } = this.cellLatLon(id);
    this.flyTo = { yaw: lon * Math.PI / 180, pitch: lat * Math.PI / 180 };
    this.dirty = true;
  }

  frame(t) {
    const dt = Math.min(0.1, (t - this.last) / 1000);
    this.last = t;
    this.time += dt;
    const c = this.cam;
    let moving = false;

    if (Math.abs(c.tDist - c.dist) > 1e-7) {
      const a = 1 - Math.exp(-dt * 10);
      // interpolate in log-altitude space for a smooth feel
      const la = Math.log(c.dist - 1), lb = Math.log(c.tDist - 1);
      c.dist = 1 + Math.exp(la + (lb - la) * a);
      moving = true;
    }
    if (Math.abs(c.vYaw) + Math.abs(c.vPitch) > 1e-7) {
      c.yaw += c.vYaw; c.pitch += c.vPitch; this.clampPitch();
      const damp = Math.exp(-dt * 4.5);
      c.vYaw *= damp; c.vPitch *= damp;
      if (Math.abs(c.vYaw) + Math.abs(c.vPitch) < 1e-7) { c.vYaw = 0; c.vPitch = 0; }
      moving = true;
    }
    if (this.flyTo) {
      const a = 1 - Math.exp(-dt * 6);
      let dy = this.flyTo.yaw - c.yaw;
      dy = Math.atan2(Math.sin(dy), Math.cos(dy));
      c.yaw += dy * a; c.pitch += (this.flyTo.pitch - c.pitch) * a;
      if (Math.abs(dy) < 1e-5 && Math.abs(this.flyTo.pitch - c.pitch) < 1e-5) this.flyTo = null;
      moving = true;
    }
    if (this.autoSpin) { c.yaw += this.autoSpin * dt; moving = true; }
    if (Math.abs(this.tShift - this.shift) > 1e-4) {
      this.shift += (this.tShift - this.shift) * (1 - Math.exp(-dt * 5));
      moving = true;
    } else this.shift = this.tShift;

    if (this.pickPending && this.grid && this.interactive) {
      this.pickPending = false;
      const id = this.mouse.inside ? this.pick(this.mouse.x, this.mouse.y) : -1;
      if (id !== this.hover) { this.hover = id; this.dirty = true; this.onHover && this.onHover(id); }
    } else if (moving && this.interactive && this.mouse.inside) this.pickPending = true;

    // Stars twinkle only while something else redraws; an idle globe costs nothing.
    if ((moving || this.dirty) && !this.lost) { this.render(); this.dirty = false; }
    // (the next frame is asked for first: an error in the per-frame work must not stop the globe and the clock for good)
    requestAnimationFrame((tt) => this.frame(tt));
    try { this.onFrame && this.onFrame(dt); } catch (err) { console.error(err); }
  }

  render() {
    const gl = this.gl;
    this.resize();
    const W = this.canvas.width, H = this.canvas.height;
    gl.viewport(0, 0, W, H);
    const { eye, fwd, right, up } = this.basis();
    const alt = this.cam.dist - 1;
    const near = Math.max(alt * 0.3, 1e-5), far = this.cam.dist + 3;
    const proj = perspective(this.fov, W / H, near, far);
    const rot = new Float32Array([right[0], up[0], -fwd[0], 0, right[1], up[1], -fwd[1], 0, right[2], up[2], -fwd[2], 0, 0, 0, 0, 1]);
    // Light: moonlight from upper left of the view, so the globe always reads.
    let L = [0, 1, 2].map(i => -fwd[i] * 0.55 - right[i] * 0.55 + up[i] * 0.62);
    const ll = Math.hypot(...L); L = L.map(x => x / ll);

    gl.disable(gl.DEPTH_TEST); gl.disable(gl.BLEND);
    // Sky + atmosphere
    gl.useProgram(this.progSky.p);
    const su = this.progSky.u;
    gl.uniform3fv(su.uEye, eye); gl.uniform3fv(su.uRight, right); gl.uniform3fv(su.uUp, up); gl.uniform3fv(su.uFwd, fwd);
    gl.uniform3fv(su.uLight, L);
    const ty = Math.tan(this.fov / 2);
    gl.uniform1f(su.uTanY, ty); gl.uniform1f(su.uTanX, ty * W / H);
    gl.uniform1f(su.uShift, this.shift); gl.uniform1f(su.uAtmo, this.opts.atmosphere ? 1 : 0);
    gl.bindVertexArray(this.emptyVao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);

    // Stars
    if (this.opts.stars) {
      gl.enable(gl.BLEND); gl.blendFunc(gl.ONE, gl.ONE);
      gl.useProgram(this.progStar.p);
      const u = this.progStar.u;
      gl.uniformMatrix4fv(u.uProj, false, perspective(this.fov, W / H, 0.1, 10));
      gl.uniformMatrix4fv(u.uRot, false, rot);
      gl.uniform1f(u.uDpr, this.dpr); gl.uniform1f(u.uTime, this.time); gl.uniform1f(u.uShift, this.shift);
      gl.bindVertexArray(this.starVao);
      gl.drawArrays(gl.POINTS, 0, this.starCount);
      gl.disable(gl.BLEND);
    }

    // Globe
    if (this.vao) {
      gl.enable(gl.DEPTH_TEST); gl.depthFunc(gl.LEQUAL);
      gl.enable(gl.CULL_FACE); gl.cullFace(gl.BACK); gl.frontFace(gl.CCW);
      gl.clear(gl.DEPTH_BUFFER_BIT);
      gl.useProgram(this.progGlobe.p);
      const u = this.progGlobe.u;
      gl.uniformMatrix4fv(u.uProj, false, proj);
      gl.uniformMatrix4fv(u.uRot, false, rot);
      gl.uniform3fv(u.uEye, eye); gl.uniform3fv(u.uLight, L);
      gl.uniform1f(u.uShift, this.shift);
      gl.uniform1i(u.uHover, this.hover); gl.uniform1i(u.uSel, this.sel);
      gl.uniform1f(u.uBorders, this.opts.borders ? 1 : 0);
      gl.uniform1f(u.uGrat, this.opts.graticule ? 1 : 0);
      gl.bindVertexArray(this.vao);
      gl.drawElements(gl.TRIANGLES, this.indexCount, gl.UNSIGNED_INT, 0);
      gl.disable(gl.CULL_FACE);

      if (this.chanVao && this.chanCount) {
        gl.useProgram(this.progChan.p);
        const cu = this.progChan.u;
        gl.uniformMatrix4fv(cu.uProj, false, proj);
        gl.uniformMatrix4fv(cu.uRot, false, rot);
        gl.uniform3fv(cu.uEye, eye); gl.uniform3fv(cu.uLight, L);
        gl.uniform1f(cu.uShift, this.shift); gl.uniform3fv(cu.uColor, this.chanColor);
        gl.bindVertexArray(this.chanVao);
        gl.drawArrays(gl.TRIANGLES, 0, this.chanCount);
      }

      if (this.riverVao && this.opts.rivers && this.showRivers) {
        const altKm = alt * 6371;
        const minSize = altKm > 9000 ? 7 : altKm > 4500 ? 5 : altKm > 2000 ? 3.5 : altKm > 800 ? 2 : 0;
        const widthScale = Math.max(0.55, Math.min(1.7, 2400 / altKm)) * this.dpr;
        gl.enable(gl.BLEND); gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
        gl.depthMask(false);
        gl.useProgram(this.progRiver.p);
        const ru = this.progRiver.u;
        gl.uniformMatrix4fv(ru.uProj, false, proj);
        gl.uniformMatrix4fv(ru.uRot, false, rot);
        gl.uniform3fv(ru.uEye, eye); gl.uniform3fv(ru.uLight, L);
        gl.uniform1f(ru.uShift, this.shift);
        gl.uniform2f(ru.uView, W, H);
        gl.uniform1f(ru.uWidth, widthScale);
        gl.uniform1f(ru.uMinSize, minSize);
        gl.bindVertexArray(this.riverVao);
        gl.drawElements(gl.TRIANGLES, this.riverCount, gl.UNSIGNED_INT, 0);
        gl.depthMask(true); gl.disable(gl.BLEND);
      }

      if (this.markVao && this.markCount) {
        gl.enable(gl.BLEND); gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
        gl.depthMask(false);
        gl.useProgram(this.progMark.p);
        const mu = this.progMark.u;
        gl.uniformMatrix4fv(mu.uProj, false, proj);
        gl.uniformMatrix4fv(mu.uRot, false, rot);
        gl.uniform3fv(mu.uEye, eye); gl.uniform1f(mu.uShift, this.shift);
        gl.uniform2f(mu.uView, W, H);
        gl.bindVertexArray(this.markVao);
        gl.drawElements(gl.TRIANGLES, this.markCount, gl.UNSIGNED_INT, 0);
        gl.depthMask(true); gl.disable(gl.BLEND);
      }
      gl.disable(gl.DEPTH_TEST);
    }
    gl.bindVertexArray(null);
  }

  // Closest zoom shows roughly eight tiles top to bottom.
  setTileKm(km) {
    this.tileKm = km;
    this.minDist = 1 + (8 * km) / 6371;
    this.cam.tDist = Math.max(this.cam.tDist, this.minDist);
  }

  altitudeKm() { return (this.cam.dist - 1) * 6371; }
}
