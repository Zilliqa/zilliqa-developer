// ABOUTME: Framework-free assertions that client.request always settles (never hangs the UI).
// ABOUTME: Run via ts-node transpile-only; mocks window + fetch. See command in the fix PR.
import assert from "assert";

(global as any).window = { VUE_APP_HUB_URL: "http://hub.test" };

// eslint-disable-next-line @typescript-eslint/no-var-requires
import client from "./client";

async function nonJsonErrorDoesNotHang() {
  // A gateway 504 returns an HTML body → res.json() throws. Must still reject, not hang.
  (global as any).fetch = async () => ({
    ok: false,
    status: 504,
    json: async () => {
      throw new SyntaxError("Unexpected token < in JSON");
    }
  });
  let rejected = false;
  let val: any;
  try {
    await client.request("message", { a: 1 });
  } catch (e) {
    rejected = true;
    val = e;
  }
  assert.strictEqual(rejected, true, "504 with non-JSON body must reject, not hang");
  assert.ok(
    val && typeof val.error_description === "string",
    "rejection must carry a usable error_description"
  );
  console.log("OK nonJsonErrorDoesNotHang:", val.error_description);
}

async function successResolves() {
  (global as any).fetch = async () => ({
    ok: true,
    status: 200,
    json: async () => ({ ipfsHash: "Qm123" })
  });
  const out: any = await client.request("message", { a: 1 });
  assert.deepStrictEqual(out, { ipfsHash: "Qm123" }, "success resolves to parsed JSON");
  console.log("OK successResolves");
}

async function abortRejects() {
  (global as any).fetch = async () => {
    const e: any = new Error("aborted");
    e.name = "AbortError";
    throw e;
  };
  let rejected = false;
  let val: any;
  try {
    await client.request("spaces");
  } catch (e) {
    rejected = true;
    val = e;
  }
  assert.strictEqual(rejected, true, "abort/timeout must reject");
  assert.ok(val.error_description.includes("timed out"), "AbortError => timeout message");
  console.log("OK abortRejects:", val.error_description);
}

async function errorWithJsonRejectsServerObject() {
  // A normal 400 (e.g. MIN_BALANCE) carries JSON the UI shows; reject with the server's object.
  (global as any).fetch = async () => ({
    ok: false,
    status: 400,
    json: async () => ({
      code: 11,
      error_description: "You require 30 $gZIL or more to submit a proposal."
    })
  });
  let val: any;
  try {
    await client.request("message", { a: 1 });
  } catch (e) {
    val = e;
  }
  assert.strictEqual(val.code, 11, "rejects with the server's JSON error object");
  assert.ok(val.error_description.includes("gZIL"), "preserves server error_description");
  console.log("OK errorWithJsonRejectsServerObject");
}

async function successEmptyBodyResolves() {
  // A 2xx with a non-JSON/empty body must resolve (to undefined), never hang or throw.
  (global as any).fetch = async () => ({
    ok: true,
    status: 200,
    json: async () => {
      throw new SyntaxError("empty body");
    }
  });
  const out = await client.request("spaces");
  assert.strictEqual(out, undefined, "non-JSON 2xx resolves to undefined");
  console.log("OK successEmptyBodyResolves");
}

(async () => {
  await nonJsonErrorDoesNotHang();
  await successResolves();
  await abortRejects();
  await errorWithJsonRejectsServerObject();
  await successEmptyBodyResolves();
  console.log("ALL PASS");
})().catch((e) => {
  console.error("FAIL:", e.message);
  process.exit(1);
});
