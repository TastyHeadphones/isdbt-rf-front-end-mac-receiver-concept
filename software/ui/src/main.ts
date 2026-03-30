const baseUrl = (window as any).__ISDBT_API__ || "http://127.0.0.1:8088";

const healthEl = getEl("health");
const devicesEl = getEl("devices");
const metricsEl = getEl("metrics");

async function refresh(): Promise<void> {
  await Promise.all([fetchInto("/health", healthEl), fetchInto("/api/v1/devices", devicesEl), fetchInto("/api/v1/metrics", metricsEl)]);
}

async function fetchInto(path: string, node: HTMLElement): Promise<void> {
  try {
    const response = await fetch(`${baseUrl}${path}`);
    const body = await response.text();
    const parsed = tryParseJson(body);
    node.textContent = response.ok ? JSON.stringify(parsed ?? body, null, 2) : `HTTP ${response.status}\n${body}`;
  } catch (error) {
    node.textContent = `Request failed: ${(error as Error).message}`;
  }
}

function tryParseJson(raw: string): unknown | null {
  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

function getEl(id: string): HTMLElement {
  const node = document.getElementById(id);
  if (!node) {
    throw new Error(`missing element: ${id}`);
  }
  return node;
}

refresh().catch((error) => {
  healthEl.textContent = `initial refresh failed: ${(error as Error).message}`;
});

setInterval(() => {
  refresh().catch((error) => {
    healthEl.textContent = `refresh failed: ${(error as Error).message}`;
  });
}, 2000);
