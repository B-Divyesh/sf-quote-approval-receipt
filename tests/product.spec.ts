import { test, expect, APIRequestContext } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { getDocument } from 'pdfjs-dist/legacy/build/pdf.mjs';

const quotePayload = (overrides: Record<string, unknown> = {}) => ({
  creator_name: 'Ari Lane', business_name: 'Lane Workshop', quote_number: 'LW-19',
  client_name: 'Red Oak Cafe', currency: 'USD', summary: 'Build and install one oak service counter.',
  items: [{ description: 'Oak service counter', quantity: 1, rate: 2400 }], tax_percent: 0,
  consent_text: 'I confirm that I can decide for the client named above.', retention_days: 30,
  ...overrides,
});

async function createQuote(request: APIRequestContext, ip: string, overrides: Record<string, unknown> = {}) {
  const response = await request.post('/api/quotes', { data: quotePayload(overrides), headers: { 'x-forwarded-for': ip } });
  expect(response.status()).toBe(201);
  return response.json();
}

test('landing says what to do and has no serious accessibility errors', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/Quote Approval Receipt — Record quote decisions/);
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Record who approved your quote');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact ?? ''))).toEqual([]);
});

test('@claim:demo-sandbox sample demo stays separate and readable after creation', async ({ page, request }) => {
  await page.goto('/?demo=1');
  await expect(page.getByText('Demo — sample data, nothing is saved to your records')).toBeVisible();
  await expect(page.getByText('Half-day product shoot')).toBeVisible();
  expect(await page.evaluate(() => Object.keys(localStorage).filter(k => k.startsWith('owner:')))).toEqual([]);
  const demo = await page.evaluate(() => JSON.parse(sessionStorage.getItem('demo:workspace')!));
  for (let i = 0; i < 20; i++) {
    const quote = await request.get(`/api/share/${demo.quote.public_token}`, { headers: { 'x-forwarded-for': `198.51.100.${i + 20}` } });
    expect(quote.status()).toBe(200);
    expect((await quote.json()).demo).toBe(true);
  }
});

test('@claim:quote-snapshot an approval link shows the saved quote details', async ({ request }) => {
  const made = await createQuote(request, '198.51.100.44', {
    quote_number: 'FIXED-71', client_name: 'North Pier Books',
    summary: 'Install a window display and deliver twelve printed cards.',
    items: [{ description: 'Window display', quantity: 2, rate: 375 }],
  });
  const shown = await request.get(`/api/share/${made.public_token}`);
  expect(shown.status()).toBe(200);
  expect(await shown.json()).toMatchObject({
    quote_number: 'FIXED-71', client_name: 'North Pier Books',
    summary: 'Install a window display and deliver twelve printed cards.',
    items: [{ description: 'Window display', quantity: 2, rate: 375 }],
  });
});

test('@claim:studio-offer Studio shows checkout only when it is available and accepts license restoration', async ({ browser }) => {
  const available = await browser.newContext();
  const page = await available.newPage();
  await page.route('**/api/studio', route => route.fulfill({ json: { available: true, checkout_url: 'https://api.sociobot.in/api/v1/products/quote-approval-receipt/checkout' } }));
  await page.goto('/');
  await expect(page.getByText('$29 once.')).toBeVisible();
  await expect(page.getByRole('link', { name: 'Buy Studio for $29' })).toHaveAttribute('href', /checkout$/);
  await page.getByRole('button', { name: 'Enter a Studio license' }).click();
  await expect(page.getByLabel('License token')).toBeVisible();
  await page.route('**/api/v1/products/quote-approval-receipt/verify?license=*', route => route.fulfill({ json: { valid: true, reason: 'ok' } }));
  await page.getByLabel('License token').fill('restored-license-token');
  await page.getByRole('button', { name: 'Verify license' }).click();
  await expect(page.getByText('Studio is active on this device.')).toBeVisible();
  await expect.poll(() => page.evaluate(() => localStorage.getItem('sb_license:quote-approval-receipt'))).toBe('restored-license-token');
  await available.close();

  const unavailable = await browser.newContext();
  const unavailablePage = await unavailable.newPage();
  await unavailablePage.route('**/api/studio', route => route.fulfill({ json: { available: false, checkout_url: null } }));
  await unavailablePage.goto('/');
  await expect(unavailablePage.getByText('Studio checkout is not available. Free links keep records for 30 days.')).toBeVisible();
  await expect(unavailablePage.getByRole('link', { name: 'Buy Studio for $29' })).toHaveCount(0);
  await unavailable.close();
});

