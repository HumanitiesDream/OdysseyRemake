// App shell: screens, menus, HUD, settings and saves. The hex globe (globe.js, grid.js) draws what the Rust engine says.
'use strict';

const $ = (s, r = document) => r.querySelector(s);
const el = (tag, attrs = {}, ...kids) => {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (k === 'class') e.className = v;
    else if (k === 'html') e.innerHTML = v;
    else if (k.startsWith('on')) e.addEventListener(k.slice(2), v);
    else if (v !== false && v != null) e.setAttribute(k, v === true ? '' : v);
  }
  for (const k of kids.flat()) if (k != null) e.append(k.nodeType ? k : document.createTextNode(k));
  return e;
};

// ---------- Host (Electron preload, or a stand-in so the page opens in a browser) ----------
const host = window.owHost || {
  isDesktop: false, send() {}, onEvent() {}, readData: async () => null, listSaves: async () => [], readSave: async () => null,
  saveExists: async () => false, writeSave: async () => true, deleteSave: async () => true, openSavesFolder: async () => {},
  getSettings: async () => null, setSettings: async () => true, toggleFullscreen: async () => {}, setFullscreen: async () => {}, quit: async () => {}
};

// ---------- Constants ----------
const SAVE_VERSION = 1;
const APP_VERSION = '0.50.0';
const EARTH_AREA_KM2 = 510.1e6;
const TITLE_N = 160;
// map sizes: the simulation grid (n) and the grid the globe draws (view)
const SIZES = {
  n24: { key: 'n24', n: 24, view: 24, km: 300, label: 'Quick', sub: '300 km tiles. About five years a second at full speed.' },
  n48: { key: 'n48', n: 48, view: 48, km: 150, label: 'Standard', sub: '150 km tiles. About a year a second at full speed.' },
  n480: { key: 'n480', n: 48, view: 480, km: 16, label: 'Full detail', sub: '16 km coasts and mountains drawn over the 150 km simulation. Slow to load.' }
};
const MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
// the five speeds, in months a second (0: as fast as the engine goes)
const SPEEDS = [1, 3, 12, 60, 0];
const SPEED_TEXT = ['1 month a second', '3 months a second', '1 year a second', '5 years a second', 'As fast as it goes'];

const DEFAULT_SETTINGS = { borders: true, graticule: false, atmosphere: true, sensitivity: 1, uiScale: 1, autosaveMin: 10, fullscreen: false, testMode: false, relief: true };

const ICONS = {
  menu: '<svg viewBox="0 0 16 16"><path d="M2 4h12M2 8h12M2 12h12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>',
  history: '<svg viewBox="0 0 16 16"><path d="M2 13h12" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M2.5 10l3-3 2.5 2 5-5.5" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  terrain: '<svg viewBox="0 0 18 18"><path d="M1.5 14.5l5-7 3 4 2-2.5 5 5.5z" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/><circle cx="13" cy="4.5" r="1.6" fill="none" stroke="currentColor" stroke-width="1.2"/></svg>',
  elevation: '<svg viewBox="0 0 18 18"><path d="M2 14.5c2-1.2 4-3.6 5.5-7 1 2.2 2 3.2 3.2 3.6 1-1.4 1.8-2 2.6-2 .8 1.4 1.6 3.6 2.7 5.4" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" stroke-linecap="round"/><path d="M3 11.5h12M4.5 8.5h9" stroke="currentColor" stroke-width=".9" stroke-dasharray="1.2 1.8" opacity=".8"/></svg>',
  soil: '<svg viewBox="0 0 18 18"><path d="M2 7h14M2 11h14M2 15h14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="6" cy="9" r=".8" fill="currentColor"/><circle cx="11" cy="13" r=".8" fill="currentColor"/><path d="M9 7V3m0 0c-1.5 0-2.5-1-2.5-2M9 3c1.5 0 2.5-1 2.5-2" fill="none" stroke="currentColor" stroke-width="1.1"/></svg>',
  ice: '<svg viewBox="0 0 16 16"><path d="M2 13.5l3.5-8 2.5 4 2-3 4 7z" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/><path d="M4.4 8h2.2M9.2 9h2" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/></svg>',
  temperature: '<svg viewBox="0 0 18 18"><path d="M7 3.2a2 2 0 014 0v7.3a3.4 3.4 0 11-4 0z" fill="none" stroke="currentColor" stroke-width="1.2"/><path d="M9 7v5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>',
  rainfall: '<svg viewBox="0 0 18 18"><path d="M5 10.5a3.3 3.3 0 01-.2-6.6 4 4 0 017.6 1.1 2.6 2.6 0 01-.2 5.5z" fill="none" stroke="currentColor" stroke-width="1.2"/><path d="M6 12.5l-1 2.5M9.2 12.5l-1 2.5M12.4 12.5l-1 2.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>',
  snow: '<svg viewBox="0 0 18 18"><path d="M9 1.5v15M2.5 5.2l13 7.6M2.5 12.8l13-7.6M7 2.8L9 4.6l2-1.8M7 15.2l2-1.8 2 1.8M3 7.7l2.6.6-.8 2.5M15 10.3l-2.6-.6.8-2.5M3 10.3l2.6-.6-.8-2.5M15 7.7l-2.6.6.8 2.5" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  vegetation: '<svg viewBox="0 0 18 18"><path d="M3 15c0-5 3-8 8-9-1 5-4 8-8 9zM3 15l5-5M11 15c0-3 1.5-5 4-6-.5 3-2 5-4 6z" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" stroke-linecap="round"/></svg>',
  animals: '<svg viewBox="0 0 18 18"><path d="M3 14.5v-3.2c0-2.8 2.3-4.8 5.3-4.8h2.4c1.6 0 2.8-.8 3.3-2l1.5.7c-.2 1.3-.9 2.3-1.9 2.9v6.4M5.5 14.5V11M11.5 14.5V11" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/><path d="M14.3 7.3c-1 1.4-.6 3 .8 3.7" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/></svg>',
  predators: '<svg viewBox="0 0 18 18"><path d="M3 14c.5-3 2-5 4-5.5L6 4.5l2.5 2.3L11 4.5l-.5 4c2 .6 3.5 2.6 4 5.5" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" stroke-linecap="round"/><path d="M7.3 11.3h.01M10.7 11.3h.01" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/><path d="M8 13.5l1 .8 1-.8" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/></svg>',
  fire: '<svg viewBox="0 0 18 18"><path d="M9 16c-3 0-5-2-5-4.8 0-2.6 2-4 2.6-6.7C8 6 8.6 7.2 8.8 8.6 9.8 7.3 10.3 5 10 2.5c2.8 2 4 5 4 8.4C14 14 12 16 9 16z" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/></svg>',
  sealife: '<svg viewBox="0 0 18 18"><path d="M1.5 10.5c2.2-3.2 7-4 10.2-1.8 1 .7 1.6 1.6 1.8 2.8M13.5 11.5l1.5-3 1.5.8-.6 2 1 1.8-2 .2zM1.5 10.5c1.5 1.8 5.5 2.6 9 1.2" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/><circle cx="5" cy="9.6" r=".6" fill="currentColor"/><path d="M1.5 15c2-1 4-1 6 0s4 1 6 0" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" opacity=".7"/></svg>',
  plankton: '<svg viewBox="0 0 18 18"><circle cx="5" cy="6" r="2" fill="none" stroke="currentColor" stroke-width="1.2"/><circle cx="12" cy="5" r="1.4" fill="none" stroke="currentColor" stroke-width="1.2"/><circle cx="9" cy="11" r="2.4" fill="none" stroke="currentColor" stroke-width="1.2"/><path d="M1.5 15.5c2-1 4-1 6 0s4 1 6 0 2.5-.6 3 0" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" opacity=".7"/></svg>',
  seaice: '<svg viewBox="0 0 18 18"><path d="M3 9.5l2.5-3h7l2.5 3-2 2H5z" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/><path d="M1.5 14c2-1 4-1 6 0s4 1 6 0 2.5-.6 3 0" fill="none" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/></svg>',
  depth: '<svg viewBox="0 0 18 18"><path d="M1.5 4.5c2-1 4-1 6 0s4 1 6 0 2.5-.6 3 0" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M9 7.5v7M6.5 12l2.5 2.5 2.5-2.5" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  trash: '<svg viewBox="0 0 16 16" width="14" height="14"><path d="M3 4.5h10M6.5 4.5V3h3v1.5M4.5 4.5l.6 8.5h5.8l.6-8.5" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/></svg>'
};

