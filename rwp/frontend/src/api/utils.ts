import axios, { type AxiosRequestConfig, type AxiosResponse } from "axios";

type ApiSuccess<X> = { status: number; data: X };
type ApiFailure = { status: number; error: string; data?: unknown };
export type ApiResult<X> = ApiSuccess<X> | ApiFailure;

function getToken(): string {
  return localStorage.getItem("token") ?? "";
}

async function handleRequest<X>(
  config: AxiosRequestConfig,
  expectedResponse: number,
): Promise<ApiResult<X>> {
  let response: AxiosResponse<X | string>;
  try {
    response = await axios.request<X | string>({
      ...config,
      headers: {
        "Content-Type": "application/json",
        token: getToken(),
        ...config.headers,
      },
      validateStatus: () => true,
    });
  } catch (error) {
    return {
      status: 500,
      error: error instanceof Error ? error.message : "Network error",
    };
  }

  if (response.status === expectedResponse) {
    return { status: response.status, data: response.data as X };
  }
  return {
    status: response.status,
    error: `expected status ${expectedResponse} got ${response.status}`,
    data: response.data,
  };
}

export async function getCall<X>(url: string, expectedResponse: number) {
  return handleRequest<X>({ method: "get", url }, expectedResponse);
}

// POST with a body: only X needs to be specified, T is inferred from `body`
export async function postCall<X, T = unknown>(
  url: string,
  body: T,
  expectedResponse: number,
) {
  return handleRequest<X>(
    { method: "post", url, data: body },
    expectedResponse,
  );
}

// POST without a body (logout, trigger action, ...)
export async function postCallNoBody<X>(
  url: string,
  expectedResponse: number,
) {
  return handleRequest<X>({ method: "post", url }, expectedResponse);
}

export async function deleteCall<X>(url: string, expectedResponse: number) {
  return handleRequest<X>({ method: "delete", url }, expectedResponse);
}

export async function putCall<X, T = unknown>(
  url: string,
  body: T,
  expectedResponse: number,
) {
  return handleRequest<X>({ method: "put", url, data: body }, expectedResponse);
}

export async function patchCall<X, T = unknown>(
  url: string,
  body: T,
  expectedResponse: number,
) {
  return handleRequest<X>(
    { method: "patch", url, data: body },
    expectedResponse,
  );
}
