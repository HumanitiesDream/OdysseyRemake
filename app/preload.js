const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld('odyssey', {
  send: obj => ipcRenderer.send('cmd', obj),
  onEvent: cb => ipcRenderer.on('engine', (_e, msg) => cb(msg)),
  readTerrain: () => ipcRenderer.invoke('terrain'),
  quit: () => ipcRenderer.send('quit')
});