// ---------- Colour ramps ----------
function makeLut(stops) {
  const lut = new Uint8Array(256 * 3), n = stops.length - 1;
  for (let i = 0; i < 256; i++) {
    const t = i / 255, k = Math.min(n - 1, Math.floor(t * n)), f = t * n - k;
    for (let j = 0; j < 3; j++) lut[i * 3 + j] = Math.round(stops[k][j] + (stops[k + 1][j] - stops[k][j]) * f);
  }
  return lut;
}
const RAMP_STOPS = {
  temp: [[38, 44, 120], [52, 92, 176], [104, 150, 206], [158, 158, 152], [214, 152, 92], [206, 84, 56], [132, 30, 42]],
  rain: [[150, 32, 38], [204, 104, 74], [146, 146, 142], [98, 146, 206], [58, 108, 196]],
  green: [[66, 62, 50], [120, 130, 76], [70, 140, 70], [36, 120, 64], [18, 82, 46]],
  soil: [[52, 44, 36], [110, 80, 52], [148, 100, 58], [196, 134, 72], [236, 164, 84]],
  ice: [[66, 62, 56], [120, 124, 132], [200, 208, 222], [246, 250, 255]],
  herb: [[40, 38, 34], [80, 74, 60], [170, 150, 100], [255, 240, 200]],
  carn: [[40, 36, 36], [90, 60, 56], [180, 90, 70], [250, 170, 130]],
  fire: [[64, 92, 70], [226, 124, 52], [196, 40, 36]],
  relief: [[70, 104, 64], [128, 140, 82], [170, 140, 96], [150, 120, 100], [220, 214, 208], [255, 255, 255]],
  sea: [[14, 34, 66], [32, 84, 126], [60, 150, 160], [170, 230, 210]],
  depth: [[90, 150, 200], [40, 90, 150], [18, 46, 96], [6, 16, 44]]
};
const RAMPS = Object.fromEntries(Object.entries(RAMP_STOPS).map(([k, v]) => [k, makeLut(v)]));
const rampCss = (k) => `linear-gradient(90deg, ${RAMP_STOPS[k].map((c, i, a) => `rgb(${c.join(',')}) ${Math.round(i / (a.length - 1) * 100)}%`).join(', ')})`;

// The map modes. layer: the engine layer; kind: land (land only), sea (sea only), both; group: rail section.
const MAP_MODES = [
  { id: 'terrain', layer: 'look', name: 'Terrain', group: 'Land', color: '#a9c27c', hint: 'plants, snow and ice as they are now',
    note: 'Green where plants grow thick, tan where they are thin. White is snow, land ice and sea ice.' },
  { id: 'elevation', layer: 'elev', name: 'Elevation', group: 'Land', kind: 'land', ramp: 'relief', color: '#d2a86e', ticks: ['0', '1,000', '2,500', '5,000 m'] },
  { id: 'soil', layer: 'soil', name: 'Soil carbon', group: 'Land', kind: 'land', ramp: 'soil', color: '#c98f5c', ticks: ['0', '10', '20', '30 kg/m²'], note: 'Carbon in litter and soil. Peat and cold soils hold the most.' },
  { id: 'ice', layer: 'ice', name: 'Land ice', group: 'Land', kind: 'land', ramp: 'ice', color: '#e6eef8', ticks: ['0', '1,000', '2,000', '3,000 m'], note: 'Thickness of glaciers and ice sheets.' },
  { id: 'temperature', layer: 'temp', name: 'Temperature this month', group: 'Weather', kind: 'both', ramp: 'temp', color: '#ee8a5a', ticks: ['-40', '-15', '10', '35 °C'] },
  { id: 'rainfall', layer: 'rain', name: 'Rain vs normal', group: 'Weather', kind: 'land', ramp: 'rain', color: '#6cb2f0', ticks: ['40%', '100%', '160%'], note: 'This year\'s weather against the usual rain for the place. Red is drought.' },
  { id: 'snow', layer: 'snow', name: 'Snow on the ground', group: 'Weather', kind: 'land', ramp: 'ice', color: '#dfe8f4', ticks: ['None', '100', '200', '400 mm'], note: 'Snow as water. A metre of fresh snow holds about 100 mm.' },
  { id: 'vegetation', layer: 'veg', name: 'Plants', group: 'Life', kind: 'land', ramp: 'green', color: '#78bf82', ticks: ['0', '5', '10', '15 kg/m²'], note: 'Carbon in leaves, wood and roots. Rainforest is about 15.' },
  { id: 'animals', layer: 'herb', name: 'Plant-eaters', group: 'Life', kind: 'land', ramp: 'herb', color: '#d9b77a', ticks: ['0', '1', '2 g/m²'], note: 'Carbon in grazers, browsers and small plant-eaters.' },
  { id: 'predators', layer: 'carn', name: 'Meat-eaters', group: 'Life', kind: 'land', ramp: 'carn', color: '#e07a5f', ticks: ['0', '0.1', '0.2 g/m²'] },
  { id: 'fire', layer: 'fire', name: 'Fires burning', group: 'Life', kind: 'land', ramp: 'fire', color: '#f0a050', ticks: ['None', 'Burning'], note: 'Fronts still burning at the end of the month. Lightning starts them.' },
  { id: 'plankton', layer: 'plank', name: 'Plankton', group: 'Sea', kind: 'sea', ramp: 'sea', color: '#5fd0c0', ticks: ['0', '2.5', '5 g/m²'], note: 'Rich where cold water rises with its nutrients.' },
  { id: 'sealife', layer: 'fish', name: 'Fish', group: 'Sea', kind: 'sea', ramp: 'sea', color: '#7fb0e0', ticks: ['0', '1.5', '3 g/m²'] },
  { id: 'seaice', layer: 'seaice', name: 'Sea ice', group: 'Sea', kind: 'sea', ramp: 'ice', color: '#cfe0f0', ticks: ['None', '1', '2 m'] },
  { id: 'depth', layer: 'depth', name: 'Sea depth', group: 'Sea', kind: 'sea', ramp: 'depth', color: '#4a7ac0', ticks: ['0', '2,000', '4,000', '6,000 m'] }
];
const MODE_GROUPS = ['Land', 'Weather', 'Life', 'Sea'];
const modeInfo = (id) => MAP_MODES.find(m => m.id === id) || MAP_MODES[0];

// ---------- State ----------
let settings = { ...DEFAULT_SETTINGS };
let globe;
const state = {
  screen: 'title',
  world: null,        // { name, seedText, seed, size, n, view, months, months0, speed, paused, createdAt, playSec, file }
  dirty: false, lastAutosave: 0, busy: false,
  mapMode: 'terrain', selected: -1, inspTab: 'land',
  parents: null, sea: null, layer: null, terrain: null, shade: null,
  stats: null, hist: [], tile: null, running: false, perf: null, gen: 0
};

// ---------- Utilities ----------
function hashSeed(str) {
  let h = 2166136261 >>> 0;
  for (const ch of String(str)) { h ^= ch.codePointAt(0); h = Math.imul(h, 16777619) >>> 0; }
  return h >>> 0;
}
function slug(name) {
  const s = String(name).toLowerCase().normalize('NFKD').replace(/[^\w\s-]/g, '').trim().replace(/[\s_]+/g, '-').replace(/-+/g, '-').slice(0, 60).replace(/^-+/, '').replace(/-+$/, '');
  return s || 'world';
}
function fmtInt(n) { return Math.round(n).toLocaleString('en-US'); }
function fmtAgo(ts) {
  if (!ts) return '';
  const s = (Date.now() - ts) / 1000;
  if (s < 60) return 'just now';
  if (s < 3600) return `${Math.floor(s / 60)} min ago`;
  if (s < 86400) return `${Math.floor(s / 3600)} h ago`;
  return new Date(ts).toLocaleDateString(undefined, { day: 'numeric', month: 'short', year: 'numeric' });
}
function fmtKm(km) {
  if (km >= 1000) return `${(km / 1000).toFixed(km >= 10000 ? 0 : 1)}k km`;
  if (km >= 10) return `${Math.round(km)} km`;
  return `${km.toFixed(1)} km`;
}
// story calendar: Year 1 is the year the story began
function yearOf(w, months) { return Math.floor((months - w.months0) / 12) + 1; }
function fmtWhen(w, months = w.months) { return `${MONTHS[((months % 12) + 12) % 12]}, Year ${fmtInt(yearOf(w, months))}`; }
function dec(b64) { const s = atob(b64), a = new Uint8Array(s.length); for (let i = 0; i < s.length; i++) a[i] = s.charCodeAt(i); return a; }

function toast(msg, err = false) {
  const t = el('div', { class: 'toast' + (err ? ' err' : '') }, el('span', { class: 'dot' }), msg);
  $('#toasts').append(t);
  setTimeout(() => { t.classList.add('out'); setTimeout(() => t.remove(), 400); }, err ? 6000 : 2400);
}

