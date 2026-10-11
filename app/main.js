// Electron shell. It starts the Rust engine as a child program and passes JSON lines between it and the window.
const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

let win = null;
let engine = null;
let buffer = '';

function enginePath() {
  const exe = process.platform === 'win32' ? 'odyssey-engine.exe' : 'odyssey-engine';
  const packed = path.join(process.resourcesPath || '', 'engine', exe);
  if (app.isPackaged && fs.existsSync(packed)) return packed;
  const here = path.join(__dirname, 'engine', exe);
  if (fs.existsSync(here)) return here;
  // development: the cargo build
  const dev = path.join(__dirname, '..', 'target', 'release', process.platform === 'win32' ? 'engine.exe' : 'engine');
  return dev;
}

function send(obj) { if (win && !win.isDestroyed()) win.webContents.send('engine', obj); }

function startEngine() {
  const exe = enginePath();
  if (!fs.existsSync(exe)) {
    send({ event: 'error', message: 'The engine program was not found: ' + exe });
    return;
  }
  const env = { ...process.env };
  const dataBeside = path.join(path.dirname(exe), 'data');
  if (fs.existsSync(dataBeside)) env.ODYSSEY_DATA = dataBeside;
  else env.ODYSSEY_DATA = env.ODYSSEY_DATA || path.join(__dirname, '..', 'data');
  engine = spawn(exe, [], { env, stdio: ['pipe', 'pipe', 'pipe'], windowsHide: true });
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
  engine.on('exit', code => { send({ event: 'error', message: 'The engine stopped (code ' + code + '). Restart the app.' }); engine = null; });
}

function createWindow() {
  win = new BrowserWindow({
    width: 1200, height: 860, minWidth: 760, minHeight: 600,
    title: 'Odyssey',
    backgroundColor: '#0f1715',
    webPreferences: { preload: path.join(__dirname, 'preload.js'), contextIsolation: true, nodeIntegration: false, sandbox: true }
  });
  win.setMenuBarVisibility(false);
  win.loadFile(path.join(__dirname, 'renderer', 'index.html'));
  win.webContents.on('did-finish-load', () => { if (!engine) startEngine(); });
}

ipcMain.handle('terrain', () => {
  const dir = engine ? (process.env.ODYSSEY_DATA_USED || '') : '';
  const exe = enginePath();
  const cand = [path.join(path.dirname(exe), 'data', 'terrain-n480.bin'), path.join(__dirname, '..', 'data', 'terrain-n480.bin'), dir && path.join(dir, 'terrain-n480.bin')].filter(Boolean);
  for (const f of cand) { try { if (fs.existsSync(f)) return fs.readFileSync(f); } catch (_) {} }
  return null;
});
ipcMain.on('quit', () => app.quit());
ipcMain.on('cmd', (_e, obj) => {
  if (engine && engine.stdin.writable) engine.stdin.write(JSON.stringify(obj) + '\n');
  else send({ event: 'error', message: 'The engine is not running.' });
});

app.whenReady().then(() => {
  createWindow();
  if (process.env.ODYSSEY_SELFTEST) {
    // used by the build checks: start a world, run two seconds, save a screenshot, then quit
    const wait = ms => new Promise(r => setTimeout(r, ms));
    win.webContents.on('did-finish-load', async () => {
      await wait(1500);
      await win.webContents.executeJavaScript("document.querySelector('[data-size=\"" + (process.env.ODYSSEY_SIZE || 'n24') + "\"]').click()");
      await wait(+(process.env.ODYSSEY_WAIT || 12000));
      await win.webContents.executeJavaScript("document.getElementById('btn-play').click()");
      await wait(6000);
      const img = await win.webContents.capturePage();
      fs.writeFileSync(process.env.ODYSSEY_SELFTEST, img.toPNG());
      console.log('YEAR:', await win.webContents.executeJavaScript("document.getElementById('year').textContent"));
      app.quit();
    });
  }
});
app.on('window-all-closed', () => { if (engine) { try { engine.stdin.write('{"cmd":"quit"}\n'); } catch (_) {} engine.kill(); } app.quit(); });
