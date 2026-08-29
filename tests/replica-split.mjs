import assert from 'node:assert/strict';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';

const binary = join(resolve('.'), 'target', 'release', 'quote-approval-receipt');
const root = await mkdtemp(join(tmpdir(), 'quote-replica-split-'));

async function start(name) {
  const port = 31_000 + Math.floor(Math.random() * 2_000);
  const child = spawn(binary, [], {
    cwd: root,
    // Deliberately model the failed production release: separate local SQLite
    // directories and no mounted DURABLE_DATA_DIR.
    env: { PORT: String(port), DATA_DIR: join(root, name) },
    stdio: 'ignore',
  });
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try {
      if ((await fetch(`http://127.0.0.1:${port}/health`)).ok) return { child, port };
    } catch { /* wait for startup */ }
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  child.kill('SIGKILL');
  throw new Error(`replica ${name} did not start`);
}

async function stop(child) {
  child.kill('SIGTERM');
  await new Promise(resolve => child.once('exit', resolve));
}

let first;
let second;
try {
  first = await start('replica-one');
  second = await start('replica-two');
  const create = await fetch(`http://127.0.0.1:${first.port}/api/demo`, { method: 'POST' });
  assert.equal(create.status, 201, 'the first replica accepts the demo write');
  const { quote } = await create.json();
  const readElsewhere = await fetch(`http://127.0.0.1:${second.port}/api/share/${quote.public_token}`);
  assert.equal(readElsewhere.status, 404,
    'separate replica-local SQLite files reproduce the failed 201 then 404 production path');

  // Model a load balancer sending one client's requests evenly to both
  // replicas. Each process-local bucket admits a full allowance.
  const readIp = '198.51.100.240';
  const readResponses = [];
  for (let index = 0; index < 40; index += 1) {
    readResponses.push(await fetch(`http://127.0.0.1:${first.port}/api/share/missing`, {
      headers: { 'x-forwarded-for': readIp },
    }));
    readResponses.push(await fetch(`http://127.0.0.1:${second.port}/api/share/missing`, {
      headers: { 'x-forwarded-for': readIp },
    }));
  }
  assert.equal(readResponses.filter(response => response.status === 404).length, 80,
    'two local buckets incorrectly admit 80 reads for one client');
  for (const replica of [first, second]) {
    const limited = await fetch(`http://127.0.0.1:${replica.port}/api/share/missing`, {
      headers: { 'x-forwarded-for': readIp },
    });
    assert.equal(limited.status, 429);
    assert.equal(limited.headers.get('retry-after'), '1');
  }

  const writeIp = '203.0.113.240';
  const invalidBody = JSON.stringify({});
  const writeResponses = [];
  for (let index = 0; index < 15; index += 1) {
    for (const replica of [first, second]) {
      writeResponses.push(await fetch(`http://127.0.0.1:${replica.port}/api/quotes`, {
        method: 'POST',
        headers: { 'content-type': 'application/json', 'x-forwarded-for': writeIp },
        body: invalidBody,
      }));
    }
  }
  assert.equal(writeResponses.filter(response => response.status === 422).length, 30,
    'two local buckets incorrectly admit 30 writes for one client');
  for (const replica of [first, second]) {
    const limited = await fetch(`http://127.0.0.1:${replica.port}/api/quotes`, {
      method: 'POST',
      headers: { 'content-type': 'application/json', 'x-forwarded-for': writeIp },
      body: invalidBody,
    });
    assert.equal(limited.status, 429);
    assert.equal(limited.headers.get('retry-after'), '1');
  }

  console.log('@regression:replica-local-split unsafe two-replica local state reproduces create 201/read 404 and doubles allowances to 80 reads/30 writes');
} finally {
  if (first) await stop(first.child);
  if (second) await stop(second.child);
  await rm(root, { recursive: true, force: true });
}