// ---------- Modals ----------
const modalStack = [];
function openModal({ eyebrow, title, body, buttons = [], wide = false, onClose, dismissable = true }) {
  const scrim = el('div', { class: 'scrim' });
  const foot = buttons.length ? el('div', { class: 'modal-foot' }) : null;
  const m = el('div', { class: 'modal' + (wide ? ' wide' : '') },
    el('div', { class: 'modal-head' }, eyebrow ? el('div', { class: 'eyebrow' }, eyebrow) : null, el('h2', { class: 'modal-title' }, title)),
    el('div', { class: 'modal-body' }, body),
    foot);
  const handle = {
    close: () => {
      const i = modalStack.indexOf(handle);
      if (i >= 0) modalStack.splice(i, 1);
      scrim.remove();
      if (document.activeElement && m.contains(document.activeElement)) document.activeElement.blur();
      onClose && onClose();
    },
    dismissable, el: m
  };
  for (const b of buttons) foot.append(el('button', { class: 'btn ' + (b.kind || ''), onclick: () => b.onClick ? b.onClick(handle) : handle.close() }, b.label));
  scrim.addEventListener('pointerdown', (e) => { if (e.target === scrim && dismissable) handle.close(); });
  scrim.append(m);
  $('#modal-root').append(scrim);
  modalStack.push(handle);
  const first = m.querySelector('input, .btn.primary');
  if (first) setTimeout(() => first.focus(), 30);
  return handle;
}
function confirmModal({ title, text, confirm = 'Confirm', danger = false, eyebrow }) {
  return new Promise((res) => {
    openModal({
      eyebrow, title,
      body: el('p', { style: 'margin:0;color:var(--muted);line-height:1.6' }, text),
      buttons: [
        { label: 'Cancel', onClick: (h) => { res(false); h.close(); } },
        { label: confirm, kind: danger ? 'danger' : 'primary', onClick: (h) => { res(true); h.close(); } }
      ],
      onClose: () => res(false)
    });
  });
}
function closeTopModal() {
  const top = modalStack[modalStack.length - 1];
  if (top && top.dismissable) { top.close(); return true; }
  return !!top;
}
function closeAllModals() { while (modalStack.length) modalStack[modalStack.length - 1].close(); }

// ---------- Settings ----------
async function loadSettings() {
  try { const s = await host.getSettings(); if (s) settings = { ...DEFAULT_SETTINGS, ...s }; } catch { }
  applySettings();
}
function applySettings() {
  document.documentElement.style.setProperty('--ui-scale', settings.uiScale);
  if (globe) { Object.assign(globe.opts, { borders: settings.borders, graticule: settings.graticule, atmosphere: settings.atmosphere, stars: false, sensitivity: settings.sensitivity }); globe.dirty = true; }
  if (state.world) { renderClock(); recolor(); }
}
function saveSettings() { host.setSettings(settings).catch(() => {}); }

function openSettings() {
  const body = el('div');
  const toggle = (key, label, sub, after) => {
    const t = el('button', { class: 'toggle' + (settings[key] ? ' on' : ''), onclick: () => { settings[key] = !settings[key]; t.classList.toggle('on', settings[key]); applySettings(); saveSettings(); after && after(); } });
    return el('div', { class: 'setting' }, el('div', {}, el('div', { class: 's-label' }, label), sub ? el('div', { class: 's-sub' }, sub) : null), t);
  };
  const seg = (key, label, opts, sub, after) => {
    const wrap = el('div', { class: 'seg' });
    for (const [v, text] of opts) {
      const b = el('button', { class: settings[key] === v ? 'on' : '', onclick: () => { settings[key] = v; [...wrap.children].forEach(c => c.classList.remove('on')); b.classList.add('on'); applySettings(); saveSettings(); after && after(v); } }, text);
      wrap.append(b);
    }
    return el('div', { class: 'setting' }, el('div', {}, el('div', { class: 's-label' }, label), sub ? el('div', { class: 's-sub' }, sub) : null), wrap);
  };
  const range = (key, label, min, max, step) => {
    const r = el('input', { type: 'range', min, max, step, value: settings[key] });
    r.addEventListener('input', () => { settings[key] = +r.value; applySettings(); });
    r.addEventListener('change', saveSettings);
    return el('div', { class: 'setting' }, el('div', { class: 's-label' }, label), r);
  };
  body.append(
    el('div', { class: 'set-group' }, 'Map'),
    toggle('borders', 'Tile borders', 'Fade in as you zoom close. Shortcut B.'),
    toggle('graticule', 'Latitude and longitude grid', 'Lines every 15°. Shortcut G.'),
    toggle('relief', 'Hill shading', 'Light from the north-west on the land and sea floor.', () => { state.shade = null; recolor(); }),
    toggle('atmosphere', 'Atmosphere glow'),
    el('div', { class: 'set-group' }, 'Controls'),
    range('sensitivity', 'Drag sensitivity', 0.4, 2, 0.05),
    el('div', { class: 'set-group' }, 'Display'),
    seg('uiScale', 'Interface scale', [[0.9, '90%'], [1, '100%'], [1.1, '110%'], [1.25, '125%']]),
    seg('fullscreen', 'Window', [[false, 'Windowed'], [true, 'Fullscreen']], 'Shortcut F11.', (v) => host.setFullscreen(v)),
    el('div', { class: 'set-group' }, 'Testing'),
    toggle('testMode', 'Test mode', 'Shows how many months the engine runs each second, and module errors.'),
    el('div', { class: 'set-group' }, 'Saving'),
    seg('autosaveMin', 'Autosave', [[0, 'Off'], [5, '5 min'], [10, '10 min'], [30, '30 min']], 'Keeps one autosave for each story.'),
    el('div', { class: 'setting' }, el('div', {}, el('div', { class: 's-label' }, 'Save folder'), el('div', { class: 's-sub' }, 'Open the folder where save files live.')),
      el('button', { class: 'btn', onclick: () => host.openSavesFolder() }, 'Open'))
  );
  openModal({ eyebrow: 'Preferences', title: 'Settings', body, buttons: [{ label: 'Done', kind: 'primary' }] });
}

// ---------- Screens ----------
function showScreen(name) {
  state.screen = name;
  if (document.activeElement && document.activeElement !== document.body) document.activeElement.blur();
  $('#screen-title').classList.toggle('hidden', name !== 'title');
  $('#screen-loading').classList.toggle('hidden', name !== 'loading');
  $('#hud').classList.toggle('hidden', name !== 'world');
  globe.interactive = name === 'world';
  $('#tb-world').textContent = name === 'world' && state.world ? state.world.name : '';
  renderLegend();
}

// ---------- Terrain and colours ----------
const terrainCache = new Map();
async function loadTerrain(n) {
  if (terrainCache.has(n)) return terrainCache.get(n);
  const buf = await host.readData(`terrain-n${n}.bin`);
  const t = buf ? new Int16Array(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength)) : null;
  if (n <= TITLE_N) terrainCache.set(n, t);
  return t;
}
// Hill shading: for each cell, the neighbour that lies most to the north-west, and the light from it.
function buildShade(grid, ter, km) {
  const V = grid.V, P = grid.centers, nb = grid.neighbors, out = new Float32Array(V);
  const scale = 1 / (km * 30);
  for (let c = 0; c < V; c++) {
    const x = P[c * 3], y = P[c * 3 + 1], z = P[c * 3 + 2];
    const r = Math.hypot(x, z) || 1, ex = -(-z / r), ez = -(x / r); // east = (-sin lon, 0, -cos lon) with lon = atan2(-z, x)
    let best = -1, bs = -1e9;
    for (let j = 0; j < 6; j++) {
      const q = nb[c * 6 + j]; if (q < 0) continue;
      const dx = P[q * 3] - x, dy = P[q * 3 + 1] - y, dz = P[q * 3 + 2] - z;
      const s = dy - (dx * ex + dz * ez);
      if (s > bs) { bs = s; best = q; }
    }
    const a = Math.max(0, ter[c]), b = best >= 0 ? Math.max(0, ter[best]) : a;
    out[c] = Math.max(-0.4, Math.min(0.4, (a - b) * scale));
  }
  return out;
}
const LAND_DRY = [200, 180, 134], LAND_MID = [150, 150, 92], LAND_WET = [46, 92, 48], WHITE = [240, 244, 250], ROCK = [138, 128, 118];
const TMP = new Float32Array(3);
function seaColor(depth) { const k = Math.sqrt(Math.min(1, depth / 5500)); return [40 - 30 * k, 92 - 70 * k, 140 - 86 * k]; }
// writes the natural colour of a tile into TMP (no allocation: this runs for every one of 2.3 million tiles)
function terrainColor(elev, green, white, isSea) {
  let r, g, b;
  if (isSea) { const k = Math.sqrt(Math.min(1, Math.max(0, -elev) / 5500)); r = 40 - 30 * k; g = 92 - 70 * k; b = 140 - 86 * k; }
  else {
    const f = green / 15;
    if (f < 0.5) { const t = f * 2; r = LAND_DRY[0] + (LAND_MID[0] - LAND_DRY[0]) * t; g = LAND_DRY[1] + (LAND_MID[1] - LAND_DRY[1]) * t; b = LAND_DRY[2] + (LAND_MID[2] - LAND_DRY[2]) * t; }
    else { const t = (f - 0.5) * 2; r = LAND_MID[0] + (LAND_WET[0] - LAND_MID[0]) * t; g = LAND_MID[1] + (LAND_WET[1] - LAND_MID[1]) * t; b = LAND_MID[2] + (LAND_WET[2] - LAND_MID[2]) * t; }
    const rock = Math.max(0, Math.min(1, (elev - 2500) / 2500)) * (1 - f * 0.6);
    if (rock > 0) { r += (ROCK[0] - r) * rock; g += (ROCK[1] - g) * rock; b += (ROCK[2] - b) * rock; }
  }
  const w = white / 14;
  if (w > 0) { r += (WHITE[0] - r) * w; g += (WHITE[1] - g) * w; b += (WHITE[2] - b) * w; }
  TMP[0] = r; TMP[1] = g; TMP[2] = b;
  return TMP;
}

