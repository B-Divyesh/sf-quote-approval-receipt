import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const app = process.env.CONTAINER_APP || 'sf-quote-approval-receipt';
const group = process.env.RESOURCE_GROUP || 'sociobot';
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
console.log(`${app} has the required one-replica mounted durable-state contract.`);
