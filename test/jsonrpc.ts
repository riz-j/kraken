export class KrakenJsonRpcClient {
	private url: string;

	constructor(url: string) {
		this.url = url;
	}

	public async invoke(
		method: string,
		params: object = {},
		id: string | number = 1
	): Promise<JsonRpcResponse> {
		const response = await fetch(this.url, {
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
}

interface JsonRpcResponse {
	jsonrpc: string,
	result: any | undefined,
	error: any | undefined,
	id: string | number
}