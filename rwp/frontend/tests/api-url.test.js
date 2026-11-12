import { beforeEach, describe, expect, test } from "bun:test";

import { Url } from "../src/api/url.ts";

describe("frontend API URLs", () => {
  beforeEach(() => {
    Object.defineProperty(globalThis, "window", {
      configurable: true,
      value: { location: { origin: "http://localhost:3000" } },
    });
  });

  test("targets the ingress API for create, list, and update", () => {
    const urls = new Url();
    expect(urls.create).toBe("http://localhost:8001/api/v1/create");
    expect(urls.getAll).toBe("http://localhost:8001/api/v1/get/all");
    expect(urls.update).toBe("http://localhost:8001/api/v1/update");
  });

  test("encodes names in get and delete path segments", () => {
    const urls = new Url();
    expect(urls.getByNameUrl("write tests/now")).toBe(
      "http://localhost:8001/api/v1/get/write%20tests%2Fnow",
    );
    expect(urls.deleteUrl("write tests/now")).toBe(
      "http://localhost:8001/api/v1/delete/write%20tests%2Fnow",
    );
  });

  test("uses the current origin outside the local frontend dev server", () => {
    Object.defineProperty(globalThis, "window", {
      configurable: true,
      value: { location: { origin: "https://todo.example" } },
    });
    expect(new Url().getAll).toBe("https://todo.example/api/v1/get/all");
  });
});
