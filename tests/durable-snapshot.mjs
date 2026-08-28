import assert from 'node:assert/strict';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';

const binary = join(resolve('.'), 'target', 'release', 'quote-approval-receipt');
const root = await mkdtemp(join(tmpdir(), 'quote-durable-'));
const durable = join(root, 'durable');

async function start(dataDir) {
  const port = 28_000 + Math.floor(Math.random() * 3_000);
  const child = spawn(binary, [], {
    cwd: root,
    env: { PORT: String(port), DATA_DIR: dataDir, DURABLE_DATA_DIR: durable },
    stdio: 'ignore',
  });
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try { if ((await fetch(`http://127.0.0.1:${port}/health`)).ok) return { child, port }; }
    catch { /* wait for the server */ }
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  child.kill('SIGKILL');
  throw new Error('durable test server did not start');
}

async function stop(child) {
  child.kill('SIGTERM');
  await new Promise(resolve => child.once('exit', resolve));
}

try {
  const first = await start(join(root, 'local-one'));
  const created = await (await fetch(`http://127.0.0.1:${first.port}/api/demo`, { method: 'POST' })).json();
  assert.equal(created.quote.public_token.length, 32);
  await stop(first.child);

  const second = await start(join(root, 'local-two'));
  const response = await fetch(`http://127.0.0.1:${second.port}/api/share/${created.quote.public_token}`);
  assert.equal(response.status, 200, 'a new process restores the committed durable snapshot');
  assert.equal((await response.json()).demo, true);
  await stop(second.child);
  console.log('@claim:durable-snapshot a committed record survives a fresh process through the durable state directory');
} finally {
  await rm(root, { recursive: true, force: true });
}