function recolor() {
  if (!globe || !globe.grid) return;
  const V = globe.grid.V, rgb = new Uint8ClampedArray(V * 3);
  const ter = state.terrain && state.terrain.length === V ? state.terrain : null;
  if (settings.relief && ter && !state.shade) state.shade = buildShade(globe.grid, ter, state.world ? SIZES[state.world.size].km : 50);
  const shade = settings.relief && ter ? state.shade : null;
  // the title globe: today's Earth with no engine data
  if (!state.world) {
    const lk = state.titleLook && state.titleLook.length === V ? state.titleLook : null;
    for (let c = 0; c < V; c++) {
      const e = ter ? ter[c] : 0, k = lk && lk[c] ? lk[c] - 1 : 0, sea = lk ? lk[c] === 1 && e <= 0 : e <= 0;
      const col = terrainColor(e, sea ? 0 : lk ? k & 15 : 8, sea ? 0 : k >> 4, sea);
      const s = shade ? 1 + shade[c] : 1;
      rgb[c * 3] = col[0] * s; rgb[c * 3 + 1] = col[1] * s; rgb[c * 3 + 2] = col[2] * s;
    }
    globe.setColors(rgb); return;
  }
  if (!state.layer || !state.sea) return;
  const mi = modeInfo(state.mapMode);
  if (state.layer.name !== mi.layer) return;
  const bytes = state.layer.bytes, par = state.parents, simSea = state.sea, fine = state.world.view === 480;
  const lut = mi.ramp ? RAMPS[mi.ramp] : null, look = mi.layer === 'look';
  const landOnly = mi.kind === 'land', both = mi.kind === 'both';
  for (let c = 0; c < V; c++) {
    const t = par ? par[c] : c;
    const e = ter ? ter[c] : 0;
    const isSea = fine && ter ? e <= 0 : simSea[t] === 1;
    const v = bytes[t];
    let r, g, b, s = 1;
    if (look) {
      const k = v > 0 ? v - 1 : 0;
      const col = terrainColor(e, isSea ? 0 : k & 15, Math.min(14, k >> 4), isSea);
      r = col[0]; g = col[1]; b = col[2];
      if (shade) s = 1 + shade[c];
    } else if (v > 0 && (both || landOnly === !isSea)) {
      r = lut[v * 3]; g = lut[v * 3 + 1]; b = lut[v * 3 + 2];
      if (isSea && both) { r *= 0.85; g *= 0.85; b *= 0.85; }
      if (shade && !isSea) s = 1 + shade[c] * 0.6;
    } else if (isSea) {
      const k = Math.sqrt(Math.min(1, Math.max(0, -e) / 5500));
      r = (40 - 30 * k) * 0.75; g = (92 - 70 * k) * 0.75; b = (140 - 86 * k) * 0.75;
    } else { r = 92; g = 88; b = 80; if (shade) s = 1 + shade[c] * 0.6; }
    rgb[c * 3] = r * s; rgb[c * 3 + 1] = g * s; rgb[c * 3 + 2] = b * s;
  }
  globe.setColors(rgb);
}

function renderLegend() {
  const box = $('#legend');
  if (!box) return;
  box.innerHTML = '';
  if (!box.dataset.fold) { box.dataset.fold = '1'; box.addEventListener('click', (ev) => { if (!ev.target.closest('.lg-title')) return; state.legendShut = !state.legendShut; box.classList.toggle('shut', state.legendShut); }); }
  box.classList.toggle('shut', !!state.legendShut);
  if (state.screen !== 'world') { box.classList.add('hidden'); return; }
  const mi = modeInfo(state.mapMode);
  box.append(el('div', { class: 'lg-title' }, mi.name));
  if (mi.ramp) {
    box.append(el('div', { class: 'lg-bar', style: `background:${rampCss(mi.ramp)}` }), el('div', { class: 'lg-ticks' }, ...(mi.ticks || []).map(t => el('span', {}, t))));
  } else {
    const sw = (c) => el('span', { class: 'lg-sw', style: `background: rgb(${c.map(Math.round).join(',')})` });
    box.append(el('div', { class: 'lg-row' }, sw(LAND_WET), 'Forest'), el('div', { class: 'lg-row' }, sw(LAND_MID), 'Grass and scrub'),
      el('div', { class: 'lg-row' }, sw(LAND_DRY), 'Desert'), el('div', { class: 'lg-row' }, sw(WHITE), 'Snow and ice'), el('div', { class: 'lg-row' }, sw(seaColor(3000)), 'Sea'));
  }
  if (mi.note) box.append(el('div', { class: 'lg-note' }, mi.note));
  box.classList.remove('hidden');
}

// ---------- Map modes rail ----------
function mmTip(btn, m) {
  let t = document.getElementById('mm-tip');
  if (!t) { t = document.createElement('div'); t.id = 'mm-tip'; document.body.append(t); }
  const r = btn.getBoundingClientRect();
  t.innerHTML = '';
  t.append(m.name); if (m.hint) t.append(el('em', {}, m.hint));
  t.style.left = (r.right + 10) + 'px'; t.style.top = (r.top + r.height / 2 - 14) + 'px'; t.style.display = 'block';
}
function mmTipHide() { const t = document.getElementById('mm-tip'); if (t) t.style.display = 'none'; }
function renderMapModes() {
  const box = $('#mapmodes');
  box.innerHTML = '';
  MODE_GROUPS.forEach((g, gi) => {
    if (gi) box.append(el('div', { class: 'mm-sep' }));
    box.append(el('div', { class: 'mm-cap' }, g));
    for (const m of MAP_MODES.filter(x => x.group === g)) {
      const b = el('button', { class: 'mm' + (m.id === state.mapMode ? ' on' : ''), style: `--c:${m.color}`, html: ICONS[m.id] || '', onclick: () => setMapMode(m.id) });
      b.addEventListener('pointerenter', () => mmTip(b, m));
      b.addEventListener('pointerleave', mmTipHide);
      box.append(b);
    }
  });
}
function setMapMode(id) {
  state.mapMode = id;
  renderMapModes(); renderLegend();
  host.send({ cmd: 'layer', name: modeInfo(id).layer });
}

// ---------- Title ----------
let titleGrid = null;
async function enterTitle() {
  if (state.running) host.send({ cmd: 'stop' });
  state.world = null; state.selected = -1; state.gen++; state.running = false;
  state.layer = null; state.parents = null; state.sea = null; state.tile = null;
  $('#inspector').classList.add('hidden');
  showScreen('title');
  globe.tShift = 0.42;
  globe.autoSpin = 0.045;
  globe.cam.tDist = 2.75; globe.cam.pitch = 0.32;
  globe.sel = -1; globe.hover = -1;
  refreshContinue();
  try {
    if (!titleGrid) titleGrid = await globe.buildGrid(TITLE_N, 7);
    if (state.screen !== 'title') return;
    state.terrain = await loadTerrain(TITLE_N); state.shade = null;
    if (!state.titleLook) { const b = await host.readData(`look-n${TITLE_N}.bin`); state.titleLook = b ? new Uint8Array(b.buffer.slice(b.byteOffset, b.byteOffset + b.byteLength)) : null; }
    if (state.screen !== 'title') return;
    if (!globe.grid || globe.grid.n !== TITLE_N) globe.setGrid(titleGrid);
    recolor();
  } catch (e) { console.warn('title globe', e); }
}

