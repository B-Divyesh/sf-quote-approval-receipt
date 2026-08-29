import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
const testSource = await readFile('tests/product.spec.ts', 'utf8');
const packageJson = JSON.parse(await readFile('package.json', 'utf8'));
const readme = await readFile('README.md', 'utf8');
const deploymentGuide = await readFile('.factory/deployment.md', 'utf8');
const ids = new Set();

for (const claim of claims) {
  assert.match(claim.id, /^[a-z0-9]+(?:-[a-z0-9]+)*$/, `invalid claim id: ${claim.id}`);
  assert.ok(!ids.has(claim.id), `duplicate claim id: ${claim.id}`);
  ids.add(claim.id);
  assert.ok(claim.claim && claim.where && claim.test && claim.sandbox, `${claim.id} is incomplete`);
  if (claim.test.startsWith('npm test -- --grep ')) {
    const tag = `@claim:${claim.id}`;
    assert.equal(claim.test, `npm test -- --grep ${tag}`);
    assert.equal(testSource.split(tag).length - 1, 1, `${tag} must occur in exactly one test`);
  } else {
    const script = claim.test.replace(/^npm run /, '');
    assert.ok(packageJson.scripts[script], `${claim.id} references missing script ${script}`);
    const sourcePath = packageJson.scripts[script].match(/node\s+(\S+)/)?.[1];
    assert.ok(sourcePath, `${claim.id} script must name its observable Node test`);
    const standaloneSource = await readFile(sourcePath, 'utf8');
    const tag = `@claim:${claim.id}`;
    assert.equal(standaloneSource.split(tag).length - 1, 1, `${tag} must occur in exactly one standalone test`);
  }
}

for (const tag of testSource.matchAll(/@claim:([a-z0-9-]+)/g)) {
  assert.ok(ids.has(tag[1]), `test tag has no manifest entry: ${tag[1]}`);
}

// F-4-1: a one-replica topology is a storage contract, not evidence that a
// deployed process-wide write allowance is globally shared. Keep that
// untestable production promise out of public and operator-facing copy.
assert.doesNotMatch(readme, /limit global for the live service/i);
assert.doesNotMatch(deploymentGuide, /limiter global for the live service/i);

console.log(`claims manifest maps ${claims.length} unique claims to one observable test each`);
