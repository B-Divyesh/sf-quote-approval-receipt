import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const app = process.env.CONTAINER_APP || 'sf-quote-approval-receipt';
const group = process.env.RESOURCE_GROUP || 'sociobot';
const base = (process.env.LIVE_URL || 'https://quote-approval-receipt.sociobot.in').replace(/\/$/, '');
const expectedBuild = process.env.EXPECTED_BUILD_SHA || '';
const actual = JSON.parse(execFileSync('az', [
  'containerapp', 'show', '--name', app, '--resource-group', group, '--output', 'json',
], { encoding: 'utf8' }));
const template = actual.properties.template;
const container = template.containers.find(item => item.name === 'app');
assert.deepEqual(template.scale.minReplicas, 1, 'durable SQLite release requires one minimum replica');
assert.deepEqual(template.scale.maxReplicas, 1, 'durable SQLite release must never split state or the limiter');
assert.ok(container.env.some(item => item.name === 'DURABLE_DATA_DIR' && item.value === '/durable'));
assert.ok(container.volumeMounts?.some(item => item.volumeName === 'durable' && item.mountPath === '/durable'));
assert.ok(template.volumes?.some(item => item.name === 'durable' && item.storageName === 'quote-approval-receipt-data'));

// The prior release had a seemingly healthy service while Azure silently
// omitted this whole durable topology. Verify the public, process-level signal
// too: it catches an unset mount or environment variable that a manifest-only
// assertion cannot see.
const health = await fetch(`${base}/health`);
assert.equal(health.status, 200, 'the active revision must answer /health');
const body = await health.json();
assert.equal(body.status, 'ok');
assert.equal(body.durable_snapshot, true, 'the active revision must report the mounted durable snapshot mode');
if (expectedBuild) {
  assert.equal(body.build_sha, expectedBuild, 'the checked topology must be the requested deployment');
}
console.log(`${app} has the required one-replica mounted durable-state contract and public durable health signal.`);