async function refreshContinue() {
  const b = $('#btn-continue');
  try {
    const list = (await host.listSaves()).filter(s => !s.broken);
    if (list.length) {
      b.disabled = false;
      b.dataset.file = list[0].file;
      $('#continue-sub').textContent = `${list[0].meta.name || list[0].world.name} · ${fmtAgo(list[0].meta.savedAt)}`;
    } else { b.disabled = true; $('#continue-sub').textContent = ''; }
  } catch { b.disabled = true; }
}

// ---------- Begin our story ----------
function randomSeedText() {
  const abc = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';
  let s = ''; for (let i = 0; i < 10; i++) s += abc[Math.floor(Math.random() * abc.length)];
  return s;
}
function openNewWorld() {
  let size = settings.lastSize && SIZES[settings.lastSize] ? settings.lastSize : 'n48';
  const name = el('input', { class: 'input', value: 'Our story', maxlength: 48 });
  const seed = el('input', { class: 'input', value: randomSeedText(), maxlength: 32, spellcheck: 'false', style: 'font-family: ui-monospace, Consolas, monospace; letter-spacing: .12em' });
  seed.addEventListener('input', () => { const c = seed.selectionStart; seed.value = seed.value.toUpperCase().replace(/[^A-Z0-9]/g, ''); seed.setSelectionRange(c, c); });
  const sizes = el('div', { class: 'size-pick' });
  const drawSizes = () => {
    sizes.innerHTML = '';
    for (const z of Object.values(SIZES)) sizes.append(el('button', { class: 'size-opt' + (z.key === size ? ' on' : ''), 'data-size': z.key, onclick: () => { size = z.key; drawSizes(); } },
      el('span', { class: 'size-name' }, z.label), el('span', { class: 'size-km' }, `${z.km} km`), el('span', { class: 'size-sub' }, z.sub)));
  };
  drawSizes();
  const shape = el('div', { class: 'shape', html: `
    <p>The world starts from <b>today's coasts and ice</b>. The engine has already run it for thousands of years, so the climate, plants, soils and animals have settled into balance.</p>
    <p>From here the weather changes every month. Plants grow and burn, herds follow the grass, plankton blooms where cold water rises, and snow comes and goes with the seasons. The orbit and ice sheets change slowly over centuries.</p>
    <p>People are not in this version yet. The ice-age start, with the sea 120 metres lower, comes with them. The clock counts from Year 1.</p>` });
  const body = el('div', {},
    el('div', { class: 'field' }, el('label', {}, 'Name of the story'), name),
    el('div', { class: 'field' }, el('label', {}, 'Seed'),
      el('div', { class: 'row' }, seed, el('button', { class: 'btn', onclick: () => { seed.value = randomSeedText(); } }, 'New seed')),
      el('div', { class: 'hint' }, 'The land is real Earth. The seed decides everything that happens by chance: the weather, the fires and the storms.')),
    el('div', { class: 'field' }, el('label', {}, 'Map'), sizes),
    el('div', { class: 'field', style: 'margin-bottom:0' }, el('label', {}, 'Shape of the world'), shape)
  );
  const go = async (h) => {
    const nm = name.value.trim() || 'Our story';
    const sd = seed.value.trim() || randomSeedText();
    settings.lastSize = size; saveSettings();
    h.close();
    const z = SIZES[size];
    await startWorld({ name: nm, seedText: sd, seed: hashSeed(sd), size, n: z.n, view: z.view, months: 0, months0: null, speed: 3, paused: true, createdAt: Date.now(), playSec: 0, file: null }, null);
  };
  const h = openModal({ eyebrow: 'A new world', title: 'Begin our story', body, wide: true, buttons: [{ label: 'Cancel' }, { label: 'Begin our story', kind: 'primary', onClick: go }] });
  name.addEventListener('keydown', (e) => { if (e.key === 'Enter') go(h); });
}

// ---------- Engine ----------
let waiters = [];
function waitFor(event, test = () => true) { return new Promise((resolve, reject) => waiters.push({ event, test, resolve, reject })); }
function onEngine(m) {
  for (const w of waiters.slice()) {
    if (m.event === 'error' || m.event === 'crash') { waiters.splice(waiters.indexOf(w), 1); w.reject(new Error(m.message)); }
    else if (m.event === w.event && w.test(m)) { waiters.splice(waiters.indexOf(w), 1); w.resolve(m); }
  }
  switch (m.event) {
    case 'ready': if (state.world) { state.stats = m.stats; } break;
    case 'parents': state.parents = new Uint32Array(dec(m.data).buffer); break;
    case 'sea': state.sea = dec(m.data); break;
    case 'layer': state.layer = { name: m.name, bytes: dec(m.data) }; if (state.world && modeInfo(state.mapMode).layer === m.name) recolor(); break;
    case 'stats': onStats(m.stats); break;
    case 'tile': if (m.id === state.tileSim) { state.tile = m; renderInspector(); } break;
    case 'done': state.running = false; if (state.world && !state.world.paused) startRun(); break;
    case 'crash':
      state.running = false;
      if (state.world) { state.world.paused = true; renderClock(); toast(m.message, true); }
      break;
    case 'error': toast(m.message, true); break;
    case 'log': console.log(m.message); break;
  }
}
function onStats(s) {
  const w = state.world; if (!w) return;
  state.stats = s;
  if (state.perf && s.months > w.months) { const now = performance.now(); state.perf.push([now, s.months]); while (state.perf.length > 2 && now - state.perf[0][0] > 3000) state.perf.shift(); }
  w.months = s.months;
  state.dirty = true;
  // one history point a year
  const y = yearOf(w, s.months);
  const last = state.hist[state.hist.length - 1];
  if (!last || last[0] !== y) {
    state.hist.push([y, +s.t_mean.toFixed(2), +s.co2.toFixed(1), +s.ice_sle.toFixed(2), Math.round(s.veg), Math.round(s.soil)]);
    if (state.hist.length > 4000) state.hist = state.hist.filter((_, i) => i % 2 === 0);
  }
  renderDate(); renderReadout();
  if (state.selected >= 0 && !state.tileBusy) { state.tileBusy = true; setTimeout(() => { state.tileBusy = false; if (state.selected >= 0) host.send({ cmd: 'tile', id: state.tileSim }); }, 500); }
}

function startRun() {
  const w = state.world; if (!w || state.running) return;
  state.running = true;
  host.send({ cmd: 'run', pace: SPEEDS[w.speed - 1] });
}
function stopRun() { if (state.running) host.send({ cmd: 'stop' }); }

// ---------- Loading a world into the globe ----------
async function startWorld(world, saved) {
  if (state.busy) return;
  state.busy = true;
  stopRun();
  const gen = ++state.gen;
  const z = SIZES[world.size] || SIZES.n48;
  showScreen('loading');
  $('#load-title').textContent = world.name;
  $('#load-eyebrow').textContent = saved ? 'Recalling the story' : 'Shaping the world';
  const bar = (p, label) => { $('#load-fill').style.width = Math.round(p * 100) + '%'; $('#load-label').textContent = label; };
  try {
    state.world = null;
    state.layer = null; state.parents = null; state.sea = null; state.tile = null; state.selected = -1;
    if (!globe.grid || globe.grid.n !== z.view) {
      bar(0.02, `Laying out ${fmtInt(10 * z.view * z.view + 2)} hex tiles`);
      let grid;
      if (z.view === TITLE_N && titleGrid) grid = titleGrid;
      else grid = await globe.buildGrid(z.view, 7, (p, label) => bar(0.02 + 0.6 * p, label));
      if (gen !== state.gen) return;
      bar(0.63, 'Handing the tiles to the graphics card');
      await new Promise(r => setTimeout(r, 30));
      globe.setGrid(grid);
    }
    bar(0.66, 'Reading the land and sea floor');
    state.terrain = await loadTerrain(z.view); state.shade = null;
    bar(0.72, saved ? 'Waking the world where it was left' : 'Waking the spun-up world');
    const ready = waitFor('ready');
    const layer0 = waitFor('layer');
    host.send(saved ? { cmd: 'load', n: z.n, view: z.view, file: world.file || saved.stateFile } : { cmd: 'load', n: z.n, view: z.view, seed: world.seed });
    const r = await ready;
    await layer0;
    if (gen !== state.gen) return;
    if (world.months0 == null) world.months0 = r.months;
    state.stats = r.stats;
    world.months = r.months;
    state.world = world;
    state.hist = saved && Array.isArray(saved.hist) ? saved.hist : [];
    state.dirty = !!(saved && saved.meta && saved.meta.autosave);
    state.lastAutosave = Date.now();
    state.perf = [];
    state.mapMode = saved && saved.ui && MAP_MODES.some(m => m.id === saved.ui.mapMode) ? saved.ui.mapMode : 'terrain';
    bar(1, 'Ready');
    // the globe: centred, no spin
    globe.tShift = 0; globe.autoSpin = 0;
    if (saved && saved.camera) { globe.cam.yaw = saved.camera.yaw; globe.cam.pitch = saved.camera.pitch; globe.cam.tDist = saved.camera.dist; }
    else { globe.cam.tDist = 3.3; globe.cam.pitch = 0.35; }
    showScreen('world');
    $('#ro-tiles').textContent = fmtInt(10 * z.view * z.view + 2) + (z.view !== z.n ? ` (${fmtInt(10 * z.n * z.n + 2)} simulated)` : '');
    $('#ro-size').textContent = z.view !== z.n ? `~${z.km} km drawn, ~${SIZES.n48.km} km simulated` : `~${z.km} km`;
    renderMapModes(); renderClock(); renderReadout();
    setMapMode(state.mapMode);
    if (!saved) toast(`"${world.name}" begins. Press Space or the dial's centre to start the clock.`);
  } catch (e) {
    console.error(e);
    toast('Could not start the world: ' + (e.message || e), true);
    state.busy = false;
    enterTitle();
    return;
  } finally { state.busy = false; }
}

