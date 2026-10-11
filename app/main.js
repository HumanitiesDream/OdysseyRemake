// Electron shell. It starts the Rust engine as a child program and passes JSON lines between it and the window.
// It also keeps the save files and the settings.
const { app, BrowserWindow, ipcMain, shell } = require('electron');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

let win = null;
let engine = null;
let buffer = '';

const userDir = () => app.getPath('userData');
const savesDir = () => path.join(userDir(), 'saves');
const settingsFile = () => path.join(userDir(), 'settings.json');

// Only plain file names inside the saves folder: letters, digits, '-' and '_'.
function safeName(file) {
  const base = String(file || '').replace(/\.owsave$/, '');
  if (!/^[a-z0-9][a-z0-9_-]{0,79}$/i.test(base)) throw new Error('Bad save name');
  return base;
}
function writeAtomic(target, text) {
  const tmp = target + '.tmp';
  fs.writeFileSync(tmp, text);
  fs.renameSync(tmp, target);
}

function enginePath() {
  const exe = process.platform === 'win32' ? 'odyssey-engine.exe' : 'odyssey-engine';
  const packed = path.join(process.resourcesPath || '', 'engine', exe);
  if (app.isPackaged && fs.existsSync(packed)) return packed;
  const here = path.join(__dirname, 'engine', exe);
  if (fs.existsSync(here)) return here;
  // development: the cargo build
  return path.join(__dirname, '..', 'target', 'release', process.platform === 'win32' ? 'engine.exe' : 'engine');
}
function dataDir() {
  const beside = path.join(path.dirname(enginePath()), 'data');
  return fs.existsSync(beside) ? beside : path.join(__dirname, '..', 'data');
}

function send(obj) { if (win && !win.isDestroyed()) win.webContents.send('engine', obj); }

function startEngine() {
  const exe = enginePath();
  if (!fs.existsSync(exe)) { send({ event: 'error', message: 'The engine program was not found: ' + exe }); return; }
  fs.mkdirSync(savesDir(), { recursive: true });
  const env = { ...process.env, ODYSSEY_DATA: dataDir(), ODYSSEY_SAVES: savesDir() };
  engine = spawn(exe, [], { env, stdio: ['pipe', 'pipe', 'pipe'], windowsHide: true });
  buffer = '';
  engine.stdout.on('data', chunk => {
    buffer += chunk.toString('utf8');
    let i;
    while ((i = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, i); buffer = buffer.slice(i + 1);
      if (!line.trim()) continue;
      try { send(JSON.parse(line)); } catch (e) { send({ event: 'error', message: 'Bad line from the engine: ' + line.slice(0, 120) }); }
    }
  });
  engine.stderr.on('data', d => send({ event: 'log', message: d.toString('utf8').slice(0, 500) }));
  engine.on('exit', code => {
    engine = null;
    if (!quitting) send({ event: 'crash', message: 'The engine stopped (code ' + code + '). It will restart when you begin or recall a story.' });
  });
}

function createWindow() {
  win = new BrowserWindow({
    width: 1360, height: 880, minWidth: 900, minHeight: 620,
    title: 'Odyssey',
    backgroundColor: '#04060d',
    webPreferences: { preload: path.join(__dirname, 'preload.js'), contextIsolation: true, nodeIntegration: false, sandbox: true }
  });
  win.setMenuBarVisibility(false);
  win.loadFile(path.join(__dirname, 'renderer', 'index.html'));
  win.webContents.on('did-finish-load', () => { if (!engine) startEngine(); });
}

let quitting = false;
function quitNow() {
  quitting = true;
  if (engine) { try { engine.stdin.write('{"cmd":"quit"}\n'); } catch (_) {} setTimeout(() => { try { engine && engine.kill(); } catch (_) {} }, 400); }
  app.quit();
}

ipcMain.handle('data:read', (_e, name) => {
  if (!/^(terrain|look)-n\d+\.bin$/.test(String(name))) return null;
  const f = path.join(dataDir(), name);
  try { return fs.existsSync(f) ? fs.readFileSync(f) : null; } catch (_) { return null; }
});
ipcMain.handle('app:quit', () => quitNow());
ipcMain.on('cmd', (_e, obj) => {
  if (!engine) startEngine();
  if (engine && engine.stdin.writable) engine.stdin.write(JSON.stringify(obj) + '\n');
  else send({ event: 'error', message: 'The engine is not running.' });
});

