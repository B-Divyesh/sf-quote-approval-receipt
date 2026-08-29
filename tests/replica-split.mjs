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
  console.log('@regression:replica-local-split unsafe two-replica local SQLite reproduces create 201 then read 404');
} finally {
  if (first) await stop(first.child);
  if (second) await stop(second.child);
  await rm(root, { recursive: true, force: true });
}
