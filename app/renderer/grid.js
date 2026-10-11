// Hex grid builder. Runs inside a Web Worker (created from a Blob).
// Builds a Goldberg-style hex sphere: the dual of a subdivided icosahedron.
// Frequency n gives 10n^2 + 2 cells (12 of them pentagons).
// Longitude convention everywhere: lon = atan2(-z, x), lat = asin(y).

function gridWorkerMain() {
  const BIN_LAT = 512, BIN_LON = 1024;

  self.onmessage = (e) => {
    const { id, n, seed, centerFirst, plain } = e.data;
    try {
      const r = build(n, seed >>> 0, centerFirst, !!plain, (p, label) => self.postMessage({ type: 'progress', id, p, label }));
      self.postMessage({ type: 'done', id, grid: r },
        [r.mpos.buffer, r.col.buffer, r.edge.buffer, r.idx.buffer, r.centers.buffer, r.binStart.buffer, r.binCells.buffer, r.neighbors.buffer]);
    } catch (err) {
      self.postMessage({ type: 'error', id, message: String((err && err.stack) || err) });
    }
  };

  function hash3(x, y, z, s) {
    x |= 0; y |= 0; z |= 0; s |= 0;
    let h = Math.imul(x, 374761393) ^ Math.imul(y, 668265263) ^ Math.imul(z, 1440662683) ^ Math.imul(s, 1274126177);
    h = Math.imul(h ^ (h >>> 13), 1274126177);
    h ^= h >>> 16;
    return (h >>> 0) / 4294967296;
  }
  function vnoise(x, y, z, s) {
    const xi = Math.floor(x), yi = Math.floor(y), zi = Math.floor(z);
    let fx = x - xi, fy = y - yi, fz = z - zi;
    fx = fx * fx * (3 - 2 * fx); fy = fy * fy * (3 - 2 * fy); fz = fz * fz * (3 - 2 * fz);
    const c000 = hash3(xi, yi, zi, s), c100 = hash3(xi + 1, yi, zi, s);
    const c010 = hash3(xi, yi + 1, zi, s), c110 = hash3(xi + 1, yi + 1, zi, s);
    const c001 = hash3(xi, yi, zi + 1, s), c101 = hash3(xi + 1, yi, zi + 1, s);
    const c011 = hash3(xi, yi + 1, zi + 1, s), c111 = hash3(xi + 1, yi + 1, zi + 1, s);
    const x00 = c000 + (c100 - c000) * fx, x10 = c010 + (c110 - c010) * fx;
    const x01 = c001 + (c101 - c001) * fx, x11 = c011 + (c111 - c011) * fx;
    const y0 = x00 + (x10 - x00) * fy, y1 = x01 + (x11 - x01) * fy;
    return y0 + (y1 - y0) * fz;
  }
  function fbm(x, y, z, s) {
    let a = 0.5, f = 1.6, sum = 0, norm = 0;
    for (let o = 0; o < 5; o++) {
      sum += a * vnoise(x * f + 17.3 * o, y * f - 9.1 * o, z * f + 3.7 * o, s + o);
      norm += a; a *= 0.5; f *= 2.03;
    }
    return sum / norm;
  }

  function build(n, seed, centerFirst, plain, progress) {
    progress(0.02, 'Laying out the icosahedron');
    const t = (1 + Math.sqrt(5)) / 2;
    const IV = [[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0], [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t], [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]]
      .map(v => { const l = Math.hypot(v[0], v[1], v[2]); return [v[0] / l, v[1] / l, v[2] / l]; });
    const F = [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
      [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];

    const V = 10 * n * n + 2;
    const T = 20 * n * n;
    const edgeIndex = new Map();
    let ec = 0;
    for (const f of F) for (let k = 0; k < 3; k++) {
      const a = f[k], b = f[(k + 1) % 3];
      const key = Math.min(a, b) * 12 + Math.max(a, b);
      if (!edgeIndex.has(key)) edgeIndex.set(key, ec++);
    }
    const edgeBase = 12;
    const faceBase = 12 + 30 * (n - 1);
    const perFace = ((n - 1) * (n - 2)) / 2;
    const rowStart = new Int32Array(n + 1);
    for (let u = 1; u < n; u++) rowStart[u + 1] = rowStart[u] + (n - 1 - u);

    const ev = (x, y, s) => {
      const e = edgeIndex.get(Math.min(x, y) * 12 + Math.max(x, y));
      const start = edgeBase + e * (n - 1);
      return x < y ? start + (s - 1) : start + (n - 1 - s);
    };

    // Face orientation: flip if icosahedron face is not CCW seen from outside.
    const flip = F.map(([a, b, c]) => {
      const A = IV[a], B = IV[b], C = IV[c];
      const ux = B[0] - A[0], uy = B[1] - A[1], uz = B[2] - A[2];
      const vx = C[0] - A[0], vy = C[1] - A[1], vz = C[2] - A[2];
      const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      return nx * A[0] + ny * A[1] + nz * A[2] < 0;
    });

    const pos = new Float32Array(V * 3);
    const written = new Uint8Array(V);
    const tris = new Uint32Array(T * 3);
    let tc = 0;

    progress(0.06, 'Subdividing faces');
    for (let fi = 0; fi < 20; fi++) {
      const [a, b, c] = F[fi];
      const A = IV[a], B = IV[b], C = IV[c];
      const fb = faceBase + fi * perFace;
      const vid = (u, v) => {
        if (u === 0 && v === 0) return a;
        if (u === n) return b;
        if (v === n) return c;
        if (v === 0) return ev(a, b, u);
        if (u === 0) return ev(a, c, v);
        if (u + v === n) return ev(b, c, v);
        return fb + rowStart[u] + (v - 1);
      };
      // Positions. A mild spherical correction evens out cell sizes.
      for (let u = 0; u <= n; u++) {
        for (let v = 0; v <= n - u; v++) {
          const id = vid(u, v);
          if (written[id]) continue;
          written[id] = 1;
          let wa = (n - u - v) / n, wb = u / n, wc = v / n;
          // atan-warp barycentrics: evens cell sizes to within ~8%
          wa = Math.atan(wa); wb = Math.atan(wb); wc = Math.atan(wc);
          let x = A[0] * wa + B[0] * wb + C[0] * wc;
          let y = A[1] * wa + B[1] * wb + C[1] * wc;
          let z = A[2] * wa + B[2] * wb + C[2] * wc;
          const l = Math.hypot(x, y, z);
          pos[id * 3] = x / l; pos[id * 3 + 1] = y / l; pos[id * 3 + 2] = z / l;
        }
      }
      for (let u = 0; u < n; u++) {
        for (let v = 0; v < n - u; v++) {
          const p0 = vid(u, v), p1 = vid(u + 1, v), p2 = vid(u, v + 1);
          if (flip[fi]) { tris[tc++] = p0; tris[tc++] = p2; tris[tc++] = p1; }
          else { tris[tc++] = p0; tris[tc++] = p1; tris[tc++] = p2; }
          if (u + v < n - 1) {
            const p3 = vid(u + 1, v + 1);
            if (flip[fi]) { tris[tc++] = p1; tris[tc++] = p2; tris[tc++] = p3; }
            else { tris[tc++] = p1; tris[tc++] = p3; tris[tc++] = p2; }
          }
        }
      }
      progress(0.06 + 0.24 * (fi + 1) / 20, 'Subdividing faces');
    }

    progress(0.32, 'Tracing hex cells');
    const vt = new Int32Array(V * 6).fill(-1);
    const vc = new Uint8Array(V);
    for (let ti = 0; ti < T; ti++) {
      for (let k = 0; k < 3; k++) {
        const v = tris[ti * 3 + k];
        vt[v * 6 + vc[v]++] = ti;
      }
    }

    const NV = V + T;
    const mpos = new Float32Array(NV * 3);
    mpos.set(pos);
    for (let ti = 0; ti < T; ti++) {
      const a = tris[ti * 3] * 3, b = tris[ti * 3 + 1] * 3, c = tris[ti * 3 + 2] * 3;
      let x = pos[a] + pos[b] + pos[c], y = pos[a + 1] + pos[b + 1] + pos[c + 1], z = pos[a + 2] + pos[b + 2] + pos[c + 2];
      const l = Math.hypot(x, y, z);
      const o = (V + ti) * 3;
      mpos[o] = x / l; mpos[o + 1] = y / l; mpos[o + 2] = z / l;
    }
    progress(0.45, 'Tracing hex cells');

    const edge = new Uint8Array(NV);
    edge.fill(255, V);
    const idx = new Uint32Array(T * 9);
    const neighbors = new Int32Array(V * 6).fill(-1);
    const nc = new Uint8Array(V);
    let ic = 0;
    const step = Math.max(1, Math.floor(T / 20));
    for (let ti = 0; ti < T; ti++) {
      for (let k = 0; k < 3; k++) {
        const x = tris[ti * 3 + k], y = tris[ti * 3 + (k + 1) % 3];
        let t2 = -1;
        for (let j = 0; j < vc[x]; j++) {
          const cand = vt[x * 6 + j];
          if (cand === ti) continue;
          if (tris[cand * 3] === y || tris[cand * 3 + 1] === y || tris[cand * 3 + 2] === y) { t2 = cand; break; }
        }
        if (t2 < ti) continue;
        neighbors[x * 6 + nc[x]++] = y;
        neighbors[y * 6 + nc[y]++] = x;
        const C1 = V + ti, C2 = V + t2;
        if (centerFirst) {
          idx[ic++] = x; idx[ic++] = C2; idx[ic++] = C1;
          idx[ic++] = y; idx[ic++] = C1; idx[ic++] = C2;
        } else {
          idx[ic++] = C2; idx[ic++] = C1; idx[ic++] = x;
          idx[ic++] = C1; idx[ic++] = C2; idx[ic++] = y;
        }
      }
      if (ti % step === 0) progress(0.45 + 0.3 * ti / T, 'Tracing hex cells');
    }

    progress(0.76, 'Filling the oceans');
    const col = new Uint8Array(NV * 3);
    const depth = new Float32Array(V);
    for (let i = 0; i < V; i++) {
      const x = pos[i * 3], y = pos[i * 3 + 1], z = pos[i * 3 + 2];
      // (plain: a grid rebuilt for a lost GPU context is repainted at once, so it skips the noise)
      const d = plain ? 0.5 : fbm(x * 2.2, y * 2.2, z * 2.2, seed);
      depth[i] = d;
      const tt = Math.min(1, Math.max(0, (d - 0.3) / 0.45));
      const s = tt * tt * (3 - 2 * tt);
      const j = (hash3(i, 7, 13, seed) - 0.5) * 5;
      const lat = Math.abs(y);
      col[i * 3] = Math.max(0, Math.min(255, 9 + s * 12 + lat * 6 + j));
      col[i * 3 + 1] = Math.max(0, Math.min(255, 20 + s * 22 + lat * 8 + j));
      col[i * 3 + 2] = Math.max(0, Math.min(255, 46 + s * 38 + lat * 10 + j));
      if (i % 200000 === 0) progress(0.76 + 0.14 * i / V, 'Filling the oceans');
    }

    progress(0.9, 'Indexing tiles');
    const bins = BIN_LAT * BIN_LON;
    const binStart = new Uint32Array(bins + 1);
    const cellBin = new Uint32Array(V);
    for (let i = 0; i < V; i++) {
      const x = pos[i * 3], y = pos[i * 3 + 1], z = pos[i * 3 + 2];
      const lat = Math.asin(Math.max(-1, Math.min(1, y)));
      const lon = Math.atan2(-z, x);
      const bi = Math.min(BIN_LAT - 1, Math.floor((lat + Math.PI / 2) / Math.PI * BIN_LAT));
      const bj = Math.min(BIN_LON - 1, Math.floor((lon + Math.PI) / (2 * Math.PI) * BIN_LON));
      const b = bi * BIN_LON + bj;
      cellBin[i] = b;
      binStart[b + 1]++;
    }
    for (let b = 0; b < bins; b++) binStart[b + 1] += binStart[b];
    const fill = binStart.slice(0, bins);
    const binCells = new Uint32Array(V);
    for (let i = 0; i < V; i++) binCells[fill[cellBin[i]]++] = i;

    const centers = pos;
    progress(0.95, 'Handing the world to the GPU');
    return { n, V, T, mpos, col, edge, idx, centers, binStart, binCells, neighbors, BIN_LAT, BIN_LON, depthMin: 0, depthMax: 1 };
  }
}