// ---------- Save system ----------
function saveMeta(file, { autosave = false } = {}) {
  const w = state.world;
  return {
    format: 'odyssey-save', version: SAVE_VERSION,
    meta: { name: autosave ? w.name + ' (autosave)' : w.name, savedAt: Date.now(), createdAt: w.createdAt, app: APP_VERSION, autosave },
    world: { name: w.name, seedText: w.seedText, seed: w.seed, size: w.size, n: w.n, view: w.view, months: w.months, months0: w.months0, speed: w.speed, playSec: Math.round(w.playSec) },
    stateFile: file,
    hist: state.hist.length > 1500 ? state.hist.filter((_, i, a) => i % Math.ceil(a.length / 1500) === 0 || i === a.length - 1) : state.hist,
    camera: { yaw: globe.cam.yaw, pitch: globe.cam.pitch, dist: globe.cam.tDist },
    ui: { mapMode: state.mapMode }
  };
}
async function writeWorld(file, { quiet = false, autosave = false } = {}) {
  if (!state.world) return false;
  try {
    const saved = waitFor('saved', (m) => m.file === file);
    host.send({ cmd: 'save', file });
    await saved;
    await host.writeSave(file, saveMeta(file, { autosave }));
    if (!autosave) { state.world.file = file; state.dirty = false; }
    state.lastAutosave = Date.now();
    if (!quiet) toast(autosave ? 'Autosaved' : `Saved "${state.world.name}"`);
    refreshContinue();
    return true;
  } catch (e) {
    toast('Save failed: ' + (e.message || e), true);
    return false;
  }
}
async function quickSave() {
  if (!state.world) return;
  if (!state.world.file) return saveAs();
  return writeWorld(state.world.file);
}
function saveAs() {
  return new Promise((resolve) => {
    const input = el('input', { class: 'input', value: state.world.name, maxlength: 48 });
    const hint = el('div', { class: 'hint' });
    const upd = () => { hint.textContent = `File: ${slug(input.value)}.owsave`; };
    input.addEventListener('input', upd); upd();
    const go = async (h) => {
      const nm = input.value.trim() || state.world.name;
      const file = slug(nm);
      if (file !== state.world.file && await host.saveExists(file)) {
        const ok = await confirmModal({ title: 'Overwrite save?', text: `A save named "${nm}" already exists. Replace it?`, confirm: 'Overwrite', danger: true });
        if (!ok) return;
      }
      h.close();
      state.world.name = nm;
      $('#tb-world').textContent = nm;
      resolve(await writeWorld(file));
    };
    const h = openModal({ eyebrow: 'Save', title: 'Save as', body: el('div', {}, el('div', { class: 'field', style: 'margin:0' }, el('label', {}, 'Save name'), input, hint)),
      buttons: [{ label: 'Cancel', onClick: (h) => { h.close(); resolve(false); } }, { label: 'Save', kind: 'primary', onClick: go }] });
    input.addEventListener('keydown', (e) => { if (e.key === 'Enter') go(h); });
  });
}
async function openLoad() {
  let list = [];
  try { list = await host.listSaves(); } catch (e) { toast('Could not read saves: ' + e.message, true); }
  const wrap = el('div', { class: 'saves' });
  let chosen = null, modal;
  const render = () => {
    wrap.innerHTML = '';
    if (!list.length) {
      wrap.append(el('div', { class: 'empty' }, el('span', { class: 'serif' }, 'No stories kept yet'), 'Begin a story, then save it from the menu.'));
      return;
    }
    for (const s of list) {
      const w = s.world || {};
      const row = el('div', { class: 'save' + (chosen === s ? ' on' : '') },
        el('div', { class: 's-main' },
          el('div', { class: 's-name' }, s.broken ? `${s.file} (unreadable)` : (s.meta.name || w.name || s.file)),
          el('div', { class: 's-meta' }, s.broken ? 'This save is missing its world file.' :
            `${fmtWhen(w)} · ${SIZES[w.size] ? SIZES[w.size].label + ' map' : ''} · saved ${fmtAgo(s.meta.savedAt)}`)),
        s.meta.autosave ? el('span', { class: 's-tag' }, 'Auto') : null,
        el('button', { class: 's-del', title: 'Delete', html: ICONS.trash, onclick: async (e) => {
          e.stopPropagation();
          const ok = await confirmModal({ title: 'Delete this save?', text: `"${s.meta.name || s.file}" will be gone for good.`, confirm: 'Delete', danger: true });
          if (!ok) return;
          await host.deleteSave(s.file);
          list = list.filter(x => x !== s); if (chosen === s) chosen = null;
          render(); refreshContinue();
        } }));
      row.addEventListener('click', () => { if (s.broken) return; chosen = s; render(); });
      row.addEventListener('dblclick', () => { if (s.broken) return; chosen = s; doLoad(modal); });
      wrap.append(row);
    }
    const lb = modal && modal.el.querySelector('.btn.primary');
    if (lb) lb.disabled = !chosen;
  };
  const doLoad = async (h) => {
    if (!chosen) return;
    if (state.world && state.dirty) {
      const ok = await confirmModal({ title: 'Leave this story?', text: 'Anything since your last save will be lost. Recall the other one anyway?', confirm: 'Recall', danger: true });
      if (!ok) return;
    }
    h.close();
    closeAllModals();
    loadFile(chosen.file);
  };
  modal = openModal({ eyebrow: 'Stories kept', title: 'Recall our story', body: wrap, wide: true, buttons: [{ label: 'Cancel' }, { label: 'Recall', kind: 'primary', onClick: doLoad }] });
  render();
}
async function loadFile(file) {
  try {
    const d = await host.readSave(file);
    if (!d || d.format !== 'odyssey-save') throw new Error('Not a saved story');
    if (d.version > SAVE_VERSION) throw new Error('Save is from a newer version');
    const w = d.world;
    if (!SIZES[w.size]) throw new Error('Unknown map size');
    const world = {
      name: w.name, seedText: w.seedText, seed: w.seed >>> 0, size: w.size, n: w.n, view: w.view, months: w.months, months0: w.months0,
      speed: Math.max(1, Math.min(5, w.speed || 3)), paused: true, createdAt: d.meta.createdAt || Date.now(), playSec: w.playSec || 0,
      file: d.meta.autosave ? null : file
    };
    d.stateFile = file;
    await startWorld(world, d);
  } catch (e) {
    toast('Could not load: ' + e.message, true);
  }
}
function worldKey() { const w = state.world; return `${slug(w.name)}-${(w.seed >>> 0).toString(36)}`; }

