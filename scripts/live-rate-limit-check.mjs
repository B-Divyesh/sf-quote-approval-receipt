const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
if (!base) throw new Error('Set LIVE_URL to the deployed product URL.');

const suffix = (Date.now() % 240) + 10;
const readIp = process.env.READ_RATE_LIMIT_IP || `198.51.100.${suffix}`;
const writeIp = process.env.WRITE_RATE_LIMIT_IP || `203.0.113.${suffix}`;

const reads = await Promise.all(Array.from({ length: 41 }, (_, index) => fetch(
  `${base}/api/share/rateLimitProbeMissingToken${String(index).padStart(2, '0')}`,
  { headers: { 'x-forwarded-for': readIp } },
)));
const acceptedReads = reads.filter(response => response.status === 404);
const limitedReads = reads.filter(response => response.status === 429);
if (acceptedReads.length !== 40 || limitedReads.length !== 1
    || limitedReads[0].headers.get('retry-after') !== '1') {
  throw new Error(`expected 40 read admissions and one 429 with Retry-After: 1, got ${reads.map(response => response.status).join(', ')}`);
}
console.log('40 concurrent reads reached the app; the 41st admission returned 429 with Retry-After: 1.');

// Invalid writes exercise the write bucket without creating records that
// would need cleanup if this check exposes a replica split.
const writes = await Promise.all(Array.from({ length: 16 }, () => fetch(`${base}/api/quotes`, {
  method: 'POST',
  headers: { 'content-type': 'application/json', 'x-forwarded-for': writeIp },
  body: '{}',
})));
const acceptedWrites = writes.filter(response => response.status === 422);
const limitedWrites = writes.filter(response => response.status === 429);
if (acceptedWrites.length !== 15 || limitedWrites.length !== 1
    || limitedWrites[0].headers.get('retry-after') !== '1') {
  throw new Error(`expected 15 write admissions and one 429 with Retry-After: 1, got ${writes.map(response => response.status).join(', ')}`);
}
console.log('15 concurrent invalid writes reached validation; the 16th admission returned 429 with Retry-After: 1.');
