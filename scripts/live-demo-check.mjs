import { chromium } from '@playwright/test';

const base = (process.env.LIVE_URL || process.argv[2] || '').replace(/\/$/, '');
if (!base) {
  throw new Error('Set LIVE_URL to the deployed product URL.');
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
    const widthOk = await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth);
    if (!widthOk) throw new Error(`attempt ${attempt}: demo page overflowed its viewport`);
    console.log(`attempt ${attempt}: POST /api/demo 201, sample quote visible`);
  } finally {
    await browser.close();
  }
}

console.log('20/20 isolated Chromium demo starts succeeded.');
