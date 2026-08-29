const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
if (!base) throw new Error('Set LIVE_URL to the deployed product URL.');

const ip = process.env.RATE_LIMIT_IP || '198.51.100.77';
const workspaces = [];
const responses = await Promise.all(Array.from({ length: 16 }, () => fetch(`${base}/api/demo`, {
    method: 'POST', headers: { 'x-forwarded-for': ip },
  })));
try {
  for (const response of responses.filter(response => response.status === 201)) {
    workspaces.push((await response.json()).workspace);
  }
  const limited = responses.filter(response => response.status === 429);
  if (workspaces.length !== 15 || limited.length !== 1 || limited[0].headers.get('retry-after') !== '1') {
    throw new Error(`expected 15 writes then a 16th 429 with Retry-After: 1, got ${responses.map(response => response.status).join(', ')}`);
  }
  console.log('15 concurrent writes succeeded; the 16th admission returned 429 with Retry-After: 1.');
} finally {
  // Avoid leaving sample workspaces behind. A fresh address keeps cleanup
  // outside the asserted allowance, and each workspace is isolated demo data.
  for (const [index, workspace] of workspaces.entries()) {
    const response = await fetch(`${base}/api/demo/${workspace}`, {
      method: 'DELETE', headers: { 'x-forwarded-for': `198.51.200.${index + 1}` },
    });
    if (response.status !== 204) throw new Error(`cleanup ${index + 1}: expected 204, got ${response.status}`);
  }
}
console.log('All sample workspaces were removed.');
