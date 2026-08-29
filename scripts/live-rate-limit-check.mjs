const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
if (!base) throw new Error('Set LIVE_URL to the deployed product URL.');

const ip = process.env.RATE_LIMIT_IP || '198.51.100.77';
const workspaces = [];
for (let request = 1; request <= 16; request += 1) {
  const response = await fetch(`${base}/api/demo`, {
    method: 'POST', headers: { 'x-forwarded-for': ip },
  });
  if (request <= 15) {
    if (response.status !== 201) throw new Error(`request ${request}: expected 201, got ${response.status}`);
    workspaces.push((await response.json()).workspace);
  } else {
    if (response.status !== 429 || response.headers.get('retry-after') !== '1') {
      throw new Error(`request 16: expected 429 with Retry-After: 1, got ${response.status} / ${response.headers.get('retry-after')}`);
    }
  }
}

// Avoid leaving sample workspaces behind. A fresh address keeps this cleanup
// outside the asserted allowance, and each workspace is isolated demo data.
for (const [index, workspace] of workspaces.entries()) {
  const response = await fetch(`${base}/api/demo/${workspace}`, {
    method: 'DELETE', headers: { 'x-forwarded-for': `198.51.200.${index + 1}` },
  });
  if (response.status !== 204) throw new Error(`cleanup ${index + 1}: expected 204, got ${response.status}`);
}
console.log('15 writes succeeded; request 16 returned 429 with Retry-After: 1; all sample workspaces were removed.');
