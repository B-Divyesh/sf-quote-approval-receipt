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
  const tokens = [];
  for (let attempt = 0; attempt < 20; attempt += 1) {
    const create = await fetch(`http://127.0.0.1:${first.port}/api/demo`, {
      method: 'POST',
      headers: { 'x-forwarded-for': `198.51.100.${attempt + 1}` },
    });
    assert.equal(create.status, 201, `demo ${attempt + 1} is committed before replacement`);
    const created = await create.json();
    assert.equal(created.quote.public_token.length, 32);
    tokens.push(created.quote.public_token);
  }
  await stop(first.child);

  const second = await start(join(root, 'local-two'));
  for (const [index, token] of tokens.entries()) {
    const response = await fetch(`http://127.0.0.1:${second.port}/api/share/${token}`, {
      headers: { 'x-forwarded-for': `198.51.101.${index + 1}` },
    });
    assert.equal(response.status, 200, `demo ${index + 1} is restored after replacement`);
    assert.equal((await response.json()).demo, true);
  }
  await stop(second.child);
  console.log('@claim:durable-snapshot 20 committed records survive a replacement process through the durable state directory');
} finally {
  await rm(root, { recursive: true, force: true });
}