ipcMain.handle('saves:list', () => {
  fs.mkdirSync(savesDir(), { recursive: true });
  const out = [];
  for (const f of fs.readdirSync(savesDir())) {
    if (!f.endsWith('.owsave')) continue;
    const base = f.slice(0, -7);
    try {
      const d = JSON.parse(fs.readFileSync(path.join(savesDir(), f), 'utf8'));
      out.push({ file: base, meta: d.meta, world: d.world, broken: !fs.existsSync(path.join(savesDir(), base + '.owstate')) });
    } catch (e) { out.push({ file: base, meta: { savedAt: 0 }, world: {}, broken: true }); }
  }
  return out.sort((a, b) => (b.meta.savedAt || 0) - (a.meta.savedAt || 0));
});
ipcMain.handle('saves:read', (_e, file) => JSON.parse(fs.readFileSync(path.join(savesDir(), safeName(file) + '.owsave'), 'utf8')));
ipcMain.handle('saves:exists', (_e, file) => fs.existsSync(path.join(savesDir(), safeName(file) + '.owsave')));
// the engine writes <file>.owstate itself; this writes the story's details beside it
ipcMain.handle('saves:write', (_e, file, data) => { fs.mkdirSync(savesDir(), { recursive: true }); writeAtomic(path.join(savesDir(), safeName(file) + '.owsave'), JSON.stringify(data)); return true; });
ipcMain.handle('saves:delete', (_e, file) => {
  const b = safeName(file);
  for (const ext of ['.owsave', '.owstate']) { const p = path.join(savesDir(), b + ext); if (fs.existsSync(p)) fs.unlinkSync(p); }
  return true;
});
ipcMain.handle('saves:open', () => { fs.mkdirSync(savesDir(), { recursive: true }); return shell.openPath(savesDir()); });
ipcMain.handle('settings:get', () => { try { return JSON.parse(fs.readFileSync(settingsFile(), 'utf8')); } catch { return null; } });
ipcMain.handle('settings:set', (_e, s) => { fs.mkdirSync(userDir(), { recursive: true }); writeAtomic(settingsFile(), JSON.stringify(s, null, 2)); return true; });
ipcMain.handle('win:fullscreen', () => { if (win) win.setFullScreen(!win.isFullScreen()); });
ipcMain.handle('win:setFullscreen', (_e, on) => { if (win) win.setFullScreen(!!on); });

app.whenReady().then(() => {
  createWindow();
  if (process.env.ODYSSEY_SELFTEST) {
    // used by the build checks: start a world, run a while, save screenshots, then quit
    const wait = ms => new Promise(r => setTimeout(r, ms));
    const shot = async (tag) => { const img = await win.webContents.capturePage(); fs.writeFileSync(process.env.ODYSSEY_SELFTEST.replace(/\.png$/, '') + '-' + tag + '.png', img.toPNG()); };
    const js = (s) => win.webContents.executeJavaScript(s);
    win.webContents.once('did-finish-load', async () => {
      try {
        await wait(+(process.env.ODYSSEY_TITLE_WAIT || 6000));
        await shot('title');
        await js("document.querySelector('[data-act=new]').click()");
        await wait(800);
        await js("document.querySelector('[data-size=\"" + (process.env.ODYSSEY_SIZE || 'n24') + "\"]').click()");
        await wait(500);
        await shot('new');
        await js("document.querySelector('.modal .btn.primary').click()");
        await wait(+(process.env.ODYSSEY_WAIT || 12000));
        await js("setSpeed(5); togglePause();");
        await wait(6000);
        await shot('world');
        await js("selectCell(1234)");
        await wait(1500);
        await shot('tile');
        await js("openPauseMenu()");
        await wait(600);
        await shot('pause');
        console.log('DATE:', await js("document.getElementById('date').textContent + ' / ' + document.getElementById('year').textContent"));
        await js("closeAllModals(); state.world.paused || togglePause();");
        await wait(1500);
        console.log('SAVED:', await js("writeWorld('selftest-save')"));
        await js("state.dirty = false; enterTitle()");
        await wait(3000);
        await shot('title2');
        console.log('CONTINUE:', await js("document.getElementById('continue-sub').textContent"));
        await js("loadFile('selftest-save')");
        await wait(+(process.env.ODYSSEY_WAIT || 12000));
        await shot('recalled');
        console.log('RECALLED:', await js("state.world && fmtWhen(state.world)"));
        await js("setMapMode('temperature')");
        await wait(1500);
        await shot('temp');
      } catch (e) { console.log('SELFTEST ERROR', e); }
      quitNow();
    });
  }
});
app.on('window-all-closed', () => quitNow());
