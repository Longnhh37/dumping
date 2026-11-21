import axios from "axios";
import { afterEach, beforeEach, describe, expect, mock, spyOn, test } from "bun:test";

import { deleteCall, getCall, patchCall, postCall } from "../src/api/utils.ts";

describe("frontend API request helpers", () => {
	const originalLocalStorage = globalThis.localStorage;
	beforeEach(() => {
		Object.defineProperty(globalThis, "localStorage", {
			configurable: true,
			value: { getItem: (key) => (key === "token" ? "signed-token" : null) },
		});
	});

	afterEach(() => {
		mock.restore();
		Object.defineProperty(globalThis, "localStorage", {
			configurable: true,
			value: originalLocalStorage,
		});
	});

	test("sends the stored token, JSON headers, and request body", async () => {
		const request = mock(() => Promise.resolve({ status: 201, data: { id: 7 } }));
		spyOn(axios, "request").mockImplementation(request);

		const result = await postCall("/api/create", { title: "task" }, 201);

		expect(result).toEqual({ status: 201, data: { id: 7 } });
		expect(request).toHaveBeenCalledWith({
			method: "post",
			url: "/api/create",
			data: { title: "task" },
			headers: { "Content-Type": "application/json", token: "signed-token" },
			validateStatus: expect.any(Function),
		});
	});

	test("returns the response status and payload when it differs from the expected status", async () => {
		spyOn(axios, "request").mockResolvedValue({ status: 404, data: "missing" });

		expect(await getCall("/api/items/99", 200)).toEqual({
			status: 404,
			error: "expected status 200 got 404",
			data: "missing",
		});
	});

	test("converts network exceptions into a failure result", async () => {
		spyOn(axios, "request").mockRejectedValue(new Error("offline"));

		expect(await deleteCall("/api/items/7", 200)).toEqual({ status: 500, error: "offline" });
	});

	test("patch helper sends the update payload", async () => {
		const requestSpy = spyOn(axios, "request").mockResolvedValue({ status: 200, data: {} });

		await patchCall("/api/update", { id: 7, title: "updated" }, 200);

		expect(requestSpy.mock.calls[0][0]).toMatchObject({
			method: "patch",
			url: "/api/update",
			data: { id: 7, title: "updated" },
		});
	});
});
