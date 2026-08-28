import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('landing says what to do and has no serious accessibility errors', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/Quote Approval Receipt — Record quote decisions/);
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Record who approved your quote');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact ?? ''))).toEqual([]);
});

test('@claim:demo-sandbox sample demo stays separate from real records', async ({ page }) => {
  await page.goto('/demo');
  await expect(page.getByText('Demo — sample data, nothing is saved to your records')).toBeVisible();
  await expect(page.getByText('Half-day product shoot')).toBeVisible();
  const owners = await page.evaluate(() => Object.keys(localStorage).filter(k => k.startsWith('owner:')));
  expect(owners).toEqual([]);
  const demo = await page.evaluate(() => JSON.parse(sessionStorage.getItem('demo:workspace')!));
  const quote = await page.request.get(`/api/share/${demo.quote.public_token}`);
  expect((await quote.json()).demo).toBe(true);
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact ?? ''))).toEqual([]);
});

test('@claim:pdf-receipt a decision produces a timestamped PDF receipt', async ({ page }) => {
  await page.goto('/demo');
  await page.getByLabel('Approve quote').check();
  await page.getByLabel(/I confirm that I am authorised/).check();
  await page.getByRole('button', { name: 'Record this decision' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This quote was approved');
  await expect(page.getByText('Mara Chen', { exact: true })).toBeVisible();
  const href = await page.getByRole('link', { name: 'Download PDF receipt' }).getAttribute('href');
  const pdf = await page.request.get(href!);
  expect(pdf.headers()['content-type']).toContain('application/pdf');
  expect((await pdf.body()).subarray(0, 8).toString()).toContain('%PDF-1.4');
});

test('@claim:record-control owner can export and delete a record', async ({ request }) => {
  const payload = { creator_name:'Ari Lane', business_name:'Lane Workshop', quote_number:'LW-19', client_name:'Red Oak Cafe', currency:'USD', summary:'Build and install one oak service counter.', items:[{description:'Oak service counter',quantity:1,rate:2400}], tax_percent:0, consent_text:'I confirm that I can decide for the client named above.', retention_days:30 };
  const madeRes = await request.post('/api/quotes', { data: payload, headers: {'x-forwarded-for':'198.51.100.11'} });
  expect(madeRes.status()).toBe(201); const made = await madeRes.json();
  const exported = await request.get(`/api/quotes/${made.id}/export`, { headers: {'x-owner-token':made.owner_token,'x-forwarded-for':'198.51.100.11'} });
  expect(exported.status()).toBe(200); expect((await exported.json()).quote.quote_number).toBe('LW-19');
  const deleted = await request.delete(`/api/quotes/${made.id}`, { headers: {'x-owner-token':made.owner_token,'x-forwarded-for':'198.51.100.11'} });
  expect(deleted.status()).toBe(204);
  const gone = await request.get(`/api/share/${made.public_token}`, { headers: {'x-forwarded-for':'198.51.100.11'} });
  expect(gone.status()).toBe(404);
});

test('@claim:first-party-only the landing and demo load only same-origin resources', async ({ page }) => {
  const external: string[] = [];
  page.on('request', req => { if (new URL(req.url()).origin !== 'http://127.0.0.1:4173') external.push(req.url()); });
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page.getByText('Half-day product shoot')).toBeVisible();
  expect(external).toEqual([]);
});

test('@claim:studio-retention Studio terms state the price and retention', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('$29 once. Studio adds 365-day retention.')).toBeVisible();
  await expect(page.getByRole('link', { name: 'Buy Studio for $29' })).toHaveAttribute('href', 'https://api.sociobot.in/api/v1/products/quote-approval-receipt/checkout');
});

test('write endpoints return 429 with Retry-After during a burst', async ({ request }) => {
  const statuses: number[] = []; let retry = '';
  for (let i=0;i<20;i++) { const response=await request.post('/api/demo',{headers:{'x-forwarded-for':'203.0.113.77'}}); statuses.push(response.status()); if(response.status()===429)retry=response.headers()['retry-after']; }
  expect(statuses).toContain(429); expect(retry).toBe('1');
});

test('@mobile landing and demo fit a 390px screen', async ({ page }) => {
  await page.goto('/');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});
