import { test, expect } from "bun:test";
import { v4 as uuid } from "uuid";

const RPC_ROUTE = "http://localhost:2900/rpc";
const MOCK_ID = uuid();

async function call_rpc(method, params, id = MOCK_ID) {
	const response = await fetch(RPC_ROUTE, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			id: id,
			method: method,
			params: params,
		})
	});
	const result = await response.json();

	return result;
} 

test("add", async () => {
	const { id, result, error } = await call_rpc(
		"add", 
		{ a: 15, b: 25 },
	);
	
	expect(result).toBe(40);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
});

test("subtract", async () => {
	const { id, result, error } = await call_rpc(
		"subtract", 
		{ a: 15, b: 25 },
	);
	
	expect(result).toBe(-10);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
});

test("multiply", async () => {
	const { id, result, error } = await call_rpc(
		"multiply", 
		{ a: 5, b: 9 },
	);
	
	expect(result).toBe(45);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
})

test("divide", async () => {
	const { id, result, error } = await call_rpc(
		"divide", 
		{ a: 27, b: 4 },
	);
	
	expect(result).toBe(6.75);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
})