test('@claim:pdf-receipt PDF contains the full snapshot, time, and international identity', async ({ request }) => {
  const made = await createQuote(request, '198.51.100.50', { quote_number: 'Q-Á1', client_name: 'Café São Bento' });
  const decision = await request.post(`/api/share/${made.public_token}/decision`, {
    headers: { 'x-forwarded-for': '198.51.100.50' },
    data: { name: 'José Núñez', title: 'Direção', email: null, decision: 'approved', note: 'Aprovado.', consent: true },
  });
  expect(decision.status()).toBe(201);
  const receipt = await decision.json();
  const pdfResponse = await request.get(receipt.pdf_path, { headers: { 'x-forwarded-for': '198.51.100.50' } });
  expect(pdfResponse.headers()['content-type']).toContain('application/pdf');
  const bytes = await pdfResponse.body();
  expect(bytes.subarray(0, 8).toString()).toContain('%PDF-1.');
  const pdf = await getDocument({ data: new Uint8Array(bytes) }).promise;
  const pdfPage = await pdf.getPage(1);
  const content = await pdfPage.getTextContent();
  const text = content.items.map(item => 'str' in item ? item.str : '').join(' ');
  expect(text).toContain('José Núñez — Direção');
  expect(text).toContain('Q-Á1');
  expect(text).toContain('Café São Bento');
  expect(text).toContain(receipt.receipt.decided_at);
  expect(text).toContain(receipt.receipt.snapshot_hash);
  expect(text).toContain(receipt.receipt.consent_text);
});

test('@claim:record-control owner can export and delete a record', async ({ request }) => {
  const made = await createQuote(request, '198.51.100.60');
  const headers = { 'x-owner-token': made.owner_token, 'x-forwarded-for': '198.51.100.60' };
  const exported = await request.get(`/api/quotes/${made.id}/export`, { headers });
  expect(exported.status()).toBe(200); expect((await exported.json()).quote.quote_number).toBe('LW-19');
  expect((await request.delete(`/api/quotes/${made.id}`, { headers })).status()).toBe(204);
  expect((await request.get(`/api/share/${made.public_token}`, { headers })).status()).toBe(404);
});

test('@regression:owner-receipt-link a fresh sender context can open the receipt and download its PDF', async ({ browser, request }) => {
  const made = await createQuote(request, '198.51.100.61');
  const decision = await request.post(`/api/share/${made.public_token}/decision`, {
    headers: { 'x-forwarded-for': '198.51.100.62' },
    data: { name: 'Mara Chen', title: 'Director', decision: 'approved', consent: true },
  });
  expect(decision.status()).toBe(201);
  const receipt = await decision.json();
  const publicQuote = await request.get(`/api/share/${made.public_token}`);
  expect((await publicQuote.json()).receipt_id).toBeNull();
  const ownedQuote = await request.get(`/api/quotes/${made.id}`, { headers: { 'x-owner-token': made.owner_token } });
  expect((await ownedQuote.json()).receipt_id).toBe(receipt.receipt.id);

  const sender = await browser.newContext();
  const page = await sender.newPage();
  await page.goto('/');
  await page.evaluate(({ id, ownerToken }) => localStorage.setItem(`owner:${id}`, ownerToken), {
    id: made.id,
    ownerToken: made.owner_token,
  });
  await page.goto(`/manage/${made.id}`);
  await expect(page.getByRole('heading', { name: 'A decision is recorded' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'View decision receipt' })).toHaveAttribute('href', `/receipt/${receipt.receipt.id}`);
  const pdf = page.getByRole('link', { name: 'Download PDF receipt' });
  await expect(pdf).toHaveAttribute('href', receipt.pdf_path);
  const [download] = await Promise.all([page.waitForEvent('download'), pdf.click()]);
  expect(download.suggestedFilename()).toBe('approval-receipt.pdf');
  await sender.close();
});

