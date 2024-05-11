import { test, expect } from "bun:test";
import { CityExtendedSchema } from "../bindings/CityExtendedSchema";

const RPC_ROUTE = "http://localhost:2900/json-rpc";
const MOCK_ID = Date.now();

async function call_rpc(method, params, id = MOCK_ID) {
	const response = await fetch(RPC_ROUTE, {
		method: "POST",
		headers: { 
			"Content-Type": "application/json",
			"Cookie": process.env.COOKIE,
		},
		body: JSON.stringify({
			id: id,
			method: method,
			params: params,
		})
	});
	const result = await response.json();

	return result;
} 

test("countries.list", async () => {
	const result = await call_rpc("countries.list", {});
	expect(typeof result.result).toBe("object");

	result.result.forEach(item => {
		const { id, name, continent, isArchived } = item;

		expect(typeof id).toBe("number");
		expect(typeof name).toBe("string");
		expect(typeof continent).toBe("string");
		expect(typeof isArchived).toBe("boolean");
	});
})

test("countries.get", async () => {
  const result = await call_rpc("countries.get", { id: 10 });
	const { id, name, continent, isArchived, cities } = result.result;

	expect(typeof id).toBe("number");
	expect(typeof name).toBe("string");
	expect(typeof continent).toBe("string");
	expect(typeof isArchived).toBe("boolean");
	expect(typeof cities).toBe("object");
});