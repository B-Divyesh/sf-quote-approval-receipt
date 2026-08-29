import assert from 'node:assert/strict';
import { chromium } from '@playwright/test';

const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
const expectedBuild = process.env.EXPECTED_BUILD_SHA || '';
if (!base) throw new Error('Set LIVE_URL to the deployed product URL.');

const health = await fetch(`${base}/health`);
assert.equal(health.status, 200, 'the live service must answer health');
const healthBody = await health.json();
assert.equal(healthBody.durable_snapshot, true, 'the live workflow requires durable snapshot mode');
if (expectedBuild) assert.equal(healthBody.build_sha, expectedBuild, 'the workflow must exercise the requested build');

const browser = await chromium.launch({ headless: true });
const sender = await browser.newContext();
const client = await browser.newContext();
let created;

try {
  const create = await sender.request.post(`${base}/api/quotes`, {
    headers: { 'x-forwarded-for': '198.51.100.231' },
    data: {
      creator_name: 'Samira Noor',
      business_name: 'Northline Joinery',
      quote_number: 'LIVE-REPAIR-7',
      client_name: 'Elm Street Books',
      currency: 'EUR',
      summary: 'Build and install two oak display tables.',
      items: [
        { description: 'Oak display table', quantity: 2, rate: 1250 },
        { description: 'Delivery and installation', quantity: 1, rate: 350 },
      ],
      tax_percent: 20,
      consent_text: 'I confirm that I can decide for the client named above.',
      retention_days: 30,
    },
  });
  assert.equal(create.status(), 201, 'a sender must be able to create a real quote');
  created = await create.json();
  assert.match(created.public_token, /^[A-Za-z0-9]{32}$/);
  assert.match(created.owner_token, /^[A-Za-z0-9]{40}$/);

  const clientRead = await client.request.get(`${base}/api/share/${created.public_token}`, {
    headers: { 'x-forwarded-for': '198.51.100.232' },
  });
  assert.equal(clientRead.status(), 200, 'a clean client context must read the sender-created quote');
  assert.deepEqual((await clientRead.json()).items, [
    { description: 'Oak display table', quantity: 2, rate: 1250 },
    { description: 'Delivery and installation', quantity: 1, rate: 350 },
  ]);

  const decision = await client.request.post(`${base}/api/share/${created.public_token}/decision`, {
    headers: { 'x-forwarded-for': '198.51.100.232' },
    data: {
      name: 'Leonie Weber',
      title: 'Shop owner',
      email: 'leonie@example.test',
      decision: 'approved',
      note: 'Approved for the September installation.',
      consent: true,
    },
  });
  assert.equal(decision.status(), 201, 'the clean client context must record its decision');
  const decided = await decision.json();
  assert.equal(decided.quote.total, 3420);

  const ownerHeaders = {
    'x-forwarded-for': '198.51.100.231',
    'x-owner-token': created.owner_token,
  };
  const senderRead = await sender.request.get(`${base}/api/quotes/${created.id}`, { headers: ownerHeaders });
  assert.equal(senderRead.status(), 200, 'the sender context must read the client decision');
  assert.equal((await senderRead.json()).receipt_id, decided.receipt.id);
  const receipt = await sender.request.get(`${base}/api/receipts/${decided.receipt.id}`, {
    headers: { 'x-forwarded-for': '198.51.100.231' },
  });
  assert.equal(receipt.status(), 200, 'the sender must be able to retrieve the decision receipt');

  const deleted = await sender.request.delete(`${base}/api/quotes/${created.id}`, { headers: ownerHeaders });
  assert.equal(deleted.status(), 204, 'the sender must be able to delete the record');
  const afterDelete = await client.request.get(`${base}/api/share/${created.public_token}`, {
    headers: { 'x-forwarded-for': '198.51.100.232' },
  });
  assert.equal(afterDelete.status(), 404, 'the deleted approval link must be gone in the clean client context');
  created = undefined;
  console.log(`Cross-context create, read, decision, owner read, receipt, and delete passed on build ${healthBody.build_sha}.`);
} finally {
  if (created) {
    await sender.request.delete(`${base}/api/quotes/${created.id}`, {
      headers: {
        'x-forwarded-for': '198.51.100.233',
        'x-owner-token': created.owner_token,
      },
    });
  }
  await sender.close();
  await client.close();
  await browser.close();
}
