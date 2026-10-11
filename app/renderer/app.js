'use strict';
// Odyssey desktop app: the hex globe (globe.js, grid.js) drawing what the Rust engine says.
const $ = (id) => document.getElementById(id);
const host = window.odyssey;

const SIZES = {
  n24: { n: 24, view: 24, label: 'Quick world', sub: '300 km tiles' },
  n48: { n: 48, view: 48, label: 'Standard world', sub: '150 km tiles' },
  n480: { n: 48, view: 480, label: '16 km globe', sub: '16 km tiles, 150 km simulation' }
};

function lerp(a, b, t) { return [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]; }
function makeLut(stops) {
  const lut = new Uint8Array(256 * 3);
  for (let i = 0; i < 256; i++) {
    const t = i / 255, n = stops.length - 1, k = Math.min(n - 1, Math.floor(t * n));
    const c = lerp(stops[k], stops[k + 1], t * n - k);
    lut[i * 3] = c[0]; lut[i * 3 + 1] = c[1]; lut[i * 3 + 2] = c[2];
  }
  return lut;
}
const RAMPS = {
  temp: makeLut([[49, 54, 149], [69, 117, 180], [171, 217, 233], [255, 255, 191], [253, 174, 97], [215, 48, 39], [165, 0, 38]]),
  green: makeLut([[70, 80, 60], [120, 160, 90], [90, 175, 90], [40, 140, 80], [12, 95, 60]]),
  brown: makeLut([[90, 80, 60], [140, 105, 70], [180, 130, 80], [215, 175, 115], [245, 230, 190]]),
  ice: makeLut([[80, 100, 135], [140, 180, 220], [205, 228, 247], [245, 250, 255]]),
  warm: makeLut([[95, 70, 55], [200, 120, 55], [240, 165, 75], [255, 225, 160]]),
  sea: makeLut([[40, 80, 105], [50, 125, 145], [85, 180, 175], [180, 230, 215]]),
  relief: makeLut([[85, 105, 70], [140, 140, 85], [165, 125, 95], [210, 200, 190], [255, 255, 255]])
};
// kind: land (shown on land only), sea (sea only), both
const LAYERS = [
  { name: 'temp', label: 'Temperature', ramp: 'temp', kind: 'both' },
  { name: 'veg', label: 'Plants', ramp: 'green', kind: 'land' },
  { name: 'soil', label: 'Soil carbon', ramp: 'brown', kind: 'land' },
  { name: 'ice', label: 'Land ice', ramp: 'ice', kind: 'land' },
  { name: 'herb', label: 'Plant-eaters', ramp: 'warm', kind: 'land' },
  { name: 'carn', label: 'Meat-eaters', ramp: 'warm', kind: 'land' },
  { name: 'elev', label: 'Land height', ramp: 'relief', kind: 'land' },
  { sep: true },
  { name: 'plank', label: 'Plankton', ramp: 'sea', kind: 'sea' },
  { name: 'fish', label: 'Fish', ramp: 'sea', kind: 'sea' },
  { name: 'seaice', label: 'Sea ice', ramp: 'ice', kind: 'sea' },
  { name: 'depth', label: 'Sea depth', ramp: 'sea', kind: 'sea' }
];

const st = {
  sizeKey: null, simN: 0, viewN: 0, parents: null, sea: null, layerName: 'temp', layer: null, terrain: null,
  running: false, every: 1, pendingRestart: false, ready: null, gridFor: 0, grid: null
};
let globe = null;

function dec(b64) { const s = atob(b64), a = new Uint8Array(s.length); for (let i = 0; i < s.length; i++) a[i] = s.charCodeAt(i); return a; }
function toast(msg, bad) { const t = $('toast'); t.textContent = msg; t.className = 'toast' + (bad ? ' bad' : ''); clearTimeout(toast.h); toast.h = setTimeout(() => t.classList.add('hidden'), bad ? 9000 : 3500); }
function show(id, on) { $(id).classList.toggle('hidden', !on); }

