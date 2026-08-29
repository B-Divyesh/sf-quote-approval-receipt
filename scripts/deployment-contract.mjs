import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

export const DEPLOYMENT_FILE = '.factory/containerapp-deploy.json';

export function assertDeploymentContract(template) {
  assert.deepEqual(template.scale?.minReplicas, 1,
    'durable SQLite releases require one minimum replica');
  assert.deepEqual(template.scale?.maxReplicas, 1,
    'durable SQLite releases must never permit a second writer');

  const app = template.containers?.find(container => container.name === 'app');
  assert.ok(app, 'deployment template must contain the app container');
  assert.ok(app.env?.some(item => item.name === 'DURABLE_DATA_DIR' && item.value === '/durable'),
    'app must snapshot committed state to /durable');
  assert.ok(app.volumeMounts?.some(item => item.volumeName === 'durable' && item.mountPath === '/durable'),
    'app must mount the durable volume at /durable');
  assert.ok(template.volumes?.some(item => item.name === 'durable'
    && item.storageType === 'AzureFile'
    && item.storageName === 'quote-approval-receipt-data'),
  'deployment must use the product Azure Files storage');
  return app;
}

export function renderDeploymentTemplate(deployment, image, port = '8080') {
  assert.match(image, /^sociobotregistry\.azurecr\.io\/sf-quote-approval-receipt:[A-Za-z0-9._-]+$/,
    'release image must come from the product ACR repository');
  assert.match(String(port), /^\d{2,5}$/, 'PORT must be numeric');

  const template = structuredClone(deployment.properties?.template);
  const app = assertDeploymentContract(template);
  app.image = image;
  const portEntry = app.env.find(item => item.name === 'PORT');
  if (portEntry) portEntry.value = String(port);
  else app.env.unshift({ name: 'PORT', value: String(port) });
  assertDeploymentContract(template);
  return template;
}

async function main() {
  const [image, port = '8080'] = process.argv.slice(2);
  if (!image) throw new Error('Usage: node scripts/deployment-contract.mjs <image> [port]');
  const deployment = JSON.parse(await readFile(DEPLOYMENT_FILE, 'utf8'));
  process.stdout.write(`${JSON.stringify(renderDeploymentTemplate(deployment, image, port))}\n`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  await main();
}