// ---------- HUD ----------
// The clock's dial: the five speeds on an arc from the sun to the moon; the hub pauses and resumes.
const DIAL_GLYPHS = [
  '<circle class="gf" r="4.2"/><path class="g" d="M0-7.5v-2.5M0 7.5v2.5M-7.5 0h-2.5M7.5 0h2.5M-5.3-5.3l-1.8-1.8M5.3 5.3l1.8 1.8M-5.3 5.3l-1.8 1.8M5.3-5.3l1.8-1.8"/>',
  '<path class="gf" d="M-5 2a5 5 0 0 1 10 0z"/><path class="g" d="M-9 2.5h18M0-5v-2.5M-5.5-2.5l-1.8-1.3M5.5-2.5l1.8-1.3"/>',
  '<circle class="g" r="3.2" cy="2"/><path class="g" d="M-7-3.5a7.5 7.5 0 0 0 14 0"/>',
  '<path class="gf" d="M-7.5-2.5a7.5 7.5 0 0 0 15 0a6 5 0 0 1-15 0z"/>',
  '<circle class="gf" r="4.3"/><path class="g" d="M-8.5-1a8.5 8.5 0 0 0 17 0"/>'
];
function buildDial() {
  const svg = $('#dial'); if (!svg || svg.dataset.built) return;
  svg.dataset.built = '1';
  const n = 5, R0 = 46, R1 = 92, P = (r, a) => `${(r * Math.cos(a)).toFixed(2)} ${(-r * Math.sin(a)).toFixed(2)}`;
  let h = '<defs><radialGradient id="clayHub" cx=".38" cy=".32" r=".8"><stop offset="0" stop-color="#e2bd92"/><stop offset=".6" stop-color="#b98a62"/><stop offset="1" stop-color="#8c6243"/></radialGradient></defs>';
  const names = ['Sun', 'Sunset', 'Dusk', 'Crescent moon', 'Full moon'];
  for (let i = 0; i < n; i++) {
    const a0 = Math.PI - i * Math.PI / n - 0.012, a1 = Math.PI - (i + 1) * Math.PI / n + 0.012, am = (a0 + a1) / 2;
    const d = `M${P(R0, a0)} L${P(R1, a0)} A${R1} ${R1} 0 0 1 ${P(R1, a1)} L${P(R0, a1)} A${R0} ${R0} 0 0 0 ${P(R0, a0)}Z`;
    h += `<g class="seg${i === 4 ? ' max' : ''}" data-s="${i + 1}"><title>${names[i]}: ${SPEED_TEXT[i].toLowerCase()} (${i + 1})</title><path class="w" d="${d}"/><g transform="translate(${P(69, am).replace(' ', ',')})">${DIAL_GLYPHS[i]}</g></g>`;
  }
  h += '<g class="hub" id="dial-hub"><title>Pause or resume (Space)</title><circle r="33"/><path id="dial-hub-g" d=""/></g>';
  svg.innerHTML = h;
  svg.querySelectorAll('.seg').forEach(s => s.addEventListener('click', () => setSpeed(+s.dataset.s)));
  $('#dial-hub').addEventListener('click', togglePause);
}
function renderClock() {
  const w = state.world; if (!w) return;
  buildDial();
  $('#ro-perf-row').classList.toggle('hidden', !settings.testMode);
  document.querySelectorAll('#dial .seg').forEach((c) => c.classList.toggle('on', +c.dataset.s <= w.speed));
  const hub = $('#dial-hub-g');
  if (hub) hub.setAttribute('d', w.paused ? 'M-7-11L12 0L-7 11Z' : 'M-9-10h6v20h-6zM3-10h6v20h-6z');
  renderDate();
  $('#date-sub').textContent = w.paused ? 'Paused' : SPEED_TEXT[w.speed - 1];
}
function renderDate() {
  const w = state.world; if (!w) return;
  $('#date').textContent = MONTHS[((w.months % 12) + 12) % 12];
  $('#year').textContent = `Year ${fmtInt(yearOf(w, w.months))}`;
}
function renderReadout() {
  const s = state.stats; if (!s) return;
  $('#ro-temp').textContent = `${s.t_mean.toFixed(1)} °C`;
  $('#ro-co2').textContent = `${s.co2.toFixed(0)} ppm`;
  $('#ro-ice').textContent = `${s.ice_sle.toFixed(1)} m of sea`;
  if (settings.testMode && state.perf && state.perf.length > 1) {
    const a = state.perf[0], b = state.perf[state.perf.length - 1];
    const mps = (b[1] - a[1]) / Math.max(0.001, (b[0] - a[0]) / 1000);
    $('#ro-perf').textContent = `${mps.toFixed(1)} months/s${s.skipped ? ` · ${s.skipped} skipped` : ''}`;
  }
}
function setSpeed(s) {
  const w = state.world; if (!w) return;
  w.speed = Math.max(1, Math.min(5, s));
  if (state.running) host.send({ cmd: 'pace', pace: SPEEDS[w.speed - 1] });
  state.perf = [];
  renderClock();
}
function togglePause() {
  const w = state.world; if (!w) return;
  w.paused = !w.paused;
  if (w.paused) stopRun(); else startRun();
  renderClock();
}

// ---------- Inspector ----------
const LABEL_TAB = {
  'Surface temperature': 'climate', 'Rain vs normal': 'climate', 'Snow on the ground': 'climate', 'Sea ice': 'climate',
  'Plant carbon': 'life', 'Plant-eating animals': 'life', 'Meat-eating animals': 'life', 'Burning now': 'life', 'Plankton': 'life', 'Fish': 'life',
  'Soil carbon': 'land', 'Land ice thickness': 'land', 'Land height': 'land', 'Sea depth': 'land'
};
const INSP_TABS = [['land', 'Land'], ['climate', 'Climate'], ['life', 'Life']];
function selectCell(id) {
  state.selected = id;
  globe.sel = id; globe.dirty = true;
  state.tile = null;
  if (id < 0) { $('#inspector').classList.add('hidden'); return; }
  state.tileSim = state.parents ? state.parents[id] : id;
  host.send({ cmd: 'tile', id: state.tileSim });
  renderInspector();
}
function fmtVal(v, unit) {
  if (unit === '%') return `${Math.round(v)}%`;
  if (unit === 'C') return `${v.toFixed(1)} °C`;
  const a = Math.abs(v);
  const s = a >= 100 ? fmtInt(v) : a >= 1 ? v.toFixed(1) : a >= 0.01 ? v.toFixed(3) : a > 0 ? v.toExponential(1) : '0';
  const u = { 'kgC/m2': 'kg C/m²', 'gC/m2': 'g C/m²', 'mm w.e.': 'mm of water' }[unit] || unit;
  return u ? `${s} ${u}` : s;
}
function renderInspector() {
  const box = $('#inspector');
  const id = state.selected;
  if (id < 0 || !globe.grid || !state.world) { box.classList.add('hidden'); return; }
  const { lat, lon } = globe.cellLatLon(id);
  const latS = `${Math.abs(lat).toFixed(2)}° ${lat >= 0 ? 'N' : 'S'}`, lonS = `${Math.abs(lon).toFixed(2)}° ${lon >= 0 ? 'E' : 'W'}`;
  const ter = state.terrain && state.terrain.length === globe.grid.V ? state.terrain : null;
  const t = state.tile;
  const isSea = t ? t.sea : false;
  const nb = globe.grid.neighbors;
  let sides = 0; for (let j = 0; j < 6; j++) if (nb[id * 6 + j] >= 0) sides++;
  $('#insp-kind').textContent = [isSea ? 'Sea' : 'Land', sides === 5 ? 'pentagon' : null].filter(Boolean).join(' · ');
  $('#insp-title').textContent = !t ? '...' : describe(t, ter ? ter[id] : null);
  const kv = (k, v, tab) => el('div', { class: 'kv', 'data-tab': tab }, el('span', {}, k), el('span', {}, v));
  const body = $('#insp-body');
  body.innerHTML = '';
  const tab = state.inspTab || 'land';
  const rows = [];
  if (ter && state.world.view !== state.world.n) rows.push(kv(ter[id] > 0 ? 'Height here' : 'Depth here', `${fmtInt(Math.abs(ter[id]))} m`, 'land'));
  if (t) for (const r of t.rows) rows.push(kv(r.label, fmtVal(r.value, r.unit), LABEL_TAB[r.label] || 'land'));
  rows.push(kv('Position', `${latS}, ${lonS}`, 'land'), kv('Tile', '#' + fmtInt(id), 'land'), kv('Area', `~ ${fmtInt(EARTH_AREA_KM2 / globe.grid.V)} km²`, 'land'));
  if (state.parents) rows.push(kv('Simulated as', `tile #${fmtInt(state.tileSim)}, ~${fmtInt(t ? t.area_km2 : 0)} km²`, 'land'));
  for (const r of rows) if (r.dataset.tab === tab) body.append(r);
  if (tab === 'land') body.append(el('button', { class: 'link-btn', onclick: () => { globe.focusCell(id); globe.cam.tDist = Math.min(globe.cam.tDist, 1 + 30 * SIZES[state.world.size].km / 6371); } }, 'Fly to tile →'));
  const bar = $('#insp-tabs'); bar.innerHTML = '';
  for (const [k, name] of INSP_TABS) bar.append(el('button', { class: 'insp-tab' + (tab === k ? ' on' : ''), onclick: () => { state.inspTab = k; renderInspector(); } }, name));
  box.classList.remove('hidden');
}
function describe(t, elev) {
  const v = (lab) => { const r = t.rows.find(x => x.label === lab); return r ? r.value : 0; };
  if (t.sea) return v('Sea ice') > 300 ? 'Frozen sea' : Math.abs(t.lat) > 55 ? 'Cold sea' : Math.abs(t.lat) < 23 ? 'Warm sea' : 'Open sea';
  const ice = v('Land ice thickness'), veg = v('Plant carbon'), temp = v('Surface temperature');
  if (ice > 50) return 'Ice sheet';
  if ((elev != null ? elev : v('Land height')) > 3000) return 'High mountains';
  if (veg > 9) return Math.abs(t.lat) < 23 ? 'Rainforest' : 'Forest';
  if (veg > 4) return 'Woodland';
  if (Math.abs(t.lat) > 55) return veg > 1.5 ? 'Open taiga' : 'Tundra';
  if (veg > 1.2) return 'Grassland';
  return temp < -5 ? 'Cold desert' : 'Desert';
}

