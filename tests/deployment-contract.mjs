import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import {
  assertDeploymentContract,
  DEPLOYMENT_FILE,
  renderDeploymentTemplate,
} from '../scripts/deployment-contract.mjs';

const deployment = JSON.parse(await readFile(DEPLOYMENT_FILE, 'utf8'));
const candidateImage = 'sociobotregistry.azurecr.io/sf-quote-approval-receipt:0123456789ab';
const rendered = renderDeploymentTemplate(deployment, candidateImage, '8080');
const app = assertDeploymentContract(rendered);

assert.equal(app.image, candidateImage,
  'the current candidate image must replace the manifest example tag');
assert.deepEqual(rendered.scale, { minReplicas: 1, maxReplicas: 1 });
assert.deepEqual(app.env, [
  { name: 'PORT', value: '8080' },
  { name: 'DURABLE_DATA_DIR', value: '/durable' },
]);
assert.deepEqual(app.volumeMounts, [{ volumeName: 'durable', mountPath: '/durable' }]);
assert.deepEqual(rendered.volumes, [
  { name: 'durable', storageType: 'AzureFile', storageName: 'quote-approval-receipt-data' },
]);

for (const mutate of [
  template => { template.scale.maxReplicas = 3; },
  template => { template.containers[0].env = template.containers[0].env.filter(item => item.name !== 'DURABLE_DATA_DIR'); },
  template => { template.containers[0].volumeMounts = []; },
  template => { template.volumes = []; },
]) {
  const unsafe = structuredClone(rendered);
  mutate(unsafe);
  assert.throws(() => assertDeploymentContract(unsafe),
    'the release renderer must reject every topology from verification 11');
}

console.log('@regression:deployment-overlay current images retain one writer, /durable, and Azure Files; verifier-11 topology is rejected');
