import fs from 'node:fs';
import { chromium } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
const evidence = process.env.EVIDENCE_DIR || process.argv[3];
if (!base || !evidence) throw new Error('Set LIVE_URL and EVIDENCE_DIR.');
fs.mkdirSync(evidence, { recursive: true });

const browser = await chromium.launch({ headless: true });
const results = [];
const expected = [
  ['/', 200, 'Quote Approval Receipt — Record quote decisions', 'Record who approved your quote', 'Capture who approved a fixed quote and issue a timestamped PDF receipt.', '/'],
  ['/new', 200, 'Make an approval link — Quote Approval Receipt', 'Make a quote approval link', 'Enter an existing quote and create a private decision link.', '/new'],
  ['/privacy', 200, 'Privacy — Quote Approval Receipt', 'How we store quote and approver data', 'What quote and approver data we store and how to remove it.', '/privacy'],
  ['/terms', 200, 'Terms — Quote Approval Receipt', 'Terms for quote approval records', 'Terms for using Quote Approval Receipt.', '/terms'],
  ['/missing-review-route', 404, 'Not found — Quote Approval Receipt', 'This page was not found', 'The requested page was not found.', '/missing-review-route'],
];

for (const [path, status, title, h1, description, canonicalPath] of expected) {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(String(error)));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  const response = await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
  const actual = await page.evaluate(() => ({
    title: document.title,
    h1: document.querySelector('h1')?.textContent?.trim(),
    h1Count: document.querySelectorAll('h1').length,
    overflow: document.documentElement.scrollWidth > innerWidth,
    canonical: document.querySelector('link[rel="canonical"]')?.getAttribute('href'),
    description: document.querySelector('meta[name="description"]')?.getAttribute('content'),
    ogTitle: document.querySelector('meta[property="og:title"]')?.getAttribute('content'),
    ogDescription: document.querySelector('meta[property="og:description"]')?.getAttribute('content'),
    twitterTitle: document.querySelector('meta[name="twitter:title"]')?.getAttribute('content'),
    twitterDescription: document.querySelector('meta[name="twitter:description"]')?.getAttribute('content'),
    privacy: Boolean(document.querySelector('footer a[href="/privacy"]')),
    terms: Boolean(document.querySelector('footer a[href="/terms"]')),
  }));
  const axe = await new AxeBuilder({ page }).analyze();
  const serious = axe.violations.filter(violation => ['serious', 'critical'].includes(violation.impact || ''));
  const unexpectedErrors = status === 404 ? errors.filter(error => !/status of 404/i.test(error)) : errors;
  const canonical = `${base}${canonicalPath}`;
  if (response?.status() !== status || actual.title !== title || actual.h1 !== h1 || actual.h1Count !== 1 || actual.overflow || !actual.privacy || !actual.terms || actual.canonical !== canonical || actual.description !== description || actual.ogTitle !== title || actual.ogDescription !== description || actual.twitterTitle !== title || actual.twitterDescription !== description || unexpectedErrors.length || serious.length) {
    throw new Error(`${path} failed: ${JSON.stringify({ status: response?.status(), actual, errors: unexpectedErrors, serious })}`);
  }
  if (path === '/missing-review-route') await page.screenshot({ path: `${evidence}/404-mobile.png`, fullPage: true });
  results.push({ path, status, ...actual, seriousCritical: serious.length, consoleErrors: unexpectedErrors.length });
  await context.close();
}

const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
const page = await context.newPage();
const origins = new Set();
page.on('request', request => origins.add(new URL(request.url()).origin));
await page.goto(`${base}/`, { waitUntil: 'networkidle' });
const firstScreen = await page.evaluate(() => {
  const visibleBeforeFold = selector => {
    const box = document.querySelector(selector)?.getBoundingClientRect();
    return Boolean(box && box.top >= 0 && box.bottom <= innerHeight);
  };
  return {
    headline: visibleBeforeFold('h1'),
    action: visibleBeforeFold('a[href="/?demo=1"]'),
    facts: visibleBeforeFold('.facts'),
    overflow: document.documentElement.scrollWidth > innerWidth,
  };
});
if (!firstScreen.headline || !firstScreen.action || !firstScreen.facts || firstScreen.overflow) {
  throw new Error(`first screen failed: ${JSON.stringify(firstScreen)}`);
}
await page.screenshot({ path: `${evidence}/landing-mobile.png`, fullPage: true });
await page.getByRole('link', { name: 'Try it with sample data' }).click();
await page.getByText('Half-day product shoot').waitFor();
await page.screenshot({ path: `${evidence}/demo-mobile.png`, fullPage: true });
const demo = await page.evaluate(() => JSON.parse(sessionStorage.getItem('demo:workspace')));
await page.getByRole('link', { name: 'Start for real' }).click();
await page.getByRole('heading', { name: 'Make a quote approval link' }).waitFor();
const demoKeys = await page.evaluate(() => Object.keys(sessionStorage).filter(key => key.startsWith('demo:')));
const deleted = await page.request.get(`${base}/api/share/${demo.quote.public_token}`);
if (demoKeys.length || deleted.status() !== 404 || [...origins].some(origin => origin !== new URL(base).origin)) {
  throw new Error(`demo/privacy failed: ${JSON.stringify({ demoKeys, deleted: deleted.status(), origins: [...origins] })}`);
}
results.push({ path: '/?demo=1 → /new', deletedStatus: deleted.status(), demoKeys, origins: [...origins] });
await context.close();
await browser.close();

fs.writeFileSync(`${evidence}/live-review.json`, JSON.stringify({ base, checkedAt: new Date().toISOString(), firstScreen, results }, null, 2));
console.log(JSON.stringify({ routes: results.length, firstScreen, demoDeleted: deleted.status(), origins: [...origins] }));
