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
  ['/', 200, 'Quote Approval Receipt — Record quote decisions', 'Record who approved your quote'],
  ['/new', 200, 'Make an approval link — Quote Approval Receipt', 'Make a quote approval link'],
  ['/privacy', 200, 'Privacy — Quote Approval Receipt', 'Privacy that fits the record'],
  ['/terms', 200, 'Terms — Quote Approval Receipt', 'Terms for clear quote records'],
  ['/missing-review-route', 404, 'Not found — Quote Approval Receipt', 'This page was not found'],
];

for (const [path, status, title, h1] of expected) {
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
    ogTitle: document.querySelector('meta[property="og:title"]')?.getAttribute('content'),
    twitterTitle: document.querySelector('meta[name="twitter:title"]')?.getAttribute('content'),
    privacy: Boolean(document.querySelector('footer a[href="/privacy"]')),
    terms: Boolean(document.querySelector('footer a[href="/terms"]')),
  }));
  const axe = await new AxeBuilder({ page }).analyze();
  const serious = axe.violations.filter(violation => ['serious', 'critical'].includes(violation.impact || ''));
  const unexpectedErrors = status === 404 ? errors.filter(error => !/status of 404/i.test(error)) : errors;
  if (response?.status() !== status || actual.title !== title || actual.h1 !== h1 || actual.h1Count !== 1 || actual.overflow || !actual.privacy || !actual.terms || actual.ogTitle !== title || actual.twitterTitle !== title || unexpectedErrors.length || serious.length) {
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
