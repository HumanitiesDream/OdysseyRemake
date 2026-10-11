const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld('owHost', {
  send: (obj) => ipcRenderer.send('cmd', obj),
  onEvent: (cb) => ipcRenderer.on('engine', (_e, msg) => cb(msg)),
  readData: (name) => ipcRenderer.invoke('data:read', name),
  listSaves: () => ipcRenderer.invoke('saves:list'),
  readSave: (file) => ipcRenderer.invoke('saves:read', file),
  saveExists: (file) => ipcRenderer.invoke('saves:exists', file),
  writeSave: (file, data) => ipcRenderer.invoke('saves:write', file, data),
  deleteSave: (file) => ipcRenderer.invoke('saves:delete', file),
  openSavesFolder: () => ipcRenderer.invoke('saves:open'),
  getSettings: () => ipcRenderer.invoke('settings:get'),
  setSettings: (s) => ipcRenderer.invoke('settings:set', s),
  toggleFullscreen: () => ipcRenderer.invoke('win:fullscreen'),
  setFullscreen: (on) => ipcRenderer.invoke('win:setFullscreen', on),
  quit: () => ipcRenderer.invoke('app:quit'),
  isDesktop: true
});
