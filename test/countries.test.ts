import { test, expect } from "bun:test";
import { KrakenJsonRpcClient } from "./jsonrpc";

const c = new KrakenJsonRpcClient(process.env.RPC_URI!);

test("countries.list", async () => {
	const { result } = await c.invoke("countries.list");
	expect(typeof result).toBe("object");

	result.forEach(item => {
		const { id, name, continent, isArchived } = item;

		expect(typeof id).toBe("number");
		expect(typeof name).toBe("string");
		expect(typeof continent).toBe("string");
		expect(typeof isArchived).toBe("boolean");
	});
})

test("countries.get", async () => {
  const { result } = await c.invoke("countries.get", { id: 10 });
	const { id, name, continent, isArchived, cities } = result;

	expect(typeof id).toBe("number");
	expect(typeof name).toBe("string");
	expect(typeof continent).toBe("string");
	expect(typeof isArchived).toBe("boolean");
	expect(typeof cities).toBe("object");
});