import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';

const root = resolve('.');
const binary = join(root, 'target', 'release', 'quote-approval-receipt');
const workdir = await mkdtemp(join(tmpdir(), 'quote-runtime-'));
const port = 18_000 + Math.floor(Math.random() * 10_000);
const child = spawn(binary, [], { cwd: workdir, env: { PORT: String(port) }, stdio: 'ignore' });

async function waitForHealth() {
  let last;
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try {
      const response = await fetch(`http://127.0.0.1:${port}/health`);
      if (response.ok) return response.json();
      last = new Error(`health returned ${response.status}`);
    } catch (error) { last = error; }
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw last;
}

try {
  const health = await waitForHealth();
  assert.equal(health.status, 'ok');
  assert.equal(typeof health.build_sha, 'string');
  assert.ok(health.build_sha.length > 0);
  const salt = await readFile(join(workdir, 'data', 'privacy_salt'));
  assert.ok(salt.length >= 32, 'first boot creates a random privacy salt');
  console.log('@claim:runtime-contract only PORT starts the service; health exposes build identity; first boot creates its salt');
} finally {
  child.kill('SIGTERM');
  await new Promise(resolve => child.once('exit', resolve));
  await rm(workdir, { recursive: true, force: true });
}

const dockerfile = await readFile(join(root, 'Dockerfile'), 'utf8');
assert.match(dockerfile, /^FROM rust:1-slim AS backend$/m);
assert.match(dockerfile, /^USER app$/m);
assert.match(dockerfile, /^ENV PORT=8080 DATA_DIR=\/data STATIC_DIR=\/app\/dist$/m);