// ---------- World history ----------
function openHistory() {
  const H = state.hist;
  const body = el('div', { class: 'hist' });
  if (H.length < 2) body.append(el('p', { style: 'color:var(--muted)' }, 'Run the world for a few years to see its history here.'));
  else {
    const chart = (title, k, unit, color) => {
      const W = 620, Hh = 120, xs = H.map(r => r[0]), ys = H.map(r => r[k]);
      const x0 = xs[0], x1 = xs[xs.length - 1] || x0 + 1;
      let y0 = Math.min(...ys), y1 = Math.max(...ys); if (y1 - y0 < 1e-6) { y0 -= 1; y1 += 1; }
      const pad = (y1 - y0) * 0.1; y0 -= pad; y1 += pad;
      const X = (x) => 40 + (x - x0) / Math.max(1, x1 - x0) * (W - 50), Y = (y) => 10 + (1 - (y - y0) / (y1 - y0)) * (Hh - 30);
      const d = H.map((r, i) => `${i ? 'L' : 'M'}${X(r[0]).toFixed(1)} ${Y(r[k]).toFixed(1)}`).join('');
      const svg = `<svg viewBox="0 0 ${W} ${Hh}" width="100%"><path d="${d}" fill="none" stroke="${color}" stroke-width="1.6"/>
        <text x="4" y="${Y(y1 - pad) + 4}" class="ht">${(y1 - pad).toFixed(1)}</text><text x="4" y="${Y(y0 + pad) + 4}" class="ht">${(y0 + pad).toFixed(1)}</text>
        <text x="40" y="${Hh - 4}" class="ht">Year ${fmtInt(x0)}</text><text x="${W - 10}" y="${Hh - 4}" text-anchor="end" class="ht">Year ${fmtInt(x1)}</text></svg>`;
      return el('div', { class: 'hist-chart' }, el('div', { class: 'hist-title' }, `${title} `, el('span', {}, unit)), el('div', { html: svg }));
    };
    body.append(chart('World temperature', 1, '°C', '#ee8a5a'), chart('Carbon dioxide', 2, 'ppm', '#d9c894'), chart('Ice on land', 3, 'm of sea level', '#cfe0f0'), chart('Carbon in plants', 4, 'billion tonnes', '#78bf82'));
  }
  openModal({ eyebrow: state.world ? fmtWhen(state.world) : '', title: 'World history', body, wide: true, buttons: [{ label: 'Close', kind: 'primary' }] });
}

// ---------- Pause menu ----------
function openPauseMenu() {
  if (!state.world) return;
  const wasPaused = state.world.paused;
  if (!wasPaused) togglePause();
  const item = (label, fn) => el('button', { class: 'menu-item', onclick: fn }, el('span', {}, label));
  let h;
  const body = el('div', { class: 'pause-menu' },
    item('Resume', () => h.close()),
    el('div', { class: 'pause-sep' }),
    item('Save', async () => { h.close(); await quickSave(); }),
    item('Save as...', async () => { h.close(); await saveAs(); }),
    item('Recall our story...', () => { h.close(); openLoad(); }),
    item('World history', () => { openHistory(); }),
    item('Settings', () => { openSettings(); }),
    el('div', { class: 'pause-sep' }),
    item('Back to the title', async () => {
      if (state.dirty && !(await confirmModal({ title: 'Back to the title?', text: 'Anything since your last save will be lost.', confirm: 'Leave', danger: true }))) return;
      h.close(); enterTitle();
    }),
    item('Pause our story', async () => {
      if (state.dirty && !(await confirmModal({ title: 'Close the game?', text: 'Anything since your last save will be lost.', confirm: 'Close', danger: true }))) return;
      host.quit();
    })
  );
  h = openModal({
    eyebrow: fmtWhen(state.world), title: state.world.name, body,
    onClose: () => { if (state.world && !wasPaused && state.world.paused && state.screen === 'world') togglePause(); }
  });
}

// ---------- Main loop ----------
let clockAcc = 0;
function onFrame(dt) {
  if (state.screen !== 'world' || !state.world) return;
  const w = state.world;
  if (!w.paused) w.playSec += dt;
  clockAcc += dt;
  if (clockAcc > 0.15) { clockAcc = 0; $('#ro-alt').textContent = fmtKm(globe.altitudeKm()); }
  if (settings.autosaveMin > 0 && state.dirty && !modalStack.length && Date.now() - state.lastAutosave > settings.autosaveMin * 60000) {
    state.lastAutosave = Date.now();
    writeWorld(worldKey() + '-autosave', { autosave: true, quiet: false });
  }
}

// ---------- Keyboard ----------
function onKey(e) {
  const typing = e.target && (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA');
  if (e.key === 'Escape') {
    e.preventDefault();
    if (closeTopModal()) return;
    if (state.screen === 'world') { if (state.selected >= 0) { selectCell(-1); return; } openPauseMenu(); }
    return;
  }
  if (e.key === 'F11') { e.preventDefault(); host.toggleFullscreen(); return; }
  if (typing || modalStack.length || state.screen !== 'world') return;
  const k = e.key.toLowerCase();
  if (e.code === 'Space') { e.preventDefault(); togglePause(); }
  else if (/^[1-5]$/.test(e.key)) setSpeed(+e.key);
  else if (e.key === '+' || e.key === '=') setSpeed(state.world.speed + 1);
  else if (e.key === '-') setSpeed(state.world.speed - 1);
  else if ((e.ctrlKey && k === 's') || e.key === 'F5') { e.preventDefault(); quickSave(); }
  else if (k === 'b') { settings.borders = !settings.borders; applySettings(); saveSettings(); toast(`Tile borders ${settings.borders ? 'on' : 'off'}`); }
  else if (k === 'g') { settings.graticule = !settings.graticule; applySettings(); saveSettings(); toast(`Grid ${settings.graticule ? 'on' : 'off'}`); }
  else if (k === 'h') openHistory();
  else if (k === 'home') globe.cam.tDist = 2.4;
}

// ---------- Boot ----------
async function boot() {
  try {
    globe = new Globe($('#gl'));
  } catch (e) {
    document.body.append(el('div', { style: 'position:fixed;inset:0;display:grid;place-items:center;color:#dde2f2;font-family:Georgia,serif;font-size:20px' }, 'This machine does not support WebGL2. ' + e.message));
    return;
  }
  globe.onFrame = onFrame;
  globe.onSelect = (id) => selectCell(id);
  globe.onRestored = () => { state.shade = null; recolor(); };
  host.onEvent(onEngine);
  $('#ver').textContent = 'v' + APP_VERSION;
  await loadSettings();
  if (settings.fullscreen) host.setFullscreen(true);
  $('#btn-menu').innerHTML = ICONS.menu;
  $('#btn-menu').addEventListener('click', openPauseMenu);
  $('#btn-history').innerHTML = ICONS.history;
  $('#btn-history').addEventListener('click', openHistory);
  $('#insp-close').addEventListener('click', () => selectCell(-1));
  $('#title-menu').addEventListener('click', (e) => {
    const b = e.target.closest('.menu-item'); if (!b || b.disabled || state.busy || state.screen !== 'title' || modalStack.length) return;
    const act = b.dataset.act;
    if (act === 'new') openNewWorld();
    else if (act === 'load') openLoad();
    else if (act === 'continue' && b.dataset.file) loadFile(b.dataset.file);
    else if (act === 'settings') openSettings();
    else if (act === 'quit') host.quit();
  });
  window.addEventListener('keydown', onKey);
  enterTitle();
}

boot();
