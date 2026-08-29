import { chromium } from '@playwright/test';

const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
const expectedBuild = process.env.EXPECTED_BUILD_SHA || '';
if (!base) {
  throw new Error('Set LIVE_URL to the deployed product URL.');
}

const health = await fetch(`${base}/health`);
if (!health.ok) throw new Error(`health returned ${health.status}`);
const healthBody = await health.json();
if (expectedBuild && healthBody.build_sha !== expectedBuild) {
  throw new Error(`health build ${healthBody.build_sha} does not match ${expectedBuild}`);
}

for (let attempt = 1; attempt <= 20; attempt += 1) {
  // A separate Chromium process for every pass catches replica-local state.
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
  try {
    const demoResponse = page.waitForResponse(response => response.url().includes('/api/demo') && response.request().method() === 'POST');
    const shareResponse = page.waitForResponse(response => response.url().includes('/api/share/') && response.request().method() === 'GET');
    await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
    const response = await demoResponse;
    if (response.status() !== 201) throw new Error(`attempt ${attempt}: demo create returned ${response.status()}`);
    const share = await shareResponse;
    if (share.status() !== 200) throw new Error(`attempt ${attempt}: sample read returned ${share.status()}`);
    await page.getByText('Half-day product shoot').waitFor({ state: 'visible', timeout: 10_000 });
    await page.getByText('Demo — sample data, nothing is saved to your records').waitFor({ state: 'visible' });
    const widthOk = await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth);
    if (!widthOk) throw new Error(`attempt ${attempt}: demo page overflowed its viewport`);
    let demo = await page.evaluate(() => JSON.parse(sessionStorage.getItem('demo:workspace')));
    if (attempt === 1) {
      const oldToken = demo.quote.public_token;
      await page.getByRole('button', { name: 'Reset demo' }).first().click();
      await page.getByText('Half-day product shoot').waitFor({ state: 'visible', timeout: 10_000 });
      if ((await page.request.get(`${base}/api/share/${oldToken}`)).status() !== 404) {
        throw new Error('attempt 1: reset did not delete the previous sample');
      }
      demo = await page.evaluate(() => JSON.parse(sessionStorage.getItem('demo:workspace')));
    }
    await page.getByRole('link', { name: 'Start for real' }).click();
    await page.getByRole('heading', { name: 'Make a quote approval link' }).waitFor({ state: 'visible', timeout: 10_000 });
    const retainedDemoKeys = await page.evaluate(() => Object.keys(sessionStorage).filter(key => key.startsWith('demo:')));
    if (retainedDemoKeys.length) throw new Error(`attempt ${attempt}: demo session pointer was retained`);
    const deletedShare = await page.request.get(`${base}/api/share/${demo.quote.public_token}`);
    if (deletedShare.status() !== 404) throw new Error(`attempt ${attempt}: discarded sample returned ${deletedShare.status()}`);
    console.log(`attempt ${attempt}: create 201, read 200, exit cleanup 404`);
  } finally {
    await browser.close();
  }
}

console.log(`20/20 isolated Chromium demo starts and exits succeeded on build ${healthBody.build_sha}.`);
