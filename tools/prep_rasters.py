#!/usr/bin/env python3
"""Turn the raw downloads into plain float32 rasters the Rust baker reads.

Inputs (in RAW dir): ETOPO1_Bed_g_gmt4.grd, wc2.1_10m_*.zip (unzipped to wc/), ersst.nc, gpcp_ltm.nc
      ncep_air.nc (NCEP/NCAR long-term mean surface air temperature), ETOPO1_Ice_g_gmt4.grd
Outputs (in OUT dir), all row-major, north to south, longitude from -180 east, pixel-centred:
  topo3.f32     3 arc-minute bed elevation, m (ETOPO1, 3x3 block mean)      7200 x 3600
  wc_elev.f32   WorldClim elevation, m, NaN off land                         2160 x 1080
  wc_tavg.f32   12 x WorldClim monthly mean temperature, degC, NaN off land
  wc_prec.f32   12 x WorldClim monthly precipitation, mm, NaN off land
  sst.f32       12 x ERSST v5 climatology 1991-2020, degC                    180 x 89  (lat 88 .. -88)
  ncep.f32      12 x NCEP/NCAR surface air temperature, degC                 144 x 73  (lat 90 .. -90, lon from -180)
  ice3.f32      3 arc-minute ice thickness, m = max(ice surface - bed, 0)     7200 x 3600
  gpcp.f32      12 x GPCP climatology 1991-2020, mm/month                    144 x 72  (lat 88.75 .. -88.75)
A small text file raster_meta.txt records sizes.
"""
import sys, os, numpy as np
from netCDF4 import Dataset
import tifffile

raw, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)

def save(name, a):
    np.asarray(a, dtype='<f4').tofile(os.path.join(out, name))

# ETOPO1, grid registered: 21601 x 10801, lat -90..90 (south to north), lon -180..180
nc = Dataset(os.path.join(raw, 'ETOPO1_Bed_g_gmt4.grd'))
z = nc.variables['z']
H, W = 10800, 21600
topo3 = np.zeros((H // 3, W // 3), dtype=np.float32)
step = 300  # rows per strip
for r0 in range(0, H, step):
    strip = np.array(z[r0:r0 + step, :W], dtype=np.float32)       # south to north
    strip = strip.reshape(step // 3, 3, W // 3, 3).mean(axis=(1, 3))
    topo3[r0 // 3:(r0 + step) // 3] = strip
topo3 = topo3[::-1]  # north to south
save('topo3.f32', topo3)
print('topo3', topo3.shape, float(topo3.min()), float(topo3.max()))

def wc(name):
    a = tifffile.imread(os.path.join(raw, 'wc', name)).astype(np.float32)
    a[(a < -1e4) | (a > 1e5)] = np.nan   # nodata is -3.4e38 or -32768
    return a
elev = wc('wc2.1_10m_elev.tif'); save('wc_elev.f32', elev)
t = np.stack([wc(f'wc2.1_10m_tavg_{m:02d}.tif') for m in range(1, 13)]); save('wc_tavg.f32', t)
p = np.stack([wc(f'wc2.1_10m_prec_{m:02d}.tif') for m in range(1, 13)]); save('wc_prec.f32', p)
print('wc', elev.shape, np.nanmean(t), np.nanmean(p) * 12)

d = Dataset(os.path.join(raw, 'ersst.nc'))
sst = d.variables['sst']; time = d.variables['time']
import cftime
dates = cftime.num2date(time[:], time.units)
idx = [i for i, x in enumerate(dates) if 1991 <= x.year <= 2020]
months = np.array([dates[i].month for i in idx])
arr = np.ma.filled(sst[idx], np.nan).astype(np.float32)
clim = np.stack([np.nanmean(arr[months == m], axis=0) for m in range(1, 13)])
lat = d.variables['lat'][:]
if lat[0] < lat[-1]: clim = clim[:, ::-1, :]
lon = d.variables['lon'][:]
# ERSST lon 0..358 -> shift to start at -180
shift = int(np.argmax(lon >= 180))
clim = np.roll(clim, -shift, axis=2)
save('sst.f32', clim)
print('sst', clim.shape, np.nanmean(clim))

g = Dataset(os.path.join(raw, 'gpcp_ltm.nc'))
pr = np.ma.filled(g.variables['precip'][:], np.nan).astype(np.float32)  # mm/day
lat = g.variables['lat'][:]; lon = g.variables['lon'][:]
if lat[0] < lat[-1]: pr = pr[:, ::-1, :]
shift = int(np.argmax(lon >= 180))
pr = np.roll(pr, -shift, axis=2)
days = np.array([31, 28.25, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], dtype=np.float32)
pr = pr * days[:pr.shape[0], None, None]
save('gpcp.f32', pr)
print('gpcp', pr.shape, np.nanmean(pr) * 12 / 12)
n = Dataset(os.path.join(raw, 'ncep_air.nc'))
air = np.ma.filled(n.variables['air'][:], np.nan).astype(np.float32)
lon = n.variables['lon'][:]
shift = int(np.argmax(lon >= 180))
air = np.roll(air, -shift, axis=2)
save('ncep.f32', air)
print('ncep', air.shape, np.nanmean(air))

icefile = os.path.join(raw, 'ETOPO1_Ice_g_gmt4.grd')
if os.path.exists(icefile):
    zi = Dataset(icefile).variables['z']
    ice3 = np.zeros((H // 3, W // 3), dtype=np.float32)
    for r0 in range(0, H, step):
        ice_s = np.array(zi[r0:r0 + step, :W], dtype=np.float32)
        bed_s = np.array(z[r0:r0 + step, :W], dtype=np.float32)
        # grounded ice: on land, or in a marine basin where the surface stands high (shelves sit below ~150 m and are left out)
        grounded = (bed_s > -50) | (ice_s > 150)
        thick = np.where(grounded, np.maximum(ice_s - bed_s, 0), 0)
        thick = np.where(thick < 20, 0, thick)                            # cut the noise between the two grids
        ice3[r0 // 3:(r0 + step) // 3] = thick.reshape(step // 3, 3, W // 3, 3).mean(axis=(1, 3))
    ice3 = ice3[::-1]
    save('ice3.f32', ice3)
    print('ice3 max', float(ice3.max()), 'volume km3', float(ice3.sum()) * (0.05 * 111.2) ** 2 * 0.7)
open(os.path.join(out, 'raster_meta.txt'), 'w').write(
    f"topo3 7200 3600\nice3 7200 3600\nncep 144 73\nwc 2160 1080\nsst {clim.shape[2]} {clim.shape[1]}\ngpcp {pr.shape[2]} {pr.shape[1]}\n")
