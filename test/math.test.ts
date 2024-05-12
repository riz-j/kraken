import { test, expect } from "bun:test";
import { KrakenJsonRpcClient } from "./jsonrpc";

const MOCK_ID = Date.now();

const c = new KrakenJsonRpcClient(process.env.RPC_URI!);

test("add", async () => {
	const { result, error, id } = await c.invoke(
		"add", 
		{ a: 15, b: 25 },
		MOCK_ID
	);
	
	expect(result).toBe(40);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
});

test("subtract", async () => {
	const { result, error, id } = await c.invoke(
		"subtract", 
		{ a: 15, b: 25 },
		MOCK_ID
	);
	
	expect(result).toBe(-10);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
});

test("multiply", async () => {
	const { result, error, id } = await c.invoke(
		"multiply", 
		{ a: 5, b: 9 },
		MOCK_ID
	);
	
	expect(result).toBe(45);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
})

test("divide", async () => {
	const { result, error, id } = await c.invoke(
		"divide", 
		{ a: 27, b: 4 },
		MOCK_ID
	);
	
	expect(result).toBe(6.75);
	expect(id).toBe(MOCK_ID);
	expect(error).toBeUndefined();
})