import axios, { type AxiosRequestConfig, type AxiosResponse } from "axios";

type ApiSuccess<X> = { status: number; data: X };
type ApiFailure = { status: number; error: string; data?: unknown };
export type ApiResult<X> = ApiSuccess<X> | ApiFailure;

function getToken(): string {
  return localStorage.getItem("token") ?? "";
}

async function request<X>(
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
  return request<X>({ method: "get", url }, expectedResponse);
}

export async function postCall<T, X>(
  url: string,
  body: T,
  expectedResponse: number,
) {
  return request<X>({ method: "post", url, data: body }, expectedResponse);
}