// ---- colours -------------------------------------------------------------------------------------
function recolor() {
  if (!globe || !globe.grid || !st.layer) return;
  const V = globe.grid.V, rgb = new Uint8ClampedArray(V * 3);
  const def = LAYERS.find(l => l.name === st.layer.name) || LAYERS[0];
  const lut = RAMPS[def.ramp], bytes = st.layer.bytes;
  const par = st.parents, sea = st.sea, ter = st.terrain;
  for (let c = 0; c < V; c++) {
    const t = par ? par[c] : c;
    let isSea, elev = 0;
    if (ter) { elev = ter[c]; isSea = elev <= 0; } else isSea = sea[t] === 1;
    let r, g, b;
    const v = bytes[t];
    const useLayer = v > 0 && (def.kind === 'both' || (def.kind === 'land' && !isSea) || (def.kind === 'sea' && isSea));
    if (useLayer) {
      r = lut[v * 3]; g = lut[v * 3 + 1]; b = lut[v * 3 + 2];
      if (isSea && def.kind === 'both') { r *= 0.8; g *= 0.8; b *= 0.85; }
      if (ter && !isSea && def.name !== 'elev') { const sh = 0.82 + 0.26 * Math.min(1, elev / 3500); r *= sh; g *= sh; b *= sh; }
    } else if (isSea) {
      const d = ter ? Math.min(1, -elev / 5500) : 0.35;
      const k = Math.sqrt(d);
      r = 44 - 36 * k; g = 98 - 76 * k; b = 150 - 90 * k;
    } else {
      const e = ter ? Math.min(255, Math.max(0, (elev / 4500) * 255)) : 40;
      const rl = RAMPS.relief;
      r = rl[(e | 0) * 3]; g = rl[(e | 0) * 3 + 1]; b = rl[(e | 0) * 3 + 2];
    }
    rgb[c * 3] = r; rgb[c * 3 + 1] = g; rgb[c * 3 + 2] = b;
  }
  globe.setColors(rgb);
  drawLegend(def);
}
function drawLegend(def) {
  const cv = $('lg-ramp'), cx = cv.getContext('2d'), lut = RAMPS[def.ramp];
  for (let x = 0; x < 200; x++) { const i = Math.round(x / 199 * 255); cx.fillStyle = 'rgb(' + lut[i * 3] + ',' + lut[i * 3 + 1] + ',' + lut[i * 3 + 2] + ')'; cx.fillRect(x, 0, 1, 10); }
  $('lg-title').textContent = st.layer.label; $('lg-lo').textContent = st.layer.min; $('lg-hi').textContent = st.layer.max; $('lg-unit').textContent = st.layer.unit;
}

// ---- engine events -------------------------------------------------------------------------------
function onEngine(m) {
  switch (m.event) {
    case 'ready': st.ready && (st.ready.info = m); showStats(m.stats); break;
    case 'parents': st.parents = new Uint32Array(dec(m.data).buffer); break;
    case 'sea': st.sea = dec(m.data); break;
    case 'layer': st.layer = { name: m.name, label: m.label, unit: m.unit, min: m.min, max: m.max, bytes: dec(m.data) }; recolor(); if (st.ready) { const r = st.ready; st.ready = null; r.resolve(); } break;
    case 'stats': showStats(m.stats); break;
    case 'tile': showTile(m); break;
    case 'done': st.running = false; setRun(); if (st.pendingRestart) { st.pendingRestart = false; run(); } break;
    case 'error': toast(m.message, true); if (st.ready) { st.ready.reject(new Error(m.message)); st.ready = null; } st.running = false; setRun(); break;
    case 'log': console.log(m.message); break;
  }
}
function showStats(s) {
  $('year').textContent = 'Year ' + s.year;
  const rows = [['Mean temperature', s.t_mean.toFixed(2) + ' C'], ['CO2', s.co2.toFixed(1) + ' ppm'], ['Plant carbon', s.veg.toFixed(0) + ' PgC'], ['Soil carbon', s.soil.toFixed(0) + ' PgC'], ['Animal carbon', s.fauna.toFixed(2) + ' PgC'], ['Land ice', s.ice_sle.toFixed(1) + ' m of sea level']];
  if (s.skipped) rows.push(['Skipped modules', s.skipped]);
  $('readout').innerHTML = rows.map(([k, v]) => '<div class="ro-row"><span class="ro-k">' + k + '</span><span class="ro-v">' + v + '</span></div>').join('');
}
function showTile(m) {
  const sel = st.selFine;
  const rows = [['Latitude', m.lat.toFixed(1)], ['Longitude', m.lon.toFixed(1)], ['Simulation tile', Math.round(Math.sqrt(m.area_km2)) + ' km wide']];
  if (st.terrain && sel != null) rows.splice(0, 0, ['Height at this 16 km tile', st.terrain[sel] + ' m']);
  for (const r of m.rows) rows.push([r.label, (Math.abs(r.value) >= 100 ? r.value.toFixed(0) : r.value.toPrecision(3)) + ' ' + r.unit]);
  $('insp-title').textContent = m.sea ? 'Open sea' : 'Land';
  $('insp-body').innerHTML = rows.map(([k, v]) => '<div class="kv"><span>' + k + '</span><span>' + v + '</span></div>').join('');
  show('inspector', true);
}