test('@claim:first-party-only the landing and demo load only same-origin resources', async ({ page }) => {
  const external: string[] = [];
  page.on('request', req => { if (new URL(req.url()).origin !== 'http://127.0.0.1:4173') external.push(req.url()); });
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page.getByText('Half-day product shoot')).toBeVisible();
  expect(external).toEqual([]);
});

test('@claim:retention-policy free and demo expiry are exact and Studio cannot be forged', async ({ request }) => {
  const before = Date.now();
  const free = await createQuote(request, '198.51.100.70');
  expect(new Date(free.expires_at).getTime() - before).toBeGreaterThan(29.99 * 86_400_000);
  expect(new Date(free.expires_at).getTime() - before).toBeLessThan(30.01 * 86_400_000);
  const demoResponse = await request.post('/api/demo', { headers: { 'x-forwarded-for': '198.51.100.71' } });
  const demo = await demoResponse.json();
  const demoQuote = await request.get(`/api/share/${demo.quote.public_token}`, { headers: { 'x-forwarded-for': '198.51.100.71' } });
  expect(new Date((await demoQuote.json()).expires_at).getTime() - before).toBeGreaterThan(23.99 * 3_600_000);
  const forged = await request.post('/api/quotes', { data: quotePayload({ retention_days: 365 }), headers: { 'x-forwarded-for': '198.51.100.72' } });
  expect(forged.status()).toBe(403);
});

test('@claim:private-links tokens differ and private routes carry noindex and no-store', async ({ request }) => {
  const one = await createQuote(request, '198.51.100.80');
  const two = await createQuote(request, '198.51.100.81');
  expect(one.public_token).toMatch(/^[A-Za-z0-9]{32}$/);
  expect(one.owner_token).toMatch(/^[A-Za-z0-9]{40}$/);
  expect(one.public_token).not.toBe(two.public_token);
  const privatePage = await request.get(one.share_path);
  expect(privatePage.headers()['x-robots-tag']).toContain('noindex');
  expect(privatePage.headers()['cache-control']).toBe('no-store');
  const api = await request.get(`/api/share/${one.public_token}`);
  expect(api.headers()['cache-control']).toBe('no-store');
});

test('@claim:single-decision concurrent decisions produce one final receipt', async ({ request }) => {
  const made = await createQuote(request, '198.51.100.90');
  const input = { name: 'Mara Chen', title: 'Director', decision: 'approved', consent: true };
  const [a, b] = await Promise.all([
    request.post(`/api/share/${made.public_token}/decision`, { data: input, headers: { 'x-forwarded-for': '198.51.100.91' } }),
    request.post(`/api/share/${made.public_token}/decision`, { data: input, headers: { 'x-forwarded-for': '198.51.100.92' } }),
  ]);
  expect([a.status(), b.status()].sort()).toEqual([201, 409]);
});

test('@claim:network-address-privacy receipts and exports never reveal the network address', async ({ request }) => {
  const address = '198.51.100.212';
  const made = await createQuote(request, address);
  const decision = await request.post(`/api/share/${made.public_token}/decision`, {
    data: { name: 'Nia Boyd', title: 'Owner', decision: 'approved', consent: true },
    headers: { 'x-forwarded-for': address, 'x-owner-token': made.owner_token },
  });
  expect(JSON.stringify(await decision.json())).not.toContain(address);
  const exported = await request.get(`/api/quotes/${made.id}/export`, {
    headers: { 'x-forwarded-for': address, 'x-owner-token': made.owner_token },
  });
  expect(await exported.text()).not.toContain(address);
});

