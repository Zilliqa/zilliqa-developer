// ABOUTME: Thin fetch wrapper for the governance-api. Guarantees the returned promise always
// ABOUTME: settles (timeout + defensive error parsing) so the UI never hangs on a 504/non-JSON.

// Proposal creation pins the full gZIL holder snapshot (~30s); the gateway backend timeout is
// raised to 90s to match, so keep this client ceiling just above it to surface a real 504
// instead of aborting first. It is a safety net against an indefinitely hung backend.
const REQUEST_TIMEOUT_MS = 95000;

class Client {
  async request(command, body?) {
    const url = `${window['VUE_APP_HUB_URL']}/api/${command}`;
    const init: any = {};
    if (body) {
      init.method = 'POST';
      init.headers = {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      };
      init.body = JSON.stringify(body);
    }

    // Bound the request so a hung/slow backend can never leave the caller waiting forever.
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    init.signal = controller.signal;

    let res;
    try {
      res = await fetch(url, init);
    } catch (e) {
      // Network failure, CORS block, or abort (timeout). Reject with a usable shape.
      const aborted = e && e.name === 'AbortError';
      return Promise.reject({
        error_description: aborted
          ? 'Request timed out. Please try again.'
          : 'Network error. Please try again.'
      });
    } finally {
      clearTimeout(timer);
    }

    // Error responses are frequently NOT JSON (a gateway 504 returns an HTML page), so parse
    // defensively instead of letting `res.json()` reject and strand the outer promise.
    const payload = await res.json().catch(() => undefined);

    if (res.ok) {
      return payload;
    }

    return Promise.reject(
      payload && typeof payload === 'object'
        ? payload
        : { error_description: `Request failed (HTTP ${res.status}).` }
    );
  }
}

const client = new Client();

export default client;