// ---- world loading -------------------------------------------------------------------------------
async function begin(key) {
  const z = SIZES[key];
  show('screen-title', false); show('screen-loading', true); show('hud', false);
  $('load-title').textContent = z.label; $('load-eyebrow').textContent = z.sub;
  const bar = (p, label) => { $('load-fill').style.width = Math.round(p * 100) + '%'; $('load-label').textContent = label; };
  try {
    if (!globe) { globe = new Globe($('gl')); globe.interactive = true; globe.onSelect = onSelect; }
    if (st.gridFor !== z.view) {
      bar(0.02, 'Laying out ' + (10 * z.view * z.view + 2).toLocaleString() + ' hex tiles');
      const grid = await globe.buildGrid(z.view, 7, (p, label) => bar(0.02 + 0.6 * p, label));
      globe.setGrid(grid); st.gridFor = z.view;
    }
    st.terrain = null;
    if (z.view === 480) { bar(0.65, 'Reading the 16 km land and sea floor'); const buf = await host.readTerrain(); if (!buf) throw new Error('terrain-n480.bin was not found'); st.terrain = new Int16Array(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength)); }
    bar(0.72, 'Loading the spun-up world');
    st.parents = null; st.sea = null; st.layer = null;
    await new Promise((resolve, reject) => { st.ready = { resolve, reject }; host.send({ cmd: 'load', n: z.n, view: z.view }); });
    st.sizeKey = key; st.simN = z.n; st.viewN = z.view;
    $('tb-world').textContent = z.label + ' - ' + z.sub;
    bar(1, 'Ready');
    show('screen-loading', false); show('hud', true); buildModes();
    globe.cam.dist = globe.cam.tDist = 3.3;
  } catch (e) {
    show('screen-loading', false); show('screen-title', true); toast(String(e.message || e), true);
  }
}
function buildModes() {
  const box = $('mapmodes'); box.innerHTML = '';
  for (const l of LAYERS) {
    if (l.sep) { const d = document.createElement('div'); d.className = 'mm-sep'; box.appendChild(d); continue; }
    const b = document.createElement('button'); b.className = 'mm' + (l.name === st.layerName ? ' on' : ''); b.textContent = l.label; b.dataset.name = l.name;
    b.onclick = () => { st.layerName = l.name; for (const x of box.querySelectorAll('.mm')) x.classList.toggle('on', x.dataset.name === l.name); host.send({ cmd: 'layer', name: l.name }); };
    box.appendChild(b);
  }
  const steps = $('steps'); steps.innerHTML = '';
  for (const y of [1, 5, 20, 100]) {
    const s = document.createElement('button'); s.className = 'step' + (y === st.every ? ' on' : ''); s.textContent = y + 'y';
    s.onclick = () => { st.every = y; for (const x of steps.children) x.classList.toggle('on', x === s); if (st.running) { st.pendingRestart = true; host.send({ cmd: 'stop' }); } };
    steps.appendChild(s);
  }
  setRun();
}
function onSelect(id) {
  if (id < 0) { show('inspector', false); return; }
  st.selFine = id;
  host.send({ cmd: 'tile', id: st.parents ? st.parents[id] : id });
}

// ---- running -------------------------------------------------------------------------------------
function run() { st.running = true; setRun(); host.send({ cmd: 'run', years: 1000000, maps_every: st.every }); }
function setRun() { $('btn-play').textContent = st.running ? 'Pause' : 'Run'; $('date-sub').textContent = st.running ? 'Running' : 'Paused'; }
function toggleRun() { if (st.running) host.send({ cmd: 'stop' }); else run(); }

$('btn-play').onclick = toggleRun;
$('insp-close').onclick = () => { show('inspector', false); if (globe) { globe.sel = -1; globe.dirty = true; } };
document.querySelectorAll('#title-menu .menu-item').forEach(b => b.onclick = () => { if (b.dataset.act === 'quit') host.quit(); else begin(b.dataset.size); });
$('p-resume').onclick = () => show('pause', false);
$('p-title').onclick = () => { if (st.running) host.send({ cmd: 'stop' }); show('pause', false); show('hud', false); show('screen-title', true); };
$('p-quit').onclick = () => host.quit();
window.addEventListener('keydown', (e) => {
  if ($('hud').classList.contains('hidden')) return;
  if (e.code === 'Space') { e.preventDefault(); toggleRun(); }
  else if (e.key === 'b' || e.key === 'B') { globe.opts.borders = !globe.opts.borders; globe.dirty = true; }
  else if (e.key === 'g' || e.key === 'G') { globe.opts.graticule = !globe.opts.graticule; globe.dirty = true; }
  else if (e.key === 'Escape') { $('pause').classList.toggle('hidden'); }
});
host.onEvent(onEngine);