test('@claim:rate-limits write endpoints return 429 with Retry-After during a burst', async ({ request }) => {
  const statuses: number[] = []; let retry = '';
  for (let i=0;i<20;i++) { const response=await request.post('/api/demo',{headers:{'x-forwarded-for':'203.0.113.77'}}); statuses.push(response.status()); if(response.status()===429)retry=response.headers()['retry-after']; }
  expect(statuses).toContain(429); expect(retry).toBe('1');
});

test('invalid currencies fail cleanly and response policies are enforced', async ({ request }) => {
  const invalid = await request.post('/api/quotes', { data: quotePayload({ currency: '!!!' }), headers: { 'x-forwarded-for': '198.51.100.101' } });
  expect(invalid.status()).toBe(422);
  expect((await invalid.json()).error).toContain('supported currency');
  expect((await request.get('/not-a-real-page')).status()).toBe(404);
  const manage = await request.get('/manage/not-real');
  expect(manage.headers()['x-robots-tag']).toContain('noindex');
  const home = await request.get('/');
  const assetPath = (await home.text()).match(/\/assets\/[^"']+\.js/)?.[0];
  expect(assetPath).toBeTruthy();
  const asset = await request.get(assetPath!, { headers: { 'accept-encoding': 'br' } });
  expect(asset.headers()['cache-control']).toContain('immutable');
  expect(asset.headers()['content-encoding']).toBe('br');
});

test('keyboard navigation and route focus work', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('#main')).toBeFocused();
  await page.getByRole('link', { name: 'Try it with sample data' }).focus();
  await page.keyboard.press('Enter');
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await expect(page.getByText('Half-day product shoot')).toBeVisible();
});

test('each public route updates social metadata and the 404 stays within a 390px viewport', async ({ page }) => {
  const expected = [
    ['/privacy', 'Privacy — Quote Approval Receipt', 'What quote and approver data we store and how to remove it.'],
    ['/terms', 'Terms — Quote Approval Receipt', 'Terms for using Quote Approval Receipt.'],
    ['/new', 'Make an approval link — Quote Approval Receipt', 'Enter an existing quote and create a private decision link.'],
    ['/?demo=1', 'Demo — Quote Approval Receipt', 'Try a sample quote decision in an isolated demo.'],
    ['/missing-page', 'Not found — Quote Approval Receipt', 'The requested page was not found.'],
  ];
  for (const [path, title, description] of expected) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', title);
    await expect(page.locator('meta[property="og:description"]')).toHaveAttribute('content', description);
    await expect(page.locator('meta[name="twitter:title"]')).toHaveAttribute('content', title);
    await expect(page.locator('meta[name="twitter:description"]')).toHaveAttribute('content', description);
  }
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/404-review-missing');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});

test('@mobile 390px pages reflow at 200% text and controls meet target size', async ({ page }) => {
  for (const path of ['/', '/new', '/privacy', '/404-review-missing']) {
    await page.goto(path);
    await page.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  }
  await page.goto('/');
  const undersized = await page.locator('a:visible, button:visible').evaluateAll(nodes => nodes
    .map(node => ({ text: node.textContent?.trim(), box: node.getBoundingClientRect() }))
    .filter(({ box }) => box.width < 44 || box.height < 44));
  expect(undersized).toEqual([]);
});

test('all public screens have no serious accessibility errors and reduced motion stops animation', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  for (const path of ['/', '/new', '/?demo=1', '/privacy', '/terms', '/missing-page']) {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact ?? ''))).toEqual([]);
  }
  expect(await page.evaluate(() => document.getAnimations().filter(a => a.playState === 'running').length)).toBe(0);
});
