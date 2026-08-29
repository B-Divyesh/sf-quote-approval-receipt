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

// The app-level template is only desired state. Verification 9 found two
// serving replicas, so bind the check to the one active revision and its
// actual running replica instead of accepting the template alone.
const revisions = JSON.parse(execFileSync('az', [
  'containerapp', 'revision', 'list', '--name', app, '--resource-group', group, '--output', 'json',
], { encoding: 'utf8' })).filter(revision => revision.properties.active);
assert.equal(revisions.length, 1, 'exactly one revision may actively serve this single-writer app');
const [revision] = revisions;
assert.equal(revision.name, actual.properties.latestReadyRevisionName, 'the latest ready revision must be the active revision');
assert.equal(revision.properties.trafficWeight, 100, 'the durable revision must receive all traffic');
assert.equal(revision.properties.replicas, 1, 'exactly one replica may serve SQLite state and rate-limit buckets');
const revisionTemplate = revision.properties.template;
const revisionContainer = revisionTemplate.containers.find(item => item.name === 'app');
assert.deepEqual(revisionTemplate.scale.minReplicas, 1);
assert.deepEqual(revisionTemplate.scale.maxReplicas, 1);
assert.ok(revisionContainer.env.some(item => item.name === 'DURABLE_DATA_DIR' && item.value === '/durable'));
assert.ok(revisionContainer.volumeMounts?.some(item => item.volumeName === 'durable' && item.mountPath === '/durable'));
assert.ok(revisionTemplate.volumes?.some(item => item.name === 'durable' && item.storageName === 'quote-approval-receipt-data'));
const replicas = JSON.parse(execFileSync('az', [
  'containerapp', 'replica', 'list', '--name', app, '--resource-group', group,
  '--revision', revision.name, '--output', 'json',
], { encoding: 'utf8' }));
assert.equal(replicas.length, 1, 'Azure must report one running replica for the active revision');
assert.equal(replicas[0].properties.containers.find(item => item.name === 'app')?.ready, true,
  'the sole app container must be ready');

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
console.log(`${app} has one active revision, one ready replica, the mounted durable-state contract, and the public durable health signal.`);
