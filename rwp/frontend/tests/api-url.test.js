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
    expect(urls.getAll).toBe("http://localhost:8001/api/v1/get");
    expect(urls.update).toBe("http://localhost:8001/api/v1/update");
  });

  test("uses integer ids in get and delete path segments", () => {
    const urls = new Url();
    expect(urls.getByIdUrl(42)).toBe("http://localhost:8001/api/v1/get/42");
    expect(urls.deleteUrl(42)).toBe("http://localhost:8001/api/v1/delete/42");
  });

  test("uses the current origin outside the local frontend dev server", () => {
    Object.defineProperty(globalThis, "window", {
      configurable: true,
      value: { location: { origin: "https://todo.example" } },
    });
    expect(new Url().getAll).toBe("https://todo.example/api/v1/get");
  });
});
